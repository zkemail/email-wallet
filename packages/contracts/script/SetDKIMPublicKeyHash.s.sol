// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/utils/ECDSAOwnedDKIMRegistry.sol";

contract Update is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        address registryAddr = vm.envAddress("DKIM_REGISTRY");
        if (registryAddr == address(0)) {
            console.log("DKIM_REGISTRY env var not set");
            return;
        }
        ECDSAOwnedDKIMRegistry registry = ECDSAOwnedDKIMRegistry(registryAddr);
        string memory selector = vm.envString("SELECTOR");
        console.log("selector: %s", selector);
        string memory domainName = vm.envString("DOMAIN_NAME");
        console.log("domainName: %s", domainName);
        bytes32 publicKeyHash = vm.envBytes32("PUBLIC_KEY_HASH");
        console.logBytes32(publicKeyHash);
        bytes memory signature = vm.envBytes("SIGNATURE");
        console.logBytes(signature);
        vm.startBroadcast(deployerPrivateKey);
        registry.setDKIMPublicKeyHash(selector, domainName, publicKeyHash, signature);
        vm.stopBroadcast();
    }
}
