// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
    Registry for storing tokenName to address
    Return address(0x11) if tokenName is 1

    TODO: Store popular token address as constants would save gas
 */
contract TokenRegistry is Ownable {
    // Mapping of token name to token address
    mapping(string => address) public tokenAddressMapping;

    function getTokenAddress(
        string memory tokenName
    ) public view returns (address) {
        if (Strings.equal(tokenName, "ETH")) {
            return address(0x1111111111111111111111111111111111111111);
        }

        return tokenAddressMapping[tokenName];
    }

    function setTokenAddress(
        string memory tokenName,
        address tokenAddress
    ) public onlyOwner {
        require(
            !Strings.equal(tokenName, "ETH"),
            "ETH is a reserved token name"
        );

        tokenAddressMapping[tokenName] = tokenAddress;
    }
}
