// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/access/Ownable.sol";

/**
  Store the hash of public key of DKIM record for each domain
 */
contract DKIMPublicKeyStorage is Ownable {
    // Mapping from domain name to DKIM public key hash
    mapping(string => bytes32) public dkimPublicKeyHashes;

    function setDKIMPublicKeyHash(
        string memory domainName,
        bytes32 publicKeyHash
    ) public onlyOwner {
        dkimPublicKeyHashes[domainName] = publicKeyHash;
    }
}
