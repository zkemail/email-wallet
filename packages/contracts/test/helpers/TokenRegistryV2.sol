// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

// This contract is for testing upgradeability
contract TokenRegistryV2 is Initializable, UUPSUpgradeable, OwnableUpgradeable {
    event ChainRegistered(string indexed chainName, uint256 indexed chainId);
    event TokenRegistered(uint256 indexed chainId, string indexed tokenName, address indexed addr);

    // Mapping of chainId to token name to address
    mapping(uint256 => mapping(string => address)) public addressOfTokenName;

    // Mapping of chainId to address of token
    mapping(uint256 => mapping(address => string)) public tokenNameOfAddress;

    // Mapping of chain name to chain id
    mapping(string => uint256) public chainIdOfName;

    constructor() {
        _disableInitializers();
    }

    function initialize() initializer public {
        __Ownable_init();
        chainIdOfName["mainnet"] = 0;
        chainIdOfName["optimism"] = 10;
        chainIdOfName["arbitrum"] = 42161;
    }

    function _authorizeUpgrade(address newImplementation)
        internal
        onlyOwner
        override
    {}

    /// @notice Set token address for a chain
    /// @param chainId Chain id of the network
    /// @param tokenName Token name - other than ones hardcoded in `getTokenAddress()`
    /// @param addr Address of the token in the chain
    function setTokenAddress(uint256 chainId, string memory tokenName, address addr) public onlyOwner {
        require(addressOfTokenName[chainId][tokenName] == address(0), "Token already registered");
        require(bytes(tokenNameOfAddress[chainId][addr]).length == 0, "Address already registered");

        addressOfTokenName[chainId][tokenName] = addr;
        tokenNameOfAddress[chainId][addr] = tokenName;

        emit TokenRegistered(chainId, tokenName, addr);
    }

    /// @notice Set token addresses for a chain
    /// @param chainId Chain id of the network
    /// @param tokenNames Token names - other than ones hardcoded in `getTokenAddress()`
    /// @param addrs Addresses of the tokens in the chain
    function setTokenAddresses(uint256 chainId, string[] memory tokenNames, address[] memory addrs) public onlyOwner {
        require(tokenNames.length == addrs.length, "tokenNames and addrs length mismatch");

        for (uint256 i = 0; i < tokenNames.length; i++) {
            setTokenAddress(chainId, tokenNames[i], addrs[i]);
        }
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
    /// @dev "0" is a valid input for `chainId` arg (mainnet). Be careful to not pass uninitialized uint variable.
    /// @dev "ETH" with return the address of "WETH"
    function getTokenAddress(uint256 chainId, string memory tokenName) public view returns (address) {
        // Return WETH
        if (Strings.equal(tokenName, "ETH")) {
            tokenName = "WETH";
        }

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
            if (chainId == 0) return address(0);
            if (chainId == 10) return 0x7F5c764cBc14f9669B88837ca1490cCa17c31607;
            if (chainId == 42161) return 0xaf88d065e77c8cC2239327C5EDb3A432268e5831; //0xFF970A61A04b1cA14834A43f5dE4533eBDDB5CC8;
        }

        return addressOfTokenName[chainId][tokenName];
    }

    /// @notice Get token address for the given chain name
    /// @param chainName Name of the chain / network
    /// @param tokenName Name of the token for which address if needed
    /// @dev "ETH" with return the address of "WETH"
    function getTokenAddress(string memory chainName, string memory tokenName) public view returns (address) {
        require(chainIdOfName[chainName] != 0, "unknown chain name");

        return getTokenAddress(chainIdOfName[chainName], tokenName);
    }

    /// @notice Get token address for the current chain
    /// @param tokenName Name of the token for which address if needed
    /// @dev "ETH" with return the address of "WETH"
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

    /// @notice Set chain id for the given chain name
    /// @param chainName Name of the chain
    /// @param chainId Chain id of the network
    function setChainId(string memory chainName, uint256 chainId) public onlyOwner {
        require(chainId != 0, "chain id cannot be 0");
        require(Strings.equal(chainName, "mainnet") == false, "cannot set mainnet chain id");
        require(chainIdOfName[chainName] == 0, "chain id already set");

        chainIdOfName[chainName] = chainId;

        emit ChainRegistered(chainName, chainId);
    }

    /// @notice Get chain id for the given chain name
    /// @param chainName Name of the chain
    function getChainIdOfName(string memory chainName) public view returns (uint256) {
        return chainIdOfName[chainName];
    }
}
