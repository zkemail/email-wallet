// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/access/Ownable.sol";

/**
  Store the hash of public key of DKIM record for each domain
 */
contract DKIMPublicKeyStorage is Ownable {
    // Mapping from domain name to DKIM public key hash
    mapping(string => string) public dkimPublicKeyHashes;

    function setDKIMPublicKey(string memory domainName, string memory publicKeyHash) public onlyOwner {
        dkimPublicKeyHashes[domainName] = publicKeyHash;
    }
}