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
    mapping(bytes => bytes32) public accountSaltOfPSIPoint;

    // Mapping of accountSalt to dkim registry address
    mapping(bytes32 => address) public dkimRegistryOfAccountSalt;

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

    /// @notice Register a PSI point before deploying the wallet in `createAccount`.
    /// @param psiPoint PSI point of the user
    /// @param accountSalt Wallet salt used to deploy the wallet - hash(emailAddr, accountSalt)
    function registerPSIPoint(bytes memory psiPoint, bytes32 accountSalt, bytes calldata proof) public {
        require(accountSalt != bytes32(0), "invalid wallet salt");
        require(accountSaltOfPSIPoint[psiPoint] == bytes32(0), "PSI point exists");
        (string memory relayerEmailAddr, ) = relayerHandler.relayers(msg.sender);
        require(bytes(relayerEmailAddr).length != 0, "caller is not a relayer");
        require(verifier.verifyPsiPointProof(accountSalt, psiPoint, proof), "invalid PSI point proof");
        accountSaltOfPSIPoint[psiPoint] = accountSalt;
    }

    /// @notice Create new account and wallet for a user
    /// @param emailProof Proof and instances of the email proof
    function createAccount(EmailProof calldata emailProof) public returns (Wallet wallet) {
        require(emailProof.accountSalt != bytes32(0), "invalid wallet salt");
        require(Address.isContract(getWalletOfSalt(emailProof.accountSalt)) == false, "wallet already deployed");
        require(emailNullifiers[emailProof.emailNullifier] == false, "email already nullified");
        require(
            isDKIMPublicKeyHashValid(emailProof.accountSalt, emailProof.emailDomain, emailProof.dkimPublicKeyHash),
            "invalid DKIM public key hash"
        );
        require(emailProof.isCodeExist == true, "the invitation code must exist");

        (string memory relayerEmailAddr, ) = relayerHandler.relayers(msg.sender);
        require(bytes(relayerEmailAddr).length != 0, "caller is not a relayer");

        if (emailProof.timestamp != 0) {
            require(emailProof.timestamp + emailValidityDuration > block.timestamp, "email expired");
        }

        require(verifier.verifyEmailProof(emailProof), "invalid account creation proof");
        emailNullifiers[emailProof.emailNullifier] = true;

        wallet = _deployWallet(emailProof.accountSalt);

        emit EmailWalletEvents.AccountCreated(emailProof.accountSalt, emailProof.emailNullifier, address(wallet));
    }

    /// @notice Return true iff the wallet is deployed for the given wallet salt
    /// @param accountSalt Salt used to deploy the wallet
    function isAccountSaltDeployed(bytes32 accountSalt) public view returns (bool) {
        return Address.isContract(getWalletOfSalt(accountSalt));
    }

    /// @notice Return the DKIM public key hash for a given email domain and accountSalt
    /// @param accountSalt Salt used to deploy the wallet
    /// @param emailDomain Email domain for which the DKIM public key hash is to be returned
    function isDKIMPublicKeyHashValid(
        bytes32 accountSalt,
        string memory emailDomain,
        bytes32 publicKeyHash
    ) public view returns (bool) {
        address dkimRegistry = dkimRegistryOfAccountSalt[accountSalt];

        if (dkimRegistry == address(0)) {
            dkimRegistry = address(defaultDkimRegistry);
        }

        return IDKIMRegistry(dkimRegistry).isDKIMPublicKeyHashValid(emailDomain, publicKeyHash);
    }

    /// @notice Update teh DKIM registry address for a given wallet salt
    /// @param accountSalt Salt used to deploy the wallet
    /// @param dkimRegistry Address of the DKIM registry
    function updateDKIMRegistryOfAccountSalt(bytes32 accountSalt, address dkimRegistry) public onlyOwner {
        dkimRegistryOfAccountSalt[accountSalt] = dkimRegistry;
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
