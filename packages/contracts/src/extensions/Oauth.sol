// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Extension} from "../interfaces/Extension.sol";
import {EmailWalletCore} from "../EmailWalletCore.sol";
import "../interfaces/Types.sol";
import {StringUtils} from "../libraries/StringUtils.sol";
import {IOauth} from "../interfaces/IOauth.sol";
import {TokenRegistry} from "../utils/TokenRegistry.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract OauthExtension is Extension, Initializable, UUPSUpgradeable, OwnableUpgradeable, IOauth {
    using StringUtils for *;

    EmailWalletCore public core;
    string[][] public templates;
    mapping(address => string) public usernameOfWallet;
    mapping(string => address) public walletOfUsername;
    mapping(address => uint256) public nextNonceOfWallet;
    mapping(bytes32 => address) public epheAddrOfNonceHash;
    mapping(address => uint256) public expiryOfEpheAddr;
    mapping(address => mapping(address => uint256)) public tokenAllowancesOfEpheAddr;
    mapping(address => bool) public isSudoOfEpheAddr;

    modifier onlyCore() {
        require((msg.sender == address(core)) || (msg.sender == address(core.unclaimsHandler())), "invalid sender");
        _;
    }

    constructor() {
        _disableInitializers();
    }

    function initialize(address coreAddr) public initializer {
        __Ownable_init();
        core = EmailWalletCore(payable(coreAddr));
        templates = new string[][](17);
        templates[0] = ["Oauth", "Sign-up", "{string}"];
        // (0,0,0) = 0
        templates[1] = ["Oauth", "Sign-in", "{string}", "at", "Nonce", "{uint}"];
        // (0,0,1) = 1
        templates[2] = ["Oauth", "Sign-in", "{string}", "at", "Nonce", "{uint}", "with", "approving", "{tokenAmount}"];
        // (0,0,2) = 2
        templates[3] = [
            "Oauth",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "with",
            "approving",
            "{tokenAmount}",
            "{tokenAmount}"
        ];
        // (0,0,3) = 3
        templates[4] = [
            "Oauth",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "with",
            "approving",
            "{tokenAmount}",
            "{tokenAmount}",
            "{tokenAmount}"
        ];
        // (0,1,0) = 4
        templates[5] = ["Oauth", "Sign-in", "{string}", "at", "Nonce", "{uint}", "until", "timestamp", "{uint}"];
        // (0,1,1) = 4 + 1 = 5
        templates[6] = [
            "Oauth",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "until",
            "timestamp",
            "{uint}",
            "with",
            "approving",
            "{tokenAmount}"
        ];
        // (0,1,2) = 4 + 2 = 6
        templates[7] = [
            "Oauth",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "until",
            "timestamp",
            "{uint}",
            "with",
            "approving",
            "{tokenAmount}",
            "{tokenAmount}"
        ];
        // (0,1,3) = 4 + 3 = 7
        templates[8] = [
            "Oauth",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "until",
            "timestamp",
            "{uint}",
            "with",
            "approving",
            "{tokenAmount}",
            "{tokenAmount}",
            "{tokenAmount}"
        ];
        // (1,0,0) = 8
        templates[9] = ["Oauth", "Sudo", "Sign-in", "{string}", "at", "Nonce", "{uint}"];
        // (1,0,1) = 8 + 1 = 9
        templates[10] = [
            "Oauth",
            "Sudo",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "with",
            "approving",
            "{tokenAmount}"
        ];
        // (1,0,2) = 8 + 2 = 10
        templates[11] = [
            "Oauth",
            "Sudo",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "with",
            "approving",
            "{tokenAmount}",
            "{tokenAmount}"
        ];
        // (1,0,3) = 8 + 3 = 11
        templates[12] = [
            "Oauth",
            "Sudo",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "with",
            "approving",
            "{tokenAmount}",
            "{tokenAmount}",
            "{tokenAmount}"
        ];
        // (1,1,0) = 8 + 4 + 0 = 12
        templates[13] = [
            "Oauth",
            "Sudo",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "until",
            "timestamp",
            "{uint}"
        ];
        // (1,1,1) = 8 + 4 + 1 = 13
        templates[14] = [
            "Oauth",
            "Sudo",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "until",
            "timestamp",
            "{uint}",
            "with",
            "approving",
            "{tokenAmount}"
        ];
        // (1,1,2) = 8 + 4 + 2 = 14
        templates[15] = [
            "Oauth",
            "Sudo",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "until",
            "timestamp",
            "{uint}",
            "with",
            "approving",
            "{tokenAmount}",
            "{tokenAmount}"
        ];
        // (1,1,3) = 8 + 4 + 3 = 15
        templates[16] = [
            "Oauth",
            "Sudo",
            "Sign-in",
            "{string}",
            "at",
            "Nonce",
            "{uint}",
            "until",
            "timestamp",
            "{uint}",
            "with",
            "approving",
            "{tokenAmount}",
            "{tokenAmount}",
            "{tokenAmount}"
        ];
    }

    function execute(
        uint8 templateIndex,
        bytes[] memory subjectParams,
        address wallet,
        bool hasEmailRecipient,
        address recipientETHAddr,
        bytes32 emailNullifier
    ) external override onlyCore {
        recipientETHAddr;
        emailNullifier;
        require(templateIndex < 17, "invalid templateIndex");
        require(!hasEmailRecipient, "recipient is not supported");

        if (templateIndex == 0) {
            _processSignup(subjectParams, wallet);
        } else {
            bool[4] memory bits = _decomposeTo4Bits(templateIndex - 1);
            string memory username = abi.decode(subjectParams[0], (string));
            uint256 nonce = abi.decode(subjectParams[1], (uint256));
            bool isSudo = false;
            uint256 expiry;
            uint8 numTokenAllowances = 2 * uint8(bits[2] ? 1 : 0) + uint8(bits[3] ? 1 : 0);
            TokenAllowance[] memory tokenAllowances = new TokenAllowance[](numTokenAllowances);
            uint256 lastSubjectParamIdx = 2;
            if (bits[0]) {
                isSudo = bits[1];
            }
            if (bits[1]) {
                expiry = abi.decode(subjectParams[lastSubjectParamIdx], (uint256));
                lastSubjectParamIdx++;
            } else {
                expiry = type(uint256).max;
            }
            uint256 tokenAmount;
            string memory tokenName;
            TokenRegistry tokenRegistry = core.tokenRegistry();
            for (uint8 i = 0; i < numTokenAllowances; i++) {
                (tokenAmount, tokenName) = abi.decode(subjectParams[lastSubjectParamIdx + i], (uint256, string));
                require(tokenAmount > 0, "invalid tokenAmount");
                tokenAllowances[i] = TokenAllowance({
                    tokenAddr: tokenRegistry.getTokenAddress(tokenName),
                    amount: tokenAmount
                });
                lastSubjectParamIdx++;
            }
            _processSignin(wallet, username, nonce, expiry, tokenAllowances, isSudo);
        }
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

    function validateSignature(address epheAddr, bytes32 hash, bytes memory signature) external view override {
        address signer = ECDSA.recover(hash, signature);
        require(signer == epheAddr, "invalid signature");
    }

    function reduceTokenAllowance(address epheAddr, address token, uint256 amount) external override {
        require(tokenAllowancesOfEpheAddr[epheAddr][token] >= amount, "insufficient token allowance");
        tokenAllowancesOfEpheAddr[epheAddr][token] -= amount;
    }

    function _processSignup(bytes[] memory subjectParams, address wallet) private {
        require(subjectParams.length == 1, "invalid subjectParams length");
        string memory username = abi.decode(subjectParams[0], (string));
        require(walletOfUsername[username] == address(0), "username already exists");
        require(bytes(usernameOfWallet[wallet]).length == 0, "wallet already takes a username");
        usernameOfWallet[wallet] = username;
        walletOfUsername[username] = wallet;
    }

    function _processSignin(
        address wallet,
        string memory username,
        uint256 nonce,
        uint256 expiry,
        TokenAllowance[] memory tokenAllowances,
        bool isSudo
    ) private {
        require(walletOfUsername[username] == wallet, "invalid username");
        require(tokenAllowances.length <= 3, "invalid tokenAmounts length");
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

    function _decomposeTo4Bits(uint8 idx) public pure returns (bool[4] memory) {
        bool[4] memory bits;
        bits[0] = (idx & 8) != 0;
        bits[1] = (idx & 4) != 0;
        bits[2] = (idx & 2) != 0;
        bits[3] = (idx & 1) != 0;
        return bits;
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
