// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "../src/utils/ECDSAOwnedDKIMRegistry.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

contract ECDSAOwnedDKIMRegistryTest is Test {
    ECDSAOwnedDKIMRegistry registry;
    using console for *;
    using ECDSA for *;
    using Strings for *;

    string public selector = "12345";
    string public domainName = "example.com";
    uint public signValidityDuration = 1 days;
    bytes32 public publicKeyHash = bytes32(uint256(1));

    function setUp() public {
        address signer = vm.addr(1);
        registry = new ECDSAOwnedDKIMRegistry(signer, signValidityDuration);
    }

    function test_SetDKIMPublicKeyHash() public {
        vm.chainId(1);
        uint timestamp = block.timestamp;
        string memory signedMsg = registry.computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(selector, domainName, timestamp, publicKeyHash, signature);
        require(registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash), "Invalid public key hash");
    }

    function test_SetDKIMPublicKeyHashMultiDomain() public {
        vm.chainId(1);
        uint timestamp = block.timestamp;
        string memory signedMsg = registry.computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(selector, domainName, timestamp, publicKeyHash, signature);
        require(registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash), "Invalid public key hash");

        selector = "67890";
        domainName = "example2.com";
        timestamp = block.timestamp;
        publicKeyHash = bytes32(uint256(2));
        signedMsg = registry.computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        digest = bytes(signedMsg).toEthSignedMessageHash();
        (v, r, s) = vm.sign(1, digest);
        signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(selector, domainName, timestamp, publicKeyHash, signature);
        require(registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash), "Invalid public key hash");
    }

    function test_RevertIfExpired() public {
        vm.chainId(1);
        uint timestamp = block.timestamp;
        string memory signedMsg = registry.computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        vm.warp(timestamp + signValidityDuration + 1);
        vm.expectRevert("Signature expired");
        registry.setDKIMPublicKeyHash(selector, domainName, timestamp, publicKeyHash, signature);
    }

    function test_RevertIfSignatureInvalid() public {
        vm.chainId(1);
        uint timestamp = block.timestamp;
        string memory signedMsg = registry.computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(2, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        vm.expectRevert("Invalid signature");
        registry.setDKIMPublicKeyHash(selector, domainName, timestamp, publicKeyHash, signature);
    }

    function test_RevertIfChainIdInvalid() public {
        vm.chainId(10);
        uint timestamp = block.timestamp;
        string memory signedMsg = registry.computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(2, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        vm.expectRevert("Invalid signature");
        registry.setDKIMPublicKeyHash(selector, domainName, timestamp, publicKeyHash, signature);
    }

    function test_Dfinity_Oracle_Response() public {
        vm.chainId(1);
        registry = new ECDSAOwnedDKIMRegistry(0x2F6e79a6E1a982a49CA248B70b02F76e921aF400, signValidityDuration);
        selector = "20230601";
        domainName = "gmail.com";
        publicKeyHash = 0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788;
        uint timestamp = 1697948713;
        vm.warp(timestamp + signValidityDuration);
        registry.setDKIMPublicKeyHash(
            selector,
            domainName,
            timestamp,
            publicKeyHash,
            vm.parseBytes(
                "0x875fae3da3e58a97971663934b3ddafd4057706ddb7281de07d25d51e3587c3b179c4fcc45b6710bacde082933d22e69076f4b49da02273ee30a3cc5d04febe81c"
            )
        );
        require(registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash), "Invalid public key hash");
    }
}
