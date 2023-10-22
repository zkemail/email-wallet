// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/utils/UniswapTWAPOracle.sol";

contract Deploy is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        address uniswapFactory = vm.envAddress("UNISWAP_FACTORY");
        if (uniswapFactory == address(0)) {
            console.log("UNISWAP_FACTORY env var not set. Deploy UniswapTWAPOracle and set env var");
            return;
        }

        address weth = vm.envAddress("WETH");
        if (weth == address(0)) {
            console.log("WETH env var not set.");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);
        UniswapTWAPOracle oracle = new UniswapTWAPOracle(uniswapFactory, weth);
        vm.stopBroadcast();

        console.log("UniswapTWAPOracle deployed at: %s", address(oracle));
    }
}
