// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@zk-email/contracts/DKIMRegistry.sol";
import "./utils/TokenRegistry.sol";
import "./interfaces/IVerifier.sol";
import "./interfaces/IExtension.sol";
import "./interfaces/Types.sol";
import "./interfaces/Constants.sol";
import "./Wallet.sol";
import "./WalletHandler.sol";

contract EmailWalletCore is WalletHandler {
    // ZK proof verifier
    IVerifier public verifier;

    // DKIM public key hashes registry
    DKIMRegistry public immutable dkimRegistry;

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

    // Mapping of recipient's emailAddressCommitment (hash(email, randomness)) to the unclaimedFund
    // Note: The array only include funds from a single sender using the same randomness; not all the funds claimable by recipient user
    mapping(bytes32 => UnclaimedFund[]) public unclaimedFundOfRecipient;

    // Global mapping of command name to extension address
    mapping(string => address) public extensionOfCommand;

    // User level mapping of command name to extension address (pointer -> (command -> extension))
    mapping(bytes32 => mapping(string => address)) public userExtensionOfCommand;

    // Time in block count for a transfer to be refundable (for uninitialized recipient)
    uint256 public constant REFUND_PERIOD_IN_BLOCKS = 5 * 60 * 24 * 30; // 30 days (5 blocks per minute)

    event UnclaimedFundRegistered(
        bytes32 recipientEmailAddressCommitment,
        address indexed sender,
        uint256 expiryTime
    );

    constructor(
        address _verifier,
        address _tokenRegistry,
        address _dkimRegistry
    ) WalletHandler(_tokenRegistry) {
        verifier = IVerifier(_verifier);
        dkimRegistry = DKIMRegistry(_dkimRegistry);
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
                bytes32(dkimRegistry.getDKIMPublicKeyHash(emailDomain)),
                proof
            ),
            "invalid account creation proof"
        );

        initializedVKCommitments[viewingKeyCommitment] = true;
    }

    /// Calculate email subject based on paramteres of EmailOp
    /// TODO: Case insensitive comparison?
    function _computeEmailSubjectForEmailOp(
        EmailOperation memory emailOp
    ) internal view returns (string memory expectedSubject) {
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
        // Sample: Set extension for Swap as 0x1234...
        else if (Strings.equal(emailOp.command, Constants.SET_EXTENSION_COMMAND)) {
            expectedSubject = string.concat(
                Constants.SET_EXTENSION_COMMAND,
                " for ",
                emailOp.command,
                " as ",
                Strings.toHexString(uint256(uint160(emailOp.extensionAddress)), 20)
            );
        }
        // Sample: Remove extension for Swap
        else if (Strings.equal(emailOp.command, Constants.REMOVE_EXTENSION_COMMAND)) {
            expectedSubject = string.concat(
                Constants.REMOVE_EXTENSION_COMMAND,
                " for ",
                emailOp.command
            );
        }
        // TODO: Implement transport
        else if (Strings.equal(emailOp.command, Constants.TRANSPORT_COMMAND)) {
            // TODO: Implement transport
        }
        // The command is for an extension
        else {
            address extensionAddress = extensionOfCommand[emailOp.command];

            address userExtensionAddress = userExtensionOfCommand[
                emailOp.senderEmailAddressPointer
            ][emailOp.command];
            if (userExtensionAddress != address(0)) {
                extensionAddress = userExtensionAddress;
            }

            require(extensionAddress != address(0), "extension not registered");

            IExtension extension = IExtension(extensionAddress);
            expectedSubject = extension.computeEmailSubject(emailOp.extensionParams);
        }
    }

    /// Validate an EmailOperation - proof of email, proof of account, etc.
    /// @param emailOp EmailOperation to be validated
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
                bytes32(dkimRegistry.getDKIMPublicKeyHash(emailOp.emailDomain)),
                emailOp.emailProof
            ),
            "invalid email proof"
        );
    }

    /// Execute an EmailOperation
    /// @param emailOp EmailOperation to be executed
    function executeEmailOp(EmailOperation memory emailOp) external {
        _validateEmailOp(emailOp);

        emailNullifiers[emailOp.emailNullifier] = true;

        // Wallet operation
        if (Strings.equal(emailOp.command, Constants.SEND_COMMAND)) {
            // Transfer to external address
            if (emailOp.isRecipientExternal) {
                WalletHandler._processTransferRequest(
                    getAddressOfSalt(walletSaltOfPointer[emailOp.senderEmailAddressPointer]),
                    emailOp.recipientExternalAddress,
                    emailOp.tokenName,
                    emailOp.amount
                );
            }
            // Transfer to email wallet user if recipient is set
            else if (emailOp.recipientEmailAddressPointer != bytes32(0)) {
                WalletHandler._processTransferRequest(
                    getAddressOfSalt(walletSaltOfPointer[emailOp.senderEmailAddressPointer]),
                    getAddressOfSalt(walletSaltOfPointer[emailOp.recipientEmailAddressPointer]),
                    emailOp.tokenName,
                    emailOp.amount
                );
            }
            // Register unclaimed fund otherwise
            else {
                _registerUnclaimedFund(
                    emailOp.recipientEmailAddressCommitment,
                    emailOp.tokenName,
                    emailOp.amount,
                    getAddressOfSalt(walletSaltOfPointer[emailOp.senderEmailAddressPointer])
                );
            }
        }
        // Set custom extension for the user
        else if (Strings.equal(emailOp.command, Constants.SET_EXTENSION_COMMAND)) {
            userExtensionOfCommand[emailOp.senderEmailAddressPointer][emailOp.command] = emailOp
                .extensionAddress;
        }
        // Remove custom extension for the user
        else if (Strings.equal(emailOp.command, Constants.REMOVE_EXTENSION_COMMAND)) {
            userExtensionOfCommand[emailOp.senderEmailAddressPointer][emailOp.command] = address(0);
        }
        // Transport to a new relayer
        else if (Strings.equal(emailOp.command, Constants.TRANSPORT_COMMAND)) {
            // TODO: Implement transport command
        }
        // The command is for an extension
        else {
            address extensionAddress = extensionOfCommand[emailOp.command];

            address userExtensionAddress = userExtensionOfCommand[
                emailOp.senderEmailAddressPointer
            ][emailOp.command];

            if (userExtensionAddress != address(0)) {
                extensionAddress = userExtensionAddress;
            }

            require(extensionAddress != address(0), "extension not registered");

            IExtension extension = IExtension(extensionAddress);

            // Get the target and calldata from extension
            (address target, bytes memory data) = extension.getExecutionCalldata(
                emailOp.extensionParams
            );

            // Ask the wallet to execute the calldata
            WalletHandler._executeExtensionCalldata(
                getAddressOfSalt(walletSaltOfPointer[emailOp.senderEmailAddressPointer]),
                target,
                data
            );
        }
    }

    function _registerUnclaimedFund(
        bytes32 recipientEmailAddressPointer,
        string memory tokenName,
        uint256 amount,
        address senderAddress
    ) private {
        uint256 expiryTime = block.timestamp + 30 days;

        UnclaimedFund memory fund = UnclaimedFund({
            tokenName: tokenName,
            amount: amount,
            expiryTime: expiryTime,
            sender: senderAddress
        });

        unclaimedFundOfRecipient[recipientEmailAddressPointer].push(fund);

        emit UnclaimedFundRegistered(recipientEmailAddressPointer, senderAddress, expiryTime);
    }

    function registerUnclaimedFund(
        bytes32 recipientEmailAddressPointer,
        string memory tokenName,
        uint256 amount
    ) public {
        _registerUnclaimedFund(recipientEmailAddressPointer, tokenName, amount, msg.sender);
    }

    function claimUnclaimedFund(
        bytes32 recipientEmailAddressPointer,
        bytes32 recipientEmailAddressCommitment,
        bytes memory proof
    ) public {
        require(
            verifier.verifyClaimFundProof(
                relayers[msg.sender],
                recipientEmailAddressPointer,
                vkCommitmentOfPointer[recipientEmailAddressPointer],
                walletSaltOfPointer[recipientEmailAddressPointer],
                recipientEmailAddressCommitment,
                proof
            ),
            "invalid proof"
        );

        UnclaimedFund[] storage funds = unclaimedFundOfRecipient[recipientEmailAddressPointer];

        for (uint256 i = 0; i < funds.length; i++) {
            UnclaimedFund storage fund = funds[i];

            if (fund.expiryTime < block.timestamp) {
                delete funds[i];

                address recipientWallet = getAddressOfSalt(
                    walletSaltOfPointer[recipientEmailAddressPointer]
                );
                transferUnclaimedFund(fund, recipientWallet);
            }
        }
    }

    function transferUnclaimedFund(UnclaimedFund storage fund, address recipient) internal {
        if (Strings.equal(fund.tokenName, Constants.ETH_TOKEN_NAME)) {
            (bool sent, bytes memory data) = recipient.call{value: fund.amount}("");
            require(sent, "unclaimed eth send failed");
        } else {
            IERC20 token = IERC20(tokenRegistry.getTokenAddress(fund.tokenName));
            token.transfer(recipient, fund.amount);
        }
    }

    function refundUnclaimedFund(bytes32 recipientEmailAddressPointer) external {
        UnclaimedFund[] storage funds = unclaimedFundOfRecipient[recipientEmailAddressPointer];

        for (uint256 i = 0; i < funds.length; i++) {
            UnclaimedFund storage fund = funds[i];

            if (fund.expiryTime > block.timestamp) {
                delete funds[i];
                transferUnclaimedFund(fund, fund.sender); // setting sender as recipient
            }
        }
    }
}
