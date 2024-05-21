// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";
import "../src/extensions/Safe2FA.sol";
import "./mocks/TestExtension.sol";
import {SafeL2} from "safe-contracts/contracts/SafeL2.sol";
import {SafeProxy} from "safe-contracts/contracts/proxies/SafeProxy.sol";
import {OwnerManager} from "safe-contracts/contracts/base/OwnerManager.sol";
import {Enum} from "safe-contracts/contracts/common/Enum.sol";
import {StringUtils} from "../src/libraries/StringUtils.sol";

// Testing extension functionality for Safe2FA extension
contract Safe2FAExtensionCommandTest is EmailWalletCoreTestHelper {
    using StringUtils for *;

    Safe2FAExtension safe2faExtension;
    string[][] public safeExtTemplates = new string[][](1);
    SafeL2 safeAccount;
    address safeEOAOwner1;
    uint256 safeEOAOwner1Pk;
    // address safeEOAOwner2 = vm.addr(12);

    fallback() external {
        // For one test below to call this contract with empty calldata
    }

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _createTestAccount();

        // Publish and install extension
        {
            Safe2FAExtension safeExtensionImpl = new Safe2FAExtension();
            ERC1967Proxy proxy = new ERC1967Proxy(
                address(safeExtensionImpl),
                abi.encodeCall(safeExtensionImpl.initialize, (address(core)))
            );
            safe2faExtension = Safe2FAExtension(payable(address(proxy)));

            safeExtTemplates[0] = ["Safe", "Transaction:", "Approve", "{string}", "from", "{address}"];
            extensionHandler.publishExtension("Safe2FA", address(safe2faExtension), safeExtTemplates, 0.1 ether);
        }

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.extensionName = "Safe2FA";
        emailOp.maskedSubject = "Install extension Safe2FA";
        emailOp.emailNullifier = bytes32(uint256(93845));

        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "failed to register safe2FA extension");

        SafeL2 safeSingleton = new SafeL2();
        SafeProxy safeProxy = new SafeProxy(address(safeSingleton));
        safeAccount = SafeL2(payable(address(safeProxy)));
        address[] memory owners = new address[](1);
        (safeEOAOwner1, safeEOAOwner1Pk) = makeAddrAndKey("safe1");
        owners[0] = safeEOAOwner1;
        // owners[1] = safeEOAOwner2;
        safeAccount.setup(owners, 1, address(0), bytes("0"), address(0), address(0), 0, payable(address(0)));
    }

    function test_Safe2FAExtension_AddEmailOwner() public {
        _addEmailOwner();
        assertTrue(safeAccount.isOwner(address(walletAddr)), "failed to add email owner");
        assertTrue(safeAccount.getThreshold() == 2, "failed to set threshold");
    }

    function test_Safe2FAExtension_AuthTxHash() public {
        _addEmailOwner();
        bytes32 txHash = keccak256(abi.encodePacked("test"));
        _authTxByEmail(txHash);
        assertTrue(safeAccount.approvedHashes(address(walletAddr), txHash) == 1, "failed to auth tx");
    }

    function test_Safe2FAExtension_AuthETHTransferByEOA() public {
        deal(address(safeAccount), 10 ether);
        assertEq(address(safeAccount).balance, 10 ether, "failed to deposit to safe account");
        address recipient = vm.addr(110);
        bytes32 txHash = safeAccount.getTransactionHash(
            recipient,
            1 ether,
            bytes(""),
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            address(safeAccount),
            safeAccount.nonce()
        );
        bytes memory signature = _signTxByEOA(safeEOAOwner1Pk, txHash);
        safeAccount.execTransaction(
            recipient,
            1 ether,
            bytes(""),
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            payable(address(safeAccount)),
            signature
        );
        assertEq(address(safeAccount).balance, 9 ether, "failed to transfer from safe account");
    }

    function test_Safe2FAExtension_AuthETHTransferByEmail() public {
        deal(address(safeAccount), 10 ether);
        assertEq(address(safeAccount).balance, 10 ether, "failed to deposit to safe account");

        _addEmailOwner();
        address recipient = vm.addr(110);
        bytes32 txHash = safeAccount.getTransactionHash(
            recipient,
            1 ether,
            bytes(""),
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            address(safeAccount),
            safeAccount.nonce()
        );
        bytes memory signatureEOA = _signTxByEOA(safeEOAOwner1Pk, txHash);
        _authTxByEmail(txHash);
        bytes memory signatureEmail = _signTxByEmail(walletAddr);
        bytes memory signature;
        if (safeEOAOwner1 < walletAddr) {
            signature = abi.encodePacked(signatureEOA, signatureEmail);
        } else {
            signature = abi.encodePacked(signatureEmail, signatureEOA);
        }
        safeAccount.execTransaction(
            recipient,
            1 ether,
            bytes(""),
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            payable(address(safeAccount)),
            signature
        );
        assertEq(address(safeAccount).balance, 9 ether, "failed to transfer from safe account");
    }

    function _addEmailOwner() private {
        vm.startPrank(address(safeAccount));
        safeAccount.addOwnerWithThreshold(address(walletAddr), 2);
        vm.stopPrank();
    }

    function _authTxByEmail(bytes32 txHash) private {
        vm.startPrank(relayer);
        string memory txHashStr = uint256(txHash).toHexString();
        // Build email op
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Safe";
        emailOp.maskedSubject = string.concat(
            "Safe Transaction: Approve ",
            txHashStr,
            " from ",
            SubjectUtils.addressToChecksumHexString(address(safeAccount))
        );
        emailOp.extensionName = "Safe2FA";
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(txHashStr);
        emailOp.extensionParams.subjectParams[1] = abi.encode(address(safeAccount));
        (bool success, , , ) = core.handleEmailOp(emailOp);
        assertTrue(success, "emailOp failed");
        vm.stopPrank();
    }

    function _signTxByEOA(uint256 pk, bytes32 txHash) private pure returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, txHash);
        return abi.encodePacked(r, s, v);
    }

    function _signTxByEmail(address signer) private pure returns (bytes memory) {
        uint8 v = 1;
        bytes32 r = bytes32(uint256(uint160(signer)));
        bytes32 s = bytes32(0);
        return abi.encodePacked(r, s, v);
    }

    // function test_UniExtension_SubjectWithSwapETHToDAI() public {
    //     // Mint 10 ETH and convert to WETH
    //     vm.startPrank(walletAddr);
    //     deal(address(walletAddr), 10 ether);
    //     weth.deposit{value: 10 ether}();
    //     vm.stopPrank();

    //     // Build email op
    //     EmailOp memory emailOp = _getBaseEmailOp();
    //     emailOp.command = "Swap";
    //     emailOp.maskedSubject = "Swap 0.2 ETH to DAI";
    //     emailOp.extensionName = "Uniswap";
    //     emailOp.extensionParams.subjectTemplateIndex = 0;
    //     emailOp.hasEmailRecipient = false;
    //     emailOp.extensionParams.subjectParams = new bytes[](2);
    //     emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(0.2 ether), "ETH");
    //     emailOp.extensionParams.subjectParams[1] = abi.encode("DAI");

    //     vm.startPrank(relayer);

    //     // Mock for isPoolExists should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
    //         abi.encode(true)
    //     );
    //     // Mock for getPoolSlot0 should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
    //         abi.encode(0, 0, 0, 0, 0, 0, true)
    //     );
    //     // Mock for exactInputSingle should return 0.
    //     vm.mockCall(
    //         address(uniExtension.router()),
    //         abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
    //         abi.encode(0)
    //     );

    //     (bool success, , , ) = core.handleEmailOp(emailOp);
    //     vm.stopPrank();

    //     assertTrue(success, "emailOp failed");
    // }

    // function test_UniExtension_SubjectWithSwapDAIToUSDC() public {
    //     // Mint 10 ETH and convert to WETH
    //     // Mint 200 DAI
    //     vm.startPrank(walletAddr);
    //     deal(address(walletAddr), 10 ether);
    //     weth.deposit{value: 10 ether}();
    //     deal(address(daiToken), walletAddr, 20 * 10000 ether);
    //     vm.stopPrank();

    //     // Build email op
    //     EmailOp memory emailOp = _getBaseEmailOp();
    //     emailOp.command = "Swap";
    //     emailOp.maskedSubject = "Swap 200 DAI to USDC";
    //     emailOp.extensionName = "Uniswap";
    //     emailOp.extensionParams.subjectTemplateIndex = 0;
    //     emailOp.hasEmailRecipient = false;
    //     emailOp.extensionParams.subjectParams = new bytes[](2);
    //     emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(200 ether), "DAI");
    //     emailOp.extensionParams.subjectParams[1] = abi.encode("USDC");

    //     vm.startPrank(relayer);

    //     // Mock for daiToken approval should return true.
    //     vm.mockCall(address(daiToken), abi.encodeWithSelector(TestERC20.approve.selector), abi.encode(true));

    //     // Mock for isPoolExists should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
    //         abi.encode(true)
    //     );

    //     // Mock for isPoolExists should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
    //         abi.encode(true)
    //     );
    //     // Mock for getPoolSlot0 should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
    //         abi.encode(0, 0, 0, 0, 0, 0, true)
    //     );
    //     // Mock for exactInputSingle should return 0.
    //     vm.mockCall(
    //         address(uniExtension.router()),
    //         abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
    //         abi.encode(0)
    //     );

    //     (bool success, , , ) = core.handleEmailOp(emailOp);
    //     vm.stopPrank();

    //     assertTrue(success, "emailOp failed");
    // }

    // function test_UniExtension_SubjectWithSwapUSDCToETH() public {
    //     // Mint 10 ETH and convert to WETH
    //     // Mint 200 USDC
    //     vm.startPrank(walletAddr);
    //     deal(address(walletAddr), 10 ether);
    //     weth.deposit{value: 10 ether}();
    //     deal(address(usdcToken), walletAddr, 20 * 10000 ether);
    //     vm.stopPrank();

    //     // Build email op
    //     EmailOp memory emailOp = _getBaseEmailOp();
    //     emailOp.command = "Swap";
    //     emailOp.maskedSubject = "Swap 200 USDC to ETH";
    //     emailOp.extensionName = "Uniswap";
    //     emailOp.extensionParams.subjectTemplateIndex = 0;
    //     emailOp.hasEmailRecipient = false;
    //     emailOp.extensionParams.subjectParams = new bytes[](2);
    //     emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(200 ether), "USDC");
    //     emailOp.extensionParams.subjectParams[1] = abi.encode("ETH");

    //     vm.startPrank(relayer);

    //     // Mock for usdcToken approval should return true.
    //     vm.mockCall(address(usdcToken), abi.encodeWithSelector(TestERC20.approve.selector), abi.encode(true));

    //     // Mock for isPoolExists should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
    //         abi.encode(true)
    //     );

    //     // Mock for isPoolExists should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
    //         abi.encode(true)
    //     );
    //     // Mock for getPoolSlot0 should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
    //         abi.encode(0, 0, 0, 0, 0, 0, true)
    //     );
    //     // Mock for exactInputSingle should return 0.
    //     vm.mockCall(
    //         address(uniExtension.router()),
    //         abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
    //         abi.encode(0)
    //     );

    //     (bool success, , , ) = core.handleEmailOp(emailOp);
    //     vm.stopPrank();

    //     assertTrue(success, "emailOp failed");
    // }

    // function test_UniExtension_SubjectWithSwapDAIToETH() public {
    //     // Mint 10 ETH and convert to WETH
    //     // Mint 200 DAI
    //     vm.startPrank(walletAddr);
    //     deal(address(walletAddr), 10 ether);
    //     weth.deposit{value: 10 ether}();
    //     deal(address(daiToken), walletAddr, 20 * 10000 ether);
    //     vm.stopPrank();

    //     // Build email op
    //     EmailOp memory emailOp = _getBaseEmailOp();
    //     emailOp.command = "Swap";
    //     emailOp.maskedSubject = "Swap 200 DAI to ETH";
    //     emailOp.extensionName = "Uniswap";
    //     emailOp.extensionParams.subjectTemplateIndex = 0;
    //     emailOp.hasEmailRecipient = false;
    //     emailOp.extensionParams.subjectParams = new bytes[](2);
    //     emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(200 ether), "DAI");
    //     emailOp.extensionParams.subjectParams[1] = abi.encode("ETH");

    //     vm.startPrank(relayer);

    //     // Mock for daiToken approval should return true.
    //     vm.mockCall(address(daiToken), abi.encodeWithSelector(TestERC20.approve.selector), abi.encode(true));

    //     // Mock for isPoolExists should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
    //         abi.encode(true)
    //     );

    //     // Mock for isPoolExists should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
    //         abi.encode(true)
    //     );
    //     // Mock for getPoolSlot0 should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
    //         abi.encode(0, 0, 0, 0, 0, 0, true)
    //     );
    //     // Mock for exactInputSingle should return 0.
    //     vm.mockCall(
    //         address(uniExtension.router()),
    //         abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
    //         abi.encode(0)
    //     );

    //     (bool success, , , ) = core.handleEmailOp(emailOp);
    //     vm.stopPrank();

    //     assertTrue(success, "emailOp failed");
    // }

    // function test_UniExtension_SubjectWithSwapETHToDAIWithSlippage() public {
    //     // Mint 10 ETH and convert to WETH
    //     vm.startPrank(walletAddr);
    //     deal(address(walletAddr), 10 ether);
    //     weth.deposit{value: 10 ether}();
    //     vm.stopPrank();

    //     // Build email op
    //     EmailOp memory emailOp = _getBaseEmailOp();
    //     emailOp.command = "Swap";
    //     emailOp.maskedSubject = "Swap 0.2 ETH to DAI with 0.5 slippage";
    //     emailOp.extensionName = "Uniswap";
    //     emailOp.extensionParams.subjectTemplateIndex = 1;
    //     emailOp.hasEmailRecipient = false;
    //     emailOp.extensionParams.subjectParams = new bytes[](3);
    //     emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(0.2 ether), "ETH");
    //     emailOp.extensionParams.subjectParams[1] = abi.encode("DAI");
    //     emailOp.extensionParams.subjectParams[2] = abi.encode(uint256(0.5 ether));

    //     vm.startPrank(relayer);

    //     // Mock for isPoolExists should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
    //         abi.encode(true)
    //     );
    //     // Mock for getPoolSlot0 should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
    //         abi.encode(0, 0, 0, 0, 0, 0, true)
    //     );
    //     // Mock for exactInputSingle should return 0.
    //     vm.mockCall(
    //         address(uniExtension.router()),
    //         abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
    //         abi.encode(0)
    //     );

    //     (bool success, , , ) = core.handleEmailOp(emailOp);
    //     vm.stopPrank();

    //     assertTrue(success, "emailOp failed");
    // }

    // function test_UniExtension_SubjectWithSwapETHToDAIWithSqrtPriceLimit() public {
    //     // Mint 10 ETH and convert to WETH
    //     vm.startPrank(walletAddr);
    //     deal(address(walletAddr), 10 ether);
    //     weth.deposit{value: 10 ether}();
    //     vm.stopPrank();

    //     // Build email op
    //     EmailOp memory emailOp = _getBaseEmailOp();
    //     emailOp.command = "Swap";
    //     emailOp.maskedSubject = "Swap 0.2 ETH to DAI under 1000000 sqrt price limit";
    //     emailOp.extensionName = "Uniswap";
    //     emailOp.extensionParams.subjectTemplateIndex = 2;
    //     emailOp.hasEmailRecipient = false;
    //     emailOp.extensionParams.subjectParams = new bytes[](3);
    //     emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(0.2 ether), "ETH");
    //     emailOp.extensionParams.subjectParams[1] = abi.encode("DAI");
    //     emailOp.extensionParams.subjectParams[2] = abi.encode(uint256(1000000));

    //     vm.startPrank(relayer);

    //     // Mock for isPoolExists should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
    //         abi.encode(true)
    //     );
    //     // Mock for getPoolSlot0 should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
    //         abi.encode(0, 0, 0, 0, 0, 0, true)
    //     );
    //     // Mock for exactInputSingle should return 0.
    //     vm.mockCall(
    //         address(uniExtension.router()),
    //         abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
    //         abi.encode(0)
    //     );

    //     (bool success, , , ) = core.handleEmailOp(emailOp);
    //     vm.stopPrank();

    //     assertTrue(success, "emailOp failed");
    // }

    // function test_UniExtension_SubjectWithSwapETHToDAIWithSlippageAndSqrtPriceLimit() public {
    //     // Mint 10 ETH and convert to WETH
    //     vm.startPrank(walletAddr);
    //     deal(address(walletAddr), 10 ether);
    //     weth.deposit{value: 10 ether}();
    //     vm.stopPrank();

    //     // Build email op
    //     EmailOp memory emailOp = _getBaseEmailOp();
    //     emailOp.command = "Swap";
    //     emailOp.maskedSubject = "Swap 0.2 ETH to DAI with 0.5 slippage under 1000000 sqrt price limit";
    //     emailOp.extensionName = "Uniswap";
    //     emailOp.extensionParams.subjectTemplateIndex = 3;
    //     emailOp.hasEmailRecipient = false;
    //     emailOp.extensionParams.subjectParams = new bytes[](4);
    //     emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(0.2 ether), "ETH");
    //     emailOp.extensionParams.subjectParams[1] = abi.encode("DAI");
    //     emailOp.extensionParams.subjectParams[2] = abi.encode(uint256(0.5 ether));
    //     emailOp.extensionParams.subjectParams[3] = abi.encode(uint256(1000000));

    //     vm.startPrank(relayer);

    //     // Mock for isPoolExists should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
    //         abi.encode(true)
    //     );
    //     // Mock for getPoolSlot0 should return slot entity.
    //     vm.mockCall(
    //         address(uniExtension.poolFinder()),
    //         abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
    //         abi.encode(0, 0, 0, 0, 0, 0, true)
    //     );
    //     // Mock for exactInputSingle should return 0.
    //     vm.mockCall(
    //         address(uniExtension.router()),
    //         abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
    //         abi.encode(0)
    //     );

    //     (bool success, , , ) = core.handleEmailOp(emailOp);
    //     vm.stopPrank();

    //     assertTrue(success, "emailOp failed");
    // }

    // function testUpgradeability() public {
    //     UniswapExtension implV2 = new UniswapExtension();

    //     uniExtension.upgradeTo(address(implV2));
    // }
}
