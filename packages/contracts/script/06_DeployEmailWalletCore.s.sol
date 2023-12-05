// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "../src/EmailWalletCore.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";

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

        address priceOracle = vm.envAddress("PRICE_ORACLE");
        if (priceOracle == address(0)) {
            console.log("PRICE_ORACLE env var not set. Deploy UniswapTWAPOracle and set env var");
            return;
        }

        address weth = vm.envAddress("WETH");
        if (weth == address(0)) {
            console.log("WETH env var not set.");
            return;
        }

        address verifier = vm.envAddress("VERIFIER");
        if (verifier == address(0)) {
            console.log("VERIFIER env var not set.");
            return;
        }

        address relayerHandler = vm.envAddress("RELAYER_HANDLER");
        if (relayerHandler == address(0)) {
            console.log("RELAYER_HANDLER env var not set");
            return;
        }

        address extensionHandler = vm.envAddress("EXTENSION_HANDLER");
        if (extensionHandler == address(0)) {
            console.log("EXTENSION_HANDLER env var not set");
            return;
        }

        address accountHandler = vm.envAddress("ACCOUNT_HANDLER");
        if (accountHandler == address(0)) {
            console.log("ACCOUNT_HANDLER env var not set");
            return;
        }

        address unclaimsHandler = vm.envAddress("UNCLAIMS_HANDLER");
        if (unclaimsHandler == address(0)) {
            console.log("UNCLAIMS_HANDLER env var not set");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);

        EmailWalletCore coreImpl = new EmailWalletCore();
        bytes memory data = abi.encodeWithSelector(
            EmailWalletCore(coreImpl).initialize.selector,
                address(relayerHandler),
                address(accountHandler),
                address(unclaimsHandler),
                address(extensionHandler),
                address(verifier),
                address(tokenRegistry),
                address(priceOracle),
                address(weth),
                2 gwei,
                1 hours,
                450000,
                500000
        );            

        ERC1967Proxy proxy = new ERC1967Proxy(address(coreImpl), data);
        EmailWalletCore core = EmailWalletCore(payable(address(proxy)));

        console.log("EmailWalletCore proxy deployed at: %s", address(core));
        console.log("EmailWalletCore implementation deployed at: %s", address(coreImpl));    

        core.relayerHandler().transferOwnership(address(core));
        core.accountHandler().transferOwnership(address(core));
        core.unclaimsHandler().transferOwnership(address(core));
        core.extensionHandler().transferOwnership(address(core));

        console.log("Please add these addresses into .env");
        console.log("---- DONE ----");

        vm.stopBroadcast();
    }
}
