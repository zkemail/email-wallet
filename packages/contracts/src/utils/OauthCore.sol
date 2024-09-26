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
    mapping(bytes32 => uint256) public expiryOfNonceHash;
    mapping(bytes32 => mapping(address => uint256)) public tokenAllowancesOfNonceHash;

    constructor() {
        _disableInitializers();
    }

    function initialize() public initializer {
        __Ownable_init();
    }

    function getUsernameOfWallet(address wallet) external view returns (string memory) {
        return usernameOfWallet[wallet];
    }

    function getWalletOfUsername(string memory username) external view returns (address) {
        return walletOfUsername[username];
    }

    function getInfoOfWalletAndNonce(address wallet, uint256 nonce) external view returns (address, uint256) {
        bytes32 nonceHash = _computeNonceHash(wallet, nonce);
        return (epheAddrOfNonceHash[nonceHash], expiryOfNonceHash[nonceHash]);
    }

    function getTokenAllowancesOfWalletAndNonce(
        address wallet,
        uint256 nonce,
        address tokenAddr
    ) external view returns (uint256) {
        bytes32 nonceHash = _computeNonceHash(wallet, nonce);
        return tokenAllowancesOfNonceHash[nonceHash][tokenAddr];
    }

    function validateEpheAddr(address wallet, address epheAddr, uint256 nonce) external view override {
        require(wallet != address(0), "invalid wallet");
        require(epheAddr != address(0), "invalid epheAddr");
        require(nonce < nextNonceOfWallet[wallet], "too large nonce");
        bytes32 nonceHash = _computeNonceHash(wallet, nonce);
        require(
            epheAddrOfNonceHash[nonceHash] == epheAddr,
            "epheAddr is not registered for the given wallet and nonce"
        );
        require(expiryOfNonceHash[nonceHash] > block.timestamp, "expired epheAddr");
    }

    function validateSignature(address epheAddr, bytes32 hash, bytes memory signature) external pure override {
        address signer = ECDSA.recover(ECDSA.toEthSignedMessageHash(hash), signature);
        require(signer == epheAddr, "invalid signature");
    }

    function reduceTokenAllowance(uint256 nonce, address token, uint256 amount) external override {
        bytes32 nonceHash = _computeNonceHash(msg.sender, nonce);
        require(tokenAllowancesOfNonceHash[nonceHash][token] >= amount, "insufficient token allowance");
        tokenAllowancesOfNonceHash[nonceHash][token] -= amount;
        emit ReducedTokenAllowance(msg.sender, token, amount, nonce);
    }

    function signup(string memory username) external override {
        address wallet = msg.sender;
        require(walletOfUsername[username] == address(0), "username already exists");
        require(bytes(usernameOfWallet[wallet]).length == 0, "wallet already takes a username");
        usernameOfWallet[wallet] = username;
        walletOfUsername[username] = wallet;
        emit Signup(wallet, username);
    }

    function signin(
        string memory username,
        uint256 nonce,
        uint256 expiry,
        TokenAllowance[] memory tokenAllowances
    ) external override {
        address wallet = msg.sender;
        require(walletOfUsername[username] == wallet, "invalid username");
        require(nonce < nextNonceOfWallet[wallet], "too large nonce");
        require(block.timestamp < expiry, "expired nonce");

        bytes32 nonceHash = _computeNonceHash(wallet, nonce);
        address epheAddr = epheAddrOfNonceHash[nonceHash];
        require(epheAddr != address(0), "invalid nonce");
        require(expiryOfNonceHash[nonceHash] == 0, "nonce already used for sign-in");

        expiryOfNonceHash[nonceHash] = expiry;
        for (uint8 i = 0; i < tokenAllowances.length; i++) {
            tokenAllowancesOfNonceHash[nonceHash][tokenAllowances[i].tokenAddr] = tokenAllowances[i].amount;
        }
        emit Signin(wallet, username, nonce, expiry, tokenAllowances);
    }

    function registerEpheAddr(address wallet, address epheAddr) external {
        require(wallet != address(0), "invalid wallet");
        require(epheAddr != address(0), "invalid epheAddr");
        uint256 nonce = nextNonceOfWallet[wallet];
        bytes32 nonceHash = _computeNonceHash(wallet, nonce);
        require(epheAddrOfNonceHash[nonceHash] == address(0), "nonce already used");
        epheAddrOfNonceHash[nonceHash] = epheAddr;
        nextNonceOfWallet[wallet]++;
        string memory username = usernameOfWallet[wallet];
        emit RegisteredEpheAddr(wallet, epheAddr, nonce, username);
    }

    function _computeNonceHash(address wallet, uint256 nonce) private view returns (bytes32) {
        return keccak256(abi.encodePacked(address(this), block.chainid, wallet, nonce));
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
