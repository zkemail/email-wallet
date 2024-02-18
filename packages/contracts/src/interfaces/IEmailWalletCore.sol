// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import { ExtensionHandler } from "../handlers/ExtensionHandler.sol";
import { TokenRegistry } from "../utils/TokenRegistry.sol";
import { AccountHandler } from "../handlers/AccountHandler.sol";
import { RelayerHandler } from "../handlers/RelayerHandler.sol";
import { UnclaimsHandler } from "../handlers/UnclaimsHandler.sol";

interface IEmailWalletCore {
    function extensionHandler() external view returns (ExtensionHandler);
    function tokenRegistry() external view returns (TokenRegistry);
    function unclaimsHandler() external view returns (address);
    function accountHandler() external view returns (address);
    function relayerHandler() external view returns (address);
}