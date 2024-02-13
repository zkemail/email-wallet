// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IDKIMRegistry} from "@zk-email/contracts/interfaces/IDKIMRegistry.sol";
import {Create2Upgradeable} from "@openzeppelin/contracts-upgradeable/utils/Create2Upgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Wallet} from "../Wallet.sol";
import {RelayerHandler} from "./RelayerHandler.sol";
import {IVerifier} from "../interfaces/IVerifier.sol";
import "../interfaces/Types.sol";
import "../interfaces/Events.sol";

contract AccountHandler is Initializable, UUPSUpgradeable, OwnableUpgradeable {
    // Default DKIM public key hashes registry
    IDKIMRegistry public defaultDkimRegistry;

    // Address of wallet implementation contract - used for deploying wallets for users via proxy
    address public walletImplementation;

    // Relayer handler contract
    RelayerHandler public relayerHandler;

    // ZK proof verifier contract
    IVerifier public verifier;

    // Deployer
    address private deployer;


    // Mapping of PSI point to emailAddrPointer
    mapping(bytes => bytes32) public pointerOfPSIPoint;

    // Mapping of emailAddrPointer to account key details
    mapping(bytes32 => AccountKeyInfo) public infoOfEmailAddr;

    // Mapping of walletSalt to dkim registry address
    mapping(bytes32 => address) public dkimRegistryOfWalletSalt;

    // Mapping to store nullifiers of initialization and transport emails
    mapping(bytes32 => bool) public emailNullifiers;

    // Duration for which an email is valid
    uint public emailValidityDuration;

    modifier onlyDeployer() {
        require(msg.sender == deployer, "caller is not a deployer");
        _;
    }

    constructor() {
        _disableInitializers();
    }

    function initialize(
        address _relayerHandler,
        address _defaultDkimRegistry,
        address _verifier,
        address _walletImplementation,
        uint _emailValidityDuration
    ) initializer public {
        __Ownable_init();
        deployer = _msgSender();
        emailValidityDuration = _emailValidityDuration;
        defaultDkimRegistry = IDKIMRegistry(_defaultDkimRegistry);
        walletImplementation = _walletImplementation;
        relayerHandler = RelayerHandler(_relayerHandler);
        verifier = IVerifier(_verifier);
    }

    function _authorizeUpgrade(address newImplementation)
        internal
        onlyDeployer
        override
    {}

    /// Create new account and wallet for a user
    /// @param emailAddr hash(emailAddr)
    /// @param walletSalt hash(accountKey, 0)
    /// @param emailDomain domain name of the sender's email
    /// @param emailTimestamp timestamp of the current block
    /// @param emailNullifier nullifier of the email used for proof generation
    /// @param dkimPublicKeyHash DKIM public key hash of the email domain used in the proof generation
    /// @param proof ZK proof as required by the verifier
    function createAccount(
        bytes32 emailAddr,
        bytes32 walletSalt,
        bytes calldata psiPoint,
        string calldata emailDomain,
        uint256 emailTimestamp,
        bytes32 emailNullifier,
        bytes32 dkimPublicKeyHash,
        bytes calldata proof
    ) public returns (Wallet wallet) {
        require(pointerOfPSIPoint[psiPoint] == bytes32(0), "PSI point exists");
        require(Address.isContract(getWalletOfSalt(walletSalt)) == false, "wallet already deployed");

        require(
            verifier.verifyAccountCreationProof(
                emailAddr,
                walletSalt,
                psiPoint,
                proof
            ),
            "invalid account creation proof"
        );

        infoOfEmailAddr[emailAddr].relayer = msg.sender;
        infoOfEmailAddr[emailAddr].walletSalt = walletSalt;
        pointerOfPSIPoint[psiPoint] = emailAddr;

        wallet = _deployWallet(walletSalt);

        emit EmailWalletEvents.AccountCreated(emailAddr, walletSalt, walletSalt, psiPoint);


        // initialize account
        require(walletSalt != bytes32(0), "account not registered");
        require(infoOfEmailAddr[emailAddr].relayer == msg.sender, "invalid relayer");
        require(infoOfEmailAddr[emailAddr].initialized == false, "account already initialized");
        require(emailNullifiers[emailNullifier] == false, "email nullified");
        require(emailTimestamp + emailValidityDuration > block.timestamp, "email expired");
        require(
            isDKIMPublicKeyHashValid(
                infoOfEmailAddr[emailAddr].walletSalt,
                emailDomain,
                dkimPublicKeyHash
            ),
            "invalid DKIM public key hash"
        );
        infoOfEmailAddr[emailAddr].initialized = true;
        emailNullifiers[emailNullifier] = true;

        emit EmailWalletEvents.AccountInitialized(
            emailAddr,
            walletSalt,
            infoOfEmailAddr[emailAddr].walletSalt
        );
    }

    /// Initialize the account when user reply to invitation email
    /// @param emailAddr hash(emailAddr)
    /// @param emailDomain domain name of the sender's email
    /// @param emailNullifier nullifier of the email used for proof generation
    /// @param dkimPublicKeyHash DKIM public key hash of the email domain used in the proof generation
    /// @param walletSalt hash(accountKey, 0)
    function initializeAccount(
        bytes32 emailAddr,
        string calldata emailDomain,
        uint256 emailTimestamp,
        bytes32 emailNullifier,
        bytes32 dkimPublicKeyHash,
        bytes32 walletSalt
    ) public {        
        // bytes32 walletSalt = walletSaltOfPointer[emailAddrPointer];

        require(walletSalt != bytes32(0), "account not registered");
        require(infoOfEmailAddr[emailAddr].relayer == msg.sender, "invalid relayer");
        require(infoOfEmailAddr[emailAddr].initialized == false, "account already initialized");
        require(emailNullifiers[emailNullifier] == false, "email nullified");
        require(emailTimestamp + emailValidityDuration > block.timestamp, "email expired");
        require(
            isDKIMPublicKeyHashValid(
                infoOfEmailAddr[emailAddr].walletSalt,
                emailDomain,
                dkimPublicKeyHash
            ),
            "invalid DKIM public key hash"
        );

        // It's not necessary because account initialization function is included in account creation.
        // TODO: Remove this commented function later.
        // require(
        //     verifier.verifyAccountInitializaionProof(
        //         emailDomain,
        //         dkimPublicKeyHash,
        //         emailTimestamp,
        //         emailAddrPointer,
        //         walletSalt,
        //         emailNullifier,
        //         proof
        //     ),
        //     "invalid account initialization proof"
        // );

        infoOfEmailAddr[emailAddr].initialized = true;
        emailNullifiers[emailNullifier] = true;

        emit EmailWalletEvents.AccountInitialized(
            emailAddr,
            walletSalt,
            infoOfEmailAddr[emailAddr].walletSalt
        );
    }

    /// @notice Return the DKIM public key hash for a given email domain and walletSalt
    /// @param walletSalt Salt used to deploy the wallet
    /// @param emailDomain Email domain for which the DKIM public key hash is to be returned
    function isDKIMPublicKeyHashValid(
        bytes32 walletSalt,
        string memory emailDomain,
        bytes32 publicKeyHash
    ) public view returns (bool) {
        address dkimRegistry = dkimRegistryOfWalletSalt[walletSalt];

        if (dkimRegistry == address(0)) {
            dkimRegistry = address(defaultDkimRegistry);
        }

        return IDKIMRegistry(dkimRegistry).isDKIMPublicKeyHashValid(emailDomain, publicKeyHash);
    }

    // TODO RENAMEME
    /// @notice Return the info of an account key commitment
    /// @param accountKeyCommit Account key commitment for which the info is to be returned
    function getInfoOfAccountKeyCommit(bytes32 accountKeyCommit) public view returns (AccountKeyInfo memory) {
        return infoOfEmailAddr[accountKeyCommit];
    }

    /// @notice Update teh DKIM registry address for a given wallet salt
    /// @param walletSalt Salt used to deploy the wallet
    /// @param dkimRegistry Address of the DKIM registry
    function updateDKIMRegistryOfWalletSalt(bytes32 walletSalt, address dkimRegistry) public onlyOwner {
        dkimRegistryOfWalletSalt[walletSalt] = dkimRegistry;
    }

    /// @notice Return the wallet address of the user given the salt
    /// @param salt Salt used to deploy the wallet
    function getWalletOfSalt(bytes32 salt) public view returns (address) {
        return
            Create2Upgradeable.computeAddress(
                salt,
                keccak256(
                    abi.encodePacked(
                        type(ERC1967Proxy).creationCode,
                        abi.encode(address(walletImplementation), abi.encodeCall(Wallet.initialize, ()))
                    )
                )
            );
    }

    // TODO: Remove this commented function later.
    // /// @notice Return the wallet address of the user given the email address pointer
    // /// @param emailAddrPointer Email address pointer of the user
    // function getWalletOfEmailAddrPointer(bytes32 emailAddrPointer) public view returns (address) {
    //     return getWalletOfSalt(infoOfEmailAddrPointer[walletSaltOfPointer[emailAddrPointer]].walletSalt);
    // }

    /// @notice Deploy a wallet contract with the given salt
    /// @param salt Salt to be used for wallet deployment
    /// @dev We are deploying a deterministic proxy contract with the wallet implementation as the target.
    function _deployWallet(bytes32 salt) internal returns (Wallet wallet) {
        wallet = Wallet(
            payable(
                new ERC1967Proxy{salt: bytes32(salt)}(
                    address(walletImplementation),
                    abi.encodeCall(Wallet.initialize, ())
                )
            )
        );

        wallet.transferOwnership(owner()); // Transfer ownership to core
    }
}
