// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";
import "../src/extensions/UniswapExtension.sol";
import "./mocks/TestExtension.sol";
import "./mocks/DummyNFT.sol";
import "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";

// Testing extension functionality for UniswapExtension
contract UniswapExtensionCommandTest is EmailWalletCoreTestHelper {
    TestExtension mockExtension;
    UniswapExtension uniExtension;
    string[][] public uniExtTemplates = new string[][](1);
    string[][] public mockExtTemplates = new string[][](10);

    fallback() external {
        // For one test below to call this contract with empty calldata
    }

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _registerAndInitializeAccount();

        // Publish and install extension
        uniExtension = new UniswapExtension(
            address(core),
            address(tokenRegistry),
            address(0),
            address(0)
        );
        uniExtTemplates[0] = ["Swap", "{tokenAmount}", "to", "{string}"];
        extensionHandler.publishExtension("Uniswap", address(uniExtension), uniExtTemplates, 0.1 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.extensionName = "Uniswap";
        emailOp.maskedSubject = "Install extension Uniswap";
        emailOp.emailNullifier = bytes32(uint256(93845));

        vm.startPrank(relayer);
        (bool success, , ,) = core.handleEmailOp(emailOp);
        vm.stopPrank();
        
        assertTrue(success, "failed to register uniswap extension");
    }

    function test_NumberIs42() public {
        uint256 testNumber = 42;
        assertEq(testNumber, 42);
    }

    function test_UniExtension_SubjectWithSwapETHToDAI() public {
        address userWallet = vm.addr(3);
        
        // Mint 10 ETH to user wallet and convert to WETH
        vm.startPrank(userWallet);
        deal(address(userWallet), 10 ether);
        weth.deposit{value: 10 ether}();
        // TODO does it needed?
        weth.approve(address(uniExtension), 10 ether);
        vm.stopPrank();

        // Mint 10 ETH to uniExtension? TODO it's not needed I guess.
        vm.startPrank(address(uniExtension));
        deal(address(uniExtension), 10 ether);
        weth.deposit{value: 10 ether}();
        vm.stopPrank();

        // Build email op
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Swap";
        emailOp.maskedSubject = "Swap 0.2 ETH to DAI";
        emailOp.extensionName = "Uniswap";        
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = false;
        emailOp.recipientETHAddr = userWallet;
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(0.2 ether), "ETH");
        emailOp.extensionParams.subjectParams[1] = abi.encode("DAI");

        vm.startPrank(relayer);
        vm.mockCall(
            address(0),
            abi.encodeWithSelector(uniExtension.execute.selector),
            abi.encode(true)
        );
        (bool success, , ,) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "emailOp failed");

/**
    │   ├─ [36341] UniswapExtension::execute(0, [0x00000000000000000000000000000000000000000000000002c68af0bb140000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000034554480000000000000000000000000000000000000000000000000000000000, 0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000034441490000000000000000000000000000000000000000000000000000000000], ERC1967Proxy: [0x5138D48987743911c3023dca7AA20E32Cd45E246], false, 0x6813Eb9362372EEF6200f3b1dbC3f819671cBA69, 0x0000000000000000000000000000000000000000000000000000000000d55637) 
    │   │   ├─ [2569] TokenRegistry::getTokenAddress(ETH) [staticcall]
    │   │   │   └─ ← WETH9: [0xB9816fC57977D5A786E654c7CF76767be63b966e]
    │   │   ├─ [4502] TokenRegistry::getTokenAddress(DAI) [staticcall]
    │   │   │   └─ ← TestERC20: [0xD30C8839c1145609E564b986F667b273Ddcb8496]
    │   │   ├─ [629] WETH9::balanceOf(UniswapExtension: [0xd04404bcf6d969FC0Ec22021b4736510CAcec492]) [staticcall]
    │   │   │   └─ ← 10000000000000000000 [1e19]
    │   │   ├─ [14245] EmailWalletCore::requestTokenAsExtension(WETH9: [0xB9816fC57977D5A786E654c7CF76767be63b966e], 200000000000000000 [2e17]) 
    │   │   │   ├─ [10961] ERC1967Proxy::execute(WETH9: [0xB9816fC57977D5A786E654c7CF76767be63b966e], 0, 0xa9059cbb000000000000000000000000d04404bcf6d969fc0ec22021b4736510cacec49200000000000000000000000000000000000000000000000002c68af0bb140000) 
    │   │   │   │   ├─ [6032] Wallet::execute(WETH9: [0xB9816fC57977D5A786E654c7CF76767be63b966e], 0, 0xa9059cbb000000000000000000000000d04404bcf6d969fc0ec22021b4736510cacec49200000000000000000000000000000000000000000000000002c68af0bb140000) [delegatecall]
    │   │   │   │   │   ├─ [2712] WETH9::transfer(UniswapExtension: [0xd04404bcf6d969FC0Ec22021b4736510CAcec492], 200000000000000000 [2e17]) 
    │   │   │   │   │   │   └─ ← "EvmError: Revert"
    │   │   │   │   │   └─ ← "EvmError: Revert"
    │   │   │   │   └─ ← "EvmError: Revert"
    │   │   │   └─ ← "request token failed: unknown wallet exec error"
    │   │   └─ ← "request token failed: unknown wallet exec error"
 */        
    }
}
