// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@zk-email/contracts/interfaces/IDKIMRegistry.sol";

contract TestDKIMRegistry is IDKIMRegistry {
    function getDKIMPublicKeyHash(string memory domainName) public view returns (bytes32) {
        return bytes32(uint256(123));
    }
}
