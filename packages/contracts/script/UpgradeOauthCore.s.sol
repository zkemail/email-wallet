// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/utils/OauthCore.sol";

contract Deploy is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        address oauthCore = vm.envAddress("OAUTH_CORE");
        if (oauthCore == address(0)) {
            console.log("OAUTH_CORE env var not set. Deploy OAUTH_CORE and set env var");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);

        OauthCore oauthCoreImpl = new OauthCore();

        OauthCore oauthCoreProxy = OauthCore(payable(address(oauthCore)));
        oauthCoreProxy.upgradeTo(address(oauthCoreImpl));

        // If you want to call some v2 function, refer to the following steps
        //
        // TokenRegistryV2 tokenRegistryV2 = TokenRegistryV2(address(tokenRegistry));
        // address usdc = tokenRegistry.getTokenAddress(0, "USDC");

        vm.stopBroadcast();

        console.log("OauthCore implementation deployed at: %s", address(oauthCoreImpl));
        console.log("---- DONE ----");
    }
}
