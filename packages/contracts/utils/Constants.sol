// SPDX-License-Identifier: MIT
// OpenZeppelin Contracts (last updated v4.7.0) (utils/Create2.sol)

pragma solidity ^0.8.0;

/**
 * @dev Helper to make usage of the `CREATE2` EVM opcode easier and safer.
 * `CREATE2` can be used to compute in advance the address where a smart
 * contract will be deployed, which allows for interesting new mechanisms known
 * as 'counterfactual interactions'.
 *
 * See the https://eips.ethereum.org/EIPS/eip-1014#motivation[EIP] for more
 * information.
 */
library Constants {
    uint256 public constant WALLET_EXTENSION_ID = 0;
    uint256 public constant CONFIG_EXTENSION_ID = 1;
    uint256 public constant EXT_EXTENSION_ID = 2;
    uint256 public constant TRANSPORT_EXTENSION_ID = 3;
}
