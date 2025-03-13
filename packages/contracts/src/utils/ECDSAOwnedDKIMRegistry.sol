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

    DKIMRegistry public dkimRegistry;
    address public signer;

    string public constant SET_PREFIX = "SET:";
    string public constant REVOKE_PREFIX = "REVOKE:";

    constructor(address _signer) {
        dkimRegistry = new DKIMRegistry();
        signer = _signer;
    }

    function isDKIMPublicKeyHashValid(string memory domainName, bytes32 publicKeyHash) public view returns (bool) {
        return dkimRegistry.isDKIMPublicKeyHashValid(domainName, publicKeyHash);
    }

    function setDKIMPublicKeyHash(
        string memory selector,
        string memory domainName,
        bytes32 publicKeyHash,
        bytes memory signature
    ) public {
        require(bytes(selector).length != 0, "Invalid selector");
        require(bytes(domainName).length != 0, "Invalid domain name");
        require(publicKeyHash != bytes32(0), "Invalid public key hash");
        require(isDKIMPublicKeyHashValid(domainName, publicKeyHash) == false, "publicKeyHash is already set");
        require(dkimRegistry.revokedDKIMPublicKeyHashes(publicKeyHash) == false, "publicKeyHash is revoked");

        string memory signedMsg = computeSignedMsg(SET_PREFIX, domainName, publicKeyHash);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        address recoveredSigner = digest.recover(signature);
        require(recoveredSigner == signer, "Invalid signature");

        dkimRegistry.setDKIMPublicKeyHash(domainName, publicKeyHash);
    }

    function revokeDKIMPublicKeyHash(string memory domainName, bytes32 publicKeyHash, bytes memory signature) public {
        require(bytes(domainName).length != 0, "Invalid domain name");
        require(publicKeyHash != bytes32(0), "Invalid public key hash");
        require(isDKIMPublicKeyHashValid(domainName, publicKeyHash) == true, "publicKeyHash is not set");
        require(dkimRegistry.revokedDKIMPublicKeyHashes(publicKeyHash) == false, "publicKeyHash is already revoked");

        string memory signedMsg = computeSignedMsg(REVOKE_PREFIX, domainName, publicKeyHash);
        bytes32 digest = bytes(signedMsg).toEthSignedMessageHash();
        address recoveredSigner = digest.recover(signature);
        require(recoveredSigner == signer, "Invalid signature");

        dkimRegistry.revokeDKIMPublicKeyHash(publicKeyHash);
    }

    /// @notice Computes a signed message string for setting or revoking a DKIM public key hash.
    /// @param prefix The operation prefix (SET: or REVOKE:).
    /// @param domainName The domain name related to the operation.
    /// @param publicKeyHash The DKIM public key hash involved in the operation.
    /// @return string The computed signed message.
    /// @dev This function is used internally to generate the message that needs to be signed for setting or revoking a public key hash.
    function computeSignedMsg(
        string memory prefix,
        string memory domainName,
        bytes32 publicKeyHash
    ) public pure returns (string memory) {
        return
            string.concat(
                prefix,
                "domain=",
                domainName,
                ";public_key_hash=",
                uint256(publicKeyHash).toHexString(),
                ";"
            );
    }
}
