// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@zk-email/contracts/interfaces/IDKIMRegistry.sol";

contract TestDKIMRegistry is IDKIMRegistry {
    function isDKIMPublicKeyHashValid(string memory domainName, bytes32 publicKeyHash) external pure returns (bool) {
        domainName;
        return bytes32(uint256(123)) == publicKeyHash;
    }
}
