// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@zk-email/contracts/DKIMRegistry.sol";
import "./utils/TokenRegistry.sol";
import "./interfaces/IVerifier.sol";
import "./interfaces/IExtension.sol";
import "./interfaces/Types.sol";
import "./interfaces/Constants.sol";
import "./Wallet.sol";
import "./handlers/WalletHandler.sol";

contract EmailWalletCore is WalletHandler {
    // ZK proof verifier
    IVerifier public verifier;

    // DKIM public key hashes registry
    DKIMRegistry public immutable dkimRegistry;

    // Mapping of relayer's wallet address to relayer config
    mapping(address => RelayerConfig) public relayers;

    // Mapping of emailAddressPointer to viewingKeyCommitment
    mapping(bytes32 => bytes32) public vkCommitmentOfPointer;

    // Mapping of emailAddressPointer to walletSalt
    mapping(bytes32 => bytes32) public walletSaltOfVKCommitment;

    // Mapping of PSI point to emailAddressPointer
    mapping(bytes => bytes32) public pointerOfPSIPoint;

    // Mapping of viewingKeyCommitment to Relayer's address
    mapping(bytes32 => address) public relayerOfVKCommitment;

    // Flag to indicate whether viewingKeyCommitment is initialized
    mapping(bytes32 => bool) public initializedVKCommitments;

    // Flag to indicate whether viewingKeyCommitment is nullifed
    mapping(bytes32 => bool) public nullifiedVKCommitments;

    // Mapping to store email nullifiers
    mapping(bytes32 => bool) public emailNullifiers;

    // Mapping from extensio name to extension address, as published by the developer
    mapping(string => address) public addressOfExtension;

    // Global mapping of command name to extension address enabled for all users by default
    mapping(string => address) public extensionOfCommand;

    // User level mapping of command name to extension address (pointer -> (command -> extension))
    mapping(bytes32 => mapping(string => address)) public userExtensionOfCommand;

    // Mapping of recipient's emailAddressCommitment (hash(email, randomness)) to the unclaimedFund
    mapping(bytes32 => UnclaimedFund) public unclaimedFundOfEmailAddrCommitment;

    // Mapping of token used for fees to this value in ETH
    mapping(string => uint256) public conversionRateOfFeeToken;

    // Max fee per gas in ETH
    uint256 maxFeePerGas;

    event RelayerRegistered(bytes32 randHash, bytes32 emailAddressHash, string hostname);

    event RelayerConfigUpdated(bytes32 randHash, string hostname);

    event UnclaimedFundRegistered(
        bytes32 emailAddressCommitment,
        address tokenAddress,
        uint256 amount,
        address sender,
        uint256 expiryTime,
        uint256 commitmentRandomness,
        string emailAddress
    );

    event UnclaimedFundClaimed(
        bytes32 emailAddressCommitment,
        address tokenAddress,
        uint256 amount,
        address recipient
    );

    event UnclaimedFundRefunded(
        bytes32 emailAddressCommitment,
        address tokenAddress,
        uint256 amount,
        address sender
    );

    /// @param _verifier ZK Proof verifier contract - must implement `IVerifier` interface
    /// @param _tokenRegistry Token registry contract with tokenName -> address - must implement `TokenRegistry` interface
    /// @param _dkimRegistry DKIM public key hashes registry - must implement `DKIMRegistry` interface
    constructor(
        address _verifier,
        address _tokenRegistry,
        address _dkimRegistry
    ) WalletHandler(_tokenRegistry) {
        verifier = IVerifier(_verifier);
        dkimRegistry = DKIMRegistry(_dkimRegistry);

        conversionRateOfFeeToken["WETH"] = 1;
        conversionRateOfFeeToken["DAI"] = 1600; // TODO: Get actual conversion rate
        conversionRateOfFeeToken["USDC"] = 1600;

        maxFeePerGas = 10 gwei; // TODO: Compute this properly
    }

    /// @notice Register as a relayer
    /// @param randHash hash of relayed private randomness `relayerRand`
    /// @param emailAddressHash hash of relayer's email address
    /// @param hostname hostname of relayer's server - used by other relayers for PSI communication
    function registerRelayer(
        bytes32 randHash,
        bytes32 emailAddressHash,
        string memory hostname
    ) public {
        require(randHash != bytes32(0), "ransHash cannot be empty");
        require(emailAddressHash != bytes32(0), "emailAddressHash cannot be empty");
        require(bytes(hostname).length != 0, "hostname cannot be empty");

        require(relayers[msg.sender].randHash == bytes32(0), "relayer already registered");

        relayers[msg.sender] = RelayerConfig({
            randHash: randHash,
            emailAddressHash: emailAddressHash,
            hostname: hostname
        });

        emit RelayerRegistered(randHash, emailAddressHash, hostname);
    }

    /// @notice Update relayer's config (hostname only for now)
    /// @param hostname new hostname of relayer's server
    function updateRelayerConfig(string memory hostname) public {
        require(bytes(hostname).length != 0, "hostname cannot be empty");
        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");

        relayers[msg.sender].hostname = hostname;

        emit RelayerConfigUpdated(relayers[msg.sender].randHash, hostname);
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
        bytes memory psiPoint,
        bytes memory proof
    ) public returns (address) {
        require(
            vkCommitmentOfPointer[emailAddressPointer] == bytes32(0),
            "VK commitment already exists"
        );
        require(pointerOfPSIPoint[psiPoint] == bytes32(0), "PSI point already exists");
        require(
            walletSaltOfVKCommitment[viewingKeyCommitment] == bytes32(0),
            "wallet salt already exists"
        );
        require(
            initializedVKCommitments[viewingKeyCommitment] == false,
            "account already initialized"
        );
        require(nullifiedVKCommitments[viewingKeyCommitment] == false, "account is nullified");
        require(
            Address.isContract(getAddressOfSalt(walletSalt)) == false,
            "wallet already deployed"
        );

        require(
            verifier.verifyAccountCreationProof(
                relayers[msg.sender].randHash,
                emailAddressPointer,
                viewingKeyCommitment,
                walletSalt,
                psiPoint,
                proof
            ),
            "invalid account creation proof"
        );

        vkCommitmentOfPointer[emailAddressPointer] = viewingKeyCommitment;
        walletSaltOfVKCommitment[viewingKeyCommitment] = walletSalt;
        relayerOfVKCommitment[viewingKeyCommitment] = msg.sender;
        pointerOfPSIPoint[psiPoint] = emailAddressPointer;

        return _deployWallet(walletSalt);
    }

    /// Initialize the account when user reply to invitation email
    /// @param emailAddressPointer hash(relayerRand, emailAddress)
    /// @param emailDomain domain name of the sender's email
    /// @param emailNullifier nullifier of the email used for proof generation
    /// @param proof ZK proof as required by the verifier
    function initializeAccount(
        bytes32 emailAddressPointer,
        string memory emailDomain,
        bytes32 emailNullifier,
        bytes memory proof
    ) public {
        bytes32 viewingKeyCommitment = vkCommitmentOfPointer[emailAddressPointer];

        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(viewingKeyCommitment != bytes32(0), "account not registered");
        require(relayerOfVKCommitment[viewingKeyCommitment] == msg.sender, "invalid relayer");
        require(
            initializedVKCommitments[viewingKeyCommitment] == false,
            "account already initialized"
        );
        require(nullifiedVKCommitments[viewingKeyCommitment] == false, "account is nullified");
        require(emailNullifiers[emailNullifier] == false, "email nullifier already used");

        require(
            verifier.verifyAccountInitializaionProof(
                relayers[msg.sender].randHash,
                emailAddressPointer,
                viewingKeyCommitment,
                emailDomain,
                bytes32(dkimRegistry.getDKIMPublicKeyHash(emailDomain)),
                emailNullifier,
                proof
            ),
            "invalid account creation proof"
        );

        initializedVKCommitments[viewingKeyCommitment] = true;
        emailNullifiers[emailNullifier] = true;
    }

    /// @notice Register unclaimed fund for the recipient - only for internal email wallet transfers.
    /// @notice `UNCLAIMED_FUNDS_REGISTRATION_FEE` ETH should be supplied to this function.
    /// @param emailAddressCommitment Hash of the recipient's email address and a random number.
    /// @param tokenAddress Address of ERC20 token contract.
    /// @param amount Amount in WEI of the token.
    /// @param senderAddress ETH address of the sender.
    function _registerUnclaimedFundInternal(
        bytes32 emailAddressCommitment,
        address tokenAddress,
        uint256 amount,
        address senderAddress
    ) private {
        // Ensure the relayer has paid ETH needed for claiming the unclaimed fee
        require(
            msg.value == Constants.UNCLAIMED_FUND_REGISTRATION_FEE,
            "invalid unclaimed fund fee"
        );

        require(amount > 0, "token amount should be greater than 0");
        require(tokenAddress != address(0), "invalid token contract");
        require(senderAddress != address(0), "invalid sender address");
        require(emailAddressCommitment != bytes32(0), "invalid emailAddressCommitment");
        require(
            unclaimedFundOfEmailAddrCommitment[emailAddressCommitment].amount == 0,
            "unclaimed fund already registered"
        );

        // Transfer token from sender's wallet to Core contract
        _processERC20TransferRequest(senderAddress, address(this), tokenAddress, amount);

        uint256 expiryTime = block.timestamp + Constants.DEFAULT_UNCLAIMED_FUNDS_EXPIRY_DURATION;

        UnclaimedFund memory fund = UnclaimedFund({
            tokenAddress: tokenAddress,
            amount: amount,
            expiryTime: expiryTime,
            senderAddress: senderAddress
        });

        unclaimedFundOfEmailAddrCommitment[emailAddressCommitment] = fund;

        emit UnclaimedFundRegistered(
            emailAddressCommitment,
            tokenAddress,
            amount,
            senderAddress,
            expiryTime,
            0,
            ""
        );
    }

    /// @notice Register unclaimed fund for the recipient - for external users to deposit tokens to an email address.
    /// @param emailAddressCommitment Hash of the recipient's email address and a random number.
    /// @param tokenAddress Address of ERC20 token contract.
    /// @param amount Amount in WEI of the token.
    /// @param expiryTime Expiry time to claim the unclaimed fund. Set `0` to use default expiry.
    /// @param announceCommitmentRandomness Randomness used to generate the `emailAddressCommitment` - if needs to be public.
    /// @param announceEmailAddress Email address of the recipient - if needs to be public.
    /// @dev   `UNCLAIMED_FUNDS_REGISTRATION_FEE` ETH should be supplied to this function.
    /// @dev   `announceCommitmentRandomness` and `announceEmailAddress` are optional. They are not validated as well.
    function registerUnclaimedFund(
        bytes32 emailAddressCommitment,
        address tokenAddress,
        uint256 amount,
        uint256 expiryTime,
        uint256 announceCommitmentRandomness,
        string memory announceEmailAddress
    ) public payable {
        if (expiryTime == 0) {
            expiryTime = block.timestamp + Constants.DEFAULT_UNCLAIMED_FUNDS_EXPIRY_DURATION;
        }

        // Ensure the sender has paid ETH needed for claiming the unclaimed fee
        require(
            msg.value == Constants.UNCLAIMED_FUND_REGISTRATION_FEE,
            "invalid unclaimed fund fee"
        );

        require(amount > 0, "token amount should be greater than 0");
        require(tokenAddress != address(0), "invalid token contract");
        require(emailAddressCommitment != bytes32(0), "invalid emailAddressCommitment");
        require(expiryTime > block.timestamp, "invalid expiry time");
        require(
            unclaimedFundOfEmailAddrCommitment[emailAddressCommitment].amount == 0,
            "unclaimed fund already registered"
        );

        // Transfer token from sender to Core contract - sender should have set enough allowance for Core contract
        ERC20(tokenAddress).transferFrom(msg.sender, address(this), amount);

        UnclaimedFund memory fund = UnclaimedFund({
            tokenAddress: tokenAddress,
            amount: amount,
            expiryTime: expiryTime,
            senderAddress: msg.sender
        });

        unclaimedFundOfEmailAddrCommitment[emailAddressCommitment] = fund;

        emit UnclaimedFundRegistered(
            emailAddressCommitment,
            tokenAddress,
            amount,
            msg.sender,
            expiryTime,
            announceCommitmentRandomness,
            announceEmailAddress
        );
    }

    /// Claim an unclaimed fund to the recipient's (initialized) wallet.
    /// @param emailAddressCommitment The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @param recipientEmailAddressPointer The pointer to the recipient's email address.
    /// @param proof Proof as required by verifier - prove `pointer` and `commitment` are of the same email address.
    function claimUnclaimedFund(
        bytes32 emailAddressCommitment,
        bytes32 recipientEmailAddressPointer,
        bytes memory proof
    ) public {
        UnclaimedFund storage fund = unclaimedFundOfEmailAddrCommitment[
            recipientEmailAddressPointer
        ];
        bytes32 vkCommitment = vkCommitmentOfPointer[recipientEmailAddressPointer];
        bytes32 walletSalt = walletSaltOfVKCommitment[vkCommitment];

        require(relayers[msg.sender].randHash != bytes32(0), "caller not relayer");
        require(relayerOfVKCommitment[vkCommitment] == msg.sender, "invalid relayer for account");
        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime > block.timestamp, "unclaimed fund expired");
        require(
            vkCommitmentOfPointer[recipientEmailAddressPointer] != bytes32(0),
            "invalid VK commitment"
        );
        require(initializedVKCommitments[vkCommitment], "account not initialized");
        require(!nullifiedVKCommitments[vkCommitment], "account is nullified");
        require(walletSalt != bytes32(0), "invalid wallet salt");

        require(
            verifier.verifyClaimFundProof(
                relayers[msg.sender].randHash,
                recipientEmailAddressPointer,
                emailAddressCommitment,
                proof
            ),
            "invalid proof"
        );

        address recipientAddress = getAddressOfSalt(walletSalt);

        delete unclaimedFundOfEmailAddrCommitment[recipientEmailAddressPointer];

        // Transfer token from Core contract to recipient's wallet
        ERC20(fund.tokenAddress).transfer(recipientAddress, fund.amount);

        // Transfer claim fee to the sender (relayer)
        (bool sent, ) = payable(msg.sender).call{value: Constants.UNCLAIMED_FUND_REGISTRATION_FEE}(
            ""
        );
        require(sent, "failed to transfer claim fee");

        emit UnclaimedFundClaimed(
            emailAddressCommitment,
            fund.tokenAddress,
            fund.amount,
            recipientAddress
        );
    }

    /// @notice Return unclaimed fund after expiry time
    /// @param emailAddressCommitment The commitment of the recipient's email address to which the unclaimed fund was registered.
    function refundUnclaimedFund(bytes32 emailAddressCommitment) external {
        UnclaimedFund storage fund = unclaimedFundOfEmailAddrCommitment[emailAddressCommitment];

        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime < block.timestamp, "unclaimed fund not expired");

        delete unclaimedFundOfEmailAddrCommitment[emailAddressCommitment];

        // Transfer token from Core contract to sender's wallet
        ERC20(fund.tokenAddress).transfer(fund.senderAddress, fund.amount);

        // Transfer claim fee to the sender - either emailWallet user or external wallet
        (bool sent, ) = payable(fund.senderAddress).call{
            value: Constants.UNCLAIMED_FUND_REGISTRATION_FEE
        }("");
        require(sent, "failed to transfer refund fee");

        emit UnclaimedFundRefunded(
            emailAddressCommitment,
            fund.tokenAddress,
            fund.amount,
            fund.senderAddress
        );
    }

    /// @notice Return the extension address for a command and user
    /// @param command Command for which the extension address is to be returned
    /// @param emailAddressPointer Pointer of the user's email address
    function getExtensionForCommand(
        string memory command,
        bytes32 emailAddressPointer
    ) public view returns (address) {
        address extensionAddress = extensionOfCommand[command]; // Global extension installed by default for all users
        address userExtensionAddress = userExtensionOfCommand[emailAddressPointer][command];

        if (userExtensionAddress != address(0)) {
            extensionAddress = userExtensionAddress;
        }

        return extensionAddress;
    }

    /// @notice Calculate the masked subject for an EmailOp from command and other params
    ///         This also validates certain parameters like tokenName, extensionName, extension command are registered.
    /// @param emailOp EmailOp from which the masked subject is to be computed
    function _computeMaskedSubjectForEmailOp(
        EmailOperation memory emailOp
    ) internal view returns (string memory maskedSubject, bool isExtension) {
        // Sample: Send 1 ETH to recipient@domain.com
        if (Strings.equal(emailOp.command, Constants.SEND_COMMAND)) {
            WalletParams memory walletParams = emailOp.walletParams;
            ERC20 token = ERC20(tokenRegistry.getTokenAddress(walletParams.tokenName));

            require(token != ERC20(address(0)), "token not supported");

            maskedSubject = string.concat(
                Constants.SEND_COMMAND,
                " ",
                Strings.toString(walletParams.amount / (10 ** token.decimals())),
                " ",
                walletParams.tokenName,
                " to "
            );

            if (emailOp.recipientETHAddress != address(0)) {
                maskedSubject = string.concat(
                    maskedSubject,
                    Strings.toHexString(uint256(uint160(emailOp.recipientETHAddress)), 20)
                );
            }
        }
        // Sample: Set extension for Swap as Uniswap
        else if (Strings.equal(emailOp.command, Constants.SET_EXTENSION_COMMAND)) {
            ExtensionManagerParams memory extManagerParams = emailOp.extManagerParams;

            require(
                addressOfExtension[extManagerParams.extensionName] != address(0),
                "extension not registered"
            );

            maskedSubject = string.concat(
                Constants.SET_EXTENSION_COMMAND,
                " for ",
                extManagerParams.command,
                " as ",
                extManagerParams.extensionName
            );
        }
        // Sample: Remove extension for Swap
        else if (Strings.equal(emailOp.command, Constants.REMOVE_EXTENSION_COMMAND)) {
            maskedSubject = string.concat(
                Constants.REMOVE_EXTENSION_COMMAND,
                " for ",
                emailOp.extManagerParams.command
            );
        }
        // The command is for an extension
        else {
            isExtension = true;
            address extensionAddress = getExtensionForCommand(
                emailOp.command,
                emailOp.emailAddressPointer
            );

            require(extensionAddress != address(0), "invalid command or extension");

            IExtension extension = IExtension(extensionAddress);
            maskedSubject = extension.computeEmailSubject(emailOp.extParams);
        }
    }

    /// @notice Validate an EmailOp, including proof verification
    /// @param emailOp EmailOperation to be validated
    function _validateEmailOp(EmailOperation memory emailOp) internal view {
        bytes32 relayerHash = relayers[msg.sender].randHash;
        bytes32 vkCommitment = vkCommitmentOfPointer[emailOp.emailAddressPointer];
        bytes32 walletSalt = walletSaltOfVKCommitment[vkCommitment];
        bytes32 dkimPublicKeyHash = bytes32(dkimRegistry.getDKIMPublicKeyHash(emailOp.emailDomain));

        require(relayerHash != bytes32(0), "relayer not registered");
        require(relayerOfVKCommitment[vkCommitment] == msg.sender, "invalid relayer");
        require(initializedVKCommitments[vkCommitment], "account not initialized");
        require(!nullifiedVKCommitments[vkCommitment], "account is nullified");
        require(walletSalt != bytes32(0), "wallet salt not set");
        require(emailNullifiers[emailOp.emailNullifier] == false, "email nullifier already used");
        require(bytes(emailOp.command).length != 0, "command cannot be empty");
        require(emailOp.dkimPublicKeyHash != bytes32(0), "DKIM pubkey hash cannot be empty");
        require(dkimPublicKeyHash == emailOp.dkimPublicKeyHash, "DKIM pubkey hash mismatch");
        require(
            tokenRegistry.getTokenAddress(emailOp.feeTokenName) != address(0),
            "invalid fee token"
        );
        require(conversionRateOfFeeToken[emailOp.feeTokenName] != 0, "unsupported fee token");
        require(emailOp.feePerGas <= maxFeePerGas, "fee per gas too high");

        if (emailOp.hasEmailRecipient) {
            require(emailOp.recipientETHAddress == address(0), "cannot have both recipient types");
            require(
                emailOp.recipientEmailAddressCommitment != bytes32(0),
                "recipient  emailCommitment cannot be empty"
            );
        } else {
            require(
                emailOp.recipientEmailAddressCommitment == bytes32(0),
                "recipient emailCommitment not allowed"
            );
        }

        (string memory maskedSubject, bool isExtension) = _computeMaskedSubjectForEmailOp(emailOp);
        require(Strings.equal(maskedSubject, emailOp.maskedSubject), "computed subject mismatch");

        if (isExtension) {
            require(emailOp.extParams.length > 0, "extension params cannot be empty");
        }

        require(
            verifier.verifyEmailProof(
                emailOp.emailDomain,
                emailOp.dkimPublicKeyHash,
                emailOp.maskedSubject,
                emailOp.emailNullifier,
                relayerHash,
                emailOp.emailAddressPointer,
                emailOp.hasEmailRecipient,
                emailOp.recipientEmailAddressCommitment,
                emailOp.emailProof
            ),
            "invalid email proof"
        );
    }

    /// Execute an EmailOperation
    /// @param emailOp EmailOperation to be executed
    /// @dev Fee for unclaimed fund registration should be send if the recipient is an email address
    function executeEmailOp(EmailOperation memory emailOp) public payable {
        uint256 initialGas = gasleft();

        _validateEmailOp(emailOp);

        emailNullifiers[emailOp.emailNullifier] = true;

        address sender = getAddressOfSalt(
            walletSaltOfVKCommitment[vkCommitmentOfPointer[emailOp.emailAddressPointer]]
        );

        // Wallet operation
        if (Strings.equal(emailOp.command, Constants.SEND_COMMAND)) {
            WalletParams memory walletParams = emailOp.walletParams;
            address tokenAddress = tokenRegistry.getTokenAddress(walletParams.tokenName);
            address recipient = emailOp.hasEmailRecipient
                ? address(this)
                : emailOp.recipientETHAddress;

            _processERC20TransferRequest(sender, recipient, tokenAddress, walletParams.amount);

            // Register unclaimed fund for the recipient if the recipient is an email address
            if (emailOp.hasEmailRecipient) {
                require(msg.value == Constants.UNCLAIMED_FUND_REGISTRATION_FEE, "invalid fund fee");

                _registerUnclaimedFundInternal(
                    emailOp.recipientEmailAddressCommitment,
                    tokenAddress,
                    walletParams.amount,
                    sender
                );
            }
        }
        // Set custom extension for the user
        else if (Strings.equal(emailOp.command, Constants.SET_EXTENSION_COMMAND)) {
            ExtensionManagerParams memory extManagerParams = emailOp.extManagerParams;
            address extensionAddress = addressOfExtension[extManagerParams.extensionName];

            userExtensionOfCommand[emailOp.emailAddressPointer][
                extManagerParams.command
            ] = extensionAddress;
        }
        // Remove custom extension for the user
        else if (Strings.equal(emailOp.command, Constants.REMOVE_EXTENSION_COMMAND)) {
            userExtensionOfCommand[emailOp.emailAddressPointer][emailOp.command] = address(0);
        }
        // The command is for an extension
        else {
            address extAddress = getExtensionForCommand(
                emailOp.command,
                emailOp.emailAddressPointer
            );

            require(extAddress != address(0), "extension not registered");

            // IExtension extension = IExtension(extAddress);
            // TODO: Call extension
        }

        // Refund gas to the relayer from sender
        uint256 consumedGas = initialGas - gasleft();   // TODO: Add gas for refund operation
        uint256 feePerGas = emailOp.feePerGas != 0 ? emailOp.feePerGas : maxFeePerGas;
        uint256 gasFeeAmount = consumedGas * feePerGas;

        uint256 feeAmount = gasFeeAmount / conversionRateOfFeeToken[emailOp.feeTokenName];
        address feeToken = tokenRegistry.getTokenAddress(emailOp.feeTokenName);

        _processERC20TransferRequest(sender, msg.sender, feeToken, feeAmount);
    }
}
