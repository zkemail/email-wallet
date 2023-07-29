// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./TokenRegistry.sol";

contract WalletHandler is TokenRegistry {
    // Mapping to store the balance of each user
    // Mapping of accountSalt -> (tokenAddress -> balance)
    // accountSalt is derived from the viewing key hash(vk)
    // for ETH balance the address should be 0x111111...1111
    mapping(bytes32 => mapping(address => uint256)) public balancesOfWallet;

    function processTransferRequest(
        bytes32 senderSalt,
        bytes32 recipientSalt,
        string memory tokenName,
        uint256 amount
    ) internal {
        // Get the token address from token name
        address tokenAddress = getTokenAddress(tokenName);
        uint256 senderBalance = balancesOfWallet[senderSalt][tokenAddress];

        require(senderBalance >= amount, "insufficient balance");

        balancesOfWallet[senderSalt][tokenAddress] -= amount;
        balancesOfWallet[recipientSalt][tokenAddress] += amount;
    }
}
