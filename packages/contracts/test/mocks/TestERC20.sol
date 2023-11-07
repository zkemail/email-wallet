// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestERC20 is ERC20 {
    constructor(string memory name, string memory tick) ERC20(name, tick) {}

    function freeMint(uint256 amount) public {
        _mint(msg.sender, amount);
    }

    function freeMint(address to, uint256 amount) public {
        _mint(to, amount);
    }

    function approve(address spender, uint256 amount) public override returns (bool) {
        _approve(msg.sender, spender, amount);
        return true;
    }
}
