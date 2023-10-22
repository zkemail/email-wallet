// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {DKIMRegistry} from "@zk-email/contracts/DKIMRegistry.sol";
import {Wallet} from "../../src/Wallet.sol";
import {EmailWalletCore} from "../../src/EmailWalletCore.sol";
import {TokenRegistry} from "../../src/utils/TokenRegistry.sol";
import {BytesUtils} from "../../src/libraries/BytesUtils.sol";
import {TestVerifier} from "../mocks/TestVerifier.sol";
import {TestERC20} from "../mocks/TestERC20.sol";
import {TestOracle} from "../mocks/TestOracle.sol";
import {TestExtension} from "../mocks/TestExtension.sol";
import {WETH9} from "../helpers/WETH9.sol";
import {Extension} from "../../src/interfaces/Extension.sol";
import {EmailWalletEvents} from "../../src/interfaces/Events.sol";
import {IPriceOracle} from "../../src/interfaces/IPriceOracle.sol";
import {RelayerHandler} from "../../src/handlers/RelayerHandler.sol";
import "../../src/interfaces/Types.sol";
import "../../src/interfaces/Commands.sol";

contract EmailWalletCoreTestHelper is Test {
    EmailWalletCore core;
    TestVerifier verifier;
    TokenRegistry tokenRegistry;
    DKIMRegistry dkimRegistry;
    IPriceOracle priceOracle;
    WETH9 weth;

    TestERC20 daiToken;
    TestERC20 usdcToken;

    address deployer;
    address relayer;

    bytes mockProof = abi.encodePacked(bytes1(0x01));

    uint256 maxFeePerGas = 5 gwei;
    uint256 emailValidityDuration = 1 hours;
    uint256 unclaimedFundClaimGas = 100000;
    uint256 unclaimedStateClaimGas = 200000;
    uint256 unclaimsExpiryDuration = 30 days;

    // Relayer details
    uint256 relayerRand = 10001;
    bytes32 randHash = keccak256(abi.encodePacked(relayerRand));

    // User details (sender) - for when sender failure is not expected
    // Computing hashes to resemble the actual process
    string senderEmail = "sender@test.com";
    string emailDomain = "test.com";
    uint256 accountKey = 2001;
    bytes32 emailAddrPointer = keccak256(abi.encodePacked(relayerRand, senderEmail));
    bytes32 accountKeyCommit = keccak256(abi.encodePacked(accountKey, senderEmail, randHash));
    bytes32 walletSalt = keccak256(abi.encodePacked(accountKeyCommit, uint(0)));
    bytes psiPoint = abi.encodePacked(uint(1004));
    address walletAddr;

    bytes32 emailNullifier = bytes32(uint256(5000001));
    bytes32 emailNullifier2 = bytes32(uint256(5000002));
    bytes32 emailNullifier3 = bytes32(uint256(5000003));

    string[][] _defaultExtTemplates = new string[][](1);

    address defaultExtAddr;

    function setUp() public virtual {
        deployer = vm.addr(1);
        relayer = vm.addr(2);

        vm.startPrank(deployer);

        verifier = new TestVerifier();
        tokenRegistry = new TokenRegistry();
        dkimRegistry = new DKIMRegistry();
        priceOracle = new TestOracle();
        weth = new WETH9();

        Wallet walletImp = new Wallet(address(weth));

        // Deploy a test extension with command "DEF" - only for testing defaultExtensions
        TestExtension defaultExt = new TestExtension(address(0), address(0), address(0));
        defaultExtAddr = address(defaultExt);
        bytes[] memory defaultExtensions = new bytes[](1);
        _defaultExtTemplates[0] = ["DEF_EXT", "NOOP"];
        defaultExtensions[0] = abi.encode("DEF_EXT_NAME", address(defaultExt), _defaultExtTemplates, 1 ether);

        RelayerHandler relayerHandler = new RelayerHandler();

        // Deploy core contract as proxy
        core = new EmailWalletCore(
            address(verifier),
            address(walletImp),
            address(tokenRegistry),
            address(dkimRegistry),
            address(priceOracle),
            address(weth),
            maxFeePerGas,
            emailValidityDuration,
            unclaimedFundClaimGas,
            unclaimedStateClaimGas,
            unclaimsExpiryDuration,
            address(relayerHandler)
        );
        core.initialize(defaultExtensions);
        relayerHandler.transferOwnership(address(core));

        // Set test sender's wallet addr
        walletAddr = core.getWalletOfSalt(walletSalt);

        // Set a mock DKIM public key hash for test sender's emailDomain
        dkimRegistry.setDKIMPublicKeyHash(emailDomain, bytes32(uint256(111122223333)));

        // Deploy some ERC20 test tokens and add them to registry
        daiToken = new TestERC20("DAI", "DAI");
        usdcToken = new TestERC20("USDC", "USDC");
        tokenRegistry.setTokenAddress("WETH", address(weth));
        tokenRegistry.setTokenAddress("DAI", address(daiToken));
        tokenRegistry.setTokenAddress("USDC", address(usdcToken));

        vm.stopPrank();
    }

    // Register the test relayer - when not testing relayer functionality
    function _registerRelayer() internal {
        vm.startPrank(relayer);
        core.registerRelayer(randHash, "relayer@relayer.xyz", "relayer.xyz");
        vm.stopPrank();
    }

    // Register test user account - for using as sender when not testing accoung functionality
    function _registerAndInitializeAccount() internal {
        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();
    }

    // Helper function for creating a base EmailOp
    function _getBaseEmailOp() internal view returns (EmailOp memory) {
        return
            EmailOp({
                emailAddrPointer: emailAddrPointer,
                hasEmailRecipient: false,
                recipientEmailAddrCommit: bytes32(0),
                recipientETHAddr: address(0),
                command: "",
                emailNullifier: bytes32(uint256(13981239)),
                emailDomain: emailDomain,
                timestamp: block.timestamp,
                maskedSubject: "",
                feeTokenName: "ETH",
                feePerGas: 0,
                executeCallData: abi.encodePacked(""),
                newWalletOwner: address(0),
                walletParams: WalletParams({tokenName: "", amount: 0}),
                extensionName: "",
                extensionParams: ExtensionParams({subjectTemplateIndex: 0, subjectParams: new bytes[](0)}),
                newDkimRegistry: address(0),
                emailProof: mockProof
            });
    }

    function _getTokenSendingEmailOp() internal view returns (EmailOp memory) {
        EmailOp memory emailOp = _getBaseEmailOp();

        emailOp.command = "Send";
        emailOp.maskedSubject = "Send 1 DAI to ";
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = bytes32(uint256(3333));
        emailOp.walletParams.amount = 1 ether;
        emailOp.walletParams.tokenName = "DAI";

        return emailOp;
    }
}
