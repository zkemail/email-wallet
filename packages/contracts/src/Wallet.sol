// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

contract Wallet {
    address public owner;
    bool private initialized;

    fallback() external payable {}

    receive() external payable {}

    modifier onlyOwner() {
        require(msg.sender == owner, "only owner");
        _;
    }

    modifier notInitialized() {
        require(!initialized, "already initialized");
        _;
    }

    function initialize() external notInitialized {
        initialized = true;
        owner = msg.sender;
    }

    function setOwner(address newOwner) external onlyOwner {
        owner = newOwner;
    }

    function execute(
        address target,
        uint256 value,
        bytes calldata data
    ) external onlyOwner returns (bool success, bytes memory returnData) {
        (success, returnData) = target.call{value: value}(data);
    }
}
