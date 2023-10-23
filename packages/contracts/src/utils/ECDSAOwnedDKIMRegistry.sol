// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@zk-email/contracts/DKIMRegistry.sol";

/// @title ECDSAOwnedDKIMRegistry
/// @notice A DKIM Registry that could be updated by predefined ECDSA signer
contract ECDSAOwnedDKIMRegistry is IDKIMRegistry {
    using Strings for *;
    using ECDSA for *;

    // mapping(string => uint256) public nonceOfDomain;
    uint signValidityDuration;
    DKIMRegistry public dkimRegistry;
    address public signer;

    constructor(address _signer, uint _signValidityDuration) {
        dkimRegistry = new DKIMRegistry();
        signer = _signer;
        signValidityDuration = _signValidityDuration;
    }

    function setDKIMPublicKeyHash(
        string memory selector,
        string memory domainName,
        uint timestamp,
        bytes32 publicKeyHash,
        bytes memory signature
    ) public {
        require(bytes(selector).length != 0, "Invalid selector");
        require(bytes(domainName).length != 0, "Invalid domain name");
        require(publicKeyHash != bytes32(0), "Invalid public key hash");
        require(block.timestamp - timestamp <= signValidityDuration, "Signature expired");
        string memory signedMsg = computeSignedMsg(selector, domainName, publicKeyHash, timestamp);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        address recoveredSigner = digest.recover(signature);
        require(recoveredSigner == signer, "Invalid signature");
        dkimRegistry.setDKIMPublicKeyHash(domainName, publicKeyHash);
    }

    function getDKIMPublicKeyHash(string memory domainName) public view returns (bytes32) {
        return bytes32(dkimRegistry.getDKIMPublicKeyHash(domainName));
    }

    function computeSignedMsg(
        string memory selector,
        string memory domainName,
        bytes32 publicKeyHash,
        uint256 timestamp
    ) public view returns (string memory) {
        return
            string.concat(
                "chain_id=",
                block.chainid.toString(),
                ";selector=",
                selector,
                ";domain=",
                domainName,
                ";timestamp=",
                timestamp.toString(),
                ";public_key_hash=",
                uint256(publicKeyHash).toHexString(),
                ";"
            );
    }
}
