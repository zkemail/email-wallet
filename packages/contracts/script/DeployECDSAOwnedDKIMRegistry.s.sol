// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/utils/ECDSAOwnedDKIMRegistry.sol";

contract Deploy is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        address signer = vm.envAddress("SIGNER");
        if (signer == address(0)) {
            console.log("SIGNER env var not set");
            return;
        }

        // uint signValidityDuration = 1 hours;

        vm.startBroadcast(deployerPrivateKey);
        ECDSAOwnedDKIMRegistry dkim = new ECDSAOwnedDKIMRegistry(signer);
        vm.stopBroadcast();

        console.log("ECDSAOwnedDKIMRegistry deployed at: %s", address(dkim));
    }
}
