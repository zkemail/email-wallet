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
    bytes32 public constant SEND_HASH = keccak256(bytes(SEND));
    string public constant EXECUTE = "Execute";
    bytes32 public constant EXECUTE_HASH = keccak256(bytes(EXECUTE));
    // string public constant INSTALL_EXTENSION = "Install extension";
    string public constant INSTALL_EXTENSION = "Install";
    bytes32 public constant INSTALL_EXTENSION_HASH = keccak256(bytes(INSTALL_EXTENSION));
    // string public constant UNINSTALL_EXTENSION = "Uninstall extension";
    string public constant UNINSTALL_EXTENSION = "Uninstall";
    bytes32 public constant UNINSTALL_EXTENSION_HASH = keccak256(bytes(UNINSTALL_EXTENSION));
    // string public constant EXIT_EMAIL_WALLET = "Exit Email Wallet. Change wallet ownership to";
    string public constant EXIT_EMAIL_WALLET = "Exit";
    bytes32 public constant EXIT_EMAIL_WALLET_HASH = keccak256(bytes(EXIT_EMAIL_WALLET));
}
