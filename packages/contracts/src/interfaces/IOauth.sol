// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./Types.sol";

interface IOauth {
    event Signup(address indexed wallet, string indexed username);
    event Signin(
        address indexed wallet,
        string indexed username,
        uint256 indexed nonce,
        uint256 expiry,
        TokenAllowance[] tokenAllowances
    );
    event RegisteredEpheAddr(address indexed wallet, address indexed epheAddr, uint256 indexed nonce, string username);
    event ReducedTokenAllowance(address indexed wallet, address indexed token, uint256 indexed amount, uint256 nonce);

    /// @notice Get the username of the wallet.
    /// @param wallet Address of the wallet.
    /// @return username of the wallet.
    function getUsernameOfWallet(address wallet) external view returns (string memory);

    /// @notice Get the wallet of the username.
    /// @param username Username of the wallet.
    /// @return wallet Address of the wallet.
    function getWalletOfUsername(string memory username) external view returns (address);

    /// @notice Get the oauth info for the wallet and nonce.
    /// @param wallet Address of the wallet.
    /// @param nonce Nonce of the ephemeral address.
    /// @return epheAddr Address of the ephemeral address.
    /// @return expiry Expiry of the ephemeral address.
    function getInfoOfWalletAndNonce(address wallet, uint256 nonce) external view returns (address, uint256);

    /// @notice Get the token allowances of the wallet and nonce.
    /// @param wallet Address of the wallet.
    /// @param nonce Nonce of the ephemeral address.
    /// @param tokenAddr Address of the token.
    function getTokenAkkowancesOfWalletAndNonce(
        address wallet,
        uint256 nonce,
        address tokenAddr
    ) external view returns (uint256);

    /// @notice Validate if the given ephemeral address is valid.
    /// @param wallet Address of the wallet.
    /// @param epheAddr Address of the ephemeral address.
    /// @param nonce Nonce of the ephemeral address.
    function validateEpheAddr(address wallet, address epheAddr, uint256 nonce) external view;

    /// @notice Validate the signature of the ephemeral address.
    /// @param epheAddr Address of the ephemeral address.
    /// @param hash Hash to validate.
    /// @param signature Signature to validate.
    function validateSignature(address epheAddr, bytes32 hash, bytes memory signature) external view;

    /// @notice Reduce the token allowance of the ephemeral address.
    /// @param nonce Nonce of the ephemeral address.
    /// @param token Address of the token.
    /// @param amount Amount of the token to reduce.
    function reduceTokenAllowance(uint256 nonce, address token, uint256 amount) external;

    /// @notice Signup a username for the wallet.
    /// @param username Username to signup.
    function signup(string memory username) external;

    /// @notice Signin the username for the wallet.
    /// @param username Username to signin.
    /// @param nonce Nonce of the signin.
    /// @param expiry Expiry of the signin.
    /// @param tokenAllowances Array of token allowances.
    function signin(
        string memory username,
        uint256 nonce,
        uint256 expiry,
        TokenAllowance[] memory tokenAllowances
    ) external;

    /// @notice Register the ephemeral address for the wallet.
    /// @param wallet Address of the wallet.
    /// @param epheAddr Address of the ephemeral address.
    /// @dev This function MUST allows that the same `epheAddr` is used more than once; otherwise, an adversary can post the same `epheAddr` to block the user's registration.
    function registerEpheAddr(address wallet, address epheAddr) external;
}
