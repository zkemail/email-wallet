// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/EmailWalletCore.sol";

contract Upgrade is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        address core = vm.envAddress("CORE");
        if (core == address(0)) {
            console.log("CORE env var not set. Deploy EmailWalletCore and set env var");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);

        // FIXME Set new TokenRegistry implementation
        // e.g. TokenRegistryV2 tokenRegistryImpl = new TokenRegistryV2();
        EmailWalletCore coreImpl = new EmailWalletCore();

        EmailWalletCore coreProxy = EmailWalletCore(payable(core));
        coreProxy.upgradeToAndCall(address(coreImpl), new bytes(0));

        // If you want to call some v2 function, refer to the following steps
        //
        // TokenRegistryV2 tokenRegistryV2 = TokenRegistryV2(address(tokenRegistry));
        // address usdc = tokenRegistry.getTokenAddress(0, "USDC");

        vm.stopBroadcast();

        console.log("Core implementation deployed at: %s", address(coreImpl));
        console.log("---- DONE ----");
    }
}
