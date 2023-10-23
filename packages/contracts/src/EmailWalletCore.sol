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
import {IDKIMRegistry} from "@zk-email/contracts/interfaces/IDKIMRegistry.sol";
import {DecimalUtils} from "./libraries/DecimalUtils.sol";
import {BytesUtils} from "./libraries/BytesUtils.sol";
import {SubjectUtils} from "./libraries/SubjectUtils.sol";
import {TokenRegistry} from "./utils/TokenRegistry.sol";
import {IVerifier} from "./interfaces/IVerifier.sol";
import {Extension} from "./interfaces/Extension.sol";
import {IPriceOracle} from "./interfaces/IPriceOracle.sol";
import {EmailWalletEvents} from "./interfaces/Events.sol";
import {RelayerHandler} from "./handlers/RelayerHandler.sol";
import {AccountHandler} from "./handlers/AccountHandler.sol";
import {UnclaimsHandler} from "./handlers/UnclaimsHandler.sol";
import {Wallet} from "./Wallet.sol";
import "./interfaces/Types.sol";
import "./interfaces/Commands.sol";

contract EmailWalletCore is ReentrancyGuard {
    // ZK proof verifier
    IVerifier public immutable verifier;

    // Default DKIM public key hashes registry
    address public immutable defaultDkimRegistry;

    // Token registry
    TokenRegistry public immutable tokenRegistry;

    // Price oracle for feeToken conversion
    IPriceOracle public immutable priceOracle;

    // Address of WETH contract
    address public immutable wethContract;

    // Address of wallet implementation contract - used for deploying wallets for users via proxy
    address public immutable walletImplementation;

    // Max fee per gas in wei that relayer can set in a UserOp
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

    // Mapping to store email nullifiers
    mapping(bytes32 => bool) public emailNullifiers;

    // Mapping from extensio name to extension address, as published by the developer
    mapping(string => address) public addressOfExtensionName;

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

    // Context of currently executing EmailOp - reset on every EmailOp
    ExecutionContext public currContext;

    RelayerHandler public relayerHandler;

    AccountHandler public accountHandler;

    UnclaimsHandler public unclaimsHandler;

    modifier nullifyEmail(bytes32 emailNullifier) {
        require(emailNullifiers[emailNullifier] == false, "email nullified");
        _;
        emailNullifiers[emailNullifier] = true;
    }

    constructor(
        address _verifier,
        address _walletImplementationAddr,
        address _tokenRegistry,
        address _defaultDkimRegistry,
        address _priceOracle,
        address _wethContract,
        uint256 _maxFeePerGas,
        uint256 _emailValidityDuration,
        uint256 _unclaimedFundClaimGas,
        uint256 _unclaimedStateClaimGas,
        uint256 _unclaimsExpiryDuration
    ) {
        verifier = IVerifier(_verifier);
        walletImplementation = _walletImplementationAddr;
        defaultDkimRegistry = _defaultDkimRegistry;
        tokenRegistry = TokenRegistry(_tokenRegistry);
        priceOracle = IPriceOracle(_priceOracle);
        wethContract = _wethContract;
        maxFeePerGas = _maxFeePerGas;
        emailValidityDuration = _emailValidityDuration;
        unclaimsExpiryDuration = _unclaimsExpiryDuration;
        unclaimedFundClaimGas = _unclaimedFundClaimGas;
        unclaimedStateClaimGas = _unclaimedStateClaimGas;

        relayerHandler = new RelayerHandler();
        accountHandler = new AccountHandler(
            emailValidityDuration,
            defaultDkimRegistry,
            walletImplementation,
            address(relayerHandler),
            _verifier
        );
        unclaimsHandler = new UnclaimsHandler(
            address(accountHandler),
            address(relayerHandler),
            _verifier,
            _unclaimedFundClaimGas,
            _unclaimedStateClaimGas,
            _unclaimsExpiryDuration,
            _maxFeePerGas
        );
    }

    receive() external payable {
        revert();
    }

    function initialize(bytes[] calldata defaultExtensions) public {
        // Set default extensions
        for (uint256 i = 0; i < defaultExtensions.length; i++) {
            (string memory name, address addr, string[][] memory templates, uint256 maxGas) = abi.decode(
                defaultExtensions[i],
                (string, address, string[][], uint256)
            );

            addressOfExtensionName[name] = addr;
            subjectTemplatesOfExtension[addr] = templates;
            maxGasOfExtension[addr] = maxGas;
            defaultExtensionOfCommand[templates[0][0]] = addr;
        }
    }

    function registerRelayer(bytes32 randHash, string calldata emailAddr, string calldata hostname) public {
        relayerHandler.registerRelayer(msg.sender, randHash, emailAddr, hostname);
    }

    function updateRelayerConfig(string calldata hostname) public {
        relayerHandler.updateRelayerConfig(msg.sender, hostname);
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
        bytes calldata psiPoint,
        bytes calldata proof
    ) public returns (Wallet wallet) {
        accountHandler.createAccount(msg.sender, emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, proof);

        return _deployWallet(walletSalt);
    }

    /// Initialize the account when user reply to invitation email
    /// @param emailAddrPointer hash(relayerRand, emailAddr)
    /// @param emailDomain domain name of the sender's email
    /// @param emailNullifier nullifier of the email used for proof generation
    /// @param proof ZK proof as required by the verifier
    function initializeAccount(
        bytes32 emailAddrPointer,
        string calldata emailDomain,
        uint256 emailTimestamp,
        bytes32 emailNullifier,
        bytes calldata proof
    ) public nullifyEmail(emailNullifier) {
        accountHandler.initializeAccount(
            msg.sender,
            emailAddrPointer,
            emailDomain,
            emailTimestamp,
            emailNullifier,
            proof
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
    ) public nullifyEmail(transportEmailProof.nullifier) {
        accountHandler.transportAccount(
            msg.sender,
            oldAccountKeyCommit,
            newEmailAddrPointer,
            newAccountKeyCommit,
            newPSIPoint,
            transportEmailProof,
            accountCreationProof
        );
    }

    /// @notice Validate an EmailOp, including proof verification
    /// @param emailOp EmailOp to be validated
    function validateEmailOp(EmailOp memory emailOp) public view {
        AccountKeyInfo memory accountKeyInfo = accountHandler.getInfoOfAccountKeyCommit(
            accountHandler.accountKeyCommitOfPointer(emailOp.emailAddrPointer)
        );
        bytes32 dkimPublicKeyHash = accountHandler.getDKIMPublicKeyHash(accountKeyInfo.walletSalt, emailOp.emailDomain);
        address walletAddr = accountHandler.getWalletOfSalt(accountKeyInfo.walletSalt);

        require(accountKeyInfo.relayer == msg.sender, "invalid relayer");
        require(accountKeyInfo.initialized, "account not initialized");
        require(accountKeyInfo.walletSalt != bytes32(0), "wallet salt not set");
        require(emailOp.timestamp + emailValidityDuration > block.timestamp, "email expired");
        require(dkimPublicKeyHash != bytes32(0), "cannot find DKIM for domain");
        require(relayerHandler.getRandHash(msg.sender) != bytes32(0), "relayer not registered");
        require(emailNullifiers[emailOp.emailNullifier] == false, "email nullifier already used");
        require(bytes(emailOp.command).length != 0, "command cannot be empty");
        require(_getFeeConversionRate(emailOp.feeTokenName) != 0, "unsupported fee token");
        require(emailOp.feePerGas <= maxFeePerGas, "fee per gas too high");

        if (emailOp.hasEmailRecipient) {
            require(emailOp.recipientETHAddr == address(0), "cannot have both recipient types");
            require(emailOp.recipientEmailAddrCommit != bytes32(0), "recipientEmailAddrCommit not found");
            require(
                unclaimsHandler.getSenderOfUnclaimedFund(emailOp.recipientEmailAddrCommit) == address(0),
                "unclaimed fund exist"
            );
            require(
                unclaimsHandler.getSenderOfUnclaimedState(emailOp.recipientEmailAddrCommit) == address(0),
                "unclaimed state exists"
            );
        } else {
            require(emailOp.recipientEmailAddrCommit == bytes32(0), "recipientEmailAddrCommit not allowed");
        }

        // Validate computed subject = passed subject
        (string memory maskedSubject, ) = SubjectUtils.computeMaskedSubjectForEmailOp(
            emailOp,
            walletAddr,
            tokenRegistry,
            addressOfExtensionName[emailOp.extensionName], // Address of extension (user in install / uninstall)
            subjectTemplatesOfExtension[getExtensionForCommand(emailOp.command, walletAddr)] // Subject templates of extension
        );
        require(Strings.equal(maskedSubject, emailOp.maskedSubject), string.concat("subject != ", maskedSubject));

        // Verify proof
        require(
            verifier.verifyEmailOpProof(
                emailOp.emailDomain,
                dkimPublicKeyHash,
                emailOp.timestamp,
                emailOp.maskedSubject,
                emailOp.emailNullifier,
                relayerHandler.getRandHash(msg.sender),
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
    /// @return success Whether the execution was successful
    /// @return err Error message if execution failed (execution failures will not revert)
    /// @return totalFeeInETH Total fee in ETH that should be reimbursed to the relayer
    /// @dev ETH for unclaimed fund/state registration should be send if the recipient is an email address
    /// @dev Relayer should make sure user has enough tokens to pay for the fee. This can be calculated as
    /// @dev ~ verificationGas(fixed) + executionGas(extension.maxGas if extension) + feeForReimbursement(50k) + msg.value
    function handleEmailOp(
        EmailOp calldata emailOp
    ) public payable nonReentrant returns (bool success, bytes memory err, uint256 totalFeeInETH) {
        uint256 initialGas = gasleft();

        // Set context for this EmailOp
        currContext.recipientEmailAddrCommit = emailOp.recipientEmailAddrCommit;
        currContext.walletAddr = accountHandler.getWalletOfSalt(
            accountHandler
                .getInfoOfAccountKeyCommit(accountHandler.accountKeyCommitOfPointer(emailOp.emailAddrPointer))
                .walletSalt
        );

        // Validate emailOp - will revert on failure. Relayer should ensure validate pass by simulation.
        validateEmailOp(emailOp);

        emailNullifiers[emailOp.emailNullifier] = true;

        // Execute EmailOp - wont revert on failure. Relayer will be compensated for gas even in failure.
        (success, err) = _executeEmailOp(emailOp);

        require(
            !(currContext.unclaimedFundRegistered && currContext.unclaimedStateRegistered),
            "cannot register both unclaimed fund and state"
        );

        if (currContext.unclaimedFundRegistered) {
            require(msg.value == unclaimedFundClaimGas * maxFeePerGas, "incorrect ETH sent for unclaimed fund");
            totalFeeInETH += (unclaimedFundClaimGas * maxFeePerGas);
        } else if (currContext.unclaimedStateRegistered) {
            require(msg.value == unclaimedStateClaimGas * maxFeePerGas, "incorrect ETH sent for unclaimed state");
            totalFeeInETH += (unclaimedStateClaimGas * maxFeePerGas);
        } else {
            // Return whatever ETH was sent in case unclaimed fund/state registration didnt happen
            (bool sent, ) = payable(msg.sender).call{value: msg.value}("");
            require(sent, "failed to transfer ETH");
        }

        // Reset context
        currContext.extensionAddr = address(0);
        currContext.unclaimedFundRegistered = false;
        currContext.unclaimedStateRegistered = false;
        delete currContext.tokenAllowances;

        uint256 gasForRefund = 50000; // Rough estimate of gas cost for refund operation below (ERC20 transfer)
        uint256 totalGas = initialGas - gasleft() + gasForRefund;
        totalFeeInETH += (totalGas * emailOp.feePerGas);
        uint256 rate = _getFeeConversionRate(emailOp.feeTokenName);
        uint256 feeAmountInToken = (totalFeeInETH * rate) / (10 ** 18);

        if (feeAmountInToken > 0) {
            address feeToken = tokenRegistry.getTokenAddress(emailOp.feeTokenName);
            (success, err) = _transferERC20(currContext.walletAddr, msg.sender, feeToken, feeAmountInToken);
            require(success, string.concat("fee reimbursement failed: ", string(err)));
        }
    }

    /// Register unclaimed state from an extension
    /// @param extensionAddr Address of the extension contract to which the state is registered
    /// @param state State to be registered
    function registerUnclaimedStateAsExtension(address extensionAddr, bytes calldata state) public {
        require(maxGasOfExtension[extensionAddr] != 0, "invalid extension");
        require(msg.sender == currContext.extensionAddr, "caller not extension");
        require(currContext.unclaimedStateRegistered == false, "unclaimed state exists");

        unclaimsHandler.registerUnclaimedStateInternal(
            extensionAddr,
            currContext.walletAddr,
            currContext.recipientEmailAddrCommit,
            state
        );

        currContext.unclaimedStateRegistered = true;
    }

    /// @notice For extensions to request token from user's wallet (context wallet)
    /// @param tokenAddr Address of the ERC20 token requested
    /// @param amount Amount requested
    function requestTokenAsExtension(address tokenAddr, uint256 amount) public {
        require(msg.sender == currContext.extensionAddr, "caller not extension in context");

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
        require(msg.sender == currContext.extensionAddr, "caller not extension in context");

        IERC20(tokenAddr).transferFrom(msg.sender, currContext.walletAddr, amount);
    }

    /// @notice For extensions to execute a call on user's wallet
    /// @param target Address of the contract on which the call is to be executed
    /// @param data Calldata to be executed on the target contract
    /// @dev Do not use this method to transfer tokens. Use `requestTokenAsExtension()` instead
    function executeAsExtension(address target, bytes calldata data) public {
        require(msg.sender == currContext.extensionAddr, "caller not extension in context");
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
        string calldata name,
        address addr,
        string[][] memory subjectTemplates,
        uint256 maxExecutionGas
    ) public {
        require(addressOfExtensionName[name] == address(0), "extension name already used");
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
                if (Strings.equal(subjectTemplates[i][j], Commands.RECIPIENT_TEMPLATE)) {
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
            !Strings.equal(command, Commands.TOKEN_AMOUNT_TEMPLATE) &&
                !Strings.equal(command, Commands.AMOUNT_TEMPLATE) &&
                !Strings.equal(command, Commands.STRING_TEMPLATE) &&
                !Strings.equal(command, Commands.UINT_TEMPLATE) &&
                !Strings.equal(command, Commands.INT_TEMPLATE) &&
                !Strings.equal(command, Commands.ADDRESS_TEMPLATE),
            "command cannot be a template matcher"
        );

        addressOfExtensionName[name] = addr;
        subjectTemplatesOfExtension[addr] = subjectTemplates;
        maxGasOfExtension[addr] = maxExecutionGas;

        emit EmailWalletEvents.ExtensionPublished(name, addr, subjectTemplates, maxExecutionGas);
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

    /// @notice Execute an EmailOp
    /// @param emailOp EmailOp to be executed
    /// @return success Whether the operation is successful
    /// @return returnData Return data from the operation (error)
    function _executeEmailOp(EmailOp memory emailOp) internal returns (bool success, bytes memory returnData) {
        // Wallet operation
        if (Strings.equal(emailOp.command, Commands.SEND)) {
            WalletParams memory walletParams = emailOp.walletParams;
            address tokenAddr = tokenRegistry.getTokenAddress(emailOp.walletParams.tokenName);

            // Register unclaimed fund if the recipient is email wallet user + move tokens to unclaims handler
            if (emailOp.hasEmailRecipient) {
                (success, returnData) = _transferERC20(
                    currContext.walletAddr,
                    address(unclaimsHandler),
                    tokenAddr,
                    walletParams.amount
                );

                if (!success) {
                    return (success, returnData);
                }

                unclaimsHandler.registerUnclaimedFundInternal(
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

            if (!emailOp.hasEmailRecipient) {
                // If sending ETH to external wallet, use ETH instead of WETH
                if (Strings.equal(emailOp.walletParams.tokenName, "ETH")) {
                    Wallet wallet = Wallet(payable(currContext.walletAddr));

                    try
                        wallet.execute(
                            wethContract,
                            0,
                            abi.encodeWithSignature("withdraw(uint256)", walletParams.amount)
                        )
                    {
                        wallet.execute(emailOp.recipientETHAddr, walletParams.amount, "");
                        success = true;
                    } catch Error(string memory reason) {
                        success = false;
                        returnData = bytes(reason);
                    } catch {
                        success = false;
                        returnData = bytes("err converting WETH to ETH");
                    }
                } else {
                    // Transfer tokens to recipient
                    (success, returnData) = _transferERC20(
                        currContext.walletAddr,
                        emailOp.recipientETHAddr,
                        tokenAddr,
                        walletParams.amount
                    );
                }
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
            address extensionAddr = addressOfExtensionName[emailOp.extensionName];
            string memory command = subjectTemplatesOfExtension[extensionAddr][0][0];

            userExtensionOfCommand[currContext.walletAddr][command] = extensionAddr;
            success = true;
        }
        // Remove custom extension for the user
        else if (Strings.equal(emailOp.command, Commands.UNINSTALL_EXTENSION)) {
            address extensionAddr = addressOfExtensionName[emailOp.extensionName];
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
            bytes32 accountKeyCommit = accountHandler.accountKeyCommitOfPointer(emailOp.emailAddrPointer);
            accountHandler.updateDKIMRegistryOfWalletSalt(
                accountHandler.getInfoOfAccountKeyCommit(accountKeyCommit).walletSalt,
                emailOp.newDkimRegistry
            );
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

                if (Strings.equal(matcher, Commands.TOKEN_AMOUNT_TEMPLATE)) {
                    (uint256 amount, string memory tokenName) = abi.decode(
                        emailOp.extensionParams.subjectParams[nextParamIndex],
                        (uint256, string)
                    );
                    currContext.tokenAllowances.push(
                        TokenAllowance({tokenAddr: tokenRegistry.getTokenAddress(tokenName), amount: amount})
                    );
                    nextParamIndex++;
                } else if (
                    Strings.equal(matcher, Commands.AMOUNT_TEMPLATE) ||
                    Strings.equal(matcher, Commands.STRING_TEMPLATE) ||
                    Strings.equal(matcher, Commands.UINT_TEMPLATE) ||
                    Strings.equal(matcher, Commands.INT_TEMPLATE) ||
                    Strings.equal(matcher, Commands.ADDRESS_TEMPLATE) ||
                    Strings.equal(matcher, Commands.RECIPIENT_TEMPLATE)
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

    /// @notice Return the conversion rate for a token. i.e returns how many tokens 1 ETH could buy in wei format
    /// @param tokenName Name of the token
    /// @return Conversion rate in wei format
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

    // /// @notice Upgrade the implementation of the proxy contract
    // /// @param newImplementation Address of the new implementation contract
    // function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
