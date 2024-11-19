pragma solidity ^0.8.13;

interface IWETHWithdraw {
    /// @notice Withdraw WETH from the contract
    /// @param wad Amount of WETH to withdraw
    function withdraw(uint256 wad) external;
}
