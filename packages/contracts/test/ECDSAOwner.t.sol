// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "../src/dkimRegistryOwners/ECDSAOwner.sol";

contract ECDSAOwnerTest is Test {
    ECDSAOwner owner;

    function setUp() public {
        owner = new ECDSAOwner(0x1882DDEDf1d0aCc9DA2CD67D4e5FA30f0CCFcD8b);
    }

    function testSetDKIMPublicKeyHash() public {
        vm.chainId(1);
        owner.setDKIMPublicKeyHash(
            "20230601",
            "gmail.com",
            0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
            vm.parseBytes(
                "0x5c86998a0f95a537a4127a76dd2c874d7ed46717eb7401f8e90ea04ab1ccefac27b1378ca430cf06aa1cec5f9b9467f75b232e84ba9c5782f0d1f8d7ac8472f11b"
            )
        );
        require(
            owner.getDKIMPublicKeyHash("gmail.com") ==
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
            "Invalid public key hash"
        );
    }
}
