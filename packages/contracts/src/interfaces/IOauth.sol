// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

interface IOauth {
    /// @notice Validate if the given ephemeral address is valid.
    /// @param wallet Address of the wallet.
    /// @param epheAddr Address of the ephemeral address.
    /// @param nonce Nonce of the ephemeral address.
    /// @param isSudo Whether the ephemeral address is sudo or not.
    function validateEpheAddr(address wallet, address epheAddr, uint256 nonce, bool isSudo) external view;

    /// @notice Validate the signature of the ephemeral address.
    /// @param epheAddr Address of the ephemeral address.
    /// @param hash Hash to validate.
    /// @param signature Signature to validate.
    function validateSignature(address epheAddr, bytes32 hash, bytes memory signature) external view;

    /// @notice Reduce the token allowance of the ephemeral address.
    /// @param epheAddr Address of the ephemeral address.
    /// @param token Address of the token.
    /// @param amount Amount of the token to reduce.
    function reduceTokenAllowance(address epheAddr, address token, uint256 amount) external;
}
