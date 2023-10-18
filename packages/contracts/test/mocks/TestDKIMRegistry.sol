// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@zk-email/contracts/interfaces/IDKIMRegistry.sol";

contract TestDKIMRegistry is IDKIMRegistry {
    function getDKIMPublicKeyHash(string memory domainName) public pure returns (bytes32) {
        domainName;
        return bytes32(uint256(123));
    }
}
