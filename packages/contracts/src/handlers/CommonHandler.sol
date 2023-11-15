// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

contract CommonHandler {
    modifier onlyBeforeLimit() {
        require(block.timestamp < 1701388799, "this function is not allowed from 2023-12-01");
        _;
    }
}