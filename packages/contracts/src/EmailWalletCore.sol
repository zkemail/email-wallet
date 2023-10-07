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
import {LibZip} from "solady/utils/LibZip.sol";
import "./libraries/DecimalUtils.sol";
import "./utils/TokenRegistry.sol";
import "./interfaces/IVerifier.sol";
import "./interfaces/Extension.sol";
import "./interfaces/Types.sol";
import "./interfaces/Commands.sol";
import "./interfaces/IPriceOracle.sol";
import "./Wallet.sol";

contract EmailWalletCore is ReentrancyGuard, OwnableUpgradeable, UUPSUpgradeable {
    uint constant test = 123;

    // ZK proof verifier
    IVerifier public immutable verifier;

    // DKIM public key hashes registry
    DKIMRegistry public immutable dkimRegistry;

    // Token registry
    TokenRegistry public immutable tokenRegistry;

    // Price oracle for feeToken conversion
    IPriceOracle public immutable priceOracle;

    // Address of WETH contract
    address weth;

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

    // Mapping to store email nullifiers
    mapping(bytes32 => bool) public emailNullifiers;

    // Mapping from extensio name to extension address, as published by the developer
    mapping(string => address) public addressOfExtension;

    // Sora's comment: I think it is better to rename `extensionOfCommand` to `defaultExtensionOfCommand`.
    // Global mapping of command name to extension address enabled for all users by default
    mapping(string => address) public extensionOfCommand;

    // Mapping of extension address to maximum gas that will be consumed by `extension.execute()`
    // Relayer can use this to ensure user has enough tokens to pay for the gas
    mapping(address => uint256) public maxGasOfExtension;

    // User level mapping of command name to extension address (pointer -> (command -> extension))
    mapping(bytes32 => mapping(string => address)) public userExtensionOfCommand;

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

    // Sora's comment: this value is used in `registerUnclaimedState` to set a max gas for calling `extension.registerUnclaimedState`.
    // Gas required to register unclaimed state
    uint256 public immutable unclaimedStateRegisterGas;

    // Sora's comment: Maybe it is better to rename it to `unclaimsExpiryDuration`.
    // Default expiry duration for unclaimed funds and states
    uint256 public immutable unclaimedFundExpiryDuration;

    // Context of currently executing EmailOp - reset on every EmailOp
    ExecutionContext internal currContext;

    event RelayerRegistered(bytes32 randHash, string emailAddr, string hostname);

    event RelayerConfigUpdated(bytes32 randHash, string hostname);

    event UnclaimedFundRegistered(
        bytes32 emailAddrCommit,
        address tokenAddr,
        uint256 amount,
        address sender,
        uint256 expiryTime,
        uint256 commitmentRandomness,
        string emailAddr
    );

    event UnclaimedFundClaimed(bytes32 emailAddrCommit, address tokenAddr, uint256 amount, address recipient);

    event UnclaimedFundReverted(bytes32 emailAddrCommit, address tokenAddr, uint256 amount, address sender);

    event UnclaimedStateRegistered(
        bytes32 emailAddrCommit,
        address extensionAddr,
        address sender,
        uint256 expiryTime,
        bytes state,
        uint256 commitmentRandomness,
        string emailAddr
    );

    event UnclaimedStateClaimed(bytes32 emailAddrCommit, address recipient);

    event UnclaimedStateReverted(bytes32 emailAddrCommit, address sender);

    constructor(
        address _verifier,
        address _tokenRegistry,
        address _dkimRegistry,
        address _priceOracle,
        address _wethContract,
        uint256 _maxFeePerGas,
        uint256 _emailValidityDuration,
        uint256 _unclaimedFundClaimGas,
        uint256 _unclaimedStateClaimGas,
        uint256 _unclaimedStateRegisterGas,
        uint256 _unclaimedFundExpirationDuration
    ) {
        verifier = IVerifier(_verifier);
        dkimRegistry = DKIMRegistry(_dkimRegistry);
        tokenRegistry = TokenRegistry(_tokenRegistry);
        priceOracle = IPriceOracle(_priceOracle);
        maxFeePerGas = _maxFeePerGas;
        emailValidityDuration = _emailValidityDuration;
        unclaimedFundClaimGas = _unclaimedFundClaimGas;
        unclaimedStateClaimGas = _unclaimedStateClaimGas;
        unclaimedStateRegisterGas = _unclaimedStateRegisterGas;
        unclaimedFundExpiryDuration = _unclaimedFundExpirationDuration;

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
    /// @param accountKeyCommit hash(accountKey, emailAddr, relayerHash)
    /// @param walletSalt hash(accountKey, 0)
    /// @param proof ZK proof as required by the verifier
    function createAccount(
        bytes32 emailAddrPointer,
        bytes32 accountKeyCommit,
        bytes32 walletSalt,
        bytes memory psiPoint,
        bytes memory proof
    ) public returns (Wallet) {
        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(accountKeyCommitOfPointer[emailAddrPointer] == bytes32(0), "pointer exists");
        require(pointerOfPSIPoint[psiPoint] == bytes32(0), "PSI point exists");
        bool initialized = infoOfAccountKeyCommit[accountKeyCommit].initialized;
        bool nullified = infoOfAccountKeyCommit[accountKeyCommit].nullified;
        bool walletSaltSet = infoOfAccountKeyCommit[accountKeyCommit].walletSaltSet;
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

        pointerOfPSIPoint[psiPoint] = emailAddrPointer;

        return _deployWallet(walletSalt);
    }

    // Sora's comment: Use EmailProof instead of domain, timestamp, nullifier, and proof bytes. 
    /// Initialize the account when user reply to invitation email
    /// @param emailAddrPointer hash(relayerRand, emailAddr)
    /// @param initEmailProof Proof of user's initialization email
    function initializeAccount(
        bytes32 emailAddrPointer,
        // string memory emailDomain,
        // uint256 emailTimestamp,
        // bytes32 emailNullifier,
        // bytes memory proof
        EmailProof memory initEmailProof
    ) public {
        bytes32 accountKeyCommit = accountKeyCommitOfPointer[emailAddrPointer];

        require(initEmailProof.timestamp + emailValidityDuration > block.timestamp, "email expired");
        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(accountKeyCommit != bytes32(0), "account not registered");
        require(infoOfAccountKeyCommit[accountKeyCommit].relayer == msg.sender, "invalid relayer");
        require(infoOfAccountKeyCommit[accountKeyCommit].nullified == false, "account is nullified");
        require(infoOfAccountKeyCommit[accountKeyCommit].initialized == false, "account already initialized");
        require(emailNullifiers[initEmailProof.nullifier] == false, "email nullifier already used");

        require(
            verifier.verifyAccountInitializaionProof(
                initEmailProof.domain,
                bytes32(dkimRegistry.getDKIMPublicKeyHash(initEmailProof.domain)),
                initEmailProof.timestamp,
                relayers[msg.sender].randHash,
                emailAddrPointer,
                accountKeyCommit,
                initEmailProof.nullifier,
                initEmailProof.proof
            ),
            "invalid account creation proof"
        );

        infoOfAccountKeyCommit[accountKeyCommit].initialized = true;
        emailNullifiers[initEmailProof.nullifier] = true;
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

        require(transportEmailProof.timestamp + emailValidityDuration > block.timestamp, "email expired");
        require(relayers[msg.sender].randHash != bytes32(0), "relayer not registered");
        require(oldAccountKeyInfo.relayer != address(0), "old relayer not registered");
        require(oldAccountKeyInfo.relayer != msg.sender, "new relayer cannot be same");
        require(oldAccountKeyInfo.initialized, "account not initialized");
        require(!oldAccountKeyInfo.nullified, "account is nullified");
        // require(accountKeyCommitOfPointer[newEmailAddrPointer] == bytes32(0), "new pointer already exist");
        // Sora's comment: although it is different from the spec, this function needs to consider the case where the new relayer already creates an account for the transported user.
        // For example, if two users under relayers 1 and 2 send tokens to Alice, she first initializes her account under relayer 1, and then transports her account to relayer 2, the relayer 2 has already created her account with a different account key.
        // Therefore, even in honest case, `accountKeyCommitOfPointer[newEmailAddrPointer]` can be non-zero bytes.
        bytes32 preAccountKeyOfNewPointer = accountKeyCommitOfPointer[newEmailAddrPointer];
        if(preAccountKeyOfNewPointer!=bytes32(0)) {
            require(!infoOfAccountKeyCommit[preAccountKeyOfNewPointer].initialized, "new account is registered and initialized.");
            require(!infoOfAccountKeyCommit[preAccountKeyOfNewPointer].nullified, "new account is registered and nullified.");
            require(!infoOfAccountKeyCommit[preAccountKeyOfNewPointer].walletSaltSet, "new account is registered and its salt already exists.");
        } else {
            require(!infoOfAccountKeyCommit[newAccountKeyCommit].initialized, "new account is already initialized");
            require(!infoOfAccountKeyCommit[newAccountKeyCommit].nullified, "new account is already nullified");
            require(!infoOfAccountKeyCommit[newAccountKeyCommit].walletSaltSet, "salt already exists");
        }
        // require(!infoOfAccountKeyCommit[newAccountKeyCommit].walletSaltSet, "salt already exists");
        require(pointerOfPSIPoint[newPSIPoint] == bytes32(0), "new PSI point already exists");
        require(emailNullifiers[transportEmailProof.nullifier] == false, "email nullifier already used");

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
            verifier.verifiyAccountTransportProof(
                transportEmailProof.domain,
                bytes32(dkimRegistry.getDKIMPublicKeyHash(transportEmailProof.domain)),
                transportEmailProof.timestamp,
                transportEmailProof.nullifier,
                relayers[oldAccountKeyInfo.relayer].randHash,
                oldAccountKeyCommit,
                transportEmailProof.proof
            ),
            "invalid account transport proof"
        );

        emailNullifiers[transportEmailProof.nullifier] = true;

        accountKeyCommitOfPointer[newEmailAddrPointer] = newAccountKeyCommit;
        pointerOfPSIPoint[newPSIPoint] = newEmailAddrPointer;
        infoOfAccountKeyCommit[newAccountKeyCommit].walletSalt = infoOfAccountKeyCommit[oldAccountKeyCommit].walletSalt;
        infoOfAccountKeyCommit[newAccountKeyCommit].walletSaltSet = true;
        infoOfAccountKeyCommit[newAccountKeyCommit].relayer = msg.sender;
        infoOfAccountKeyCommit[newAccountKeyCommit].initialized = true;
        infoOfAccountKeyCommit[newAccountKeyCommit].nullified = false;

        infoOfAccountKeyCommit[oldAccountKeyCommit].walletSalt = bytes32(0);
        infoOfAccountKeyCommit[oldAccountKeyCommit].walletSaltSet = false;
        infoOfAccountKeyCommit[oldAccountKeyCommit].nullified = true;

        if(preAccountKeyOfNewPointer!=bytes32(0)) {
            delete infoOfAccountKeyCommit[preAccountKeyOfNewPointer];
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
        // Ensure the sender has paid ETH needed for claiming / expiring the unclaimed fee
        require(msg.value == unclaimedFundClaimGas * maxFeePerGas, "invalid unclaimed fund fee");
        require(amount > 0, "amount should be greater than 0");
        require(tokenAddr != address(0), "invalid token contract");
        require(emailAddrCommit != bytes32(0), "invalid emailAddrCommit");
        // Sora's comment: Should this function replace `expiryTime` with `block.timestamp + unclaimedFundExpiryDuration` if the given value is zero?
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
        UnclaimedFund storage fund = unclaimedFundOfEmailAddrCommit[recipientEmailAddrPointer];
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

        delete unclaimedFundOfEmailAddrCommit[recipientEmailAddrPointer];

        // Transfer token from Core contract to recipient's wallet
        ERC20(fund.tokenAddr).transfer(recipientAddr, fund.amount);

        // Transfer claim fee to the sender (relayer)
        _transferETH(msg.sender, unclaimedFundClaimGas * maxFeePerGas);

        emit UnclaimedFundClaimed(emailAddrCommit, fund.tokenAddr, fund.amount, recipientAddr);
    }

    /// @notice Return unclaimed fund after expiry time
    /// @param emailAddrCommit The commitment of the recipient's email address to which the unclaimed fund was registered.
    /// @dev Callee should dry run this call, as they will only get claim fee (gas reimbursement) if this succeeds.
    function revertUnclaimedFund(bytes32 emailAddrCommit) public nonReentrant {
        uint256 initialGas = gasleft();

        UnclaimedFund storage fund = unclaimedFundOfEmailAddrCommit[emailAddrCommit];

        require(fund.amount > 0, "unclaimed fund not registered");
        require(fund.expiryTime < block.timestamp, "unclaimed fund not expired");

        delete unclaimedFundOfEmailAddrCommit[emailAddrCommit];

        // Transfer token from Core contract to sender's wallet
        ERC20(fund.tokenAddr).transfer(fund.sender, fund.amount);

        // Gas consumed so far + cost for 2 ETH transfers
        uint256 consumedGas = initialGas - gasleft() + 21000 + 21000;

        // Transfer consumedGas to callee, and rest of the locked funds to user who locked up the funds
        _transferETH(fund.sender, (unclaimedFundClaimGas - consumedGas) * maxFeePerGas);
        _transferETH(msg.sender, consumedGas * maxFeePerGas);

        emit UnclaimedFundReverted(emailAddrCommit, fund.tokenAddr, fund.amount, fund.sender);
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
            expiryTime = block.timestamp + unclaimedFundExpiryDuration;
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
        // Sora's comment: Here try-and-catch should be used to prevent a malicious extension that returns error.  
        // The max gas to call it is calculated from `unclaimedStateRegisterGas`.
        bool registered = extension.registerUnclaimedState(us,false);

        require(registered, "unclaimed state reg failed");

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

    /// Registere unclaimed state from an extension
    /// @param emailAddrCommit Email address commitment of the recipient
    /// @param extensionAddr Address of the extension contract
    /// @param state State to be registered
    /// @dev This dont call `extension.registerUnclaimedState` as extension is expected to make necessary changes internally
    /// Sora's comment: the fee to register the unclaimed state is paid by the relayer in `handleEmailOp`, right?
    function registerUnclaimedStateAsExtension(
        bytes32 emailAddrCommit,
        address extensionAddr,
        bytes memory state
    ) public {
        require(msg.sender == currContext.extensionAddr, "invalid caller");
        require(currContext.unclaimedStateRegistered == false, "unclaimed state exists");
        require(state.length > 0, "state cannot be empty");
        require(extensionAddr != address(0), "invalid extension contract");
        require(emailAddrCommit != bytes32(0), "invalid emailAddrCommit");
        // Sora's comment: the following check is performed in `validateEmailOp`, right?
        // require(unclaimedStateOfEmailAddrCommit[emailAddrCommit].sender == address(0), "unclaimed state exists");

        uint256 expiryTime = block.timestamp + unclaimedFundExpiryDuration;

        UnclaimedState memory us = UnclaimedState({
            emailAddrCommit: emailAddrCommit,
            extensionAddr: extensionAddr,
            sender: currContext.walletAddr,
            state: state,
            expiryTime: expiryTime
        });

        // Sora's comment: `extension.registerUnclaimedState` should be called here.
        // Sora's comment: Here try-and-catch should be used to prevent a malicious extension that returns error.  
        // The max gas to call it is calculated from `unclaimedStateRegisterGas`.
        Extension extension = Extension(extensionAddr);
        bool registered = extension.registerUnclaimedState(us,true);
        require(registered, "unclaimed state reg failed");

        unclaimedStateOfEmailAddrCommit[emailAddrCommit] = us;
        currContext.unclaimedStateRegistered = true;
        
        emit UnclaimedStateRegistered(emailAddrCommit, extensionAddr, currContext.walletAddr, expiryTime, state, 0, "");
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

        UnclaimedState storage us = unclaimedStateOfEmailAddrCommit[recipientEmailAddrPointer];
        bytes32 accountKeyCommit = accountKeyCommitOfPointer[recipientEmailAddrPointer];

        require(relayers[msg.sender].randHash != bytes32(0), "caller not relayer");
        require(infoOfAccountKeyCommit[accountKeyCommit].relayer == msg.sender, "invalid relayer for account");
        require(us.sender != address(0), "unclaimed state not registered");
        require(us.extensionAddr != address(0), "invalid extension address");

        // Sora's comment: we have to check `expiryTime` is less than `block.timestamp`;
        require(us.expiryTime < block.timestamp, "unclaimed fund expired");

        // Sora's comment: we can use `accountKeyCommit`.
        // require(accountKeyCommitOfPointer[recipientEmailAddrPointer] != bytes32(0), "invalid account key commit.");
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

        delete unclaimedStateOfEmailAddrCommit[recipientEmailAddrPointer];

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
    function revertUnclaimedState(
        bytes32 emailAddrCommit
    ) public nonReentrant returns (bool success, bytes memory returnData) {
        uint256 initialGas = gasleft();

        UnclaimedState storage us = unclaimedStateOfEmailAddrCommit[emailAddrCommit];

        require(us.sender != address(0), "unclaimed state not registered");

         // Sora's comment: we have to check `expiryTime` is larger than `block.timestamp`;
        require(us.expiryTime > block.timestamp, "unclaimed fund expired");

        delete unclaimedStateOfEmailAddrCommit[emailAddrCommit];

        Extension extension = Extension(us.extensionAddr);

        // Gas consumed for verification and next steps is deducated from `unclaimedStateClaimGas`
        // and rest is passed to extension
        uint256 gasForExt = unclaimedStateClaimGas - (initialGas - gasleft()) - 21000 - 21000;

        // Callee should get gas reimbursement even if extension call fails
        // Simulation wont work, as extension logic can depend on global variables
        try extension.revertUnclaimedState{gas: gasForExt}(us) {
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

        emit UnclaimedStateReverted(emailAddrCommit, us.sender);
    }

    /// @notice Return the extension address for a command and user
    /// @param command Command for which the extension address is to be returned
    /// @param emailAddrPointer Pointer of the user's email address
    function getExtensionForCommand(string memory command, bytes32 emailAddrPointer) public view returns (address) {
        address extensionAddr = extensionOfCommand[command]; // Global extension installed by default for all users
        address userextensionAddr = userExtensionOfCommand[emailAddrPointer][command];

        if (userextensionAddr != address(0)) {
            extensionAddr = userextensionAddr;
        }

        return extensionAddr;
    }

    /// @notice Validate an EmailOp, including proof verification
    /// @param emailOp EmailOp to be validated
    function validateEmailOp(EmailOp memory emailOp) public view {
        bytes32 dkimPublicKeyHash = bytes32(dkimRegistry.getDKIMPublicKeyHash(emailOp.emailDomain));
        bytes32 accountKeyCommit = accountKeyCommitOfPointer[emailOp.emailAddrPointer];

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
                unclaimedFundOfEmailAddrCommit[emailOp.recipientEmailAddrCommit].amount == 0,
                "Unclaimed fund exist"
            );
            require(
                unclaimedStateOfEmailAddrCommit[emailOp.recipientEmailAddrCommit].state.length == 0,
                "Unclaimed state exists"
            );
        } else {
            require(emailOp.recipientEmailAddrCommit == bytes32(0), "recipientEmailAddrCommit not allowed");
        }

        (string memory maskedSubject, bool isExtension) = _computeMaskedSubjectForEmailOp(emailOp);
        require(Strings.equal(maskedSubject, emailOp.maskedSubject), string.concat("subject != ", maskedSubject));

        require(
            verifier.verifyEmailProof(
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
        currContext.unclaimedStateRegistered = false;
        // currContext.unclaimedFundRegistered = false;
        delete currContext.tokenAllowances;
        currContext.walletAddr = getWalletOfSalt(
            infoOfAccountKeyCommit[accountKeyCommitOfPointer[emailOp.emailAddrPointer]].walletSalt
        );

        // Validate emailOp - will revert on failure. Relayer should ensure validate pass by simulation.
        validateEmailOp(emailOp);

        emailNullifiers[emailOp.emailNullifier] = true;

        // Execute EmailOp - wont revert on failure. Relayer will be compensated for gas even in failure.
        bool unclaimedFundRegistered;
        (success, unclaimedFundRegistered, returnData) = _executeEmailOp(emailOp);

        uint totalFee; // Total fee in ETH to be paid to relayer

        require(unclaimedFundRegistered && currContext.unclaimedStateRegistered == false, "both of unclaimedFundRegistered and unclaimedStateRegistered must not be true");

        if (unclaimedFundRegistered) {
            require(msg.value == unclaimedFundClaimGas * maxFeePerGas, "incorrect ETH sent for unclaimed fund");
            totalFee += unclaimedFundClaimGas * maxFeePerGas;
        } else if (currContext.unclaimedStateRegistered) {
            require(msg.value == unclaimedStateClaimGas * maxFeePerGas, "incorrect ETH sent for unclaimed state");
            totalFee += unclaimedStateClaimGas * maxFeePerGas;
        } else {
            // Revert whatever was sent in case unclaimed fund/state registration didnt happen
            _transferETH(msg.sender, msg.value);
        }

        uint256 gasForRefund = 50000; // Rough estimate of gas cost for refund operation below (ERC20 transfer)
        uint256 consumedGas = initialGas - gasleft() + gasForRefund;
        totalFee += consumedGas * emailOp.feePerGas;

        address feeToken = _getTokenAddrFromName(emailOp.feeTokenName);
        uint256 feeAmount = totalFee / _getFeeConversionRate(emailOp.feeTokenName);

        if (feeAmount > 0) {
            _transferERC20(currContext.walletAddr, msg.sender, feeToken, feeAmount);
        }
    }

    /// @notice For extensions to request token from user's wallet (context wallet)
    /// @param tokenAddr Address of the ERC20 token requested
    /// @param amount Amount requested
    function requestTokenFromAccount(address tokenAddr, uint256 amount) public {
        require(msg.sender == currContext.extensionAddr, "invalid caller");

        for (uint256 i = 0; i < currContext.tokenAllowances.length; i++) {
            if (currContext.tokenAllowances[i].tokenAddr == tokenAddr) {
                require(currContext.tokenAllowances[i].amount >= amount, "insufficient allowance");
                currContext.tokenAllowances[i].amount -= amount;

                _transferERC20(currContext.walletAddr, currContext.extensionAddr, tokenAddr, amount);
                return;
            }
        }

        require(false, "no allowance for requested token");
    }

    /// @notice For extensions to deposit token to user's wallet (context wallet)
    /// @param tokenAddr Address of the ERC20 token to be deposited
    /// @param amount Amount to be deposited
    /// @dev Extension should add allowance to Core contract before calling this function
    function depositTokenToAccount(address tokenAddr, uint256 amount) public {
        require(msg.sender == currContext.extensionAddr, "invalid caller");

        IERC20(tokenAddr).transferFrom(msg.sender, currContext.walletAddr, amount);
    }

    /// Register a new extension
    /// @param name Name of the extension
    /// @param extensionAddr Address of the extension contract
    function publishExtension(string memory name, address extensionAddr, uint256 maxGas) public {
        require(addressOfExtension[name] == address(0), "extension name already used");

        addressOfExtension[name] = extensionAddr;
        maxGasOfExtension[extensionAddr] = maxGas;
    }

    /// @notice Calculate the masked subject for an EmailOp from command and other params
    ///         This also do "null" check for certain parameters used in the EmailOp
    /// @param emailOp EmailOp from which the masked subject is to be computed
    function _computeMaskedSubjectForEmailOp(
        EmailOp memory emailOp
    ) internal view returns (string memory maskedSubject, bool isExtension) {
        // Sample: Send 1 ETH to recipient@domain.com
        if (Strings.equal(emailOp.command, Commands.SEND)) {
            WalletParams memory walletParams = emailOp.walletParams;
            ERC20 token = ERC20(_getTokenAddrFromName(emailOp.walletParams.tokenName));

            require(token != ERC20(address(0)), "token not supported");
            require(emailOp.walletParams.amount > 0, "send amount should be >0");

            maskedSubject = string.concat(
                Commands.SEND,
                " ",
                DecimalUtils.uintToDecimalString(walletParams.amount),
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
            require(emailOp.executeCallData.length > 0, "executeCallData cannot be empty");

            (address target, , bytes memory data) = abi.decode(emailOp.executeCallData, (address, uint256, bytes));

            require(target != address(0), "invalid execute target");
            require(target != address(this), "cannot execute on Core contract");
            require(target != currContext.walletAddr, "cannot execute on wallet");
            require(data.length > 0, "execute data cannot be empty");

            maskedSubject = string.concat(Commands.EXECUTE, " 0x", bytesToHexString(emailOp.executeCallData));
        }
        // Sample: Set extension for Swap as Uniswap
        else if (Strings.equal(emailOp.command, Commands.INSTALL_EXTENSION)) {
            ExtensionManagerParams memory extManagerParams = emailOp.extManagerParams;

            require(bytes(extManagerParams.command).length > 0, "command cannot be empty");
            require(addressOfExtension[extManagerParams.extensionName] != address(0), "extension not registered");

            maskedSubject = string.concat(
                Commands.INSTALL_EXTENSION,
                " for ",
                extManagerParams.command,
                " as ",
                extManagerParams.extensionName
            );
        }
        // Sample: Remove extension for Swap
        else if (Strings.equal(emailOp.command, Commands.UNINSTALL_EXTENSION)) {
            require(bytes(emailOp.extManagerParams.command).length > 0, "command cannot be empty");

            maskedSubject = string.concat(Commands.UNINSTALL_EXTENSION, " for ", emailOp.extManagerParams.command);
        }
        // Sample: Exit email wallet. Change owner to 0x000112aa..
        else if (Strings.equal(emailOp.command, Commands.EXIT_EMAIL_WALLET)) {
            require(emailOp.newWalletOwner != address(0), "newWalletOwner cannot be empty");

            maskedSubject = string.concat(
                Commands.EXIT_EMAIL_WALLET,
                " ",
                Strings.toHexString(uint256(uint160(emailOp.newWalletOwner)), 20)
            );
        }
        // The command is for an extension
        else {
            isExtension = true;
            address extensionAddr = getExtensionForCommand(emailOp.command, emailOp.emailAddrPointer);

            require(extensionAddr != address(0), "invalid command or extension");

            for (uint8 i = 0; i < emailOp.extensionParams.tokenAmounts.length; i++) {
                require(emailOp.extensionParams.tokenAmounts[i].amount > 0, "token amount should be >0");
                require(
                    _getTokenAddrFromName(emailOp.extensionParams.tokenAmounts[i].tokenName) != address(0),
                    "token not supported"
                );
            }

            Extension extension = Extension(extensionAddr);
            maskedSubject = extension.computeEmailSubject(
                emailOp.extensionParams.subjectTemplateIndex,
                emailOp.extensionParams.tokenAmounts,
                emailOp.extensionParams.subjectParams
            );
        }
    }

    /// @notice Execute an EmailOp
    /// @param emailOp EmailOp to be executed
    /// @return success Whether the operation is successful
    /// @return unclaimedFundRegistered Whether unclaimed fund is registered
    /// @return returnData Return data from the operation (error)
    function _executeEmailOp(EmailOp memory emailOp) internal returns (bool success, bool unclaimedFundRegistered, bytes memory returnData) {
        unclaimedFundRegistered = false;
        // Wallet operation
        if (Strings.equal(emailOp.command, Commands.SEND)) {
            WalletParams memory walletParams = emailOp.walletParams;

            // If sending ETH to external wallet, use ETH instead of WETH
            if (!emailOp.hasEmailRecipient && Strings.equal(emailOp.walletParams.tokenName, "ETH")) {
                Wallet wallet = Wallet(payable(currContext.walletAddr));

                try wallet.execute(weth, 0, abi.encodeWithSignature("withdraw(uint256)", walletParams.amount)) {
                    // Sora's comment: Is this `execute` doing raw ETH transfer? 
                    wallet.execute(emailOp.recipientETHAddr, walletParams.amount, abi.encode(""));
                    success = true;
                } catch Error(string memory reason) {
                    success = false;
                    returnData = bytes(reason);
                } catch {
                    success = false;
                    returnData = bytes("err converting WETH to ETH");
                }

                return (success, false, returnData);
            }

            // Token transfer for both external contract and email wallet
            address recipient = emailOp.hasEmailRecipient ? address(this) : emailOp.recipientETHAddr;
            address tokenAddr = _getTokenAddrFromName(emailOp.walletParams.tokenName);

            (success, returnData) = _transferERC20(currContext.walletAddr, recipient, tokenAddr, walletParams.amount);

            if (!success) {
                return (success, false, returnData);
            }

            // Register unclaimed fund if the recipient is email wallet user
            if (emailOp.hasEmailRecipient) {
                _registerUnclaimedFund(
                    currContext.walletAddr,
                    emailOp.recipientEmailAddrCommit,
                    tokenAddr,
                    walletParams.amount,
                    0,
                    0,
                    ""
                );
                unclaimedFundRegistered = true;
                // currContext.unclaimedStateRegistered = true;
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
            ExtensionManagerParams memory extManagerParams = emailOp.extManagerParams;
            address extensionAddr = addressOfExtension[extManagerParams.extensionName];

            userExtensionOfCommand[emailOp.emailAddrPointer][extManagerParams.command] = extensionAddr;
        }
        // Remove custom extension for the user
        else if (Strings.equal(emailOp.command, Commands.UNINSTALL_EXTENSION)) {
            userExtensionOfCommand[emailOp.emailAddrPointer][emailOp.command] = address(0);
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
        // The command is for an extension
        else {
            address extAddress = getExtensionForCommand(emailOp.command, emailOp.emailAddrPointer);
            currContext.extensionAddr = extAddress;

            // Set token+amount pair in subject as allowances in context
            // We are assuming one token appear only once
            // Sora's comment: "We are assuming one token appear only once" <- This assumption should be verified in some ways, (e.g., when an extension is published, its templates are verified.)?
            for (uint8 i = 0; i < emailOp.extensionParams.tokenAmounts.length; i++) {
                address tokenAddr = _getTokenAddrFromName(emailOp.extensionParams.tokenAmounts[i].tokenName);

                currContext.tokenAllowances.push(
                    TokenAllowance({tokenAddr: tokenAddr, amount: emailOp.extensionParams.tokenAmounts[i].amount})
                );
            }

            // We only pass pre-configured gas to execute()
            try
                Extension(extAddress).execute{gas: maxGasOfExtension[extAddress]}(
                    emailOp.extensionParams.subjectTemplateIndex,
                    emailOp.extensionParams.tokenAmounts,
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
        if (expiryTime == 0) {
            expiryTime = block.timestamp + unclaimedFundExpiryDuration;
        }

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

    function _getTokenAddrFromName(string memory tokenName) internal view returns (address) {
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
        // Sora's comment: Is this `if (!validToken)`?
        if (!validToken) {
            return 0;
        }

        address tokenAddr = tokenRegistry.getTokenAddress(tokenName);
        if (tokenAddr == address(0)) {
            return 0;
        }

        return priceOracle.getRecentPriceInETH(tokenAddr);
    }

    function bytesToHexString(bytes memory data) public pure returns (string memory) {
        bytes memory hexChars = "0123456789abcdef";
        bytes memory hexString = new bytes(2 * data.length);

        for (uint256 i = 0; i < data.length; i++) {
            uint256 value = uint256(uint8(data[i]));
            hexString[2 * i] = hexChars[value >> 4];
            hexString[2 * i + 1] = hexChars[value & 0xf];
        }

        return string(hexString);
    }

    /// @notice Upgrade the implementation of the proxy contract
    /// @param newImplementation Address of the new implementation contract
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
