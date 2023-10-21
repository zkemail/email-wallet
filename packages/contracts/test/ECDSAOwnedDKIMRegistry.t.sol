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
        registry = new ECDSAOwnedDKIMRegistry(signer,signValidityDuration);
    }

    function test_SetDKIMPublicKeyHash() public {
        vm.chainId(1);
        uint timestamp = block.timestamp;
        string memory signedMsg = registry.computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(
            selector,
            domainName,
            timestamp,
            publicKeyHash,
            signature
        );
        require(registry.getDKIMPublicKeyHash(domainName) == publicKeyHash, "Invalid public key hash");
    }

    function test_SetDKIMPublicKeyHashMultiDomain() public {
        vm.chainId(1);
        uint timestamp = block.timestamp;
        string memory signedMsg = registry.computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(
            selector,
            domainName,
            timestamp,
            publicKeyHash,
            signature
        );
        require(registry.getDKIMPublicKeyHash(domainName) == publicKeyHash, "Invalid public key hash");

        selector = "67890";
        domainName = "example2.com";
        timestamp = block.timestamp;
        publicKeyHash = bytes32(uint256(2));
        signedMsg = registry.computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        digest = bytes(signedMsg).toEthSignedMessageHash();
        (v, r, s) = vm.sign(1, digest);
        signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(
            selector,
            domainName,
            timestamp,
            publicKeyHash,
            signature
        );
        require(registry.getDKIMPublicKeyHash(domainName) == publicKeyHash, "Invalid public key hash");
    }

    // function test_RevertIfNonceInvalid() public {
    //     vm.chainId(1);
    //     string memory signedMsg = string.concat(
    //         "chain_id=",
    //         block.chainid.toString(),
    //         ";selector=",
    //         selector,
    //         ";domain=",
    //         domainName,
    //         ";nonce=",
    //         1.toHexString(32),
    //         ";public_key_hash=",
    //         uint256(publicKeyHash).toHexString(),
    //         ";"
    //     );
    //     bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
    //     (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
    //     bytes memory signature = abi.encodePacked(r, s, v);
    //     require(registry.nonceOfDomain(domainName) == 0, "Invalid nonce");
    //     vm.expectRevert("Invalid signature");
    //     registry.setDKIMPublicKeyHash(
    //         selector,
    //         domainName,
    //         publicKeyHash,
    //         signature
    //     );
    //     require(registry.getDKIMPublicKeyHash(domainName) == bytes32(0), "Invalid public key hash");
    //     require(registry.nonceOfDomain(domainName) == 0, "Invalid nonce");

    // }

    function test_RevertIfSignatureInvalid() public {
        vm.chainId(1);
        uint timestamp = block.timestamp;
        string memory signedMsg = registry.computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(2, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        vm.expectRevert("Invalid signature");
        registry.setDKIMPublicKeyHash(
            selector,
            domainName,
            timestamp,
            publicKeyHash,
            signature
        );
        require(registry.getDKIMPublicKeyHash(domainName) == bytes32(0), "Invalid public key hash");
    }

    function test_RevertIfChainIdInvalid() public {
        vm.chainId(10);
        uint timestamp = block.timestamp;
        string memory signedMsg = registry.computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(2, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        vm.expectRevert("Invalid signature");
        registry.setDKIMPublicKeyHash(
            selector,
            domainName,
            timestamp,
            publicKeyHash,
            signature
        );
        require(registry.getDKIMPublicKeyHash(domainName) == bytes32(0), "Invalid public key hash");
    }
}
