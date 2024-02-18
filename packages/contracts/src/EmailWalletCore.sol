// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Create2Upgradeable} from "@openzeppelin/contracts-upgradeable/utils/Create2Upgradeable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {IERC20, ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";
import {LibZip} from "solady/utils/LibZip.sol";
import {DecimalUtils} from "./libraries/DecimalUtils.sol";
import {SubjectUtils} from "./libraries/SubjectUtils.sol";
import {TokenRegistry} from "./utils/TokenRegistry.sol";
import {IVerifier} from "./interfaces/IVerifier.sol";
import {Extension} from "./interfaces/Extension.sol";
import {IPriceOracle} from "./interfaces/IPriceOracle.sol";
import {EmailWalletEvents} from "./interfaces/Events.sol";
import {RelayerHandler} from "./handlers/RelayerHandler.sol";
import {AccountHandler} from "./handlers/AccountHandler.sol";
import {UnclaimsHandler} from "./handlers/UnclaimsHandler.sol";
import {ExtensionHandler} from "./handlers/ExtensionHandler.sol";
import {Wallet} from "./Wallet.sol";
import "./interfaces/Types.sol";
import "./interfaces/Commands.sol";
import "./interfaces/Events.sol";

contract EmailWalletCore is Initializable, UUPSUpgradeable, OwnableUpgradeable {
    using SafeERC20 for IERC20;
    // ZK proof verifier
    IVerifier public verifier;

    // Relayer handler - Methods to create and update relayer config
    RelayerHandler public relayerHandler;

    // Account handler - Methods to create, intialize, transport user account and settings
    AccountHandler public accountHandler;

    // Unclaims handler - Methods to register, claim, void unclaimed funds and states
    UnclaimsHandler public unclaimsHandler;

    // Extension handler - Methods to publish and install extensions
    ExtensionHandler public extensionHandler;

    // Token registry
    TokenRegistry public tokenRegistry;

    // Price oracle for feeToken conversion
    IPriceOracle public priceOracle;

    // Address of WETH contract
    address public wethContract;

    // Max fee per gas in wei that relayer can set in a UserOp
    uint256 public maxFeePerGas;

    // Time period until which a email is valid for EmailOp based on the timestamp of the email signature
    uint256 public emailValidityDuration;

    // Gas required to claim unclaimed funds. User (their relayer) who register unclaimed funds
    // need to lock this amount which is relesed to the relayer who claims it
    uint256 public unclaimedFundClaimGas;

    // Gas required to claim unclaimed state
    uint256 public unclaimedStateClaimGas;

    // Mapping to store email nullifiers
    mapping(bytes32 => bool) public emailNullifiers;

    // Context of currently executing EmailOp - reset on every EmailOp
    ExecutionContext internal currContext;

    constructor() {
        _disableInitializers();
    }

    /// @notice Constructor
    /// @param _relayerHandler Address of the relayer handler contract
    /// @param _accountHandler Address of the account handler contract
    /// @param _unclaimsHandler Address of the unclaims handler contract
    /// @param _extensionHandler Address of the extension handler contract
    /// @param _verifier Address of the ZK proof verifier
    /// @param _tokenRegistry Address of the token registry contract
    /// @param _priceOracle Address of the price oracle contract
    /// @param _wethContract Address of the WETH contract
    /// @param _maxFeePerGas Max fee per gas in wei that relayer can set in a UserOp
    /// @param _emailValidityDuration Time period until which a email is valid for EmailOp based on the timestamp of the email signature
    /// @param _unclaimedFundClaimGas Gas required to claim unclaimed funds
    /// @param _unclaimedStateClaimGas Gas required to claim unclaimed state
    function initialize(
        address _relayerHandler,
        address _accountHandler,
        address _unclaimsHandler,
        address _extensionHandler,
        address _verifier,
        address _tokenRegistry,
        address _priceOracle,
        address _wethContract,
        uint256 _maxFeePerGas,
        uint256 _emailValidityDuration,
        uint256 _unclaimedFundClaimGas,
        uint256 _unclaimedStateClaimGas
    ) public initializer {
        __Ownable_init();
        relayerHandler = RelayerHandler(_relayerHandler);
        accountHandler = AccountHandler(_accountHandler);
        unclaimsHandler = UnclaimsHandler(payable(_unclaimsHandler));
        extensionHandler = ExtensionHandler(_extensionHandler);
        verifier = IVerifier(_verifier);
        tokenRegistry = TokenRegistry(_tokenRegistry);
        priceOracle = IPriceOracle(_priceOracle);
        wethContract = _wethContract;
        maxFeePerGas = _maxFeePerGas;
        emailValidityDuration = _emailValidityDuration;
        unclaimedFundClaimGas = _unclaimedFundClaimGas;
        unclaimedStateClaimGas = _unclaimedStateClaimGas;
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    /// @notice Initialize contract with some defaults after deployment
    /// @param defaultExtensions List of default extensions to be set
    function initializeExtension(bytes[] calldata defaultExtensions) public {
        extensionHandler.setDefaultExtensions(defaultExtensions);
    }

    // Upgradeable LibZip for calldata decompression
    fallback() external payable {
        LibZip.cdFallback();
    }

    // Core contract should not receive ETH
    receive() external payable {
        revert();
    }

    /// @notice Validate an EmailOp, including proof verification
    /// @param emailOp EmailOp to be validated
    function validateEmailOp(EmailOp memory emailOp) public view {
        (string memory relayerEmailAddr, ) = relayerHandler.relayers(msg.sender);
        require(bytes(relayerEmailAddr).length != 0, "relayer not registered");
        require(emailOp.walletSalt != bytes32(0), "wallet salt not set");
        require(bytes(emailOp.command).length != 0, "command cannot be empty");
        require(_getFeeConversionRate(emailOp.feeTokenName) != 0, "unsupported fee token");
        require(emailOp.feePerGas <= maxFeePerGas, "fee per gas too high");
        require(emailNullifiers[emailOp.emailNullifier] == false, "email nullified");
        require(accountHandler.emailNullifiers(emailOp.emailNullifier) == false, "email nullified");
        require(
            accountHandler.isDKIMPublicKeyHashValid(emailOp.walletSalt, emailOp.emailDomain, emailOp.dkimPublicKeyHash),
            "invalid DKIM public key"
        );

        if (emailOp.timestamp != 0) {
            require(emailOp.timestamp + emailValidityDuration > block.timestamp, "email expired");
        }

        if (emailOp.hasEmailRecipient) {
            require(emailOp.recipientETHAddr == address(0), "cannot have both recipient types");
            require(emailOp.recipientEmailAddrCommit != bytes32(0), "recipientEmailAddrCommit not found");
        } else {
            require(emailOp.recipientEmailAddrCommit == bytes32(0), "recipientEmailAddrCommit not allowed");
        }

        // Validate computed subject = passed subject
        (string memory computedSubject, ) = SubjectUtils.computeMaskedSubjectForEmailOp(
            emailOp,
            accountHandler.getWalletOfSalt(emailOp.walletSalt),
            this // Core contract to read some states
        );
        bytes memory maskedSubjectBytes = bytes(emailOp.maskedSubject);
        require(emailOp.skipSubjectPrefix < maskedSubjectBytes.length, "skipSubjectPrefix too high");
        bytes memory skippedSubjectBytes = new bytes(maskedSubjectBytes.length - emailOp.skipSubjectPrefix);
        for (uint i=0; i<skippedSubjectBytes.length; i++) {
            skippedSubjectBytes[i] = maskedSubjectBytes[emailOp.skipSubjectPrefix + i];
        }
        require(Strings.equal(computedSubject, string(skippedSubjectBytes)), string.concat("subject != ", computedSubject));

        // Verify proof
        require(
            verifier.verifyEmailOpProof(
                emailOp.emailDomain,
                emailOp.dkimPublicKeyHash,
                emailOp.timestamp,
                emailOp.emailNullifier,
                emailOp.maskedSubject,
                emailOp.walletSalt,
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
    ) public payable returns (bool success, bytes memory err, uint256 totalFeeInETH, uint256 registeredUnclaimId) {
        require(currContext.walletAddr == address(0), "context already set");

        uint256 initialGas = gasleft();

        // Set context for this EmailOp
        currContext.recipientEmailAddrCommit = emailOp.recipientEmailAddrCommit;
        currContext.walletAddr = accountHandler.getWalletOfSalt(emailOp.walletSalt);

        // Validate emailOp - will revert on failure. Relayer should ensure validate pass by simulation.
        validateEmailOp(emailOp);

        emailNullifiers[emailOp.emailNullifier] = true;

        // Execute EmailOp - wont revert on failure. Relayer will be compensated for gas even in failure.
        (success, err) = _executeEmailOp(emailOp);

        require(
            !(currContext.unclaimedFundRegistered && currContext.unclaimedStateRegistered),
            "cannot register both unclaimed fund and state"
        );

        // Validate and transfer ETH to unclaims handler  if unclaimed fund registration happened
        if (currContext.unclaimedFundRegistered) {
            require(msg.value == unclaimedFundClaimGas * maxFeePerGas, "incorrect ETH sent for unclaimed fund");
            totalFeeInETH += (unclaimedFundClaimGas * emailOp.feePerGas);
            payable(address(unclaimsHandler)).transfer(unclaimedFundClaimGas * maxFeePerGas);
        }
        // Validate and transfer ETH to unclaims handler if unclaimed state registration happened
        else if (currContext.unclaimedStateRegistered) {
            require(msg.value == unclaimedStateClaimGas * maxFeePerGas, "incorrect ETH sent for unclaimed state");
            totalFeeInETH += (unclaimedStateClaimGas * emailOp.feePerGas);
            payable(address(unclaimsHandler)).transfer(unclaimedStateClaimGas * maxFeePerGas);
        }
        // Return whatever ETH was sent in case unclaimed fund/state registration didnt happen
        else {
            require(
                currContext.registeredUnclaimId == 0,
                "registeredUnclaimId must be zero if no unclaimed fund/state is registered"
            );
            payable(msg.sender).transfer(msg.value);
        }

        registeredUnclaimId = currContext.registeredUnclaimId;

        uint256 gasForRefund = 55000; // Rough estimate of gas cost for refund (ERC20 transfer) and other operation below
        uint256 totalGas = initialGas - gasleft() + gasForRefund;
        totalFeeInETH += (totalGas * emailOp.feePerGas);
        uint256 rate = _getFeeConversionRate(emailOp.feeTokenName);
        uint256 feeAmountInToken = (totalFeeInETH * rate) / (10 ** 18);

        if (feeAmountInToken > 0) {
            address feeToken = tokenRegistry.getTokenAddress(emailOp.feeTokenName);

            (bool transferSuccess, bytes memory transferErr) = _transferERC20FromUserWallet(
                currContext.walletAddr,
                msg.sender,
                feeToken,
                feeAmountInToken
            );
            require(transferSuccess, string.concat("fee reimbursement failed: ", string(transferErr)));
        }

        // Reset context
        currContext.walletAddr = address(0);
        currContext.recipientEmailAddrCommit = bytes32(0);
        currContext.extensionAddr = address(0);
        currContext.unclaimedFundRegistered = false;
        currContext.unclaimedStateRegistered = false;
        currContext.registeredUnclaimId = 0;
        delete currContext.tokenAllowances;

        emit EmailWalletEvents.EmailOpHandled(
            success,
            registeredUnclaimId,
            emailOp.emailNullifier,
            emailOp.walletSalt,
            emailOp.recipientEmailAddrCommit,
            emailOp.recipientETHAddr,
            err
        );
    }

    /// For extension in context to register Unclaimed State during handleEmailOp
    /// @param extensionAddr Address of the extension contract to which the state is registered
    /// @param state State to be registered
    function registerUnclaimedStateAsExtension(address extensionAddr, bytes calldata state) public {
        require(extensionHandler.maxGasOfExtension(extensionAddr) != 0, "invalid extension");
        require(msg.sender == currContext.extensionAddr, "caller not extension");
        require(currContext.unclaimedStateRegistered == false, "unclaimed state exists");

        currContext.unclaimedStateRegistered = true;

        currContext.registeredUnclaimId = unclaimsHandler.registerUnclaimedStateInternal(
            extensionAddr,
            currContext.walletAddr,
            currContext.recipientEmailAddrCommit,
            state,
            extensionAddr == currContext.extensionAddr
        );
    }

    /// @notice For extension in context to request token from user's wallet during handleEmailOp
    /// @param tokenAddr Address of the ERC20 token requested
    /// @param amount Amount requested
    function requestTokenAsExtension(address tokenAddr, uint256 amount) public {
        require(msg.sender == currContext.extensionAddr, "caller not extension in context");

        for (uint256 i = 0; i < currContext.tokenAllowances.length; i++) {
            if (currContext.tokenAllowances[i].tokenAddr == tokenAddr) {
                require(currContext.tokenAllowances[i].amount >= amount, "insufficient allowance");
                currContext.tokenAllowances[i].amount -= amount;

                (bool success, bytes memory returnData) = _transferERC20FromUserWallet(
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

    /// @notice For extension in context to deposit token to user's wallet during handleEmailOp
    /// @param tokenAddr Address of the ERC20 token to be deposited
    /// @param amount Amount to be deposited
    /// @dev Extension should add allowance to Core contract before calling this function
    function depositTokenAsExtension(address tokenAddr, uint256 amount) public {
        require(msg.sender == currContext.extensionAddr, "caller not extension in context");

        IERC20(tokenAddr).safeTransferFrom(msg.sender, currContext.walletAddr, amount);
    }

    /// @notice For extension in context to execute on user's wallet during handleEmailOp
    /// @param target Address of the contract on which the call is to be executed
    /// @param data Calldata to be executed on the target contract
    /// @dev Do not use this method to transfer tokens. Use `requestTokenAsExtension()` instead
    function executeAsExtension(address target, bytes calldata data) public {
        require(msg.sender == currContext.extensionAddr, "caller not extension in context");
        require(
            target != address(this) &&
                target != address(unclaimsHandler) &&
                target != address(accountHandler) &&
                target != address(relayerHandler) &&
                target != address(extensionHandler),
            "target cannot be core or handlers"
        );

        require(Address.isContract(target), "target is not a contract");

        require(target != currContext.walletAddr, "target cannot be wallet");

        // Prevent extension from calling ERC20 tokens directly (tokenName should be empty)
        require(bytes(tokenRegistry.getTokenNameOfAddress(target)).length == 0, "target cannot be a token");

        Wallet(payable(currContext.walletAddr)).execute(target, 0, data);
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
                (success, returnData) = _transferERC20FromUserWallet(
                    currContext.walletAddr,
                    address(unclaimsHandler),
                    tokenAddr,
                    walletParams.amount
                );

                if (!success) {
                    return (success, returnData);
                }

                currContext.unclaimedFundRegistered = true;

                currContext.registeredUnclaimId = unclaimsHandler.registerUnclaimedFundInternal(
                    currContext.walletAddr,
                    emailOp.recipientEmailAddrCommit,
                    tokenAddr,
                    walletParams.amount
                );
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
                    (success, returnData) = _transferERC20FromUserWallet(
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
            address extensionAddr = extensionHandler.addressOfExtensionName(emailOp.extensionName);
            string memory command = extensionHandler.subjectTemplatesOfExtension(extensionAddr, 0, 0); // First word is command

            extensionHandler.setExtensionForCommand(currContext.walletAddr, command, extensionAddr);
            success = true;
        }
        // Remove custom extension for the user
        else if (Strings.equal(emailOp.command, Commands.UNINSTALL_EXTENSION)) {
            address extensionAddr = extensionHandler.addressOfExtensionName(emailOp.extensionName);
            string memory command = extensionHandler.subjectTemplatesOfExtension(extensionAddr, 0, 0);

            extensionHandler.setExtensionForCommand(currContext.walletAddr, command, address(0));
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
            accountHandler.updateDKIMRegistryOfWalletSalt(emailOp.walletSalt, emailOp.newDkimRegistry);
            success = true;
        }
        // The command is for an extension
        else {
            address extAddress = extensionHandler.getExtensionForCommand(currContext.walletAddr, emailOp.command);
            currContext.extensionAddr = extAddress;

            // Set token+amount pair in subject as allowances in context
            // We are assuming one token appear only once
            uint8 nextParamIndex = 0;
            string[] memory subjectTemplate = extensionHandler.getSubjectTemplatesOfExtension(extAddress)[
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
                    Strings.equal(matcher, Commands.ADDRESS_TEMPLATE)
                ) {
                    nextParamIndex++;
                }
            }

            // We only pass pre-configured gas to execute()
            try
                Extension(extAddress).execute{gas: extensionHandler.maxGasOfExtension(extAddress)}(
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
    function _transferERC20FromUserWallet(
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
}
