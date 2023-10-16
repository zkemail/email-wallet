// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Create2Upgradeable} from "@openzeppelin/contracts-upgradeable/utils/Create2Upgradeable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {IERC20, ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {LibZip} from "solady/utils/LibZip.sol";
import {DKIMRegistry} from "@zk-email/contracts/DKIMRegistry.sol";
import {DecimalUtils} from "./libraries/DecimalUtils.sol";
import {BytesUtils} from "./libraries/BytesUtils.sol";
import {TokenRegistry} from "./utils/TokenRegistry.sol";
import {IVerifier} from "./interfaces/IVerifier.sol";
import {Extension} from "./interfaces/Extension.sol";
import {IPriceOracle} from "./interfaces/IPriceOracle.sol";
import {EmailWalletEvents} from "./interfaces/Events.sol";
import {Wallet} from "./Wallet.sol";
import "./interfaces/Types.sol";
import "./interfaces/Commands.sol";

contract EmailWalletCore is EmailWalletEvents, ReentrancyGuard, OwnableUpgradeable, UUPSUpgradeable {
    string public constant TOKEN_AMOUNT_TEMPLATE = "{tokenAmount}";
    string public constant AMOUNT_TEMPLATE = "{amount}";
    string public constant STRING_TEMPLATE = "{string}";
    string public constant UINT_TEMPLATE = "{uint}";
    string public constant INT_TEMPLATE = "{int}";
    string public constant ADDRESS_TEMPLATE = "{address}";
    string public constant RECIPIENT_TEMPLATE = "{recipient}";


    // ZK proof verifier
    IVerifier public immutable verifier;

    // Default DKIM public key hashes registry
    address public immutable defaultDkimRegistry;

    // Token registry
    TokenRegistry public immutable tokenRegistry;

    // Price oracle for feeToken conversion
    IPriceOracle public immutable priceOracle;

    // Address of WETH contract
    address public immutable weth;

    // Address of wallet implementation contract - used for deploying wallets for users via proxy
    address public immutable walletImplementation;

    // Mapping of relayer's wallet address to relayer config
    mapping(address => RelayerConfig) public relayers;

    // Mapping of relayer's randHash to relayer's wallet address
    mapping(bytes32 => address) public relayerOfRandHash;

    // Mapping of relayer's email address to relayer's wallet address
    mapping(string => address) public relayerOfEmailAddr;

    // Mapping of emailAddrPointer to accountKeyCommit
    mapping(bytes32 => bytes32) public accountKeyCommitOfPointer;

    // Mapping of PSI point to emailAddrPointer
    mapping(bytes => bytes32) public pointerOfPSIPoint;

    // Mapping of accountKeyCommit to account key details
    mapping(bytes32 => AccountKeyInfo) public infoOfAccountKeyCommit;

    // Mapping of walletSalt to dkim registry address
    mapping(bytes32 => address) public dkimRegistryOfWalletSalt;

    // Mapping to store email nullifiers
    mapping(bytes32 => bool) public emailNullifiers;

    // Mapping from extensio name to extension address, as published by the developer
    mapping(string => address) public addressOfExtension;

    // Global mapping of command name to extension address enabled for all users by default
    mapping(string => address) public defaultExtensionOfCommand;

    // Mapping of extension address to list of subjectTemplates
    // Each subject template is array of strings, where each item is a "matcher" or constant string
    // Eg: `["Swap", "{tokenAmount}", "to", "{string}"]`
    mapping(address => string[][]) public subjectTemplatesOfExtension;

    // Mapping of extension address to maximum gas that will be consumed by `extension.execute()`
    // Relayer can use this to ensure user has enough tokens to pay for the gas
    mapping(address => uint256) public maxGasOfExtension;

    // User level mapping of command name to extension address (walletAddress -> (command -> extension))
    mapping(address => mapping(string => address)) public userExtensionOfCommand;

    // Mapping of recipient's emailAddrCommit (hash(email, randomness)) to the unclaimedFund
    mapping(bytes32 => UnclaimedFund) public unclaimedFundOfEmailAddrCommit;

    // Mapping of emailAddrCommit to unclaimed state
    mapping(bytes32 => UnclaimedState) public unclaimedStateOfEmailAddrCommit;

    // Max fee per gas in ETH that relayer can set in a UserOp
    uint256 public immutable maxFeePerGas;

    // Time period until which a email is valid for EmailOp based on the timestamp of the email signature
    uint256 public immutable emailValidityDuration;

    // Gas required to claim unclaimed funds. User (their relayer) who register unclaimed funds
    // need to lock this amount which is relesed to the relayer who claims it
    uint256 public immutable unclaimedFundClaimGas;

    // Gas required to claim unclaimed state
    uint256 public immutable unclaimedStateClaimGas;

    // Default expiry duration for unclaimed funds and states
    uint256 public immutable unclaimsExpiryDuration;

    // Context of currently executing EmailOp - reset on every EmailOp
    ExecutionContext internal currContext;

    constructor(
        address _verifier,
        address _tokenRegistry,
        address _defaultDkimRegistry,
        address _priceOracle,
        address _wethContract,
        uint256 _maxFeePerGas,
        uint256 _emailValidityDuration,
        uint256 _unclaimedFundClaimGas,
        uint256 _unclaimedStateClaimGas,
        uint256 _unclaimedFundExpirationDuration
    ) {
        verifier = IVerifier(_verifier);
        defaultDkimRegistry = _defaultDkimRegistry;
        tokenRegistry = TokenRegistry(_tokenRegistry);
        priceOracle = IPriceOracle(_priceOracle);
        weth = _wethContract;
        maxFeePerGas = _maxFeePerGas;
        emailValidityDuration = _emailValidityDuration;
        unclaimedFundClaimGas = _unclaimedFundClaimGas;
        unclaimedStateClaimGas = _unclaimedStateClaimGas;
        unclaimsExpiryDuration = _unclaimedFundExpirationDuration;

        walletImplementation = address(new Wallet(_wethContract));
    }

    fallback() external payable {
        LibZip.cdFallback();
    }

    receive() external payable {
        revert();
    }

    function initialize() public initializer {
        __Ownable_init();
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
    /// @param accountKeyCommit hash(accountKey, emailAddr, relayerHash)
    /// @param walletSalt hash(accountKey, 0)
    /// @param proof ZK proof as required by the verifier
    function createAccount(
        bytes32 emailAddrPointer,
        bytes32 accountKeyCommit,
        bytes32 walletSalt,
        bytes memory psiPoint,
        bytes memory proof
    ) public returns (Wallet wallet) {
        bool initialized = infoOfAccountKeyCommit[accountKeyCommit].initialized;
        bool nullified = infoOfAccountKeyCommit[accountKeyCommit].nullified;
        bool walletSaltSet = infoOfAccountKeyCommit[accountKeyCommit].walletSaltSet;

        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(accountKeyCommitOfPointer[emailAddrPointer] == bytes32(0), "pointer exists");
        require(pointerOfPSIPoint[psiPoint] == bytes32(0), "PSI point exists");
        require(!walletSaltSet, "walletSalt exists");
        require(!initialized, "account is initialized");
        require(!nullified, "account is nullified");
        require(Address.isContract(getWalletOfSalt(walletSalt)) == false, "wallet already deployed");

        require(
            verifier.verifyAccountCreationProof(
                relayers[msg.sender].randHash,
                emailAddrPointer,
                accountKeyCommit,
                walletSalt,
                psiPoint,
                proof
            ),
            "invalid account creation proof"
        );

        accountKeyCommitOfPointer[emailAddrPointer] = accountKeyCommit;
        infoOfAccountKeyCommit[accountKeyCommit].relayer = msg.sender;
        infoOfAccountKeyCommit[accountKeyCommit].walletSaltSet = true;
        infoOfAccountKeyCommit[accountKeyCommit].walletSalt = walletSalt;
        dkimRegistryOfWalletSalt[walletSalt] = defaultDkimRegistry;
        // infoOfAccountKeyCommit[accountKeyCommit].dkimRegistry = defaultDkimRegistry;

        pointerOfPSIPoint[psiPoint] = emailAddrPointer;

        wallet = _deployWallet(walletSalt);

        emit AccountCreated(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint);
    }

    /// Initialize the account when user reply to invitation email
    /// @param emailAddrPointer hash(relayerRand, emailAddr)
    /// @param emailDomain domain name of the sender's email
    /// @param emailNullifier nullifier of the email used for proof generation
    /// @param proof ZK proof as required by the verifier
    function initializeAccount(
        bytes32 emailAddrPointer,
        string memory emailDomain,
        uint256 emailTimestamp,
        bytes32 emailNullifier,
        bytes memory proof
    ) public {
        bytes32 accountKeyCommit = accountKeyCommitOfPointer[emailAddrPointer];

        require(emailTimestamp + emailValidityDuration > block.timestamp, "email expired");
        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(accountKeyCommit != bytes32(0), "account not registered");
        require(infoOfAccountKeyCommit[accountKeyCommit].relayer == msg.sender, "invalid relayer");
        require(infoOfAccountKeyCommit[accountKeyCommit].nullified == false, "account is nullified");
        require(infoOfAccountKeyCommit[accountKeyCommit].initialized == false, "account already initialized");
        require(emailNullifiers[emailNullifier] == false, "email nullifier already used");
        DKIMRegistry dkimRegistry = DKIMRegistry(dkimRegistryOfWalletSalt[infoOfAccountKeyCommit[accountKeyCommit].walletSalt]);

        require(
            verifier.verifyAccountInitializaionProof(
                emailDomain,
                bytes32(dkimRegistry.getDKIMPublicKeyHash(emailDomain)),
                emailTimestamp,
                relayers[msg.sender].randHash,
                emailAddrPointer,
                accountKeyCommit,
                emailNullifier,
                proof
            ),
            "invalid account initialization proof"
        );

        infoOfAccountKeyCommit[accountKeyCommit].initialized = true;
        emailNullifiers[emailNullifier] = true;

        emit AccountInitialized(
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
        bytes32 oldAccountKeyCommit,
        bytes32 newEmailAddrPointer,
        bytes32 newAccountKeyCommit,
        bytes memory newPSIPoint,
        EmailProof memory transportEmailProof,
        bytes memory accountCreationProof
    ) public {
        AccountKeyInfo storage oldAccountKeyInfo = infoOfAccountKeyCommit[oldAccountKeyCommit];

        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(oldAccountKeyInfo.relayer != address(0), "old relayer not registered");
        require(oldAccountKeyInfo.relayer != msg.sender, "new relayer cannot be same");
        require(oldAccountKeyInfo.initialized, "account not initialized");
        require(!oldAccountKeyInfo.nullified, "account is nullified");
        require(transportEmailProof.timestamp + emailValidityDuration > block.timestamp, "email expired");
        require(emailNullifiers[transportEmailProof.nullifier] == false, "email nullifier already used");

        // New relayer might have already created an account, but not initialized.
        bytes32 existingAccountKeyOfNewPointer = accountKeyCommitOfPointer[newEmailAddrPointer];
        if (existingAccountKeyOfNewPointer != bytes32(0)) {
            require(
                !infoOfAccountKeyCommit[existingAccountKeyOfNewPointer].initialized,
                "existing account key is initialized"
            );
        } else {
            require(!infoOfAccountKeyCommit[newAccountKeyCommit].initialized, "new account is already initialized");
            require(!infoOfAccountKeyCommit[newAccountKeyCommit].nullified, "new account is already nullified");
            require(!infoOfAccountKeyCommit[newAccountKeyCommit].walletSaltSet, "walletSalt already exists");
            require(pointerOfPSIPoint[newPSIPoint] == bytes32(0), "new PSI point already exists");
        }

        require(
            verifier.verifyAccountCreationProof(
                relayers[msg.sender].randHash,
                newEmailAddrPointer,
                newAccountKeyCommit,
                oldAccountKeyInfo.walletSalt,
                newPSIPoint,
                accountCreationProof
            ),
            "invalid account creation proof"
        );

        require(
            verifier.verifyAccountTransportProof(
                transportEmailProof.domain,
                bytes32(DKIMRegistry(dkimRegistryOfWalletSalt[oldAccountKeyInfo.walletSalt]).getDKIMPublicKeyHash(transportEmailProof.domain)),
                transportEmailProof.timestamp,
                transportEmailProof.nullifier,
                relayers[oldAccountKeyInfo.relayer].randHash,
                relayers[msg.sender].randHash,
                oldAccountKeyCommit,
                newAccountKeyCommit,
                transportEmailProof.proof
            ),
            "invalid account transport proof"
        );

        emailNullifiers[transportEmailProof.nullifier] = true;

        if (existingAccountKeyOfNewPointer != bytes32(0)) {
            delete infoOfAccountKeyCommit[existingAccountKeyOfNewPointer];
        }

        accountKeyCommitOfPointer[newEmailAddrPointer] = newAccountKeyCommit;
        pointerOfPSIPoint[newPSIPoint] = newEmailAddrPointer;
        infoOfAccountKeyCommit[newAccountKeyCommit].walletSalt = infoOfAccountKeyCommit[oldAccountKeyCommit].walletSalt;
        infoOfAccountKeyCommit[newAccountKeyCommit].walletSaltSet = true;
        infoOfAccountKeyCommit[newAccountKeyCommit].relayer = msg.sender;
        infoOfAccountKeyCommit[newAccountKeyCommit].initialized = true;
        infoOfAccountKeyCommit[newAccountKeyCommit].nullified = false;
        // infoOfAccountKeyCommit[newAccountKeyCommit].dkimRegistry = oldAccountKeyInfo.dkimRegistry;

        infoOfAccountKeyCommit[oldAccountKeyCommit].walletSalt = bytes32(0);
        infoOfAccountKeyCommit[oldAccountKeyCommit].walletSaltSet = false;
        infoOfAccountKeyCommit[oldAccountKeyCommit].nullified = true;
        // infoOfAccountKeyCommit[oldAccountKeyCommit].dkimRegistry = address(0);
        infoOfAccountKeyCommit[oldAccountKeyCommit].initialized = false;

        emit AccountTransported(oldAccountKeyCommit, newEmailAddrPointer, newAccountKeyCommit);
    }

    /// @notice Validate an EmailOp, including proof verification
    /// @param emailOp EmailOp to be validated
    function validateEmailOp(EmailOp memory emailOp) public view {
        bytes32 accountKeyCommit = accountKeyCommitOfPointer[emailOp.emailAddrPointer];
        DKIMRegistry dkimRegistry = DKIMRegistry(dkimRegistryOfWalletSalt[infoOfAccountKeyCommit[accountKeyCommit].walletSalt]);
        bytes32 dkimPublicKeyHash = bytes32(dkimRegistry.getDKIMPublicKeyHash(emailOp.emailDomain));

        require(emailOp.timestamp + emailValidityDuration > block.timestamp, "email expired");
        require(dkimPublicKeyHash != bytes32(0), "cannot find DKIM for domain");
        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        {
            AccountKeyInfo storage accountKeyInfo = infoOfAccountKeyCommit[accountKeyCommit];
            address relayer = accountKeyInfo.relayer;
            bool initialized = accountKeyInfo.initialized;
            bool nullified = accountKeyInfo.nullified;
            bool walletSaltSet = accountKeyInfo.walletSaltSet;
            require(relayer == msg.sender, "invalid relayer");
            require(initialized, "account not initialized");
            require(!nullified, "account is nullified");
            require(walletSaltSet, "wallet salt not set");
        }
        require(emailNullifiers[emailOp.emailNullifier] == false, "email nullifier already used");
        require(bytes(emailOp.command).length != 0, "command cannot be empty");
        require(_getFeeConversionRate(emailOp.feeTokenName) != 0, "unsupported fee token");
        require(emailOp.feePerGas <= maxFeePerGas, "fee per gas too high");

        if (emailOp.hasEmailRecipient) {
            require(emailOp.recipientETHAddr == address(0), "cannot have both recipient types");
            require(emailOp.recipientEmailAddrCommit != bytes32(0), "recipientEmailAddrCommit not found");
            require(
                unclaimedFundOfEmailAddrCommit[emailOp.recipientEmailAddrCommit].sender == address(0),
                "unclaimed fund exist"
            );
            require(
                unclaimedStateOfEmailAddrCommit[emailOp.recipientEmailAddrCommit].sender == address(0),
                "unclaimed state exists"
            );
        } else {
            require(emailOp.recipientEmailAddrCommit == bytes32(0), "recipientEmailAddrCommit not allowed");
        }
        (string memory maskedSubject, ) = _computeMaskedSubjectForEmailOp(emailOp);
        require(Strings.equal(maskedSubject, emailOp.maskedSubject), string.concat("subject != ", maskedSubject));
        require(
            verifier.verifyEmailOpProof(
                emailOp.emailDomain,
                dkimPublicKeyHash,
                emailOp.timestamp,
                emailOp.maskedSubject,
                emailOp.emailNullifier,
                relayers[msg.sender].randHash,
                emailOp.emailAddrPointer,
                emailOp.hasEmailRecipient,
                emailOp.recipientEmailAddrCommit,
                emailOp.emailProof
            ),
            "invalid email proof"
        );
    }

    /// @notice Handle an EmailOp - the main function relayer should call for each Email
    /// @param emailOp EmailOp to be executed
    /// @dev ETH for unclaimed fund/state registration should be send if the recipient is an email address
    /// @dev Relayer should make sure user has enough tokens to pay for the fee. This can be calculated as
    /// @dev ~ verificationGas(fixed) + executionGas(extension.maxGas if extension) + feeForReimbursement(50k) + msg.value
    function handleEmailOp(
        EmailOp calldata emailOp
    ) public payable nonReentrant returns (bool success, bytes memory returnData) {
        uint256 initialGas = gasleft();

        // Reset context
        currContext.extensionAddr = address(0);
        currContext.unclaimedFundRegistered = false;
        currContext.unclaimedStateRegistered = false;
        delete currContext.tokenAllowances;
        currContext.recipientEmailAddrCommit = emailOp.recipientEmailAddrCommit;
        currContext.walletAddr = getWalletOfSalt(
            infoOfAccountKeyCommit[accountKeyCommitOfPointer[emailOp.emailAddrPointer]].walletSalt
        );
        // Validate emailOp - will revert on failure. Relayer should ensure validate pass by simulation.
        validateEmailOp(emailOp);

        emailNullifiers[emailOp.emailNullifier] = true;

        // Execute EmailOp - wont revert on failure. Relayer will be compensated for gas even in failure.
        (success, returnData) = _executeEmailOp(emailOp);

        uint totalFee; // Total fee in ETH to be paid to relayer

        require(
            !(currContext.unclaimedFundRegistered && currContext.unclaimedStateRegistered),
            "cannot register both unclaimed fund and state"
        );


        if (currContext.unclaimedFundRegistered) {
            require(msg.value == unclaimedFundClaimGas * maxFeePerGas, "incorrect ETH sent for unclaimed fund");
            totalFee += (unclaimedFundClaimGas * maxFeePerGas);
        } else if (currContext.unclaimedStateRegistered) {
            require(msg.value == unclaimedStateClaimGas * maxFeePerGas, "incorrect ETH sent for unclaimed state");
            totalFee += (unclaimedStateClaimGas * maxFeePerGas);
        } else {
            // Revert whatever was sent in case unclaimed fund/state registration didnt happen
            _transferETH(msg.sender, msg.value);
        }

        uint256 gasForRefund = 50000; // Rough estimate of gas cost for refund operation below (ERC20 transfer)
        uint256 consumedGas = initialGas - gasleft() + gasForRefund;
        totalFee += (consumedGas * emailOp.feePerGas);

        address feeToken = getTokenAddrFromName(emailOp.feeTokenName);
        uint256 rate = _getFeeConversionRate(emailOp.feeTokenName);
        uint256 feeAmount = (totalFee * rate) / (10 ** 18);


        if (feeAmount > 0) {
            (success, returnData) = _transferERC20(currContext.walletAddr, msg.sender, feeToken, feeAmount);
            require(success, string.concat("fee reimbursement failed: ", string(returnData)));
        }
    }

    /// @notice Register unclaimed fund for the recipient - for external users to deposit tokens to an email address.
    /// @param emailAddrCommit Hash of the recipient's email address and a random number.
    /// @param tokenAddr Address of ERC20 token contract.
    /// @param amount Amount in WEI of the token.
    /// @param expiryTime Expiry time to claim the unclaimed fund. Set `0` to use default expiry.
    /// @param announceCommitRandomness Randomness used to generate the `emailAddrCommit` - if needs to be public.
    /// @param announceEmailAddr Email address of the recipient - if needs to be public.
    /// @dev   `UNCLAIMED_FUNDS_REGISTRATION_FEE` ETH should be supplied to this function.
    /// @dev   `announceCommitRandomness` and `announceEmailAddr` are optional. They are not validated as well.
    function registerUnclaimedFund(
        bytes32 emailAddrCommit,
        address tokenAddr,
        uint256 amount,
        uint256 expiryTime,
        uint256 announceCommitRandomness,
        string memory announceEmailAddr
    ) public payable {
        if (expiryTime == 0) {
            expiryTime = block.timestamp + unclaimsExpiryDuration;
        }

        // Ensure the sender has paid ETH needed for claiming / expiring the unclaimed fee
        require(msg.value == unclaimedFundClaimGas * maxFeePerGas, "invalid unclaimed fund fee");
        require(amount > 0, "amount should be greater than 0");
        require(tokenAddr != address(0), "invalid token contract");
        require(emailAddrCommit != bytes32(0), "invalid emailAddrCommit");
        require(expiryTime > block.timestamp, "invalid expiry time");
        require(unclaimedFundOfEmailAddrCommit[emailAddrCommit].amount == 0, "unclaimed fund exists");

        // Transfer token from sender to Core contract - sender should have set enough allowance for Core contract
        ERC20(tokenAddr).transferFrom(msg.sender, address(this), amount);

        _registerUnclaimedFund(
            msg.sender,
            emailAddrCommit,
            tokenAddr,
            amount,
            expiryTime,
            announceCommitRandomness,
            announceEmailAddr
        );
    }

    /// Claim an unclaimed fund to the recipient's (initialized) wallet.
    /// @param emailAddrCommit The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @param recipientEmailAddrPointer The pointer to the recipient's email address.
    /// @param proof Proof as required by verifier - prove `pointer` and `commitment` are of the same email address.
    /// @dev Relayer should dry run this call, as they will only get claim fee (gas reimbursement) if this succeeds.
    function claimUnclaimedFund(
        bytes32 emailAddrCommit,
        bytes32 recipientEmailAddrPointer,
        bytes memory proof
    ) public nonReentrant {
        UnclaimedFund memory fund = unclaimedFundOfEmailAddrCommit[emailAddrCommit];
        bytes32 accountKeyCommit = accountKeyCommitOfPointer[recipientEmailAddrPointer];

        require(relayers[msg.sender].randHash != bytes32(0), "caller not relayer");
        require(infoOfAccountKeyCommit[accountKeyCommit].relayer == msg.sender, "invalid relayer for account");
        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime > block.timestamp, "unclaimed fund expired");
        require(accountKeyCommitOfPointer[recipientEmailAddrPointer] != bytes32(0), "invalid account key commit.");
        require(infoOfAccountKeyCommit[accountKeyCommit].initialized, "account not initialized");
        require(!infoOfAccountKeyCommit[accountKeyCommit].nullified, "account is nullified");
        require(infoOfAccountKeyCommit[accountKeyCommit].walletSalt != bytes32(0), "invalid wallet salt");

        require(
            verifier.verifyClaimFundProof(
                relayers[msg.sender].randHash,
                recipientEmailAddrPointer,
                emailAddrCommit,
                proof
            ),
            "invalid proof"
        );

        address recipientAddr = getWalletOfSalt(infoOfAccountKeyCommit[accountKeyCommit].walletSalt);

        delete unclaimedFundOfEmailAddrCommit[emailAddrCommit];

        // Transfer token from Core contract to recipient's wallet
        ERC20(fund.tokenAddr).transfer(recipientAddr, fund.amount);

        // Transfer claim fee to the sender (relayer)
        _transferETH(msg.sender, unclaimedFundClaimGas * maxFeePerGas);

        emit UnclaimedFundClaimed(emailAddrCommit, fund.tokenAddr, fund.amount, recipientAddr);
    }

    /// @notice Return unclaimed fund after expiry time
    /// @param emailAddrCommit The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @dev Callee should dry run this call, as they will only get claim fee (gas reimbursement) if this succeeds.
    function voidUnclaimedFund(bytes32 emailAddrCommit) public nonReentrant {
        uint256 initialGas = gasleft();

        UnclaimedFund memory fund = unclaimedFundOfEmailAddrCommit[emailAddrCommit];

        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime < block.timestamp, "unclaimed fund not expired");

        delete unclaimedFundOfEmailAddrCommit[emailAddrCommit];

        // Transfer token from Core contract to sender's wallet
        ERC20(fund.tokenAddr).transfer(fund.sender, fund.amount);

        // Gas consumed so far + approx. cost for 2 ETH transfers; Ignoring event emission gas
        uint256 consumedGas = initialGas - gasleft() + 21000 + 21000;

        // Transfer consumedGas to callee, and rest of the locked funds to user who locked up the funds
        _transferETH(fund.sender, (unclaimedFundClaimGas - consumedGas) * maxFeePerGas);
        _transferETH(msg.sender, consumedGas * maxFeePerGas);

        emit UnclaimedFundVoided(emailAddrCommit, fund.tokenAddr, fund.amount, fund.sender);
    }

    /// Register unclaimed state of an extension for the recipient email address commitment
    /// @param emailAddrCommit Email address commitment of the recipient
    /// @param extensionAddr Address of the extension contract
    /// @param state State to be registered
    /// @param expiryTime Expiry time to claim the unclaimed state.
    /// @param announceCommitRandomness Randomness used to generate the `emailAddrCommit` - if needs to be public.
    /// @param announceEmailAddr Email address of the recipient - if needs to be public.
    function registerUnclaimedState(
        bytes32 emailAddrCommit,
        address extensionAddr,
        bytes memory state,
        uint256 expiryTime,
        uint256 announceCommitRandomness,
        string memory announceEmailAddr
    ) public payable nonReentrant {
        if (expiryTime == 0) {
            expiryTime = block.timestamp + unclaimsExpiryDuration;
        }

        // Ensure the sender has paid ETH needed for claiming the unclaimed fee
        require(msg.value == unclaimedStateClaimGas * maxFeePerGas, "invalid unclaimed state fee");

        require(state.length > 0, "state cannot be empty");
        require(emailAddrCommit != bytes32(0), "invalid emailAddrCommit");
        require(expiryTime > block.timestamp, "invalid expiry time");
        require(unclaimedStateOfEmailAddrCommit[emailAddrCommit].sender == address(0), "unclaimed state exists");

        UnclaimedState memory us = UnclaimedState({
            emailAddrCommit: emailAddrCommit,
            extensionAddr: extensionAddr,
            sender: msg.sender,
            state: state,
            expiryTime: expiryTime
        });

        Extension extension = Extension(extensionAddr);

        try extension.registerUnclaimedState(us, false) {
        } catch Error(string memory reason) {
            revert(string.concat("unclaimed state reg err: ", reason));
        } catch {
            revert("unclaimed state reg err");
        }

        unclaimedStateOfEmailAddrCommit[emailAddrCommit] = us;

        emit UnclaimedStateRegistered(
            emailAddrCommit,
            extensionAddr,
            msg.sender,
            expiryTime,
            state,
            announceCommitRandomness,
            announceEmailAddr
        );
    }

    /// Claim unclaimed state to the recipient's (initialized) wallet.
    /// @param emailAddrCommit The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @param recipientEmailAddrPointer The pointer to the recipient's email address.
    /// @param proof Proof as required by verifier - prove `pointer` and `commitment` are of the same email address.
    function claimUnclaimedState(
        bytes32 emailAddrCommit,
        bytes32 recipientEmailAddrPointer,
        bytes memory proof
    ) public nonReentrant returns (bool success, bytes memory returnData) {
        uint256 initialGas = gasleft();

        UnclaimedState memory us = unclaimedStateOfEmailAddrCommit[emailAddrCommit];
        bytes32 accountKeyCommit = accountKeyCommitOfPointer[recipientEmailAddrPointer];

        require(relayers[msg.sender].randHash != bytes32(0), "caller not relayer");
        require(infoOfAccountKeyCommit[accountKeyCommit].relayer == msg.sender, "invalid relayer for account");
        require(us.sender != address(0), "unclaimed state not registered");
        require(us.extensionAddr != address(0), "invalid extension address");
        require(us.expiryTime > block.timestamp, "unclaimed state expired");
        require(accountKeyCommit != bytes32(0), "invalid account key commit.");
        require(infoOfAccountKeyCommit[accountKeyCommit].initialized, "account not initialized");
        require(!infoOfAccountKeyCommit[accountKeyCommit].nullified, "account is nullified");
        require(infoOfAccountKeyCommit[accountKeyCommit].walletSalt != bytes32(0), "invalid wallet salt");

        require(
            verifier.verifyClaimFundProof(
                relayers[msg.sender].randHash,
                recipientEmailAddrPointer,
                emailAddrCommit,
                proof
            ),
            "invalid proof"
        );

        address recipientAddr = getWalletOfSalt(infoOfAccountKeyCommit[accountKeyCommit].walletSalt);

        delete unclaimedStateOfEmailAddrCommit[emailAddrCommit];

        Extension extension = Extension(us.extensionAddr);

        // Deducated consumed gas + 21k for eth transer from `unclaimedStateClaimGas` and pass to extension
        uint256 gasForExt = unclaimedStateClaimGas - (initialGas - gasleft()) - 21000;

        // Relayer should get claim fee (gas reimbursement) even if extension call fails
        // Simulation wont work, as extension logic will depend on global variables
        try extension.claimUnclaimedState{gas: gasForExt}(us, recipientAddr) {
            success = true;
        } catch Error(string memory reason) {
            success = false;
            returnData = bytes(reason);
        } catch {
            success = false;
        }

        // Transfer claim fee to the sender (relayer)
        _transferETH(msg.sender, unclaimedStateClaimGas * maxFeePerGas);

        emit UnclaimedStateClaimed(emailAddrCommit, recipientAddr);
    }

    /// @notice Return unclaimed state after expiry time
    /// @param emailAddrCommit The commitment of the recipient's email address to which the unclaimed state was registered.
    function voidUnclaimedState(
        bytes32 emailAddrCommit
    ) public nonReentrant returns (bool success, bytes memory returnData) {
        uint256 initialGas = gasleft();

        UnclaimedState memory us = unclaimedStateOfEmailAddrCommit[emailAddrCommit];

        require(us.sender != address(0), "unclaimed state not registered");
        require(us.expiryTime < block.timestamp, "unclaimed state not expired");

        delete unclaimedStateOfEmailAddrCommit[emailAddrCommit];

        Extension extension = Extension(us.extensionAddr);

        // Gas consumed for verification and next steps is deducated from `unclaimedStateClaimGas`
        // and rest is passed to extension
        uint256 gasForExt = unclaimedStateClaimGas - (initialGas - gasleft()) - 21000 - 21000;

        // Callee should get gas reimbursement even if extension call fails
        // Simulation wont work, as extension logic can depend on global variables
        try extension.voidUnclaimedState{gas: gasForExt}(us) {
            success = true;
        } catch Error(string memory reason) {
            success = false;
            returnData = bytes(reason);
        } catch {
            success = false;
        }

        // Gas consumed so far + cost for 2 ETH transfers
        uint256 consumedGas = initialGas - gasleft() + 21000 + 21000;

        // Transfer consumedGas to callee, and rest of the locked funds to user who locked up the funds
        _transferETH(us.sender, (unclaimedStateClaimGas - consumedGas) * maxFeePerGas);
        _transferETH(msg.sender, consumedGas * maxFeePerGas);

        emit UnclaimedStateVoided(emailAddrCommit, us.sender);
    }

    /// Register unclaimed state from an extension
    /// @param extensionAddr Address of the extension contract to which the state is registered
    /// @param state State to be registered
    function registerUnclaimedStateAsExtension(address extensionAddr, bytes memory state) public {
        require(msg.sender == currContext.extensionAddr, "caller not extension");
        require(
            unclaimedStateOfEmailAddrCommit[currContext.recipientEmailAddrCommit].sender == address(0),
            "unclaimed state exists"
        );
        require(currContext.unclaimedStateRegistered == false, "only one unclaimed state reg allowed");
        require(state.length > 0, "state cannot be empty");
        require(maxGasOfExtension[extensionAddr] != 0, "invalid extension contract");

        uint256 expiryTime = block.timestamp + unclaimsExpiryDuration;

        UnclaimedState memory us = UnclaimedState({
            emailAddrCommit: currContext.recipientEmailAddrCommit,
            extensionAddr: extensionAddr,
            sender: currContext.walletAddr,
            state: state,
            expiryTime: expiryTime
        });

        Extension extension = Extension(extensionAddr);

        try extension.registerUnclaimedState(us, false) {} catch Error(string memory reason) {
            revert(string.concat("unclaimed state reg err: ", reason));
        } catch {
            revert("unclaimed state reg err");
        }

        unclaimedStateOfEmailAddrCommit[currContext.recipientEmailAddrCommit] = us;
        currContext.unclaimedStateRegistered = true;

        emit UnclaimedStateRegistered(
            currContext.recipientEmailAddrCommit,
            extensionAddr,
            currContext.walletAddr,
            expiryTime,
            state,
            0,
            ""
        );
    }

    /// @notice For extensions to request token from user's wallet (context wallet)
    /// @param tokenAddr Address of the ERC20 token requested
    /// @param amount Amount requested
    function requestTokenAsExtension(address tokenAddr, uint256 amount) public {
        require(msg.sender == currContext.extensionAddr, "caller not extension");

        for (uint256 i = 0; i < currContext.tokenAllowances.length; i++) {
            if (currContext.tokenAllowances[i].tokenAddr == tokenAddr) {
                require(currContext.tokenAllowances[i].amount >= amount, "insufficient allowance");
                currContext.tokenAllowances[i].amount -= amount;

                (bool success, bytes memory returnData) = _transferERC20(
                    currContext.walletAddr,
                    currContext.extensionAddr,
                    tokenAddr,
                    amount
                );
                require(success, string.concat("request token failed: ", string(returnData)));
                return;
            }
        }

        require(false, "no allowance for requested token");
    }

    /// @notice For extensions to deposit token to user's wallet (context wallet)
    /// @param tokenAddr Address of the ERC20 token to be deposited
    /// @param amount Amount to be deposited
    /// @dev Extension should add allowance to Core contract before calling this function
    function depositTokenAsExtension(address tokenAddr, uint256 amount) public {
        require(msg.sender == currContext.extensionAddr, "caller not extension");

        IERC20(tokenAddr).transferFrom(msg.sender, currContext.walletAddr, amount);
    }

    /// @notice For extensions to execute a call on user's wallet
    /// @param target Address of the contract on which the call is to be executed
    /// @param data Calldata to be executed on the target contract
    /// @dev Do not use this method to transfer tokens. Use `requestTokenAsExtension()` instead
    function executeAsExtension(address target, bytes memory data) public {
        require(msg.sender == currContext.extensionAddr, "caller not extension");
        require(target != address(this), "target cannot be core");
        require(target != currContext.walletAddr, "target cannot be wallet");

        // Prevent extension from calling ERC20 tokens directly (tokenName should be empty)
        require(bytes(tokenRegistry.getTokenNameOfAddress(target)).length == 0, "target cannot be a token");

        Wallet(payable(currContext.walletAddr)).execute(target, 0, data);
    }

    /// Register a new extension
    /// @param name Name of the extension
    /// @param addr Address of the extension contract
    /// @param subjectTemplates Subject templates for the extension
    /// @param maxExecutionGas Max gas allowed for the extension
    /// @dev First word of each subject template should be same and is called "command"; command should be one word
    function publishExtension(
        string memory name,
        address addr,
        string[][] memory subjectTemplates,
        uint256 maxExecutionGas
    ) public {
        require(addressOfExtension[name] == address(0), "extension name already used");
        require(addr != address(0), "invalid extension address");
        require(bytes(name).length > 0, "invalid extension name");
        require(maxExecutionGas > 0, "maxExecutionGas must be larger than zero");
        require(subjectTemplates.length > 0, "subjectTemplates array cannot be empty");
        require(maxGasOfExtension[addr] == 0, "extension already published");

        // Check if all subjectTemplates have same command (first item in array)
        string memory command;
        for (uint i = 0; i < subjectTemplates.length; i++) {
            require(subjectTemplates[i].length > 0, "subjectTemplate cannot be empty");
            if (i == 0) {
                command = subjectTemplates[i][0];
            } else {
                require(Strings.equal(command, subjectTemplates[i][0]), "subjectTemplates must have same command");
            }
            uint numRecipient = 0;
            for (uint j = 1; j < subjectTemplates[i].length; j++) {
                if (Strings.equal(subjectTemplates[i][j], RECIPIENT_TEMPLATE)) {
                    numRecipient++;
                }
            }
            require(numRecipient <= 1, "recipient template can only be used once");
        }

        // Check if command is only one word (no spaces)
        for (uint i = 0; i < bytes(command).length; i++) {
            require(bytes(command)[i] != 0x20, "command should be one word");
        }

        // Check if command is not a reserved name
        require(
            !Strings.equal(command, Commands.SEND) &&
                !Strings.equal(command, Commands.EXECUTE) &&
                !Strings.equal(command, Commands.INSTALL_EXTENSION) &&
                !Strings.equal(command, Commands.UNINSTALL_EXTENSION) &&
                !Strings.equal(command, Commands.EXIT_EMAIL_WALLET) &&
                !Strings.equal(command, Commands.DKIM),
            "command cannot be a reserved name"
        );

        // Check if command is not a template
        require(
            !Strings.equal(command, TOKEN_AMOUNT_TEMPLATE) &&
                !Strings.equal(command, AMOUNT_TEMPLATE) &&
                !Strings.equal(command, STRING_TEMPLATE) &&
                !Strings.equal(command, UINT_TEMPLATE) &&
                !Strings.equal(command, INT_TEMPLATE) &&
                !Strings.equal(command, ADDRESS_TEMPLATE),
            "command cannot be a template matcher"
        );

        addressOfExtension[name] = addr;
        subjectTemplatesOfExtension[addr] = subjectTemplates;
        maxGasOfExtension[addr] = maxExecutionGas;

        emit ExtensionPublished(name, addr, subjectTemplates, maxExecutionGas);
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

    /// @notice Return the extension address for a command and user
    /// @param command Command for which the extension address is to be returned
    /// @param walletAddr The user's wallet address
    function getExtensionForCommand(string memory command, address walletAddr) public view returns (address) {
        address extensionAddr = defaultExtensionOfCommand[command]; // Global extension installed by default for all users
        address userextensionAddr = userExtensionOfCommand[walletAddr][command];

        if (userextensionAddr != address(0)) {
            extensionAddr = userextensionAddr;
        }

        return extensionAddr;
    }

    /// @notice Return the token address for a token name.
    /// @param tokenName Name of the token
    function getTokenAddrFromName(string memory tokenName) public view returns (address) {
        if (Strings.equal(tokenName, "ETH")) {
            return tokenRegistry.getTokenAddress("WETH");
        }

        return tokenRegistry.getTokenAddress(tokenName);
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

    /// @notice Calculate the masked subject for an EmailOp from command and other params
    ///         This also do sanity checks of certain parameters used in the subject
    /// @param emailOp EmailOp from which the masked subject is to be computed
    function _computeMaskedSubjectForEmailOp(
        EmailOp memory emailOp
    ) internal view returns (string memory maskedSubject, bool isExtension) {
        // Sample: Send 1 ETH to recipient@domain.com
        if (Strings.equal(emailOp.command, Commands.SEND)) {
            address walletAddr = getWalletOfSalt(
                infoOfAccountKeyCommit[accountKeyCommitOfPointer[emailOp.emailAddrPointer]].walletSalt
            );
            WalletParams memory walletParams = emailOp.walletParams;
            ERC20 token = ERC20(getTokenAddrFromName(emailOp.walletParams.tokenName));

            require(token != ERC20(address(0)), "token not supported");
            require(emailOp.walletParams.amount > 0, "send amount should be >0");
            require(token.balanceOf(walletAddr) >= walletParams.amount, "insufficient balance");

            maskedSubject = string.concat(
                Commands.SEND,
                " ",
                DecimalUtils.uintToDecimalString(walletParams.amount, token.decimals()),
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
        // Sample: Execute 0x000112aa..
        else if (Strings.equal(emailOp.command, Commands.EXECUTE)) {
            address walletAddr = getWalletOfSalt(
                infoOfAccountKeyCommit[accountKeyCommitOfPointer[emailOp.emailAddrPointer]].walletSalt
            );

            require(emailOp.executeCallData.length > 0, "executeCallData cannot be empty");

            (address target, , bytes memory data) = abi.decode(emailOp.executeCallData, (address, uint256, bytes));

            require(target != address(0), "invalid execute target");
            require(target != address(this), "cannot execute on core");
            require(target != walletAddr, "cannot execute on wallet");
            require(bytes(tokenRegistry.getTokenNameOfAddress(target)).length == 0, "cannot execute on token");
            require(data.length > 0, "execute data cannot be empty");

            maskedSubject = string.concat(
                Commands.EXECUTE,
                " 0x",
                BytesUtils.bytesToHexString(emailOp.executeCallData)
            );
        }
        // Sample: Install extension Uniswap
        else if (Strings.equal(emailOp.command, Commands.INSTALL_EXTENSION)) {
            address extAddr = addressOfExtension[emailOp.extManagerParams.extensionName];

            require(extAddr != address(0), "extension not registered");

            maskedSubject = string.concat(
                Commands.INSTALL_EXTENSION,
                " extension ",
                emailOp.extManagerParams.extensionName
            );
        }
        // Sample: Remove extension Uniswap
        else if (Strings.equal(emailOp.command, Commands.UNINSTALL_EXTENSION)) {
            address extAddr = addressOfExtension[emailOp.extManagerParams.extensionName];
            string memory command = subjectTemplatesOfExtension[extAddr][0][0];
            address walletAddr = getWalletOfSalt(
                infoOfAccountKeyCommit[accountKeyCommitOfPointer[emailOp.emailAddrPointer]].walletSalt
            );

            require(extAddr != address(0), "extension not registered");
            require(userExtensionOfCommand[walletAddr][command] != address(0), "extension not installed");

            maskedSubject = string.concat(
                Commands.UNINSTALL_EXTENSION,
                " extension ",
                emailOp.extManagerParams.extensionName
            );
        }
        // Sample: Exit email wallet. Change owner to 0x000112aa..
        else if (Strings.equal(emailOp.command, Commands.EXIT_EMAIL_WALLET)) {
            require(emailOp.newWalletOwner != address(0), "newWalletOwner cannot be empty");

            maskedSubject = string.concat(
                Commands.EXIT_EMAIL_WALLET,
                " Email Wallet. Change wallet ownership to ",
                Strings.toHexString(uint256(uint160(emailOp.newWalletOwner)), 20)
            );
        }
        // Sample: DKIM registry as 0x000112aa..
        else if (Strings.equal(emailOp.command, Commands.DKIM)) {
            require(emailOp.newDkimRegistry != address(0), "newDkimRegistry cannot be emoty");

            maskedSubject = string.concat(
                Commands.DKIM,
                " registry is ",
                Strings.toHexString(uint256(uint160(emailOp.newDkimRegistry)), 20)
            );
        }
        // The command is for an extension
        else {
            isExtension = true;
            address walletAddr = getWalletOfSalt(
                infoOfAccountKeyCommit[accountKeyCommitOfPointer[emailOp.emailAddrPointer]].walletSalt
            );
            address extensionAddr = getExtensionForCommand(emailOp.command, walletAddr);

            require(extensionAddr != address(0), "invalid command or extension");

            // Extension extension = Extension(extensionAddr);
            string[] memory subjectTemplate = subjectTemplatesOfExtension[extensionAddr][
                emailOp.extensionParams.subjectTemplateIndex
            ];

            uint8 nextParamIndex;
            for (uint8 i = 0; i < subjectTemplate.length; i++) {
                string memory matcher = string(subjectTemplate[i]);
                string memory value;

                // {tokenAmount} is combination of tokenName and amount, encoded as (uint256,string). Eg: `30.23 DAI`
                if (Strings.equal(matcher, TOKEN_AMOUNT_TEMPLATE)) {
                    (uint256 amount, string memory tokenName) = abi.decode(
                        emailOp.extensionParams.subjectParams[nextParamIndex],
                        (uint256, string)
                    );
                    address tokenAddr = getTokenAddrFromName(tokenName);
                    require(tokenAddr != address(0), "token not supported");
                    value = string.concat(
                        DecimalUtils.uintToDecimalString(amount, ERC20(tokenAddr).decimals()),
                        " ",
                        tokenName
                    );
                    nextParamIndex++;
                }
                // {amount} is number in wei format (decimal format in subject)
                else if (Strings.equal(matcher, AMOUNT_TEMPLATE)) {
                    uint256 num = abi.decode(emailOp.extensionParams.subjectParams[nextParamIndex], (uint256));
                    value = DecimalUtils.uintToDecimalString(num);
                    nextParamIndex++;
                }
                // {string} is plain string
                else if (Strings.equal(matcher, STRING_TEMPLATE)) {
                    value = abi.decode(emailOp.extensionParams.subjectParams[nextParamIndex], (string));
                    nextParamIndex++;
                }
                // {uint} is number parsed the same way as mentioned in subject (decimals not allowed) - use {amount} for decimals
                else if (Strings.equal(matcher, UINT_TEMPLATE)) {
                    uint256 num = abi.decode(emailOp.extensionParams.subjectParams[nextParamIndex], (uint256));
                    value = Strings.toString(num);
                    nextParamIndex++;
                }
                // {int} for negative values
                else if (Strings.equal(matcher, INT_TEMPLATE)) {
                    int256 num = abi.decode(emailOp.extensionParams.subjectParams[nextParamIndex], (int256));
                    value = Strings.toString(num);
                    nextParamIndex++;
                }
                // {addres} for wallet address
                else if (Strings.equal(matcher, ADDRESS_TEMPLATE)) {
                    address addr = abi.decode(emailOp.extensionParams.subjectParams[nextParamIndex], (address));
                    value = Strings.toHexString(uint256(uint160(addr)), 20);
                    nextParamIndex++;
                } else if (Strings.equal(matcher, RECIPIENT_TEMPLATE)) {
                    if (!emailOp.hasEmailRecipient) {
                        value = Strings.toHexString(uint256(uint160(emailOp.recipientETHAddr)), 20);
                    }
                }
                // Constant string otherwise
                else {
                    value = matcher;
                }

                if (bytes(maskedSubject).length == 0) {
                    maskedSubject = value;
                } else {
                    maskedSubject = string.concat(maskedSubject, " ", value);
                }
            }

            // We should have used all items in subjectParams by now
            require(nextParamIndex == emailOp.extensionParams.subjectParams.length, "invalid subject params length");
        }
    }

    /// @notice Execute an EmailOp
    /// @param emailOp EmailOp to be executed
    /// @return success Whether the operation is successful
    /// @return returnData Return data from the operation (error)
    function _executeEmailOp(EmailOp memory emailOp) internal returns (bool success, bytes memory returnData) {
        // Wallet operation
        if (Strings.equal(emailOp.command, Commands.SEND)) {
            WalletParams memory walletParams = emailOp.walletParams;

            // If sending ETH to external wallet, use ETH instead of WETH
            if (!emailOp.hasEmailRecipient && Strings.equal(emailOp.walletParams.tokenName, "ETH")) {
                Wallet wallet = Wallet(payable(currContext.walletAddr));

                try wallet.execute(weth, 0, abi.encodeWithSignature("withdraw(uint256)", walletParams.amount)) {
                    wallet.execute(emailOp.recipientETHAddr, walletParams.amount, "");
                    success = true;
                } catch Error(string memory reason) {
                    success = false;
                    returnData = bytes(reason);
                } catch {
                    success = false;
                    returnData = bytes("err converting WETH to ETH");
                }

                return (success, returnData);
            }

            // Token transfer for both external contract and email wallet
            address recipient = emailOp.hasEmailRecipient ? address(this) : emailOp.recipientETHAddr;
            address tokenAddr = getTokenAddrFromName(emailOp.walletParams.tokenName);

            (success, returnData) = _transferERC20(currContext.walletAddr, recipient, tokenAddr, walletParams.amount);

            if (!success) {
                return (success, returnData);
            }

            // Register unclaimed fund if the recipient is email wallet user
            if (emailOp.hasEmailRecipient) {
                _registerUnclaimedFund(
                    currContext.walletAddr,
                    emailOp.recipientEmailAddrCommit,
                    tokenAddr,
                    walletParams.amount,
                    block.timestamp + unclaimsExpiryDuration,
                    0,
                    ""
                );
                currContext.unclaimedFundRegistered = true;
            }
        }
        // Execute calldata on wallet
        else if (Strings.equal(emailOp.command, Commands.EXECUTE)) {
            (address target, uint256 value, bytes memory data) = abi.decode(
                emailOp.executeCallData,
                (address, uint256, bytes)
            );

            try Wallet(payable(currContext.walletAddr)).execute(target, value, data) {
                success = true;
            } catch Error(string memory reason) {
                success = false;
                returnData = bytes(reason);
            } catch {
                success = false;
                returnData = bytes("err executing calldata on wallet");
            }
        }
        // Set custom extension for the user
        else if (Strings.equal(emailOp.command, Commands.INSTALL_EXTENSION)) {
            address extensionAddr = addressOfExtension[emailOp.extManagerParams.extensionName];
            string memory command = subjectTemplatesOfExtension[extensionAddr][0][0];

            userExtensionOfCommand[currContext.walletAddr][command] = extensionAddr;
            success = true;
        }
        // Remove custom extension for the user
        else if (Strings.equal(emailOp.command, Commands.UNINSTALL_EXTENSION)) {
            address extensionAddr = addressOfExtension[emailOp.extManagerParams.extensionName];
            string memory command = subjectTemplatesOfExtension[extensionAddr][0][0];

            userExtensionOfCommand[currContext.walletAddr][command] = address(0);
            success = true;
        }
        // Exit email wallet
        else if (Strings.equal(emailOp.command, Commands.EXIT_EMAIL_WALLET)) {
            try Wallet(payable(currContext.walletAddr)).transferOwnership(emailOp.newWalletOwner) {
                success = true;
            } catch Error(string memory reason) {
                success = false;
                returnData = bytes(reason);
            } catch {
                success = false;
                returnData = bytes("err executing transferOwnership on wallet");
            }
        }
        // Set DKIM registry
        else if (Strings.equal(emailOp.command, Commands.DKIM)) {
            bytes32 accountKeyCommit = accountKeyCommitOfPointer[emailOp.emailAddrPointer];
            dkimRegistryOfWalletSalt[infoOfAccountKeyCommit[accountKeyCommit].walletSalt] = emailOp.newDkimRegistry;
            // infoOfAccountKeyCommit[accountKeyCommitOfPointer[emailOp.emailAddrPointer]].dkimRegistry =
            //     emailOp.newDkimRegistry;
            success = true;
        }
        // The command is for an extension
        else {
            address extAddress = getExtensionForCommand(emailOp.command, currContext.walletAddr);
            currContext.extensionAddr = extAddress;

            // Set token+amount pair in subject as allowances in context
            // We are assuming one token appear only once
            uint8 nextParamIndex = 0;
            string[] memory subjectTemplate = subjectTemplatesOfExtension[extAddress][
                emailOp.extensionParams.subjectTemplateIndex
            ];
            for (uint8 i = 0; i < subjectTemplate.length; i++) {
                string memory matcher = string(subjectTemplate[i]);

                if (Strings.equal(matcher, TOKEN_AMOUNT_TEMPLATE)) {
                    (uint256 amount, string memory tokenName) = abi.decode(
                        emailOp.extensionParams.subjectParams[nextParamIndex],
                        (uint256, string)
                    );
                    currContext.tokenAllowances.push(
                        TokenAllowance({tokenAddr: getTokenAddrFromName(tokenName), amount: amount})
                    );
                    nextParamIndex++;
                }
                else if (
                    Strings.equal(matcher, AMOUNT_TEMPLATE) ||
                    Strings.equal(matcher, STRING_TEMPLATE) ||
                    Strings.equal(matcher, UINT_TEMPLATE) ||
                    Strings.equal(matcher, INT_TEMPLATE) ||
                    Strings.equal(matcher, ADDRESS_TEMPLATE) ||
                    Strings.equal(matcher, RECIPIENT_TEMPLATE)
                ) {
                    nextParamIndex++;
                }
            }

            // We only pass pre-configured gas to execute()
            try
                Extension(extAddress).execute{gas: maxGasOfExtension[extAddress]}(
                    emailOp.extensionParams.subjectTemplateIndex,
                    emailOp.extensionParams.subjectParams,
                    currContext.walletAddr,
                    emailOp.hasEmailRecipient,
                    emailOp.recipientETHAddr,
                    emailOp.emailNullifier
                )
            {
                success = true;
            } catch Error(string memory reason) {
                success = false;
                returnData = bytes(reason);
            } catch {
                success = false;
                returnData = bytes("err executing extension");
            }
        }
    }

    /// @notice Register unclaimed fund for the recipient - can be called by Core contract directly
    /// @param emailAddrCommit Hash of the recipient's email address and a random number.
    /// @param tokenAddr Address of ERC20 token contract.
    /// @param amount Amount in WEI of the token.
    function _registerUnclaimedFund(
        address sender,
        bytes32 emailAddrCommit,
        address tokenAddr,
        uint256 amount,
        uint256 expiryTime,
        uint256 announceCommitRandomness,
        string memory announceEmailAddr
    ) internal {
        UnclaimedFund memory fund = UnclaimedFund({
            emailAddrCommit: emailAddrCommit,
            tokenAddr: tokenAddr,
            amount: amount,
            expiryTime: expiryTime,
            sender: sender
        });

        unclaimedFundOfEmailAddrCommit[emailAddrCommit] = fund;

        emit UnclaimedFundRegistered(
            emailAddrCommit,
            tokenAddr,
            amount,
            sender,
            expiryTime,
            announceCommitRandomness,
            announceEmailAddr
        );
    }

    /// @notice Transfer ERC20 token from user's wallet to given recipient
    /// @param sender Address of the sender's wallet
    /// @param recipientAddr Address of the recipient
    /// @param tokenAddr Address of ERC20 token contract.
    /// @param amount Amount in WEI of the token.
    function _transferERC20(
        address sender,
        address recipientAddr,
        address tokenAddr,
        uint256 amount
    ) internal returns (bool success, bytes memory returnData) {
        require(tokenAddr != address(0), "invalid token address");
        require(amount > 0, "invalid amount");
        require(sender != address(0), "invalid sender address");
        require(recipientAddr != address(0), "invalid recipient address");

        Wallet wallet = Wallet(payable(sender));

        try wallet.execute(tokenAddr, 0, abi.encodeWithSignature("transfer(address,uint256)", recipientAddr, amount)) {
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

    function _getFeeConversionRate(string memory tokenName) internal view returns (uint256) {
        if (Strings.equal(tokenName, "ETH") || Strings.equal(tokenName, "WETH")) {
            return 1 ether;
        }

        bool validToken = Strings.equal(tokenName, "DAI") || Strings.equal(tokenName, "USDC");
        if (!validToken) {
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
