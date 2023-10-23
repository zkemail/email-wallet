// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/access/Ownable.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {IDKIMRegistry} from "@zk-email/contracts/interfaces/IDKIMRegistry.sol";
import {Create2Upgradeable} from "@openzeppelin/contracts-upgradeable/utils/Create2Upgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Wallet} from "../Wallet.sol";
import {RelayerHandler} from "./RelayerHandler.sol";
import {IVerifier} from "../interfaces/IVerifier.sol";
import "../interfaces/Types.sol";
import "../interfaces/Events.sol";

contract AccountHandler is Ownable {
    // Default DKIM public key hashes registry
    IDKIMRegistry public immutable defaultDkimRegistry;

    // Address of wallet implementation contract - used for deploying wallets for users via proxy
    address public immutable walletImplementation;

    // Relayer handler contract
    RelayerHandler public immutable relayerHandler;

    // ZK proof verifier contract
    IVerifier public immutable verifier;

     // Mapping of emailAddrPointer to accountKeyCommit
    mapping(bytes32 => bytes32) public accountKeyCommitOfPointer;

    // Mapping of PSI point to emailAddrPointer
    mapping(bytes => bytes32) public pointerOfPSIPoint;

    // Mapping of accountKeyCommit to account key details
    mapping(bytes32 => AccountKeyInfo) public infoOfAccountKeyCommit;

    // Mapping of walletSalt to dkim registry address
    mapping(bytes32 => address) public dkimRegistryOfWalletSalt;

    // Duration for which an email is valid
    uint public immutable emailValidityDuration;

    constructor(
        address _relayerHandler,
        address _defaultDkimRegistry,
        address _verifier,
        address _walletImplementation,
        uint _emailValidityDuration
    ) {
        emailValidityDuration = _emailValidityDuration;
        defaultDkimRegistry = IDKIMRegistry(_defaultDkimRegistry);
        walletImplementation = _walletImplementation;
        relayerHandler = RelayerHandler(_relayerHandler);
        verifier = IVerifier(_verifier);
    }

    /// Create new account and wallet for a user
    /// @param emailAddrPointer hash(relayerRand, emailAddr)
    /// @param accountKeyCommit hash(accountKey, emailAddr, relayerHash)
    /// @param walletSalt hash(accountKey, 0)
    /// @param proof ZK proof as required by the verifier
    function createAccount(
        address relayer,
        bytes32 emailAddrPointer,
        bytes32 accountKeyCommit,
        bytes32 walletSalt,
        bytes calldata psiPoint,
        bytes calldata proof
    ) public {
        require(relayerHandler.getRandHash(relayer) != bytes32(0), "relayer not registered");
        require(accountKeyCommitOfPointer[emailAddrPointer] == bytes32(0), "pointer exists");
        require(pointerOfPSIPoint[psiPoint] == bytes32(0), "PSI point exists");
        require(infoOfAccountKeyCommit[accountKeyCommit].walletSalt == bytes32(0), "walletSalt exists");
        require(Address.isContract(getWalletOfSalt(walletSalt)) == false, "wallet already deployed");

        require(
            verifier.verifyAccountCreationProof(
                relayerHandler.getRandHash(relayer),
                emailAddrPointer,
                accountKeyCommit,
                walletSalt,
                psiPoint,
                proof
            ),
            "invalid account creation proof"
        );

        accountKeyCommitOfPointer[emailAddrPointer] = accountKeyCommit;
        infoOfAccountKeyCommit[accountKeyCommit].relayer = relayer;
        infoOfAccountKeyCommit[accountKeyCommit].walletSalt = walletSalt;
        pointerOfPSIPoint[psiPoint] = emailAddrPointer;

        emit EmailWalletEvents.AccountCreated(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint);
    }

    /// Initialize the account when user reply to invitation email
    /// @param emailAddrPointer hash(relayerRand, emailAddr)
    /// @param emailDomain domain name of the sender's email
    /// @param emailNullifier nullifier of the email used for proof generation
    /// @param proof ZK proof as required by the verifier
    function initializeAccount(
        address relayer,
        bytes32 emailAddrPointer,
        string calldata emailDomain,
        uint256 emailTimestamp,
        bytes32 emailNullifier,
        bytes calldata proof
    ) public {
        bytes32 accountKeyCommit = accountKeyCommitOfPointer[emailAddrPointer];
        bytes32 relayerRandHash = relayerHandler.getRandHash(relayer);

        require(emailTimestamp + emailValidityDuration > block.timestamp, "email expired");
        require(relayerRandHash != bytes32(0), "relayer not registered");
        require(accountKeyCommit != bytes32(0), "account not registered");
        require(infoOfAccountKeyCommit[accountKeyCommit].relayer == relayer, "invalid relayer");
        require(infoOfAccountKeyCommit[accountKeyCommit].initialized == false, "account already initialized");

        require(
            verifier.verifyAccountInitializaionProof(
                emailDomain,
                getDKIMPublicKeyHash(infoOfAccountKeyCommit[accountKeyCommit].walletSalt, emailDomain),
                emailTimestamp,
                relayerRandHash,
                emailAddrPointer,
                accountKeyCommit,
                emailNullifier,
                proof
            ),
            "invalid account initialization proof"
        );

        infoOfAccountKeyCommit[accountKeyCommit].initialized = true;

        emit EmailWalletEvents.AccountInitialized(
            emailAddrPointer,
            accountKeyCommit,
            infoOfAccountKeyCommit[accountKeyCommit].walletSalt
        );
    }

    /// @notice Transport an account to a new Relayer - to be called by the new relayer
    /// @param oldAccountKeyCommit Account Key commitment of the account under old (current) relayer
    /// @param newEmailAddrPointer Email Address pointer of the account under new relayer
    /// @param newAccountKeyCommit Account Key commitment of the account under new relayer
    /// @param newPSIPoint PSI point of the email address under new relayer
    /// @param accountCreationProof Proof for new account creation under new relayer
    /// @param transportEmailProof Proof of user's transport email
    function transportAccount(
        address relayer,
        bytes32 oldAccountKeyCommit,
        bytes32 newEmailAddrPointer,
        bytes32 newAccountKeyCommit,
        bytes memory newPSIPoint,
        EmailProof memory transportEmailProof,
        bytes memory accountCreationProof
    ) public {
        require(relayerHandler.getRandHash(relayer) != bytes32(0), "relayer not registered");
        require(infoOfAccountKeyCommit[oldAccountKeyCommit].relayer != address(0), "old relayer not registered");
        require(infoOfAccountKeyCommit[oldAccountKeyCommit].relayer != relayer, "new relayer cannot be same");
        require(infoOfAccountKeyCommit[oldAccountKeyCommit].initialized, "account not initialized");
        require(transportEmailProof.timestamp + emailValidityDuration > block.timestamp, "email expired");

        // New relayer might have already created an account, but not initialized.
        if (accountKeyCommitOfPointer[newEmailAddrPointer] == bytes32(0)) {
            require(!infoOfAccountKeyCommit[newAccountKeyCommit].initialized, "new account is already initialized");
            require(pointerOfPSIPoint[newPSIPoint] == bytes32(0), "new PSI point already exists");
        }

        require(
            verifier.verifyAccountCreationProof(
                relayerHandler.getRandHash(relayer),
                newEmailAddrPointer,
                newAccountKeyCommit,
                infoOfAccountKeyCommit[oldAccountKeyCommit].walletSalt,
                newPSIPoint,
                accountCreationProof
            ),
            "invalid account creation proof"
        );

        require(
            verifier.verifyAccountTransportProof(
                transportEmailProof.domain,
                getDKIMPublicKeyHash(
                    infoOfAccountKeyCommit[oldAccountKeyCommit].walletSalt,
                    transportEmailProof.domain
                ),
                transportEmailProof.timestamp,
                transportEmailProof.nullifier,
                relayerHandler.getRandHash(infoOfAccountKeyCommit[oldAccountKeyCommit].relayer),
                relayerHandler.getRandHash(relayer),
                oldAccountKeyCommit,
                newAccountKeyCommit,
                transportEmailProof.proof
            ),
            "invalid account transport proof"
        );

        if (accountKeyCommitOfPointer[newEmailAddrPointer] != bytes32(0)) {
            delete infoOfAccountKeyCommit[accountKeyCommitOfPointer[newEmailAddrPointer]];
        }

        accountKeyCommitOfPointer[newEmailAddrPointer] = newAccountKeyCommit;
        pointerOfPSIPoint[newPSIPoint] = newEmailAddrPointer;
        infoOfAccountKeyCommit[newAccountKeyCommit].walletSalt = infoOfAccountKeyCommit[oldAccountKeyCommit].walletSalt;
        infoOfAccountKeyCommit[newAccountKeyCommit].relayer = relayer;
        infoOfAccountKeyCommit[newAccountKeyCommit].initialized = true;

        infoOfAccountKeyCommit[oldAccountKeyCommit].walletSalt = bytes32(0);

        emit EmailWalletEvents.AccountTransported(oldAccountKeyCommit, newEmailAddrPointer, newAccountKeyCommit);
    }

    /// @notice Return the DKIM public key hash for a given email domain and walletSalt
    /// @param walletSalt Salt used to deploy the wallet
    /// @param emailDomain Email domain for which the DKIM public key hash is to be returned
    function getDKIMPublicKeyHash(bytes32 walletSalt, string memory emailDomain) public view returns (bytes32) {
        IDKIMRegistry dkimRegistry = defaultDkimRegistry;

        if (dkimRegistryOfWalletSalt[walletSalt] != address(0)) {
            dkimRegistry = IDKIMRegistry(dkimRegistryOfWalletSalt[walletSalt]);
        }

        return dkimRegistry.getDKIMPublicKeyHash(emailDomain);
    }

    /// @notice Return the info of an account key commitment
    /// @param accountKeyCommit Account key commitment for which the info is to be returned
    function getInfoOfAccountKeyCommit(bytes32 accountKeyCommit) public view returns (AccountKeyInfo memory) {
        return infoOfAccountKeyCommit[accountKeyCommit];
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
                ),
                owner() // Core deploys the wallets
            );
    }

    /// @notice Return the wallet address of the user given the email address pointer
    /// @param emailAddrPointer Email address pointer of the user
    function getWalletOfEmailAddrPointer(bytes32 emailAddrPointer) public view returns (address) {
        return getWalletOfSalt(infoOfAccountKeyCommit[accountKeyCommitOfPointer[emailAddrPointer]].walletSalt);
    }
}
