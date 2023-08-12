// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/utils/Create2.sol";
import "./interfaces/IVerifier.sol";
import "./interfaces/Types.sol";
import "./interfaces/Constants.sol";
import "./WalletHandler.sol";
import "./helpers/DKIMPublicKeyStorage.sol";

contract EmailWalletCore is WalletHandler, DKIMPublicKeyStorage {
    // ZK proof verifier
    IVerifier public verifier;

    // Mapping of relayer's wallet address to hash(relayerRand)
    mapping(address => bytes32) public relayers;

    // Mapping of emailAddressPointer to viewingKeyCommitment
    mapping(bytes32 => bytes32) public vkCommitmentOfPointer;

    // Mapping of emailAddressPointer to walletSalt
    mapping(bytes32 => bytes32) public walletSaltOfPointer;

    // Flag to indicate whether viewingKeyCommitment is initialized
    mapping(bytes32 => bool) public initializedVKCommitments;

    // Flag to indicate whether viewingKeyCommitment is nullifed
    mapping(bytes32 => bool) public nullifiedVKCommitments;

    // Mapping to store email nullifiers
    mapping(bytes32 => bool) public emailNullifiers;

    constructor(address _verifier) {
        verifier = IVerifier(_verifier);
    }

    function getVerifier() external view returns (address) {
        return address(verifier);
    }

    /// Register as a relayer
    /// @param relayerHash hash of relayed private randomness `relayerRand`
    function registerRelayer(bytes32 relayerHash) public {
        require(relayers[msg.sender] == bytes32(0), "relayer already registered");

        require(relayerHash != bytes32(0), "relayerHash must not be zero");

        relayers[msg.sender] = relayerHash;
    }

    /// Create new account and wallet for a user
    /// @param emailAddressPointer hash(relayerRand, emailAddress)
    /// @param viewingKeyCommitment hash(viewingKey, emailAddress, relayerHash)
    /// @param walletSalt hash(viewingKey, 0)
    /// @param proof ZK proof as required by the verifier
    function createAccount(
        bytes32 emailAddressPointer,
        bytes32 viewingKeyCommitment,
        bytes32 walletSalt,
        bytes memory proof
    ) public returns (address) {
        require(vkCommitmentOfPointer[emailAddressPointer] == bytes32(0), "pointer already exists");

        require(
            verifier.verifyAccountCreationProof(
                relayers[msg.sender],
                emailAddressPointer,
                viewingKeyCommitment,
                walletSalt,
                proof
            ),
            "invalid account creation proof"
        );

        vkCommitmentOfPointer[emailAddressPointer] = viewingKeyCommitment;
        walletSaltOfPointer[emailAddressPointer] = walletSalt;

        return _deployWallet(walletSalt);
    }

    /// Initialize the account when user reply to invitation email
    /// @param emailAddressPointer hash(relayerRand, emailAddress)
    /// @param viewingKeyCommitment hash(viewingKey, emailAddress, relayerHash)
    /// @param emailDomain domain name of the sender's email
    /// @param proof ZK proof as required by the verifier
    function initializeAccount(
        bytes32 emailAddressPointer,
        bytes32 viewingKeyCommitment,
        string memory emailDomain,
        bytes memory proof
    ) public {
        require(
            vkCommitmentOfPointer[emailAddressPointer] == viewingKeyCommitment,
            "invalid viewingKeyCommitment"
        );

        require(!initializedVKCommitments[viewingKeyCommitment], "account already initialized");

        require(
            verifier.verifyAccountInitializaionProof(
                relayers[msg.sender],
                emailAddressPointer,
                viewingKeyCommitment,
                emailDomain,
                dkimPublicKeyHashes[emailDomain],
                proof
            ),
            "invalid account creation proof"
        );

        initializedVKCommitments[viewingKeyCommitment] = true;
    }

    // TODO: Case insensitive comparison?
    function _computeEmailSubjectForEmailOp(
        EmailOperation memory emailOp
    ) internal pure returns (string memory expectedSubject) {
        // Sample: Send 1 ETH to recipient@domain.com
        if (Strings.equal(emailOp.command, Constants.SEND_COMMAND)) {
            expectedSubject = string.concat(
                Constants.SEND_COMMAND,
                " ",
                Strings.toString(emailOp.amount / 1 ether),
                " ",
                emailOp.tokenName,
                " to "
            );

            if (emailOp.isRecipientExternal) {
                expectedSubject = string.concat(
                    expectedSubject,
                    Strings.toHexString(uint256(uint160(emailOp.recipientExternalAddress)), 20)
                );
            }
        }

        // Sample: Transport account to new.relayer@domain.com
        // if (Strings.equal(emailOp.command, Constants.TRANSPORT_COMMAND)) {
        //     expectedSubject = string.concat(
        //         Constants.TRANSPORT_COMMAND,
        //         " account to ",
        //         emailOp.newRelayerEmail
        //     );
        // }

        // TODO: Implement subject computation for transport, ext management, ext calling, etc.
    }

    function _validateEmailOp(EmailOperation memory emailOp) internal view {
        bytes32 senderRelayerHash = relayers[msg.sender];

        // Verify email is not nullified
        require(emailNullifiers[emailOp.emailNullifier] == false, "email nullifier already used");

        // Verify the EmailOp parms are properly derived from subject line from email
        require(
            Strings.equal(_computeEmailSubjectForEmailOp(emailOp), emailOp.maskedSubjectStr),
            "computed subject mismatch"
        );

        bytes32 viewingKeyCommitment = vkCommitmentOfPointer[emailOp.senderEmailAddressPointer];

        require(initializedVKCommitments[viewingKeyCommitment], "account not initialized");

        require(!nullifiedVKCommitments[viewingKeyCommitment], "account is nullified");

        // If the recipient email is specified in the subject, verify the recipient's account
        if (emailOp.hasRecipient && !emailOp.isRecipientExternal) {
            bytes32 recipientRelayerHash = relayers[emailOp.recipientRelayer];

            require(recipientRelayerHash != bytes32(0), "recipient relayer not registered");

            // Verify recipient account proof
            require(
                verifier.verifyRecipientAccountProof(
                    recipientRelayerHash,
                    emailOp.recipientEmailAddressPointer,
                    vkCommitmentOfPointer[emailOp.recipientEmailAddressPointer],
                    walletSaltOfPointer[emailOp.recipientEmailAddressPointer],
                    emailOp.recipientEmailAddressWitness,
                    emailOp.recipientAccountProof
                ),
                "invalid recipient account proof"
            );
        }

        // Verify senders account and email proof
        require(
            verifier.verifyEmailProof(
                senderRelayerHash,
                emailOp.senderEmailAddressPointer,
                vkCommitmentOfPointer[emailOp.senderEmailAddressPointer],
                emailOp.hasRecipient,
                emailOp.isRecipientExternal,
                emailOp.recipientEmailAddressWitness,
                emailOp.maskedSubjectStr,
                emailOp.emailNullifier,
                emailOp.emailDomain,
                dkimPublicKeyHashes[emailOp.emailDomain],
                emailOp.emailProof
            ),
            "invalid email proof"
        );
    }

    function executeEmailOp(EmailOperation memory emailOp) external {
        _validateEmailOp(emailOp);

        emailNullifiers[emailOp.emailNullifier] = true;

        if (Strings.equal(emailOp.command, Constants.SEND_COMMAND)) {
            WalletHandler._processTransferRequest(
                walletSaltOfPointer[emailOp.senderEmailAddressPointer],
                walletSaltOfPointer[emailOp.recipientEmailAddressPointer],
                emailOp.tokenName,
                emailOp.amount
            );
        }
    }

    // function processRefunds(uint256 startBlock, uint256 endBlock) public {
    //     require(startBlock <= block.number, "invalid start block");
    //     require(endBlock <= block.number, "invalid end block");
    //     require(startBlock <= endBlock, "invalid block range");

    //     for (uint256 i = startBlock; i <= endBlock; i++) {
    //         TransferNote[] storage transfers = refundableTransfersAfterBlock[i];

    //         for (uint256 j = 0; j < transfers.length; j++) {
    //             TransferNote memory transfer = transfers[j];

    //             if (!isInitialized[indicatorOfPointer[pointerOfAddress[transfer.recipient]]]) {
    //                 WalletHandler._processTransferRequest(
    //                     saltOfAddress[transfer.sender],
    //                     saltOfAddress[transfer.recipient],
    //                     getTokenName(transfer.token),
    //                     transfer.amount
    //                 );
    //             }
    //         }
    //     }
    // }
}
