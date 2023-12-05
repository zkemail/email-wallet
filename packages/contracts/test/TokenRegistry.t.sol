// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "../src/MyToken.sol";
import "../src/utils/TokenRegistry.sol";
import "./helpers/TokenRegistryV2.sol";
import "forge-std/console.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { Upgrades } from "openzeppelin-foundry-upgrades/Upgrades.sol";

contract TokenRegistry is Test {
    TokenRegistry tokenRegistry;
    ERC1967Proxy proxy;

    function setUp() public {
        deployer = vm.addr(1);
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
        Upgrades.upgradeProxy(address(proxy), "TokenRegistryV2.sol:TokenRegistryV2", "");
    }
}


