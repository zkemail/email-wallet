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

        address tokenRegistry = vm.envAddress("TOKEN_REGISTRY");
        if (tokenRegistry == address(0)) {
            console.log("TOKEN_REGISTRY env var not set. Deploy TokenRegistry and set env var");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);

        // FIXME Set new TokenRegistry implementation
        // e.g. TokenRegistryV2 tokenRegistryImpl = new TokenRegistryV2();
        TokenRegistry tokenRegistryImpl = new TokenRegistry();

        TokenRegistry tokenRegistryProxy = TokenRegistry(payable(address(tokenRegistry)));
        tokenRegistryProxy.upgradeTo(address(tokenRegistryImpl));

        // If you want to call some v2 function, refer to the following steps
        //
        // TokenRegistryV2 tokenRegistryV2 = TokenRegistryV2(address(tokenRegistry));
        // address usdc = tokenRegistry.getTokenAddress(0, "USDC");
                
        vm.stopBroadcast();

        console.log("TokenRegistry implementation deployed at: %s", address(tokenRegistryImpl));
        console.log("---- DONE ----");
    }
}
