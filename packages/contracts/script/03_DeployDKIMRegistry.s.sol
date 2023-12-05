// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "@zk-email/contracts/DKIMRegistry.sol";

contract Deploy is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);
        DKIMRegistry dkim = new DKIMRegistry();
        vm.stopBroadcast();

        console.log("DKIMRegistry deployed at: %s", address(dkim));
        console.log("Please add this address into .env");
        console.log("---- DONE ----");
    }
}
