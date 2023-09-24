// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "../../src/Wallet.sol";

// A mock wallet for testing - extents Wallet and add a test `getName` method
contract TestWallet is Wallet {
    function getName() public pure returns (string memory) {
        return "Test";
    }
}
