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
    mapping(bytes32 => bool) public isSudoOfNonceHash;

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
        require(expiryOfNonceHash[nonceHash] > block.timestamp, "expired epheAddr");
        require(isSudoOfNonceHash[nonceHash] == isSudo, "invalid isSudo");
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
        require(expiryOfNonceHash[nonceHash] == 0, "nonce already used for sign-in");

        expiryOfNonceHash[nonceHash] = expiry;
        isSudoOfNonceHash[nonceHash] = isSudo;
        for (uint8 i = 0; i < tokenAllowances.length; i++) {
            tokenAllowancesOfNonceHash[nonceHash][tokenAllowances[i].tokenAddr] = tokenAllowances[i].amount;
        }
        emit Signin(wallet, username, nonce, expiry, tokenAllowances, isSudo);
    }

    function registerEpheAddr(string memory username, address epheAddr, bytes calldata signature) external {
        require(bytes(username).length > 0, "invalid username");
        require(epheAddr != address(0), "invalid epheAddr");
        bytes32 hash = hashOfRegisterEpheAddr(username, epheAddr);
        address signer = ECDSA.recover(ECDSA.toEthSignedMessageHash(hash), signature);
        require(signer == epheAddr, "invalid signature");
        address wallet = walletOfUsername[username];
        require(wallet != address(0), "username is not registered");
        uint256 nonce = nextNonceOfWallet[wallet];
        bytes32 nonceHash = _computeNonceHash(wallet, nonce);
        require(epheAddrOfNonceHash[nonceHash] == address(0), "nonce already used");
        epheAddrOfNonceHash[nonceHash] = epheAddr;
        nextNonceOfWallet[wallet]++;
        emit RegisteredEpheAddr(wallet, epheAddr, nonce, username);
    }

    /// @notice Hash of the register epheAddr
    /// @param username Username of the wallet
    /// @param epheAddr Address of the ephemeral address
    /// @return Hash of the register epheAddr
    function hashOfRegisterEpheAddr(string memory username, address epheAddr) public view returns (bytes32) {
        return keccak256(abi.encodePacked(address(this), block.chainid, epheAddr, username));
    }

    function _computeNonceHash(address wallet, uint256 nonce) private view returns (bytes32) {
        return keccak256(abi.encodePacked(address(this), block.chainid, wallet, nonce));
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
