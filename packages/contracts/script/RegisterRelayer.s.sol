// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {RelayerHandler} from "../src/handlers/RelayerHandler.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";

contract RegisterRelayer is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        address relayerHandlerAddress = vm.envAddress("RELAYER_HANDLER");
        if (relayerHandlerAddress == address(0)) {
            console.log("RELAYER_HANDLER env var not set. Set the address of the deployed RelayerHandler contract");
            return;
        }

        string memory emailAddr = vm.envString("RELAYER_EMAIL");
        if (bytes(emailAddr).length == 0) {
            console.log("RELAYER_EMAIL env var not set. Set the email address for the relayer");
            return;
        }

        string memory hostname = vm.envString("RELAYER_HOSTNAME");
        if (bytes(hostname).length == 0) {
            console.log("RELAYER_HOSTNAME env var not set. Set the hostname for the relayer server");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);

        RelayerHandler relayerHandler = RelayerHandler(relayerHandlerAddress);

        // Register the relayer
        relayerHandler.registerRelayer(emailAddr, hostname);

        console.log("Relayer registered successfully!");
        console.log("Email Address: %s", emailAddr);
        console.log("Hostname: %s", hostname);

        vm.stopBroadcast();
    }
}
