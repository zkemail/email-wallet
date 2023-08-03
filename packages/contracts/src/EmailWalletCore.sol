// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/IVerifier.sol";
import "./interfaces/Types.sol";
import "./interfaces/Constants.sol";
import "./WalletHandler.sol";
import "./helpers/DKIMPublicKeyStorage.sol";

contract EmailWalletCore is WalletHandler, DKIMPublicKeyStorage {
    IVerifier public verifier;

    // Mapping of relayer's wallet address to hash(relayerRand)
    mapping(address => bytes32) public relayers;

    // Mapping from pointer (emailAddress commitment) to indicator (viewingKey commitment)
    mapping(bytes32 => bytes32) public indicatorOfPointer;

    // Flag to indicate whether an indicator is initialized
    mapping(bytes32 => bool) public isInitialized;

    // Flag to indicate whether an indicator is nullifed
    mapping(bytes32 => bool) public isNullifed;

    // Mapping to store email nullifiers
    mapping(bytes32 => bool) public emailNullifiers;

    constructor(address _verifier) {
        verifier = IVerifier(_verifier);
    }

    function getVerifier() external view returns (address) {
        return address(verifier);
    }

    function registerRelayer(bytes32 _relayerHash) public {
        require(
            relayers[msg.sender] == bytes32(0),
            "relayer already registered"
        );

        require(_relayerHash != bytes32(0), "relayer hash must not be zero");

        relayers[msg.sender] = _relayerHash;
    }

    function createAccount(
        bytes32 pointer,
        bytes32 indicator,
        bytes memory proof
    ) public {
        bytes32 relayerHash = relayers[msg.sender];
        require(relayerHash != bytes32(0), "relayer not registered");

        require(pointer != bytes32(0), "pointer must not be zero");
        require(indicator != bytes32(0), "indicator must not be zero");
        require(proof.length > 0, "proof must not be empty");

        require(
            indicatorOfPointer[pointer] == bytes32(0),
            "pointer already exists"
        );

        // Verify proof
        require(
            verifier.verifyAccountCreationProof(
                relayerHash,
                pointer,
                indicator,
                proof
            ),
            "invalid account creation proof"
        );

        indicatorOfPointer[pointer] = indicator;
    }

    function initializeAccount(
        bytes32 pointer,
        bytes32 indicator,
        bytes memory proof
    ) public {
        require(pointer != bytes32(0), "pointer must not be zero");
        require(indicator != bytes32(0), "indicator must not be zero");

        require(
            indicatorOfPointer[pointer] == indicator,
            "invalid pointer and indicator"
        );

        require(!isInitialized[indicator], "account already initialized");

        // This will prove that relayer received an email from the user as a reply
        // to invitation email:
        //  - the pointer is derived from senders address using relayer's rand
        //  - the indicator is derived from sender's viewing key using relayer's hash
        //  - the viewing key is present in the email header
        // TODO: Check if the recipient of the email is the relayer's email on chain?
        require(
            verifier.verifyAccountInitializaionProof(
                relayers[msg.sender],
                pointer,
                indicator,
                proof
            ),
            "invalid account creation proof"
        );

        isInitialized[indicator] = true;
    }

    // TODO: Case insensitive comparison
    function computeEmailSubjectForEmailOp(
        EmailOperation memory emailOp
    ) public pure returns (string memory expectedSubject) {
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
                    Strings.toHexString(
                        uint256(uint160(emailOp.recipientExternalAddress)),
                        20
                    )
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

    function validateEmailOp(EmailOperation memory emailOp) public view {
        bytes32 relayerHash = relayers[msg.sender];

        require(
            emailOp.senderPointer != bytes32(0),
            "sender pointer must not be zero"
        );
        require(
            indicatorOfPointer[emailOp.senderPointer] ==
                emailOp.senderIndicator,
            "invalid sender indicator"
        );
        require(
            !isNullifed[emailOp.senderIndicator],
            "relayer of sender pointer must be the sender's relayer"
        );
        require(
            isInitialized[emailOp.senderIndicator],
            "sender is not initialized"
        );

        // If the recipient is not presnet, then sender values should be used for recipient
        if (!emailOp.hasRecipient) {
            require(
                emailOp.recipientPointer == emailOp.senderPointer,
                "recipient pointer must be same as sender pointer"
            );
            require(
                emailOp.recipientIndicator == emailOp.senderIndicator,
                "recipient indicator must be same as sender indicator"
            );
            require(
                isInitialized[emailOp.recipientPointer],
                "recipient is not initialized"
            );
        }

        // If the recipient email is specified in the subject, verify the recipient's viewing key.
        if (emailOp.hasRecipient && !emailOp.isRecipientExternal) {
            require(
                emailOp.recipientPointer != bytes32(0),
                "recipient pointer must not be zero"
            );
            require(
                indicatorOfPointer[emailOp.recipientPointer] ==
                    emailOp.recipientIndicator,
                "invalid recipient indicator"
            );
            require(
                emailOp.recipientRelayer != address(0),
                "recipient relayer must not be zero address"
            );
            require(
                relayers[emailOp.recipientRelayer] != bytes32(0),
                "recipient relayer is not registered"
            );
        }

        // Verify nullifier is not used
        require(
            emailNullifiers[emailOp.emailNullifier] == false,
            "email nullifier already used"
        );

        // Verify DKIM key is same as the one stored in the contract
        require(
            keccak256(
                abi.encodePacked(dkimPublicKeyHashes[emailOp.domainName])
            ) == keccak256(abi.encodePacked(emailOp.dkimPublicKeyHash)),
            "invalid pubkey hash"
        );

        /**
            Verify email proof (proof of dkim verification)
            This will verify:
                - the email is sent from the sender's email address
                - sender pointer and indicator is computed from relayer hash
                - if the subject has email address, the pointer and indicator 
                    for relayer is calculated correctly
                - the amount and token name extracted from subject is correct
                - recipient pointer and indicator is computed from recipient relayer's hash
        */
        require(
            verifier.verifyEmailProof(
                relayerHash, // senderRelayerHash
                emailOp.senderPointer,
                emailOp.senderIndicator,
                relayers[emailOp.recipientRelayer], // recipientRelayerHash
                emailOp.recipientPointer,
                emailOp.recipientIndicator,
                emailOp.emailNullifier,
                emailOp.dkimPublicKeyHash,
                emailOp.domainName,
                emailOp.maskedSubjectStr,
                emailOp.hasRecipient,
                emailOp.isRecipientExternal,
                emailOp.emailProof
            ),
            "invalid email proof"
        );

        // Verify the EmailOp parms are properly derived from subject line from email
        require(
            Strings.equal(
                computeEmailSubjectForEmailOp(emailOp),
                emailOp.maskedSubjectStr
            ),
            "computed subject line does not match"
        );
    }

    function executeEmailOp(EmailOperation memory emailOp) public {
        validateEmailOp(emailOp);

        emailNullifiers[emailOp.emailNullifier] = true;

        if (Strings.equal(emailOp.command, Constants.SEND_COMMAND)) {
            WalletHandler._processTransferRequest(
                emailOp.senderPointer,
                emailOp.recipientPointer,
                emailOp.tokenName,
                emailOp.amount
            );
        }
    }

    function initializeDepositRequest(
        bytes32 depositCommitment,
        address tokenAddress,
        uint256 amount
    ) public {
        WalletHandler._initializeDepositRequest(
            depositCommitment,
            tokenAddress,
            amount
        );
    }

    function processDepositRequest(
        bytes32 depositCommitment,
        bytes32 recipientSalt,
        bytes memory depositProof
    ) public {
        WalletHandler._processDepositRequest(
            verifier,
            depositCommitment,
            recipientSalt,
            depositProof
        );
    }
}
