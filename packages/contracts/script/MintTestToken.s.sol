// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "./DeployTestToken.s.sol";
import "forge-std/Script.sol";

contract Mint is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        address testTokenAddr = vm.envAddress("TEST_TOKEN");
        if (testTokenAddr == address(0)) {
            console.log("TEST_TOKEN env var not set");
            return;
        }

        uint256 amount = vm.envUint("AMOUNT");
        if (amount == 0) {
            console.log("AMOUNT env var not set");
            return;
        }

        TestERC20 testToken = TestERC20(testTokenAddr);
        vm.startBroadcast(deployerPrivateKey);
        testToken.freeMint(amount);
        vm.stopBroadcast();
    }
}
