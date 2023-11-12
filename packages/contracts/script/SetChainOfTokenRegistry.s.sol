// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/utils/TokenRegistry.sol";

contract Deploy is Script {
    function run() external {
        uint256 ownerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (ownerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }
        address _tokenRegistry = vm.envAddress("TOKEN_REGISTRY");
        if (_tokenRegistry == address(0)) {
            console.log("TOKEN_REGISTRY env var not set");
            return;
        }
        string memory chainName = vm.envString("CHAIN_NAME");
        uint256 chainId = vm.envUint("CHAIN_ID");
        vm.startBroadcast(ownerPrivateKey);
        TokenRegistry tokenRegistry = TokenRegistry(_tokenRegistry);
        tokenRegistry.setChainId(chainName, chainId);
        vm.stopBroadcast();

        console.log("TokenRegistry deployed at: %s", address(tokenRegistry));
    }
}
