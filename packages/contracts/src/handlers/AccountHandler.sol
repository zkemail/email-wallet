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
    mapping(bytes => bytes32) public walletSaltOfPSIPoint;

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
    ) public initializer {
        __Ownable_init();
        deployer = _msgSender();
        emailValidityDuration = _emailValidityDuration;
        defaultDkimRegistry = IDKIMRegistry(_defaultDkimRegistry);
        walletImplementation = _walletImplementation;
        relayerHandler = RelayerHandler(_relayerHandler);
        verifier = IVerifier(_verifier);
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyDeployer {}

    /// Create new account and wallet for a user
    /// @param walletSalt Wallet salt used to deploy the wallet - hash(emailAddr, accountSalt)
    /// @param psiPoint PSI point of the user under the relayer
    /// @param emailTimestamp Timestamp when the email was sent
    /// @param emailNullifier Nullifier of the email to prevent replay attacks
    /// @param emailDomain Domain name of the user's email
    /// @param dkimPublicKeyHash DKIM public key hash of the email domain used in the proof generation
    /// @param proof ZK proof as required by the verifier
    function createAccount(
        bytes32 walletSalt,
        bytes calldata psiPoint,
        uint256 emailTimestamp,
        bytes32 emailNullifier,
        string calldata emailDomain,
        bytes32 dkimPublicKeyHash,
        bytes calldata proof
    ) public returns (Wallet wallet) {
        require(walletSalt != bytes32(0), "invalid wallet salt");
        require(walletSaltOfPSIPoint[psiPoint] == bytes32(0), "PSI point exists");
        require(Address.isContract(getWalletOfSalt(walletSalt)) == false, "wallet already deployed");
        require(emailNullifiers[emailNullifier] == false, "email already nullified");
        require(isDKIMPublicKeyHashValid(walletSalt, emailDomain, dkimPublicKeyHash), "invalid DKIM public key hash");
        require(emailTimestamp + emailValidityDuration > block.timestamp, "email expired");

        require(
            verifier.verifyAccountCreationProof(
                walletSalt,
                psiPoint,
                emailDomain,
                dkimPublicKeyHash,
                emailTimestamp,
                emailNullifier,
                proof
            ),
            "invalid account creation proof"
        );

        walletSaltOfPSIPoint[psiPoint] = walletSalt;
        emailNullifiers[emailNullifier] = true;

        wallet = _deployWallet(walletSalt);

        emit EmailWalletEvents.AccountCreated(walletSalt, psiPoint);
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
