// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/*
    Sample email subjects:
        - Send 1 ETH to recipient@domain.com
        - Send 1 DAI to recipient@domain.com
        - Send 1 ETH to 0x1ababab..
        - Send 1 DAI to 0x1ababab..
        - Execute 0x018923719283babababab123
        - Instal extension Uniswap
        - Uninstall extension Uniswap
        - Exit Email Wallet. Set owner to 0x1ababab..
        - DKIM registry set to 0x1ababab
        - Extension subject template (array): ["Swap", "{tokenAmount}", "to", "{string}"]
 */

library Commands {
    string public constant SEND = "Send";
    string public constant EXECUTE = "Execute";
    string public constant DKIM = "DKIM";
    string public constant INSTALL_EXTENSION = "Install";
    string public constant UNINSTALL_EXTENSION = "Uninstall";
    string public constant EXIT_EMAIL_WALLET = "Exit";

    string public constant TOKEN_AMOUNT_TEMPLATE = "{tokenAmount}";
    string public constant AMOUNT_TEMPLATE = "{amount}";
    string public constant STRING_TEMPLATE = "{string}";
    string public constant UINT_TEMPLATE = "{uint}";
    string public constant INT_TEMPLATE = "{int}";
    string public constant ADDRESS_TEMPLATE = "{address}";
    string public constant RECIPIENT_TEMPLATE = "{recipient}";
}
