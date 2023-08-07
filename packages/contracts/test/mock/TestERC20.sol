// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestERC20 is ERC20 {
    constructor(uint256 mintAmount) ERC20("TestERC20", "TST") {
        _mint(msg.sender, mintAmount);
    }
}
