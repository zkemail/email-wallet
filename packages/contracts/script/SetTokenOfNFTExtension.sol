// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/extensions/NFTExtension.sol";

contract Deploy is Script {
    function run() external {
        uint256 ownerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (ownerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }
        address _nftExtension = vm.envAddress("NFT_EXTENSION");
        if (_nftExtension == address(0)) {
            console.log("NFT_EXTENSION env var not set");
            return;
        }
        string memory tokenName = vm.envString("TOKEN_NAME");
        address tokenAddr = vm.envAddress("TOKEN_ADDR");
        vm.startBroadcast(ownerPrivateKey);
        NFTExtension nftExtension = NFTExtension(_nftExtension);
        nftExtension.setNFTAddress(tokenName, tokenAddr);
        vm.stopBroadcast();
    }
}
