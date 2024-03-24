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
    // uint public signValidityDuration = 1 days;
    bytes32 public publicKeyHash = bytes32(uint256(1));

    function setUp() public {
        address signer = vm.addr(1);
        registry = new ECDSAOwnedDKIMRegistry(signer);
    }

    function test_SetDKIMPublicKeyHash() public {
        // vm.chainId(1);
        string memory signedMsg = registry.computeSignedMsg(registry.SET_PREFIX(), selector, domainName, publicKeyHash);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(selector, domainName, publicKeyHash, signature);
        require(registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash), "Invalid public key hash");
    }

    function test_SetDKIMPublicKeyHashMultiDomain() public {
        // vm.chainId(1);
        string memory signedMsg = registry.computeSignedMsg(registry.SET_PREFIX(), selector, domainName, publicKeyHash);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(selector, domainName, publicKeyHash, signature);
        require(registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash), "Invalid public key hash");

        selector = "67890";
        domainName = "example2.com";
        publicKeyHash = bytes32(uint256(2));
        signedMsg = registry.computeSignedMsg(registry.SET_PREFIX(), selector, domainName, publicKeyHash);
        digest = bytes(signedMsg).toEthSignedMessageHash();
        (v, r, s) = vm.sign(1, digest);
        signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(selector, domainName, publicKeyHash, signature);
        require(registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash), "Invalid public key hash");
    }

    function test_RevokeDKIMPublicKeyHash() public {
        // vm.chainId(1);
        string memory signedMsg = registry.computeSignedMsg(registry.SET_PREFIX(), selector, domainName, publicKeyHash);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(selector, domainName, publicKeyHash, signature);

        // Revoke
        string memory revokeMsg = registry.computeSignedMsg(
            registry.REVOKE_PREFIX(),
            selector,
            domainName,
            publicKeyHash
        );
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(1, bytes(revokeMsg).toEthSignedMessageHash());
        bytes memory revokeSig = abi.encodePacked(r1, s1, v1);
        registry.revokeDKIMPublicKeyHash(selector, domainName, publicKeyHash, revokeSig);

        require(!registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash));
    }

    function test_RevertIfDuplicated() public {
        // vm.chainId(1);
        string memory signedMsg = registry.computeSignedMsg(registry.SET_PREFIX(), selector, domainName, publicKeyHash);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(selector, domainName, publicKeyHash, signature);
        require(registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash), "Invalid public key hash");

        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(1, digest);
        bytes memory signature1 = abi.encodePacked(r1, s1, v1);
        vm.expectRevert("publicKeyHash is already set");
        registry.setDKIMPublicKeyHash(selector, domainName, publicKeyHash, signature1);
    }

    function test_RevertIfRevorked() public {
        // vm.chainId(1);
        string memory signedMsg = registry.computeSignedMsg(registry.SET_PREFIX(), selector, domainName, publicKeyHash);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        registry.setDKIMPublicKeyHash(selector, domainName, publicKeyHash, signature);
        require(registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash), "Invalid public key hash");

        // Revoke
        string memory revokeMsg = registry.computeSignedMsg(
            registry.REVOKE_PREFIX(),
            selector,
            domainName,
            publicKeyHash
        );
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(1, bytes(revokeMsg).toEthSignedMessageHash());
        bytes memory revokeSig = abi.encodePacked(r1, s1, v1);
        registry.revokeDKIMPublicKeyHash(selector, domainName, publicKeyHash, revokeSig);
        require(!registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash));

        signedMsg = registry.computeSignedMsg(registry.SET_PREFIX(), selector, domainName, publicKeyHash);
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(1, digest);
        bytes memory signature2 = abi.encodePacked(r2, s2, v2);
        vm.expectRevert("publicKeyHash is revoked");
        registry.setDKIMPublicKeyHash(selector, domainName, publicKeyHash, signature2);
    }

    function test_RevertIfSignatureInvalid() public {
        // vm.chainId(1);
        string memory signedMsg = registry.computeSignedMsg(registry.SET_PREFIX(), selector, domainName, publicKeyHash);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(2, digest);
        bytes memory signature = abi.encodePacked(r, s, v);
        vm.expectRevert("Invalid signature");
        registry.setDKIMPublicKeyHash(selector, domainName, publicKeyHash, signature);
    }

    // function test_RevertIfChainIdInvalid() public {
    //     vm.chainId(10);
    //     string memory signedMsg = registry.computeSignedMsg(registry.SET_PREFIX(), selector, domainName, publicKeyHash);
    //     bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
    //     (uint8 v, bytes32 r, bytes32 s) = vm.sign(2, digest);
    //     bytes memory signature = abi.encodePacked(r, s, v);
    //     vm.expectRevert("Invalid signature");
    //     registry.setDKIMPublicKeyHash(selector, domainName, publicKeyHash, signature);
    // }

    function test_Dfinity_Oracle_Response() public {
        // vm.chainId(1);
        registry = new ECDSAOwnedDKIMRegistry(0x69Bec2Dd161d6Bbcc91ec32AA44D9333EBc864c0);
        selector = "20230601";
        domainName = "gmail.com";
        publicKeyHash = 0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788;
        registry.setDKIMPublicKeyHash(
            selector,
            domainName,
            publicKeyHash,
            vm.parseBytes(
                "0xe5fb9c45bd6468877e8ec7e04063b03e8ac89206354060e757b15d6269f7754e6c515b5825fbb6be4e939f92d1ad62dc7f548607fe4349033ed51f8da8a18c4c1c"
            )
        );
        require(registry.isDKIMPublicKeyHashValid(domainName, publicKeyHash), "Invalid public key hash");
    }
}
