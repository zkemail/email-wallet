// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/utils/OauthCore.sol";

contract DeployOauthCore is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);

        // Deploy implementation
        OauthCore oauthCoreImpl = new OauthCore();
        console.log("OauthCore implementation deployed at: %s", address(oauthCoreImpl));

        // Prepare initialization data
        bytes memory initData = abi.encodeWithSelector(OauthCore.initialize.selector);

        // Deploy proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(oauthCoreImpl), initData);
        console.log("OauthCore proxy deployed at: %s", address(proxy));

        // Get the proxy as OauthCore
        OauthCore oauthCore = OauthCore(payable(address(proxy)));

        vm.stopBroadcast();

        console.log("OauthCore deployed and initialized successfully");
        console.log("---- DONE ----");

        // Save the proxy address to be used in the upgrade script
        console.log("Set this in your .env file:");
        console.log("OAUTH=%s", address(proxy));
    }
}
