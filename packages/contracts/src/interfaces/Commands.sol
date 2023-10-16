// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/*
    Sample email subjects:
        - Send 1 ETH to recipient@domain.com
        - Send 1 DAI to recipient@domain.com
        - Send 1 ETH to 0x1234...5678
        - Send 1 DAI to 0x1234...5678
        - Set extension for Swap as Uniswap
        - Remove extension for Swap
        - Extension subject template (array): ["Swap", "{tokenAmount}", "to", "{string}"]
 */

library Commands {
    string public constant SEND = "Send";
    string public constant EXECUTE = "Execute";
    string public constant INSTALL_EXTENSION = "Install";
    string public constant UNINSTALL_EXTENSION = "Uninstall";
    string public constant EXIT_EMAIL_WALLET = "Exit";
    string public constant DKIM = "DKIM";
}
