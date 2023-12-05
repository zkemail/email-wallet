// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/utils/TokenRegistry.sol";

contract Deploy is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);
        TokenRegistry tokenRegistryImpl = new TokenRegistry();
        bytes memory data = abi.encodeWithSelector(
            TokenRegistry(tokenRegistryImpl).initialize.selector
        );
        ERC1967Proxy proxy = new ERC1967Proxy(address(tokenRegistryImpl), data);
        TokenRegistry tokenRegistry = TokenRegistry(payable(address(proxy)));

        vm.stopBroadcast();

        console.log("TokenRegistry proxy deployed at: %s", address(tokenRegistry));
        console.log("TokenRegistry implementation deployed at: %s", address(tokenRegistryImpl));
        console.log("---- DONE ----");
    }
}
