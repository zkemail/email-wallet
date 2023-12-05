// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "../src/utils/TokenRegistry.sol";
import "./helpers/TokenRegistryV2.sol";
import "forge-std/console.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract TokenRegistryTest is Test {
    TokenRegistry tokenRegistry;
    ERC1967Proxy proxy;
    TokenRegistryV2 tokenRegistryV2;

    function setUp() public {
        address deployer = vm.addr(1);
        vm.startPrank(deployer);

        TokenRegistry tokenRegistryImpl = new TokenRegistry();
        proxy = new ERC1967Proxy(address(tokenRegistryImpl), abi.encodeCall(tokenRegistryImpl.initialize, ()));
        tokenRegistry = TokenRegistry(address(proxy));
    }

    function testGetUSDCTokenAddress() public {
        address usdc = tokenRegistry.getTokenAddress(0, "USDC");
        assertEq(usdc, 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48);
    }

    function testUpgradeability() public {
        TokenRegistryV2 tokenRegistryV2Impl = new TokenRegistryV2();
        tokenRegistry.upgradeTo(address(tokenRegistryV2Impl));
        
        tokenRegistryV2 = TokenRegistryV2(address(proxy));
        address usdc = tokenRegistry.getTokenAddress(0, "USDC");
        assertEq(usdc, address(0));
    }
}


