// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

interface IPriceOracle {
    /// @notice Get the recent price of a token against ETH/WETH
    /// @param token Address of the ERC20 token
    /// @return price Price of the token in wei
    function getRecentPriceInETH(address token) external view returns (uint256);
}
