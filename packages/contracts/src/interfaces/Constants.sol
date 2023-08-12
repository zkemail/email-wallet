// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/*
    Sample email subjects
        - Send 1 ETH to recipient@domain.com
        - Send 1 DAI to recipient@domain.com
        - Send 1 ETH to 0x1234...5678
        - Send 1 DAI to 0x1234...5678
        - Transport account to new.relayer@domain.com
        - Swap 1 ETH to DAI
        - Set extension for Swap as 0x1234...5678
        - Remove extension for Swap
 */

library Constants {
    string public constant SEND_COMMAND = "Send";
    string public constant TRANSPORT_COMMAND = "Transport";
    string public constant SET_EXTENSION_COMMAND = "Set extension";
    string public constant REMOVE_EXTENSION_COMMAND = "Remove extension";

    string public constant ETH_TOKEN_NAME = "ETH";
}
