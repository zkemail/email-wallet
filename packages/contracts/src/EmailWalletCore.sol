// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/Create2Upgradeable.sol";
import "@zk-email/contracts/DKIMRegistry.sol";
import "./utils/TokenRegistry.sol";
import "./interfaces/IVerifier.sol";
import "./interfaces/Extension.sol";
import "./interfaces/Types.sol";
import "./interfaces/Commands.sol";
import "./interfaces/IPriceOracle.sol";
import "./Wallet.sol";
import {LibZip} from "solady/utils/LibZip.sol";

contract EmailWalletCore is ReentrancyGuard, OwnableUpgradeable, UUPSUpgradeable {
    // ZK proof verifier
    IVerifier public immutable verifier;

    // DKIM public key hashes registry
    DKIMRegistry public immutable dkimRegistry;

    // Token registry
    TokenRegistry public immutable tokenRegistry;

    // Price oracle for feeToken conversion
    IPriceOracle public immutable priceOracle;

    // Address of wallet implementation contract - used for deploying wallets for users via proxy
    address public immutable walletImplementation;

    // Mapping of relayer's wallet address to relayer config
    mapping(address => RelayerConfig) public relayers;

    // Mapping of relayer's randHash to relayer's wallet address
    mapping(bytes32 => address) public relayerOfRandHash;

    // Mapping of relayer's email address to relayer's wallet address
    mapping(string => address) public relayerOfEmailAddr;

    // Mapping of emailAddrPointer to viewingKeyCommitment
    mapping(bytes32 => bytes32) public vkCommitmentOfPointer;

    // Mapping of emailAddrPointer to walletSalt
    mapping(bytes32 => bytes32) public walletSaltOfVKCommitment;

    // Mapping of PSI point to emailAddrPointer
    mapping(bytes => bytes32) public pointerOfPSIPoint;

    struct ViewingKeyCommitment {
        address relayer;
        bool initialized;
        bool nullified;
    }

    mapping(bytes32 => ViewingKeyCommitment) public vkCommitments;

    // Mapping of viewingKeyCommitment to Relayer's address
    // mapping(bytes32 => address) public relayerOfVKCommitment;

    // // Flag to indicate whether viewingKeyCommitment is initialized
    // mapping(bytes32 => bool) public initializedVKCommitments;

    // // Flag to indicate whether viewingKeyCommitment is nullifed
    // mapping(bytes32 => bool) public nullifiedVKCommitments;

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

    // Max fee per gas in ETH that relayer can set in a UserOp
    uint256 public immutable maxFeePerGas;

    // Regitration fee for unclaimed funds - ideally gasForUnclaim * maxFeePerGas
    uint256 public immutable unclaimedFundRegistrationFee;

    // Default expiry duration for unclaimed funds
    uint256 public immutable unclaimedFundExpirationDuration;

    // Context of currently executing EmailOp - reset on every EmailOp
    ExecutionContext internal currContext;

    event RelayerRegistered(bytes32 randHash, string emailAddr, string hostname);

    event RelayerConfigUpdated(bytes32 randHash, string hostname);

    event UnclaimedFundRegistered(
        bytes32 emailAddrCommitment,
        address tokenAddress,
        uint256 amount,
        address sender,
        uint256 expiryTime,
        uint256 commitmentRandomness,
        string emailAddr
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
        string emailAddr
    );

    event UnclaimedStateClaimed(bytes32 emailAddrCommitment, address recipient);

    event UnclaimedStateReverted(bytes32 emailAddrCommitment, address sender);

    constructor(
        address _verifier,
        address _tokenRegistry,
        address _dkimRegistry,
        address _priceOracle,
        uint256 _maxFeePerGas,
        uint256 _unclaimedFundRegistrationFee,
        uint256 _unclaimedFundExpirationDuration
    ) {
        verifier = IVerifier(_verifier);
        dkimRegistry = DKIMRegistry(_dkimRegistry);
        tokenRegistry = TokenRegistry(_tokenRegistry);
        priceOracle = IPriceOracle(_priceOracle);
        maxFeePerGas = _maxFeePerGas;
        unclaimedFundRegistrationFee = _unclaimedFundRegistrationFee;
        unclaimedFundExpirationDuration = _unclaimedFundExpirationDuration;

        walletImplementation = address(new Wallet());
    }

    fallback() external payable {
        LibZip.cdFallback();
    }

    function initialize() public initializer {
        __Ownable_init();
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

    /// @notice Register as a relayer
    /// @param randHash hash of relayed internal randomness `relayerRand`
    /// @param emailAddr relayer's email address
    /// @param hostname hostname of relayer's server - used by other relayers for PSI communication
    function registerRelayer(bytes32 randHash, string memory emailAddr, string memory hostname) public {
        require(randHash != bytes32(0), "randHash cannot be empty");
        require(bytes(emailAddr).length != 0, "emailAddr cannot be empty");
        require(bytes(hostname).length != 0, "hostname cannot be empty");
        require(relayers[msg.sender].randHash == bytes32(0), "relayer already registered");
        require(relayerOfRandHash[randHash] == address(0), "randHash already registered");
        require(relayerOfEmailAddr[emailAddr] == address(0), "emailAddr already registered");

        relayers[msg.sender] = RelayerConfig({randHash: randHash, emailAddr: emailAddr, hostname: hostname});
        relayerOfRandHash[randHash] = msg.sender;
        relayerOfEmailAddr[emailAddr] = msg.sender;

        emit RelayerRegistered(randHash, emailAddr, hostname);
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
    /// @param emailAddrPointer hash(relayerRand, emailAddr)
    /// @param viewingKeyCommitment hash(viewingKey, emailAddr, relayerHash)
    /// @param walletSalt hash(viewingKey, 0)
    /// @param proof ZK proof as required by the verifier
    function createAccount(
        bytes32 emailAddrPointer,
        bytes32 viewingKeyCommitment,
        bytes32 walletSalt,
        bytes memory psiPoint,
        bytes memory proof
    ) public returns (Wallet) {
        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(vkCommitmentOfPointer[emailAddrPointer] == bytes32(0), "pointer exists");
        require(pointerOfPSIPoint[psiPoint] == bytes32(0), "PSI point exists");
        require(walletSaltOfVKCommitment[viewingKeyCommitment] == bytes32(0), "walletSalt exists");
        require(vkCommitments[viewingKeyCommitment].initialized == false, "account is initialized");
        require(vkCommitments[viewingKeyCommitment].nullified == false, "account is nullified");
        require(Address.isContract(getWalletOfSalt(walletSalt)) == false, "wallet already deployed");

        require(
            verifier.verifyAccountCreationProof(
                relayers[msg.sender].randHash,
                emailAddrPointer,
                viewingKeyCommitment,
                walletSalt,
                psiPoint,
                proof
            ),
            "invalid account creation proof"
        );

        vkCommitmentOfPointer[emailAddrPointer] = viewingKeyCommitment;
        walletSaltOfVKCommitment[viewingKeyCommitment] = walletSalt;
        vkCommitments[viewingKeyCommitment].relayer = msg.sender;
        pointerOfPSIPoint[psiPoint] = emailAddrPointer;

        return _deployWallet(walletSalt);
    }

    /// Initialize the account when user reply to invitation email
    /// @param emailAddrPointer hash(relayerRand, emailAddr)
    /// @param emailDomain domain name of the sender's email
    /// @param emailNullifier nullifier of the email used for proof generation
    /// @param proof ZK proof as required by the verifier
    function initializeAccount(
        bytes32 emailAddrPointer,
        string memory emailDomain,
        bytes32 emailNullifier,
        bytes memory proof
    ) public {
        bytes32 viewingKeyCommitment = vkCommitmentOfPointer[emailAddrPointer];

        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(viewingKeyCommitment != bytes32(0), "account not registered");
        require(vkCommitments[viewingKeyCommitment].relayer == msg.sender, "invalid relayer");
        require(vkCommitments[viewingKeyCommitment].nullified == false, "account is nullified");
        require(vkCommitments[viewingKeyCommitment].initialized == false, "account already initialized");
        require(emailNullifiers[emailNullifier] == false, "email nullifier already used");

        require(
            verifier.verifyAccountInitializaionProof(
                relayers[msg.sender].randHash,
                emailAddrPointer,
                viewingKeyCommitment,
                emailDomain,
                bytes32(dkimRegistry.getDKIMPublicKeyHash(emailDomain)),
                emailNullifier,
                proof
            ),
            "invalid account creation proof"
        );

        vkCommitments[viewingKeyCommitment].initialized = true;
        emailNullifiers[emailNullifier] = true;
    }

    /// @notice Transport an account to a new Relayer - to be called by the new relayer
    /// @param oldVKCommitment Viewing Key commitment of the account under old (current) relayer
    /// @param newEmailAddrPointer Email Address pointer of the account under new relayer
    /// @param newVKCommitment Viewing Key commitment of the account under new relayer
    /// @param newPSIPoint PSI point of the email address under new relayer
    /// @param emailNullifier Nullifier of the email used for proof generation (reply to invite email)
    /// @param emailDomain Domain name of the user's email address
    /// @param accountCreationProof Proof for new account creation under new relayer
    /// @param accountTransportProof Proof of user's transport email
    function transportAccount(
        bytes32 oldVKCommitment,
        bytes32 newEmailAddrPointer,
        bytes32 newVKCommitment,
        bytes memory newPSIPoint,
        bytes32 emailNullifier,
        string memory emailDomain,
        bytes memory accountCreationProof,
        bytes memory accountTransportProof
    ) public {
        bytes32 walletSalt = walletSaltOfVKCommitment[oldVKCommitment];
        address oldRelayer = vkCommitments[oldVKCommitment].relayer;

        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(oldRelayer != address(0), "old relayer not registered");
        require(oldRelayer != msg.sender, "new relayer cannot be same");
        require(vkCommitments[oldVKCommitment].initialized, "account not initialized");
        require(!vkCommitments[oldVKCommitment].nullified, "account is nullified");
        require(vkCommitmentOfPointer[newEmailAddrPointer] == bytes32(0), "new pointer already exist");
        require(walletSaltOfVKCommitment[newVKCommitment] == bytes32(0), "salt already exists");
        require(pointerOfPSIPoint[newPSIPoint] == bytes32(0), "new PSI point already exists");
        require(emailNullifiers[emailNullifier] == false, "email nullifier already used");

        require(
            verifier.verifyAccountCreationProof(
                relayers[msg.sender].randHash,
                newEmailAddrPointer,
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

        vkCommitmentOfPointer[newEmailAddrPointer] = newVKCommitment;
        walletSaltOfVKCommitment[newVKCommitment] = walletSaltOfVKCommitment[oldVKCommitment];
        vkCommitments[newVKCommitment].relayer = msg.sender;
        pointerOfPSIPoint[newPSIPoint] = newEmailAddrPointer;
        vkCommitments[newVKCommitment].initialized = true;
        vkCommitments[newVKCommitment].nullified = false;

        walletSaltOfVKCommitment[oldVKCommitment] = bytes32(0);
        vkCommitments[oldVKCommitment].nullified = true;
    }

    /// @notice Register unclaimed fund for the recipient - for external users to deposit tokens to an email address.
    /// @param emailAddrCommitment Hash of the recipient's email address and a random number.
    /// @param tokenAddress Address of ERC20 token contract.
    /// @param amount Amount in WEI of the token.
    /// @param expiryTime Expiry time to claim the unclaimed fund. Set `0` to use default expiry.
    /// @param announceCommitmentRandomness Randomness used to generate the `emailAddrCommitment` - if needs to be public.
    /// @param announceEmailAddr Email address of the recipient - if needs to be public.
    /// @dev   `UNCLAIMED_FUNDS_REGISTRATION_FEE` ETH should be supplied to this function.
    /// @dev   `announceCommitmentRandomness` and `announceEmailAddr` are optional. They are not validated as well.
    function registerUnclaimedFund(
        bytes32 emailAddrCommitment,
        address tokenAddress,
        uint256 amount,
        uint256 expiryTime,
        uint256 announceCommitmentRandomness,
        string memory announceEmailAddr
    ) public payable {
        // Ensure the sender has paid ETH needed for claiming the unclaimed fee
        require(msg.value == unclaimedFundRegistrationFee, "invalid unclaimed fund fee");
        require(amount > 0, "amount should be greater than 0");
        require(tokenAddress != address(0), "invalid token contract");
        require(emailAddrCommitment != bytes32(0), "invalid emailAddrCommitment");
        require(expiryTime > block.timestamp, "invalid expiry time");
        require(unclaimedFundOfEmailAddrCommitment[emailAddrCommitment].amount == 0, "unclaimed fund exists");

        // Transfer token from sender to Core contract - sender should have set enough allowance for Core contract
        ERC20(tokenAddress).transferFrom(msg.sender, address(this), amount);

        _registerUnclaimedFund(
            msg.sender,
            emailAddrCommitment,
            tokenAddress,
            amount,
            expiryTime,
            announceCommitmentRandomness,
            announceEmailAddr
        );
    }

    /// Claim an unclaimed fund to the recipient's (initialized) wallet.
    /// @param emailAddrCommitment The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @param recipientEmailAddrPointer The pointer to the recipient's email address.
    /// @param proof Proof as required by verifier - prove `pointer` and `commitment` are of the same email address.
    function claimUnclaimedFund(
        bytes32 emailAddrCommitment,
        bytes32 recipientEmailAddrPointer,
        bytes memory proof
    ) public nonReentrant {
        UnclaimedFund storage fund = unclaimedFundOfEmailAddrCommitment[recipientEmailAddrPointer];
        bytes32 vkCommitment = vkCommitmentOfPointer[recipientEmailAddrPointer];

        require(relayers[msg.sender].randHash != bytes32(0), "caller not relayer");
        require(vkCommitments[vkCommitment].relayer == msg.sender, "invalid relayer for account");
        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime > block.timestamp, "unclaimed fund expired");
        require(vkCommitmentOfPointer[recipientEmailAddrPointer] != bytes32(0), "invalid VK commitment");
        require(vkCommitments[vkCommitment].initialized, "account not initialized");
        require(!vkCommitments[vkCommitment].nullified, "account is nullified");
        require(walletSaltOfVKCommitment[vkCommitment] != bytes32(0), "invalid wallet salt");

        require(
            verifier.verifyClaimFundProof(
                relayers[msg.sender].randHash,
                recipientEmailAddrPointer,
                emailAddrCommitment,
                proof
            ),
            "invalid proof"
        );

        address recipientAddress = getWalletOfSalt(walletSaltOfVKCommitment[vkCommitment]);

        delete unclaimedFundOfEmailAddrCommitment[recipientEmailAddrPointer];

        // Transfer token from Core contract to recipient's wallet
        ERC20(fund.tokenAddress).transfer(recipientAddress, fund.amount);

        // Transfer claim fee to the sender (relayer)
        _transferETH(msg.sender, unclaimedFundRegistrationFee);

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
        _transferETH(fund.senderAddress, unclaimedFundRegistrationFee);

        emit UnclaimedFundReverted(emailAddrCommitment, fund.tokenAddress, fund.amount, fund.senderAddress);
    }

    /// Register unclaimed state of an extension for the recipient email address commitment
    /// @param emailAddrCommitment Email address commitment of the recipient
    /// @param extensionAddress Address of the extension contract
    /// @param state State to be registered
    /// @param expiryTime Expiry time to claim the unclaimed state.
    /// @param announceCommitmentRandomness Randomness used to generate the `emailAddrCommitment` - if needs to be public.
    /// @param announceEmailAddr Email address of the recipient - if needs to be public.
    function registerUnclaimedState(
        bytes32 emailAddrCommitment,
        address extensionAddress,
        bytes memory state,
        uint256 expiryTime,
        uint256 announceCommitmentRandomness,
        string memory announceEmailAddr
    ) public payable nonReentrant {
        if (expiryTime == 0) {
            expiryTime = block.timestamp + unclaimedFundExpirationDuration;
        }

        // Ensure the sender has paid ETH needed for claiming the unclaimed fee
        require(msg.value == unclaimedFundRegistrationFee, "invalid unclaimed state fee");

        require(state.length > 0, "state cannot be empty");
        require(emailAddrCommitment != bytes32(0), "invalid emailAddrCommitment");
        require(expiryTime > block.timestamp, "invalid expiry time");
        require(
            unclaimedStateOfEmailAddrCommitment[emailAddrCommitment].senderAddress == address(0),
            "unclaimed state exists"
        );

        UnclaimedState memory us = UnclaimedState({
            emailAddrCommitment: emailAddrCommitment,
            extensionAddress: extensionAddress,
            senderAddress: msg.sender,
            state: state
        });

        Extension extension = Extension(extensionAddress);
        bool registered = extension.registerUnclaimedState(us);

        require(registered, "unclaimed state reg failed");

        unclaimedStateOfEmailAddrCommitment[emailAddrCommitment] = us;

        emit UnclaimedStateRegistered(
            emailAddrCommitment,
            extensionAddress,
            msg.sender,
            expiryTime,
            state,
            announceCommitmentRandomness,
            announceEmailAddr
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
        require(msg.sender == currContext.extensionAddress, "invalid caller");
        require(currContext.unclaimedStateRegistered == false, "unclaimed state exists");
        require(state.length > 0, "state cannot be empty");
        require(extensionAddress != address(0), "invalid extension contract");
        require(emailAddrCommitment != bytes32(0), "invalid emailAddrCommitment");

        uint256 expiryTime = block.timestamp + unclaimedFundExpirationDuration;

        UnclaimedState memory us = UnclaimedState({
            emailAddrCommitment: emailAddrCommitment,
            extensionAddress: extensionAddress,
            senderAddress: currContext.walletAddress,
            state: state
        });

        unclaimedStateOfEmailAddrCommitment[emailAddrCommitment] = us;
        currContext.unclaimedStateRegistered = true;

        emit UnclaimedStateRegistered(
            emailAddrCommitment,
            extensionAddress,
            currContext.walletAddress,
            expiryTime,
            state,
            0,
            ""
        );
    }

    /// Claim unclaimed state to the recipient's (initialized) wallet.
    /// @param emailAddrCommitment The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @param recipientEmailAddrPointer The pointer to the recipient's email address.
    /// @param proof Proof as required by verifier - prove `pointer` and `commitment` are of the same email address.
    function claimUnclaimedState(
        bytes32 emailAddrCommitment,
        bytes32 recipientEmailAddrPointer,
        bytes memory proof
    ) public nonReentrant {
        UnclaimedState storage us = unclaimedStateOfEmailAddrCommitment[recipientEmailAddrPointer];
        bytes32 vkCommitment = vkCommitmentOfPointer[recipientEmailAddrPointer];

        require(relayers[msg.sender].randHash != bytes32(0), "caller not relayer");
        require(vkCommitments[vkCommitment].relayer == msg.sender, "invalid relayer for account");
        require(us.senderAddress != address(0), "unclaimed state not registered");
        require(us.extensionAddress != address(0), "invalid extension address");
        require(vkCommitmentOfPointer[recipientEmailAddrPointer] != bytes32(0), "invalid VK commitment");
        require(vkCommitments[vkCommitment].initialized, "account not initialized");
        require(!vkCommitments[vkCommitment].nullified, "account is nullified");
        require(walletSaltOfVKCommitment[vkCommitment] != bytes32(0), "invalid wallet salt");

        require(
            verifier.verifyClaimFundProof(
                relayers[msg.sender].randHash,
                recipientEmailAddrPointer,
                emailAddrCommitment,
                proof
            ),
            "invalid proof"
        );

        address recipientAddress = getWalletOfSalt(walletSaltOfVKCommitment[vkCommitment]);

        delete unclaimedStateOfEmailAddrCommitment[recipientEmailAddrPointer];

        Extension extension = Extension(us.extensionAddress);
        extension.claimUnclaimedState(us, recipientAddress);

        // Transfer claim fee to the sender (relayer)
        _transferETH(msg.sender, unclaimedFundRegistrationFee);

        emit UnclaimedStateClaimed(emailAddrCommitment, recipientAddress);
    }

    /// @notice Return unclaimed state after expiry time
    /// @param emailAddrCommitment The commitment of the recipient's email address to which the unclaimed state was registered.
    function revertUnclaimedState(bytes32 emailAddrCommitment) public nonReentrant {
        UnclaimedState storage us = unclaimedStateOfEmailAddrCommitment[emailAddrCommitment];

        require(us.senderAddress != address(0), "unclaimed state not registered");

        delete unclaimedStateOfEmailAddrCommitment[emailAddrCommitment];

        Extension extension = Extension(us.extensionAddress);
        extension.revertUnclaimedState(us);

        // Transfer claim fee to the sender - either emailWallet user or external wallet
        _transferETH(us.senderAddress, unclaimedFundRegistrationFee);

        emit UnclaimedStateReverted(emailAddrCommitment, us.senderAddress);
    }

    /// @notice Return the extension address for a command and user
    /// @param command Command for which the extension address is to be returned
    /// @param emailAddrPointer Pointer of the user's email address
    function getExtensionForCommand(string memory command, bytes32 emailAddrPointer) public view returns (address) {
        address extensionAddress = extensionOfCommand[command]; // Global extension installed by default for all users
        address userExtensionAddress = userExtensionOfCommand[emailAddrPointer][command];

        if (userExtensionAddress != address(0)) {
            extensionAddress = userExtensionAddress;
        }

        return extensionAddress;
    }

    /// @notice Validate an EmailOp, including proof verification
    /// @param emailOp EmailOp to be validated
    function validateEmailOp(EmailOp memory emailOp) public view {
        bytes32 dkimPublicKeyHash = bytes32(dkimRegistry.getDKIMPublicKeyHash(emailOp.emailDomain));
        bytes32 vkCommitment = vkCommitmentOfPointer[emailOp.emailAddrPointer];

        require(dkimPublicKeyHash != bytes32(0), "cannot find DKIM for domain");
        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        ViewingKeyCommitment memory vkCommitInfo = vkCommitments[vkCommitment];
        require(vkCommitInfo.relayer == msg.sender, "invalid relayer");
        require(vkCommitInfo.initialized, "account not initialized");
        require(!vkCommitInfo.nullified, "account is nullified");
        require(walletSaltOfVKCommitment[vkCommitment] != bytes32(0), "wallet salt not set");
        require(emailNullifiers[emailOp.emailNullifier] == false, "email nullifier already used");
        require(bytes(emailOp.command).length != 0, "command cannot be empty");
        require(_getFeeConversionRate(emailOp.feeTokenName) != 0, "unsupported fee token");
        require(emailOp.feePerGas <= maxFeePerGas, "fee per gas too high");

        if (emailOp.hasEmailRecipient) {
            require(emailOp.recipientETHAddr == address(0), "cannot have both recipient types");
            require(emailOp.recipientEmailAddrCommitment != bytes32(0), "recipient commitment not found");
            require(
                unclaimedFundOfEmailAddrCommitment[emailOp.recipientEmailAddrCommitment].amount == 0,
                "Unclaimed fund exist"
            );
            require(
                unclaimedStateOfEmailAddrCommitment[emailOp.recipientEmailAddrCommitment].state.length == 0,
                "Unclaimed state exists"
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
                relayers[msg.sender].randHash,
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
            require(msg.value == unclaimedFundRegistrationFee, "invalid unclaimed fund fee");
        }

        currContext = ExecutionContext({
            walletAddress: getWalletOfSalt(walletSaltOfVKCommitment[vkCommitmentOfPointer[emailOp.emailAddrPointer]]),
            extensionAddress: address(0),
            receivedETH: msg.value,
            unclaimedStateRegistered: false,
            unclaimedFundRegistered: false
        });

        validateEmailOp(emailOp);

        emailNullifiers[emailOp.emailNullifier] = true;

        (success, returnData) = _executeEmailOp(emailOp);

        // Refund ETH to relayer if unclaimed funds were not registered
        if (!currContext.unclaimedFundRegistered && !currContext.unclaimedStateRegistered) {
            _transferETH(msg.sender, msg.value);
        }

        // Refund gas cost to the relayer from sender (in the fee token)
        uint256 feeForRefund = 2100; // TODO : Calculate actual cost to process the refund
        uint256 consumedGas = initialGas - gasleft() + feeForRefund;

        uint256 totalFee = (consumedGas * emailOp.feePerGas);
        if (currContext.unclaimedFundRegistered || currContext.unclaimedStateRegistered) {
            totalFee += unclaimedFundRegistrationFee;
        }

        uint256 feeAmount = totalFee / _getFeeConversionRate(emailOp.feeTokenName);
        address feeToken = _getTokenAddressFromEmailOpTokenName(emailOp.feeTokenName);

        if (feeAmount > 0) {
            _transferERC20(currContext.walletAddress, msg.sender, feeToken, feeAmount);
        }
    }

    /// @notice For extensions to request token from user's wallet
    /// @param tokenAddress Address of the ERC20 token requested
    /// @param amount Amount requested
    function requestTokenTransfer(address tokenAddress, uint256 amount) public {
        require(msg.sender == currContext.extensionAddress, "invalid caller");

        // TODO: Validate the requested token and amound is allowed.

        _transferERC20(currContext.walletAddress, currContext.extensionAddress, tokenAddress, amount);
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
    ) internal view returns (string memory maskedSubject, bool isExtension) {
        // Sample: Send 1 ETH to recipient@domain.com
        if (Strings.equal(emailOp.command, Commands.SEND_COMMAND)) {
            WalletParams memory walletParams = emailOp.walletParams;
            ERC20 token = ERC20(_getTokenAddressFromEmailOpTokenName(emailOp.walletParams.tokenName));

            require(token != ERC20(address(0)), "token not supported");

            maskedSubject = string.concat(
                Commands.SEND_COMMAND,
                " ",
                // TODO: Ensure to string conversion doesn't truncate decimal.
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
        else if (Strings.equal(emailOp.command, Commands.SET_EXTENSION_COMMAND)) {
            ExtensionManagerParams memory extManagerParams = emailOp.extManagerParams;

            require(addressOfExtension[extManagerParams.extensionName] != address(0), "extension not registered");

            maskedSubject = string.concat(
                Commands.SET_EXTENSION_COMMAND,
                " for ",
                extManagerParams.command,
                " as ",
                extManagerParams.extensionName
            );
        }
        // Sample: Remove extension for Swap
        else if (Strings.equal(emailOp.command, Commands.REMOVE_EXTENSION_COMMAND)) {
            maskedSubject = string.concat(Commands.REMOVE_EXTENSION_COMMAND, " for ", emailOp.extManagerParams.command);
        }
        // The command is for an extension
        else {
            isExtension = true;
            address extensionAddress = getExtensionForCommand(emailOp.command, emailOp.emailAddrPointer);

            require(extensionAddress != address(0), "invalid command or extension");

            Extension extension = Extension(extensionAddress);
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
    function _executeEmailOp(EmailOp memory emailOp) internal returns (bool success, bytes memory returnData) {
        // Wallet operation
        if (Strings.equal(emailOp.command, Commands.SEND_COMMAND)) {
            WalletParams memory walletParams = emailOp.walletParams;
            address tokenAddress = _getTokenAddressFromEmailOpTokenName(emailOp.walletParams.tokenName);
            address recipient = emailOp.hasEmailRecipient ? address(this) : emailOp.recipientETHAddr;

            (success, returnData) = _transferERC20(
                currContext.walletAddress,
                recipient,
                tokenAddress,
                walletParams.amount
            );

            if (!success) {
                return (success, returnData);
            }

            // Register unclaimed fund for the recipient
            if (emailOp.hasEmailRecipient) {
                _registerUnclaimedFund(
                    currContext.walletAddress,
                    emailOp.recipientEmailAddrCommitment,
                    tokenAddress,
                    walletParams.amount,
                    0,
                    0,
                    ""
                );
                currContext.unclaimedStateRegistered = true;
            }
        }
        // Set custom extension for the user
        else if (Strings.equal(emailOp.command, Commands.SET_EXTENSION_COMMAND)) {
            ExtensionManagerParams memory extManagerParams = emailOp.extManagerParams;
            address extensionAddress = addressOfExtension[extManagerParams.extensionName];

            userExtensionOfCommand[emailOp.emailAddrPointer][extManagerParams.command] = extensionAddress;
        }
        // Remove custom extension for the user
        else if (Strings.equal(emailOp.command, Commands.REMOVE_EXTENSION_COMMAND)) {
            userExtensionOfCommand[emailOp.emailAddrPointer][emailOp.command] = address(0);
        }
        // The command is for an extension
        else {
            address extAddress = getExtensionForCommand(emailOp.command, emailOp.emailAddrPointer);
            currContext.extensionAddress = extAddress;

            (success, returnData) = extAddress.call(
                abi.encodeWithSignature(
                    "execute(bytes,uint8,address,bytes32)",
                    emailOp.extensionParams,
                    emailOp.extensionSubjectTemplateIndex,
                    currContext.walletAddress,
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
        string memory announceEmailAddr
    ) internal {
        if (expiryTime == 0) {
            expiryTime = block.timestamp + unclaimedFundExpirationDuration;
        }

        UnclaimedFund memory fund = UnclaimedFund({
            emailAddrCommitment: emailAddrCommitment,
            tokenAddress: tokenAddress,
            amount: amount,
            expiryTime: expiryTime,
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
            announceEmailAddr
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
    }

    /// @notice Transfer ERC20 token from user's wallet to given recipient
    /// @param senderAddress Address of the sender's wallet
    /// @param recipientAddress Address of the recipient
    /// @param tokenAddress Address of ERC20 token contract.
    /// @param amount Amount in WEI of the token.
    function _transferERC20(
        address senderAddress,
        address recipientAddress,
        address tokenAddress,
        uint256 amount
    ) internal returns (bool success, bytes memory returnData) {
        require(tokenAddress != address(0), "invalid token address");
        require(amount > 0, "invalid amount");
        require(senderAddress != address(0), "invalid sender address");
        require(recipientAddress != address(0), "invalid recipient address");

        Wallet wallet = Wallet(payable(senderAddress));

        try
            wallet.execute(
                tokenAddress,
                0,
                abi.encodeWithSignature("transfer(address,uint256)", recipientAddress, amount)
            )
        {
            success = true;
        } catch Error(string memory reason) {
            return (false, bytes(reason));
        } catch {
            return (false, bytes("unknown wallet exec error"));
        }
    }

    /// @notice Trasnfer ETH from core contract to recipient
    /// @param recipient    Address of the recipient
    /// @param amount      Amount in WEI to be transferred
    function _transferETH(address recipient, uint256 amount) internal {
        (bool sent, ) = payable(recipient).call{value: amount}("");
        require(sent, "failed to transfer ETH");
    }

    function _getTokenAddressFromEmailOpTokenName(string memory tokenName) internal view returns (address) {
        if (Strings.equal(tokenName, "ETH")) {
            return tokenRegistry.getTokenAddress("WETH");
        }

        return tokenRegistry.getTokenAddress(tokenName);
    }

    function _getFeeConversionRate(string memory tokenName) internal view returns (uint256) {
        if (Strings.equal(tokenName, "ETH") || Strings.equal(tokenName, "WETH")) {
            return 1;
        }

        bool validToken = Strings.equal(tokenName, "DAI") || Strings.equal(tokenName, "USDC");
        if (validToken) {
            return 0;
        }

        address tokenAddr = tokenRegistry.getTokenAddress(tokenName);
        if (tokenAddr == address(0)) {
            return 0;
        }

        return priceOracle.getRecentPriceInETH(tokenAddr);
    }

    /// @notice Upgrade the implementation of the proxy contract
    /// @param newImplementation Address of the new implementation contract
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
