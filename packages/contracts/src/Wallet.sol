// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

// TODO: Use clone factory methods to save gas for each deployment
contract Wallet {
    address public owner;

    modifier onlyOwner() {
        require(msg.sender == owner, "only owner");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function setOwner(address newOwner) external onlyOwner {
        owner = newOwner;
    }

    // Execute any all requested by the owner - works like a EOA
    function execute(
        address target,
        uint256 value,
        bytes calldata data
    ) external onlyOwner returns (bool success, bytes memory returnData) {
        (success, returnData) = target.call{value: value}(data);
    }
}
