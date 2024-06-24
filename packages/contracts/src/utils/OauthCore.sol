// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "../interfaces/Types.sol";
import {IOauth} from "../interfaces/IOauth.sol";
import {TokenRegistry} from "../utils/TokenRegistry.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract OauthCore is Initializable, UUPSUpgradeable, OwnableUpgradeable, IOauth {
    mapping(address => string) public usernameOfWallet;
    mapping(string => address) public walletOfUsername;
    mapping(address => uint256) public nextNonceOfWallet;
    mapping(bytes32 => address) public epheAddrOfNonceHash;
    mapping(address => uint256) public expiryOfEpheAddr;
    mapping(address => mapping(address => uint256)) public tokenAllowancesOfEpheAddr;
    mapping(address => bool) public isSudoOfEpheAddr;

    constructor() {
        _disableInitializers();
    }

    function initialize() public initializer {
        __Ownable_init();
    }

    function validateEpheAddr(address wallet, address epheAddr, uint256 nonce, bool isSudo) external view override {
        require(wallet != address(0), "invalid wallet");
        require(epheAddr != address(0), "invalid epheAddr");
        require(nonce < nextNonceOfWallet[wallet], "too large nonce");
        bytes32 nonceHash = _computeNonceHash(wallet, nonce);
        require(
            epheAddrOfNonceHash[nonceHash] == epheAddr,
            "epheAddr is not registered for the given wallet and nonce"
        );
        require(expiryOfEpheAddr[epheAddr] > block.timestamp, "expired epheAddr");
        require(isSudoOfEpheAddr[epheAddr] == isSudo, "invalid isSudo");
    }

    function validateSignature(address epheAddr, bytes32 hash, bytes memory signature) external pure override {
        address signer = ECDSA.recover(hash, signature);
        require(signer == epheAddr, "invalid signature");
    }

    function reduceTokenAllowance(address epheAddr, address token, uint256 amount) external override {
        require(tokenAllowancesOfEpheAddr[epheAddr][token] >= amount, "insufficient token allowance");
        tokenAllowancesOfEpheAddr[epheAddr][token] -= amount;
    }

    function signup(string memory username) external override {
        address wallet = msg.sender;
        require(walletOfUsername[username] == address(0), "username already exists");
        require(bytes(usernameOfWallet[wallet]).length == 0, "wallet already takes a username");
        usernameOfWallet[wallet] = username;
        walletOfUsername[username] = wallet;
    }

    function signin(
        string memory username,
        uint256 nonce,
        uint256 expiry,
        TokenAllowance[] memory tokenAllowances,
        bool isSudo
    ) external override {
        address wallet = msg.sender;
        require(walletOfUsername[username] == wallet, "invalid username");
        require(nonce < nextNonceOfWallet[wallet], "too large nonce");
        require(block.timestamp < expiry, "expired nonce");

        bytes32 nonceHash = _computeNonceHash(wallet, nonce);
        address epheAddr = epheAddrOfNonceHash[nonceHash];
        require(epheAddr != address(0), "invalid nonce");
        require(expiryOfEpheAddr[epheAddr] == 0, "nonce already used for sign-in");

        expiryOfEpheAddr[epheAddr] = expiry;
        isSudoOfEpheAddr[epheAddr] = isSudo;
        for (uint8 i = 0; i < tokenAllowances.length; i++) {
            tokenAllowancesOfEpheAddr[epheAddr][tokenAllowances[i].tokenAddr] = tokenAllowances[i].amount;
        }
    }

    function _computeNonceHash(address wallet, uint256 nonce) private view returns (bytes32) {
        return keccak256(abi.encodePacked(address(this), block.chainid, wallet, nonce));
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
