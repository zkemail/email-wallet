// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

/// @title TokenRegistry
/// @notice A registry of token name and their address on different chains
contract TokenRegistry is Ownable {
    // Mapping of chainId to token name to address
    mapping(uint256 => mapping(string => address)) public addressOfTokenName;

    // Mapping of chainId to address of token
    mapping(uint256 => mapping(address => string)) public tokenNameOfAddress;

    /// @notice Set token address for a chain
    /// @param chainId Chain id of the network
    /// @param tokenName Token name - other than ones hardcoded in `getTokenAddress()`
    /// @param addr Address of the token in the chain
    function setTokenAddress(uint256 chainId, string memory tokenName, address addr) public onlyOwner {
        addressOfTokenName[chainId][tokenName] = addr;
        tokenNameOfAddress[chainId][addr] = tokenName;
    }

    /// @notice Set token address for the current chain
    /// @param tokenName Token name - other than ones hardcoded in `getTokenAddress()`
    /// @param addr Address of the token in the chain
    function setTokenAddress(string memory tokenName, address addr) public onlyOwner {
        setTokenAddress(block.chainid, tokenName, addr);
    }

    /// @notice Get token address for a chain
    /// @param chainId Chain id of the network for which address is needed
    /// @param tokenName Name of the token for which address if needed
    function getTokenAddress(uint256 chainId, string memory tokenName) public view returns (address) {
        if (Strings.equal(tokenName, "WETH")) {
            if (chainId == 0) return 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
            if (chainId == 10) return 0x4200000000000000000000000000000000000006;
            if (chainId == 42161) return 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
        }

        if (Strings.equal(tokenName, "DAI")) {
            if (chainId == 0) return 0x6B175474E89094C44Da98b954EedeAC495271d0F;
            if (chainId == 10) return 0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1;
            if (chainId == 42161) return 0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1;
        }

        if (Strings.equal(tokenName, "USDC")) {
            if (chainId == 0) return 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
            if (chainId == 10) return 0x7F5c764cBc14f9669B88837ca1490cCa17c31607;
            if (chainId == 42161) return 0xFF970A61A04b1cA14834A43f5dE4533eBDDB5CC8;
        }

        return addressOfTokenName[chainId][tokenName];
    }

    /// @notice Get token address for the current chain
    /// @param tokenName Name of the token for which address if needed
    function getTokenAddress(string memory tokenName) public view returns (address) {
        return getTokenAddress(block.chainid, tokenName);
    }

    /// @notice Get token name for the given address in a chain
    /// @param chainId Chain id of the network for which address is needed
    /// @param addr Address of the token for which name if needed
    function getTokenNameOfAddress(uint256 chainId, address addr) public view returns (string memory) {
        if (addr == getTokenAddress(chainId, "WETH")) return "WETH";
        if (addr == getTokenAddress(chainId, "DAI")) return "DAI";
        if (addr == getTokenAddress(chainId, "USDC")) return "USDC";

        return tokenNameOfAddress[chainId][addr];
    }

    /// @notice Get token name for the given address in the current chain
    /// @param addr Address of the token for which name if needed
    function getTokenNameOfAddress(address addr) public view returns (string memory) {
        return getTokenNameOfAddress(block.chainid, addr);
    }
}
