// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";
import "../src/extensions/UniswapExtension.sol";
import "../src/extensions/PoolFinder.sol";
import "./mocks/TestExtension.sol";
import "./mocks/DummyNFT.sol";
import "./mocks/TestERC20.sol";
import "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";

// Testing extension functionality for UniswapExtension
contract UniswapExtensionCommandTest is EmailWalletCoreTestHelper {
    TestExtension mockExtension;
    UniswapExtension uniExtension;
    string[][] public uniExtTemplates = new string[][](4);
    string[][] public mockExtTemplates = new string[][](10);

    fallback() external {
        // For one test below to call this contract with empty calldata
    }

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _createTestAccount();

        // Publish and install extension
        {
            UniswapExtension uniExtensionImpl = new UniswapExtension();
            ERC1967Proxy proxy = new ERC1967Proxy(address(uniExtensionImpl), abi.encodeCall(uniExtensionImpl.initialize, (
                address(core), 
                address(tokenRegistry), 
                address(0), 
                address(0)
            )));
            uniExtension = UniswapExtension(payable(address(proxy)));

            uniExtTemplates[0] = ["Swap", "{tokenAmount}", "to", "{string}"];
            uniExtTemplates[1] = ["Swap", "{tokenAmount}", "to", "{string}", "with", "{amount}", "slippage"];
            uniExtTemplates[2] = ["Swap", "{tokenAmount}", "to", "{string}", "under", "{uint}", "sqrt", "price", "limit"];
            uniExtTemplates[3] = [
                "Swap",
                "{tokenAmount}",
                "to",
                "{string}",
                "with",
                "{amount}",
                "slippage",
                "under",
                "{uint}",
                "sqrt",
                "price",
                "limit"
            ];
            extensionHandler.publishExtension("Uniswap", address(uniExtension), uniExtTemplates, 0.1 ether);
        }

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.extensionName = "Uniswap";
        emailOp.maskedSubject = "Install extension Uniswap";
        emailOp.emailNullifier = bytes32(uint256(93845));

        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "failed to register uniswap extension");
    }

    function test_UniExtension_SubjectWithSwapETHToDAI() public {
        // Mint 10 ETH and convert to WETH
        vm.startPrank(walletAddr);
        deal(address(walletAddr), 10 ether);
        weth.deposit{value: 10 ether}();
        vm.stopPrank();

        // Build email op
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Swap";
        emailOp.maskedSubject = "Swap 0.2 ETH to DAI";
        emailOp.extensionName = "Uniswap";
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(0.2 ether), "ETH");
        emailOp.extensionParams.subjectParams[1] = abi.encode("DAI");

        vm.startPrank(relayer);

        // Mock for isPoolExists should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
            abi.encode(true)
        );
        // Mock for getPoolSlot0 should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
            abi.encode(0, 0, 0, 0, 0, 0, true)
        );
        // Mock for exactInputSingle should return 0.
        vm.mockCall(
            address(uniExtension.router()),
            abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
            abi.encode(0)
        );

        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "emailOp failed");
    }

    function test_UniExtension_SubjectWithSwapDAIToUSDC() public {
        // Mint 10 ETH and convert to WETH
        // Mint 200 DAI
        vm.startPrank(walletAddr);
        deal(address(walletAddr), 10 ether);
        weth.deposit{value: 10 ether}();
        deal(address(daiToken), walletAddr, 20 * 10000 ether);
        vm.stopPrank();

        // Build email op
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Swap";
        emailOp.maskedSubject = "Swap 200 DAI to USDC";
        emailOp.extensionName = "Uniswap";
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(200 ether), "DAI");
        emailOp.extensionParams.subjectParams[1] = abi.encode("USDC");

        vm.startPrank(relayer);

        // Mock for daiToken approval should return true.
        vm.mockCall(address(daiToken), abi.encodeWithSelector(TestERC20.approve.selector), abi.encode(true));

        // Mock for isPoolExists should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
            abi.encode(true)
        );

        // Mock for isPoolExists should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
            abi.encode(true)
        );
        // Mock for getPoolSlot0 should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
            abi.encode(0, 0, 0, 0, 0, 0, true)
        );
        // Mock for exactInputSingle should return 0.
        vm.mockCall(
            address(uniExtension.router()),
            abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
            abi.encode(0)
        );

        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "emailOp failed");
    }

    function test_UniExtension_SubjectWithSwapUSDCToETH() public {
        // Mint 10 ETH and convert to WETH
        // Mint 200 USDC
        vm.startPrank(walletAddr);
        deal(address(walletAddr), 10 ether);
        weth.deposit{value: 10 ether}();
        deal(address(usdcToken), walletAddr, 20 * 10000 ether);
        vm.stopPrank();

        // Build email op
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Swap";
        emailOp.maskedSubject = "Swap 200 USDC to ETH";
        emailOp.extensionName = "Uniswap";
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(200 ether), "USDC");
        emailOp.extensionParams.subjectParams[1] = abi.encode("ETH");

        vm.startPrank(relayer);

        // Mock for usdcToken approval should return true.
        vm.mockCall(address(usdcToken), abi.encodeWithSelector(TestERC20.approve.selector), abi.encode(true));

        // Mock for isPoolExists should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
            abi.encode(true)
        );

        // Mock for isPoolExists should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
            abi.encode(true)
        );
        // Mock for getPoolSlot0 should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
            abi.encode(0, 0, 0, 0, 0, 0, true)
        );
        // Mock for exactInputSingle should return 0.
        vm.mockCall(
            address(uniExtension.router()),
            abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
            abi.encode(0)
        );

        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "emailOp failed");
    }

    function test_UniExtension_SubjectWithSwapDAIToETH() public {
        // Mint 10 ETH and convert to WETH
        // Mint 200 DAI
        vm.startPrank(walletAddr);
        deal(address(walletAddr), 10 ether);
        weth.deposit{value: 10 ether}();
        deal(address(daiToken), walletAddr, 20 * 10000 ether);
        vm.stopPrank();

        // Build email op
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Swap";
        emailOp.maskedSubject = "Swap 200 DAI to ETH";
        emailOp.extensionName = "Uniswap";
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(200 ether), "DAI");
        emailOp.extensionParams.subjectParams[1] = abi.encode("ETH");

        vm.startPrank(relayer);

        // Mock for daiToken approval should return true.
        vm.mockCall(address(daiToken), abi.encodeWithSelector(TestERC20.approve.selector), abi.encode(true));

        // Mock for isPoolExists should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
            abi.encode(true)
        );

        // Mock for isPoolExists should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
            abi.encode(true)
        );
        // Mock for getPoolSlot0 should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
            abi.encode(0, 0, 0, 0, 0, 0, true)
        );
        // Mock for exactInputSingle should return 0.
        vm.mockCall(
            address(uniExtension.router()),
            abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
            abi.encode(0)
        );

        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "emailOp failed");
    }

    function test_UniExtension_SubjectWithSwapETHToDAIWithSlippage() public {
        // Mint 10 ETH and convert to WETH
        vm.startPrank(walletAddr);
        deal(address(walletAddr), 10 ether);
        weth.deposit{value: 10 ether}();
        vm.stopPrank();

        // Build email op
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Swap";
        emailOp.maskedSubject = "Swap 0.2 ETH to DAI with 0.5 slippage";
        emailOp.extensionName = "Uniswap";
        emailOp.extensionParams.subjectTemplateIndex = 1;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](3);
        emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(0.2 ether), "ETH");
        emailOp.extensionParams.subjectParams[1] = abi.encode("DAI");
        emailOp.extensionParams.subjectParams[2] = abi.encode(uint256(0.5 ether));

        vm.startPrank(relayer);

        // Mock for isPoolExists should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
            abi.encode(true)
        );
        // Mock for getPoolSlot0 should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
            abi.encode(0, 0, 0, 0, 0, 0, true)
        );
        // Mock for exactInputSingle should return 0.
        vm.mockCall(
            address(uniExtension.router()),
            abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
            abi.encode(0)
        );

        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "emailOp failed");
    }

    function test_UniExtension_SubjectWithSwapETHToDAIWithSqrtPriceLimit() public {
        // Mint 10 ETH and convert to WETH
        vm.startPrank(walletAddr);
        deal(address(walletAddr), 10 ether);
        weth.deposit{value: 10 ether}();
        vm.stopPrank();

        // Build email op
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Swap";
        emailOp.maskedSubject = "Swap 0.2 ETH to DAI under 1000000 sqrt price limit";
        emailOp.extensionName = "Uniswap";
        emailOp.extensionParams.subjectTemplateIndex = 2;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](3);
        emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(0.2 ether), "ETH");
        emailOp.extensionParams.subjectParams[1] = abi.encode("DAI");
        emailOp.extensionParams.subjectParams[2] = abi.encode(uint256(1000000));

        vm.startPrank(relayer);

        // Mock for isPoolExists should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
            abi.encode(true)
        );
        // Mock for getPoolSlot0 should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
            abi.encode(0, 0, 0, 0, 0, 0, true)
        );
        // Mock for exactInputSingle should return 0.
        vm.mockCall(
            address(uniExtension.router()),
            abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
            abi.encode(0)
        );

        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "emailOp failed");
    }

    function test_UniExtension_SubjectWithSwapETHToDAIWithSlippageAndSqrtPriceLimit() public {
        // Mint 10 ETH and convert to WETH
        vm.startPrank(walletAddr);
        deal(address(walletAddr), 10 ether);
        weth.deposit{value: 10 ether}();
        vm.stopPrank();

        // Build email op
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Swap";
        emailOp.maskedSubject = "Swap 0.2 ETH to DAI with 0.5 slippage under 1000000 sqrt price limit";
        emailOp.extensionName = "Uniswap";
        emailOp.extensionParams.subjectTemplateIndex = 3;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](4);
        emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(0.2 ether), "ETH");
        emailOp.extensionParams.subjectParams[1] = abi.encode("DAI");
        emailOp.extensionParams.subjectParams[2] = abi.encode(uint256(0.5 ether));
        emailOp.extensionParams.subjectParams[3] = abi.encode(uint256(1000000));

        vm.startPrank(relayer);

        // Mock for isPoolExists should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.isPoolExists.selector),
            abi.encode(true)
        );
        // Mock for getPoolSlot0 should return slot entity.
        vm.mockCall(
            address(uniExtension.poolFinder()),
            abi.encodeWithSelector(PoolFinder.getPoolSlot0.selector),
            abi.encode(0, 0, 0, 0, 0, 0, true)
        );
        // Mock for exactInputSingle should return 0.
        vm.mockCall(
            address(uniExtension.router()),
            abi.encodeWithSelector(ISwapRouter.exactInputSingle.selector),
            abi.encode(0)
        );

        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "emailOp failed");
    }

    function testUpgradeability() public {
        UniswapExtension implV2 = new UniswapExtension();

        uniExtension.upgradeTo(address(implV2));
    }
}
