// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "./TokenRegistry.sol";
import "./interfaces/IVerifier.sol";

contract WalletHandler is TokenRegistry {
    struct DepositNote {
        address sender;
        address tokenAddress;
        uint256 amount;
    }

    // Mapping to store the balance of each user
    // Mapping of accountSalt -> (tokenAddress -> balance)
    // accountSalt is derived from the viewing key - hash(vk, randomNonce)
    mapping(bytes32 => mapping(address => uint256)) public balancesOfWallet;

    // Mapping to store deposit commitments
    // Mapping of depositCommitment -> DepositNote
    mapping(bytes32 => DepositNote) public depositNotes;

    function _processTransferRequest(
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

    function _initializeDepositRequest(
        bytes32 depositCommitment,
        address tokenAddress,
        uint256 amount
    ) internal {
        IERC20 token = IERC20(tokenAddress);

        // Check the allowance from user is greater than or equal to amount
        require(
            token.allowance(msg.sender, address(this)) >= amount,
            "allowance not enough"
        );

        depositNotes[depositCommitment] = DepositNote(
            msg.sender,
            tokenAddress,
            amount
        );
    }

    function _processDepositRequest(
        IVerifier verifier,
        bytes32 depositCommitment,
        bytes32 recipientSalt,
        bytes memory depositProof
    ) internal {
        DepositNote memory depositNote = depositNotes[depositCommitment];

        require(depositNote.amount > 0, "deposit commitment not initialized");

        // Get the token address from token name
        IERC20 token = IERC20(depositNote.tokenAddress);

        // Verify deposit proof
        // This will verify the commitment and salt is derived from same email address
        verifier.verifyDepositProof(depositCommitment, recipientSalt, depositProof);
        
        // Transfer token to this contract
        token.transferFrom(depositNote.sender, address(this), depositNote.amount);

        // Update the balance of the recipient
        balancesOfWallet[recipientSalt][depositNote.tokenAddress] += depositNote.amount;
    }
}
