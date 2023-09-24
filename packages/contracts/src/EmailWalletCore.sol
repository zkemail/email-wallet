// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@zk-email/contracts/DKIMRegistry.sol";
import "./utils/TokenRegistry.sol";
import "./interfaces/IVerifier.sol";
import "./interfaces/EmailWalletExtension.sol";
import "./interfaces/Types.sol";
import "./interfaces/Constants.sol";
import "./Wallet.sol";
import "./handlers/WalletHandler.sol";

contract EmailWalletCore is WalletHandler, ReentrancyGuard {
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

    // Mapping of recipient's emailAddrCommitment (hash(email, randomness)) to the unclaimedFund
    mapping(bytes32 => UnclaimedFund) public unclaimedFundOfEmailAddrCommitment;

    // Mapping of emailAddrCommitment to unclaimed state
    mapping(bytes32 => UnclaimedState) public unclaimedStateOfEmailAddrCommitment;

    // Mapping of token used for fees to this value in ETH
    mapping(string => uint256) public conversionRateOfFeeToken;

    // Max fee per gas in ETH
    uint256 maxFeePerGas;

    ExecutionContext private currentContext;

    event RelayerRegistered(bytes32 randHash, bytes32 emailAddressHash, string hostname);

    event RelayerConfigUpdated(bytes32 randHash, string hostname);

    event UnclaimedFundRegistered(
        bytes32 emailAddrCommitment,
        address tokenAddress,
        uint256 amount,
        address sender,
        uint256 expiryTime,
        uint256 commitmentRandomness,
        string emailAddress
    );

    event UnclaimedFundClaimed(bytes32 emailAddrCommitment, address tokenAddress, uint256 amount, address recipient);

    event UnclaimedFundReverted(bytes32 emailAddrCommitment, address tokenAddress, uint256 amount, address sender);

    event UnclaimedStateRegistered(
        bytes32 emailAddrCommitment,
        address extensionAddress,
        address sender,
        uint256 expiryTime,
        bytes state,
        uint256 commitmentRandomness,
        string emailAddress
    );

    event UnclaimedStateClaimed(bytes32 emailAddrCommitment, address recipient);

    event UnclaimedStateReverted(bytes32 emailAddrCommitment, address sender);

    /// @param _verifier ZK Proof verifier contract - must implement `IVerifier` interface
    /// @param _tokenRegistry Token registry contract with tokenName -> address - must implement `TokenRegistry` interface
    /// @param _dkimRegistry DKIM public key hashes registry - must implement `DKIMRegistry` interface
    constructor(address _verifier, address _tokenRegistry, address _dkimRegistry) WalletHandler(_tokenRegistry) {
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
    function registerRelayer(bytes32 randHash, bytes32 emailAddressHash, string memory hostname) public {
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
        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(vkCommitmentOfPointer[emailAddressPointer] == bytes32(0), "VK commitment exists");
        require(pointerOfPSIPoint[psiPoint] == bytes32(0), "PSI point exists");
        require(walletSaltOfVKCommitment[viewingKeyCommitment] == bytes32(0), "walletSalt exists");
        require(initializedVKCommitments[viewingKeyCommitment] == false, "account is initialized");
        require(nullifiedVKCommitments[viewingKeyCommitment] == false, "account is nullified");
        require(Address.isContract(getAddressOfSalt(walletSalt)) == false, "wallet already deployed");

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
        require(initializedVKCommitments[viewingKeyCommitment] == false, "account already initialized");
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

    /// @notice Register unclaimed fund for the recipient - for external users to deposit tokens to an email address.
    /// @param emailAddrCommitment Hash of the recipient's email address and a random number.
    /// @param tokenAddress Address of ERC20 token contract.
    /// @param amount Amount in WEI of the token.
    /// @param expiryTime Expiry time to claim the unclaimed fund. Set `0` to use default expiry.
    /// @param announceCommitmentRandomness Randomness used to generate the `emailAddrCommitment` - if needs to be public.
    /// @param announceEmailAddress Email address of the recipient - if needs to be public.
    /// @dev   `UNCLAIMED_FUNDS_REGISTRATION_FEE` ETH should be supplied to this function.
    /// @dev   `announceCommitmentRandomness` and `announceEmailAddress` are optional. They are not validated as well.
    function registerUnclaimedFund(
        bytes32 emailAddrCommitment,
        address tokenAddress,
        uint256 amount,
        uint256 expiryTime,
        uint256 announceCommitmentRandomness,
        string memory announceEmailAddress
    ) public payable {
        // Ensure the sender has paid ETH needed for claiming the unclaimed fee
        require(msg.value == Constants.UNCLAIMED_FUND_REGISTRATION_FEE, "invalid unclaimed fund fee");
        require(amount > 0, "token amount should be greater than 0");
        require(tokenAddress != address(0), "invalid token contract");
        require(emailAddrCommitment != bytes32(0), "invalid emailAddrCommitment");
        require(expiryTime > block.timestamp, "invalid expiry time");
        require(
            unclaimedFundOfEmailAddrCommitment[emailAddrCommitment].amount == 0,
            "unclaimed fund already registered"
        );

        // Transfer token from sender to Core contract - sender should have set enough allowance for Core contract
        ERC20(tokenAddress).transferFrom(msg.sender, address(this), amount);

        _registerUnclaimedFund(
            msg.sender,
            emailAddrCommitment,
            tokenAddress,
            amount,
            expiryTime,
            announceCommitmentRandomness,
            announceEmailAddress
        );
    }

    /// Claim an unclaimed fund to the recipient's (initialized) wallet.
    /// @param emailAddrCommitment The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @param recipientEmailAddressPointer The pointer to the recipient's email address.
    /// @param proof Proof as required by verifier - prove `pointer` and `commitment` are of the same email address.
    function claimUnclaimedFund(
        bytes32 emailAddrCommitment,
        bytes32 recipientEmailAddressPointer,
        bytes memory proof
    ) public nonReentrant {
        UnclaimedFund storage fund = unclaimedFundOfEmailAddrCommitment[recipientEmailAddressPointer];
        bytes32 vkCommitment = vkCommitmentOfPointer[recipientEmailAddressPointer];
        bytes32 walletSalt = walletSaltOfVKCommitment[vkCommitment];

        require(relayers[msg.sender].randHash != bytes32(0), "caller not relayer");
        require(relayerOfVKCommitment[vkCommitment] == msg.sender, "invalid relayer for account");
        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime > block.timestamp, "unclaimed fund expired");
        require(vkCommitmentOfPointer[recipientEmailAddressPointer] != bytes32(0), "invalid VK commitment");
        require(initializedVKCommitments[vkCommitment], "account not initialized");
        require(!nullifiedVKCommitments[vkCommitment], "account is nullified");
        require(walletSalt != bytes32(0), "invalid wallet salt");

        require(
            verifier.verifyClaimFundProof(
                relayers[msg.sender].randHash,
                recipientEmailAddressPointer,
                emailAddrCommitment,
                proof
            ),
            "invalid proof"
        );

        address recipientAddress = getAddressOfSalt(walletSalt);

        delete unclaimedFundOfEmailAddrCommitment[recipientEmailAddressPointer];

        // Transfer token from Core contract to recipient's wallet
        ERC20(fund.tokenAddress).transfer(recipientAddress, fund.amount);

        // Transfer claim fee to the sender (relayer)
        _transferETH(msg.sender, Constants.UNCLAIMED_FUND_REGISTRATION_FEE);

        emit UnclaimedFundClaimed(emailAddrCommitment, fund.tokenAddress, fund.amount, recipientAddress);
    }

    /// @notice Return unclaimed fund after expiry time
    /// @param emailAddrCommitment The commitment of the recipient's email address to which the unclaimed fund was registered.
    function revertUnclaimedFund(bytes32 emailAddrCommitment) public nonReentrant {
        UnclaimedFund storage fund = unclaimedFundOfEmailAddrCommitment[emailAddrCommitment];

        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime < block.timestamp, "unclaimed fund not expired");

        delete unclaimedFundOfEmailAddrCommitment[emailAddrCommitment];

        // Transfer token from Core contract to sender's wallet
        ERC20(fund.tokenAddress).transfer(fund.senderAddress, fund.amount);

        // Transfer claim fee to the sender - either emailWallet user or external wallet
        _transferETH(fund.senderAddress, Constants.UNCLAIMED_FUND_REGISTRATION_FEE);

        emit UnclaimedFundReverted(emailAddrCommitment, fund.tokenAddress, fund.amount, fund.senderAddress);
    }

    /// Register unclaimed state of an extension for the recipient email address commitment
    /// @param emailAddrCommitment Email address commitment of the recipient
    /// @param extensionAddress Address of the extension contract
    /// @param state State to be registered
    /// @param expiryTime Expiry time to claim the unclaimed state.
    /// @param announceCommitmentRandomness Randomness used to generate the `emailAddrCommitment` - if needs to be public.
    /// @param announceEmailAddress Email address of the recipient - if needs to be public.
    function registerUnclaimedState(
        bytes32 emailAddrCommitment,
        address extensionAddress,
        bytes memory state,
        uint256 expiryTime,
        uint256 announceCommitmentRandomness,
        string memory announceEmailAddress
    ) public payable nonReentrant {
        if (expiryTime == 0) {
            expiryTime = block.timestamp + Constants.DEFAULT_UNCLAIMED_FUNDS_EXPIRY_DURATION;
        }

        // Ensure the sender has paid ETH needed for claiming the unclaimed fee
        require(msg.value == Constants.UNCLAIMED_FUND_REGISTRATION_FEE, "invalid unclaimed state fee");

        require(state.length > 0, "state cannot be empty");
        require(emailAddrCommitment != bytes32(0), "invalid emailAddrCommitment");
        require(expiryTime > block.timestamp, "invalid expiry time");
        require(
            unclaimedStateOfEmailAddrCommitment[emailAddrCommitment].senderAddress == address(0),
            "unclaimed state already registered"
        );

        UnclaimedState memory us = UnclaimedState({
            emailAddrCommitment: emailAddrCommitment,
            extensionAddress: extensionAddress,
            senderAddress: msg.sender,
            state: state
        });

        EmailWalletExtension extension = EmailWalletExtension(extensionAddress);
        bool registered = extension.registerUnclaimedState(us);

        require(registered, "failed to register unclaimed state");

        unclaimedStateOfEmailAddrCommitment[emailAddrCommitment] = us;

        emit UnclaimedStateRegistered(
            emailAddrCommitment,
            extensionAddress,
            msg.sender,
            expiryTime,
            state,
            announceCommitmentRandomness,
            announceEmailAddress
        );
    }

    /// Registere unclaimed state from an extension
    /// @param emailAddrCommitment Email address commitment of the recipient
    /// @param extensionAddress Address of the extension contract
    /// @param state State to be registered
    /// @dev This dont call `extension.registerUnclaimedState` as extension is expected to make necessary changes
    function registerUnclaimedStateAsExtension(
        bytes32 emailAddrCommitment,
        address extensionAddress,
        bytes memory state
    ) public {
        address senderAddress = currentContext.walletAddress;

        require(msg.sender == currentContext.extensionAddress, "invalid caller");
        require(currentContext.unclaimedStateRegistered == false, "unclaimed state already registered");
        require(state.length > 0, "state cannot be empty");
        require(extensionAddress != address(0), "invalid extension contract");
        require(senderAddress != address(0), "invalid sender address");
        require(emailAddrCommitment != bytes32(0), "invalid emailAddrCommitment");

        uint256 expiryTime = block.timestamp + Constants.DEFAULT_UNCLAIMED_FUNDS_EXPIRY_DURATION;

        UnclaimedState memory us = UnclaimedState({
            emailAddrCommitment: emailAddrCommitment,
            extensionAddress: extensionAddress,
            senderAddress: msg.sender,
            state: state
        });

        unclaimedStateOfEmailAddrCommitment[emailAddrCommitment] = us;
        currentContext.unclaimedStateRegistered = true;

        emit UnclaimedStateRegistered(emailAddrCommitment, extensionAddress, msg.sender, expiryTime, state, 0, "");
    }

    /// Claim unclaimed state to the recipient's (initialized) wallet.
    /// @param emailAddrCommitment The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @param recipientEmailAddressPointer The pointer to the recipient's email address.
    /// @param proof Proof as required by verifier - prove `pointer` and `commitment` are of the same email address.
    function claimUnclaimedState(
        bytes32 emailAddrCommitment,
        bytes32 recipientEmailAddressPointer,
        bytes memory proof
    ) public nonReentrant {
        UnclaimedState storage us = unclaimedStateOfEmailAddrCommitment[recipientEmailAddressPointer];
        bytes32 vkCommitment = vkCommitmentOfPointer[recipientEmailAddressPointer];
        bytes32 walletSalt = walletSaltOfVKCommitment[vkCommitment];

        require(relayers[msg.sender].randHash != bytes32(0), "caller not relayer");
        require(relayerOfVKCommitment[vkCommitment] == msg.sender, "invalid relayer for account");
        require(us.senderAddress != address(0), "unclaimed state not registered");
        require(us.extensionAddress != address(0), "invalid extension address");
        require(vkCommitmentOfPointer[recipientEmailAddressPointer] != bytes32(0), "invalid VK commitment");
        require(initializedVKCommitments[vkCommitment], "account not initialized");
        require(!nullifiedVKCommitments[vkCommitment], "account is nullified");
        require(walletSalt != bytes32(0), "invalid wallet salt");

        require(
            verifier.verifyClaimFundProof(
                relayers[msg.sender].randHash,
                recipientEmailAddressPointer,
                emailAddrCommitment,
                proof
            ),
            "invalid proof"
        );

        address recipientAddress = getAddressOfSalt(walletSalt);

        delete unclaimedStateOfEmailAddrCommitment[recipientEmailAddressPointer];

        EmailWalletExtension extension = EmailWalletExtension(us.extensionAddress);
        extension.claimUnclaimedState(us, recipientAddress);

        // Transfer claim fee to the sender (relayer)
        _transferETH(msg.sender, Constants.UNCLAIMED_FUND_REGISTRATION_FEE);

        emit UnclaimedStateClaimed(emailAddrCommitment, recipientAddress);
    }

    /// @notice Return unclaimed state after expiry time
    /// @param emailAddrCommitment The commitment of the recipient's email address to which the unclaimed state was registered.
    function revertUnclaimedState(bytes32 emailAddrCommitment) public nonReentrant {
        UnclaimedState storage us = unclaimedStateOfEmailAddrCommitment[emailAddrCommitment];

        require(us.senderAddress != address(0), "unclaimed state not registered");

        delete unclaimedStateOfEmailAddrCommitment[emailAddrCommitment];

        EmailWalletExtension extension = EmailWalletExtension(us.extensionAddress);
        extension.revertUnclaimedState(us);

        // Transfer claim fee to the sender - either emailWallet user or external wallet
        _transferETH(us.senderAddress, Constants.UNCLAIMED_FUND_REGISTRATION_FEE);

        emit UnclaimedStateReverted(emailAddrCommitment, us.senderAddress);
    }

    /// @notice Return the extension address for a command and user
    /// @param command Command for which the extension address is to be returned
    /// @param emailAddressPointer Pointer of the user's email address
    function getExtensionForCommand(string memory command, bytes32 emailAddressPointer) public view returns (address) {
        address extensionAddress = extensionOfCommand[command]; // Global extension installed by default for all users
        address userExtensionAddress = userExtensionOfCommand[emailAddressPointer][command];

        if (userExtensionAddress != address(0)) {
            extensionAddress = userExtensionAddress;
        }

        return extensionAddress;
    }

    /// @notice Validate an EmailOp, including proof verification
    /// @param emailOp EmailOp to be validated
    function validateEmailOp(EmailOp memory emailOp) public view {
        bytes32 relayerHash = relayers[msg.sender].randHash;
        bytes32 vkCommitment = vkCommitmentOfPointer[emailOp.emailAddrPointer];
        bytes32 walletSalt = walletSaltOfVKCommitment[vkCommitment];
        bytes32 dkimPublicKeyHash = bytes32(dkimRegistry.getDKIMPublicKeyHash(emailOp.emailDomain));

        require(dkimPublicKeyHash != bytes32(0), "cannot find DKIM for domain");
        require(relayerHash != bytes32(0), "relayer not registered");
        require(relayerOfVKCommitment[vkCommitment] == msg.sender, "invalid relayer");
        require(initializedVKCommitments[vkCommitment], "account not initialized");
        require(!nullifiedVKCommitments[vkCommitment], "account is nullified");
        require(walletSalt != bytes32(0), "wallet salt not set");
        require(emailNullifiers[emailOp.emailNullifier] == false, "email nullifier already used");
        require(bytes(emailOp.command).length != 0, "command cannot be empty");
        require(tokenRegistry.getTokenAddress(emailOp.feeTokenName) != address(0), "invalid fee token");
        require(conversionRateOfFeeToken[emailOp.feeTokenName] != 0, "unsupported fee token");
        require(emailOp.feePerGas <= maxFeePerGas, "fee per gas too high");

        if (emailOp.hasEmailRecipient) {
            require(emailOp.recipientETHAddr == address(0), "cannot have both recipient types");
            require(emailOp.recipientEmailAddrCommitment != bytes32(0), "recipient commitment not found");
            require(
                unclaimedFundOfEmailAddrCommitment[emailOp.recipientEmailAddrCommitment].amount == 0,
                "Unclaimed fund exist for commitment"
            );
            require(
                unclaimedStateOfEmailAddrCommitment[emailOp.recipientEmailAddrCommitment].state.length == 0,
                "Unclaimed state exists for commitment"
            );
        } else {
            require(emailOp.recipientEmailAddrCommitment == bytes32(0), "recipient commitment not allowed");
        }

        (string memory maskedSubject, bool isExtension) = _computeMaskedSubjectForEmailOp(emailOp);
        require(Strings.equal(maskedSubject, emailOp.maskedSubject), "computed subject mismatch");

        if (isExtension) {
            require(emailOp.extensionParams.length > 0, "extension params cannot be empty");
        }

        require(
            verifier.verifyEmailProof(
                emailOp.emailDomain,
                dkimPublicKeyHash,
                emailOp.maskedSubject,
                emailOp.emailNullifier,
                relayerHash,
                emailOp.emailAddrPointer,
                emailOp.hasEmailRecipient,
                emailOp.recipientEmailAddrCommitment,
                emailOp.emailProof
            ),
            "invalid email proof"
        );
    }

    /// @notice Handle an EmailOp - the main function relayer should call for each Email
    /// @param emailOp EmailOp to be executed
    /// @dev Fee for unclaimed fund/state registration should be send if the recipient is an email address
    function handleEmailOp(
        EmailOp calldata emailOp
    ) public payable nonReentrant returns (bool success, bytes memory returnData) {
        uint256 initialGas = gasleft();

        if (emailOp.hasEmailRecipient) {
            require(msg.value == Constants.UNCLAIMED_FUND_REGISTRATION_FEE, "invalid unclaimed fund fee");
        }

        currentContext = ExecutionContext({
            walletAddress: getAddressOfSalt(walletSaltOfVKCommitment[vkCommitmentOfPointer[emailOp.emailAddrPointer]]),
            extensionAddress: address(0),
            receivedETH: msg.value,
            unclaimedStateRegistered: false,
            unclaimedFundRegistered: false
        });

        validateEmailOp(emailOp);

        emailNullifiers[emailOp.emailNullifier] = true;

        (success, returnData) = _executeEmailOp(emailOp);

        // Refund ETH to relayer if unclaimed funds were not registered
        if (!currentContext.unclaimedFundRegistered && !currentContext.unclaimedStateRegistered) {
            _transferETH(msg.sender, msg.value);
        }

        // Refund gas cost to the relayer from sender (in the fee token)
        uint256 feeForRefund = 2100; // TODO : Calculate actual cost to process the refund
        uint256 consumedGas = initialGas - gasleft() + feeForRefund;
        uint256 feePerGas = emailOp.feePerGas != 0 ? emailOp.feePerGas : maxFeePerGas;

        uint256 totalFee = (consumedGas * feePerGas);
        if (currentContext.unclaimedFundRegistered || currentContext.unclaimedStateRegistered) {
            totalFee += Constants.UNCLAIMED_FUND_REGISTRATION_FEE;
        }

        uint256 feeAmount = totalFee / conversionRateOfFeeToken[emailOp.feeTokenName];
        address feeToken = tokenRegistry.getTokenAddress(emailOp.feeTokenName);

        _transferERC20(currentContext.walletAddress, msg.sender, feeToken, feeAmount);
    }

    /// @notice For extensions to request token from user's wallet
    /// @param tokenAddress Address of the ERC20 token requested
    /// @param amount Amount requested
    function requestTokenTransfer(address tokenAddress, uint256 amount) public {
        require(msg.sender == currentContext.extensionAddress, "invalid caller");

        // TODO: Validate the requested token and amound is allowed.

        _transferERC20(currentContext.walletAddress, currentContext.extensionAddress, tokenAddress, amount);
    }

    /// @notice Transport an account to a new Relayer - to be called by the new relayer
    /// @param oldVKCommitment Viewing Key commitment of the account under old (current) relayer
    /// @param newEmailAddressPointer Email Address pointer of the account under new relayer
    /// @param newVKCommitment Viewing Key commitment of the account under new relayer
    /// @param newPSIPoint PSI point of the email address under new relayer
    /// @param emailNullifier Nullifier of the email used for proof generation (reply to invite email)
    /// @param emailDomain Domain name of the user's email address
    /// @param accountCreationProof Proof for new account creation under new relayer
    /// @param accountTransportProof Proof of user's transport email
    function transportAccount(
        bytes32 oldVKCommitment,
        bytes32 newEmailAddressPointer,
        bytes32 newVKCommitment,
        bytes memory newPSIPoint,
        bytes32 emailNullifier,
        string memory emailDomain,
        bytes memory accountCreationProof,
        bytes memory accountTransportProof
    ) public {
        bytes32 walletSalt = walletSaltOfVKCommitment[oldVKCommitment];
        address oldRelayer = relayerOfVKCommitment[oldVKCommitment];

        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(oldRelayer != address(0), "old relayer not registered");
        require(oldRelayer != msg.sender, "new relayer cannot be same");
        require(initializedVKCommitments[oldVKCommitment], "account not initialized");
        require(!nullifiedVKCommitments[oldVKCommitment], "account is nullified");
        require(walletSalt != bytes32(0), "walletSalt not set");
        require(vkCommitmentOfPointer[newEmailAddressPointer] == bytes32(0), "new pointer already exist");
        require(walletSaltOfVKCommitment[newVKCommitment] == bytes32(0), "salt already exists");
        require(pointerOfPSIPoint[newPSIPoint] == bytes32(0), "new PSI point already exists");
        require(emailNullifiers[emailNullifier] == false, "email nullifier already used");

        require(
            verifier.verifyAccountCreationProof(
                relayers[msg.sender].randHash,
                newEmailAddressPointer,
                newVKCommitment,
                walletSalt,
                newPSIPoint,
                accountCreationProof
            ),
            "invalid account creation proof"
        );

        require(
            verifier.verifiyAccountTransportProof(
                emailDomain,
                bytes32(dkimRegistry.getDKIMPublicKeyHash(emailDomain)),
                emailNullifier,
                relayers[oldRelayer].randHash,
                oldVKCommitment,
                accountTransportProof
            ),
            "invalid account transport proof"
        );

        emailNullifiers[emailNullifier] = true;

        vkCommitmentOfPointer[oldVKCommitment] = bytes32(0);
        walletSaltOfVKCommitment[oldVKCommitment] = bytes32(0);
        nullifiedVKCommitments[oldVKCommitment] = true;

        vkCommitmentOfPointer[newEmailAddressPointer] = newVKCommitment;
        walletSaltOfVKCommitment[newVKCommitment] = walletSalt;
        relayerOfVKCommitment[newVKCommitment] = msg.sender;
        pointerOfPSIPoint[newPSIPoint] = newEmailAddressPointer;
        initializedVKCommitments[newVKCommitment] = true;
        nullifiedVKCommitments[newVKCommitment] = false;
    }

    /// Register a new extension
    /// @param name Name of the extension
    /// @param extensionAddress Address of the extension contract
    function publishExtension(string memory name, address extensionAddress) public {
        require(addressOfExtension[name] == address(0), "extension name already used");

        addressOfExtension[name] = extensionAddress;
    }

    /// @notice Calculate the masked subject for an EmailOp from command and other params
    ///         This also validates certain parameters like tokenName, extensionName, extension command are registered.
    /// @param emailOp EmailOp from which the masked subject is to be computed
    function _computeMaskedSubjectForEmailOp(
        EmailOp memory emailOp
    ) private view returns (string memory maskedSubject, bool isExtension) {
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

            if (emailOp.recipientETHAddr != address(0)) {
                maskedSubject = string.concat(
                    maskedSubject,
                    Strings.toHexString(uint256(uint160(emailOp.recipientETHAddr)), 20)
                );
            }
        }
        // Sample: Set extension for Swap as Uniswap
        else if (Strings.equal(emailOp.command, Constants.SET_EXTENSION_COMMAND)) {
            ExtensionManagerParams memory extManagerParams = emailOp.extManagerParams;

            require(addressOfExtension[extManagerParams.extensionName] != address(0), "extension not registered");

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
            address extensionAddress = getExtensionForCommand(emailOp.command, emailOp.emailAddrPointer);

            require(extensionAddress != address(0), "invalid command or extension");

            EmailWalletExtension extension = EmailWalletExtension(extensionAddress);
            maskedSubject = extension.computeEmailSubject(
                emailOp.extensionParams,
                emailOp.extensionSubjectTemplateIndex
            );
        }
    }

    /// @notice Execute an EmailOp
    /// @param emailOp EmailOp to be executed
    /// @return success Whether the operation is successful
    /// @return returnData Return data from the operation (error)
    function _executeEmailOp(EmailOp memory emailOp) private returns (bool success, bytes memory returnData) {
        // Wallet operation
        if (Strings.equal(emailOp.command, Constants.SEND_COMMAND)) {
            WalletParams memory walletParams = emailOp.walletParams;
            address tokenAddress = tokenRegistry.getTokenAddress(walletParams.tokenName);
            address recipient = emailOp.hasEmailRecipient ? address(this) : emailOp.recipientETHAddr;

            (success, returnData) = _transferERC20(
                currentContext.walletAddress,
                recipient,
                tokenAddress,
                walletParams.amount
            );

            if (!success) {
                return (success, returnData);
            }

            // Register unclaimed fund for the recipient
            _registerUnclaimedFund(
                currentContext.walletAddress,
                emailOp.recipientEmailAddrCommitment,
                tokenAddress,
                walletParams.amount,
                0,
                0,
                ""
            );
            currentContext.unclaimedStateRegistered = true;
        }
        // Set custom extension for the user
        else if (Strings.equal(emailOp.command, Constants.SET_EXTENSION_COMMAND)) {
            ExtensionManagerParams memory extManagerParams = emailOp.extManagerParams;
            address extensionAddress = addressOfExtension[extManagerParams.extensionName];

            userExtensionOfCommand[emailOp.emailAddrPointer][extManagerParams.command] = extensionAddress;
        }
        // Remove custom extension for the user
        else if (Strings.equal(emailOp.command, Constants.REMOVE_EXTENSION_COMMAND)) {
            userExtensionOfCommand[emailOp.emailAddrPointer][emailOp.command] = address(0);
        }
        // The command is for an extension
        else {
            address extAddress = getExtensionForCommand(emailOp.command, emailOp.emailAddrPointer);
            currentContext.extensionAddress = extAddress;

            (success, returnData) = extAddress.call(
                abi.encodeWithSignature(
                    "execute(bytes,uint8,address,bytes32)",
                    emailOp.extensionParams,
                    emailOp.extensionSubjectTemplateIndex,
                    currentContext.walletAddress,
                    emailOp.emailNullifier
                )
            );
        }
    }

    /// @notice Register unclaimed fund for the recipient - can be called by Core contract directly
    /// @param emailAddrCommitment Hash of the recipient's email address and a random number.
    /// @param tokenAddress Address of ERC20 token contract.
    /// @param amount Amount in WEI of the token.
    function _registerUnclaimedFund(
        address senderAddress,
        bytes32 emailAddrCommitment,
        address tokenAddress,
        uint256 amount,
        uint256 expiryTime,
        uint256 announceCommitmentRandomness,
        string memory announceEmailAddress
    ) private {
        uint256 _expiryTime = expiryTime != 0
            ? expiryTime
            : (block.timestamp + Constants.DEFAULT_UNCLAIMED_FUNDS_EXPIRY_DURATION);

        UnclaimedFund memory fund = UnclaimedFund({
            emailAddrCommitment: emailAddrCommitment,
            tokenAddress: tokenAddress,
            amount: amount,
            expiryTime: _expiryTime,
            senderAddress: senderAddress
        });

        unclaimedFundOfEmailAddrCommitment[emailAddrCommitment] = fund;

        emit UnclaimedFundRegistered(
            emailAddrCommitment,
            tokenAddress,
            amount,
            senderAddress,
            expiryTime,
            announceCommitmentRandomness,
            announceEmailAddress
        );
    }

    /// @notice Trasnfer ETH from core contract to recipient
    /// @param recipient    Address of the recipient
    /// @param amount      Amount in WEI to be transferred
    function _transferETH(address recipient, uint256 amount) private {
        (bool sent, ) = payable(recipient).call{value: amount}("");
        require(sent, "failed to transfer ETH");
    }
}
