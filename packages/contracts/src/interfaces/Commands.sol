// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/*
    Sample email subjects:
        - Send 1 ETH to recipient@domain.com
        - Send 1 DAI to recipient@domain.com
        - Send 1 ETH to 0x1234...5678
        - Send 1 DAI to 0x1234...5678
        - Swap 1 ETH to DAI
        - Set extension for Swap as Uniswap
        - Remove extension for Swap
 */

library Commands {
    string public constant SEND_COMMAND = "Send";
    string public constant SET_EXTENSION_COMMAND = "Set extension";
    string public constant REMOVE_EXTENSION_COMMAND = "Remove extension";
}
