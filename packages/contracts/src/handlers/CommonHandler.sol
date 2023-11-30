// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

contract CommonHandler {
    modifier onlyBeforeLimit() {
        require(block.timestamp < type(uint256).max, "this function is not allowed from 2023-12-01");
        _;
    }
}