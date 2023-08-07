// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
    Registry for storing tokenName to address
    TODO: Store popular token address as constants would save gas
 */
contract TokenRegistry is Ownable {
    // Mapping of token name to token address
    mapping(string => address) public tokenAddressMapping;

    function getTokenAddress(
        string memory tokenName
    ) public view returns (address) {
        return tokenAddressMapping[tokenName];
    }

    function setTokenAddress(
        string memory tokenName,
        address tokenAddress
    ) public onlyOwner {
        tokenAddressMapping[tokenName] = tokenAddress;
    }
}
