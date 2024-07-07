// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Extension} from "../interfaces/Extension.sol";
import {EmailWalletCore} from "../EmailWalletCore.sol";
import {Wallet} from "../Wallet.sol";
import "../interfaces/Types.sol";
import {StringUtils} from "../libraries/StringUtils.sol";
import {IOauth} from "../interfaces/IOauth.sol";
import {TokenRegistry} from "../utils/TokenRegistry.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract OauthSigninExtension is Extension, Initializable, UUPSUpgradeable, OwnableUpgradeable {
    using StringUtils for *;

    EmailWalletCore public core;
    string[][] public templates;

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
        templates = new string[][](8);
        // (0,0) = 0
        templates[0] = ["Sign-in", "{string}", "on", "device", "{uint}"];
        // (0,1) = 1
        templates[1] = ["Sign-in", "{string}", "on", "device", "{uint}", "for", "{tokenAmount}"];
        // (0,2) = 2
        templates[2] = ["Sign-in", "{string}", "on", "device", "{uint}", "for", "{tokenAmount}", "{tokenAmount}"];
        // (0,3) = 3
        templates[3] = [
            "Sign-in",
            "{string}",
            "on",
            "device",
            "{uint}",
            "for",
            "{tokenAmount}",
            "{tokenAmount}",
            "{tokenAmount}"
        ];
        // (1,0) = 4
        templates[4] = ["Sign-in", "{string}", "on", "device", "{uint}", "until", "timestamp", "{uint}"];
        // (1,1) = 4 + 1 = 5
        templates[5] = [
            "Sign-in",
            "{string}",
            "on",
            "device",
            "{uint}",
            "until",
            "timestamp",
            "{uint}",
            "for",
            "{tokenAmount}"
        ];
        // (1,2) = 4 + 2 = 6
        templates[6] = [
            "Sign-in",
            "{string}",
            "on",
            "device",
            "{uint}",
            "until",
            "timestamp",
            "{uint}",
            "for",
            "{tokenAmount}",
            "{tokenAmount}"
        ];
        // (1,3) = 4 + 3 = 7
        templates[7] = [
            "Sign-in",
            "{string}",
            "on",
            "device",
            "{uint}",
            "until",
            "timestamp",
            "{uint}",
            "for",
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
        address,
        bytes32
    ) external override onlyCore {
        require(templateIndex < 8, "invalid templateIndex");
        require(!hasEmailRecipient, "recipient is not supported");

        IOauth oauthCore = Wallet(payable(wallet)).getOauth();
        (
            string memory username,
            uint256 nonce,
            uint256 expiry,
            TokenAllowance[] memory tokenAllowances
        ) = _parseSigninSubjectParams(templateIndex, subjectParams);
        bytes memory data = abi.encodeWithSignature(
            "signin(string,uint256,uint256,(address,uint256)[])",
            username,
            nonce,
            expiry,
            tokenAllowances
        );
        core.executeAsExtension(address(oauthCore), data);
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    function _decomposeTo3Bits(uint8 idx) public pure returns (bool[3] memory) {
        bool[3] memory bits;
        bits[0] = (idx & 4) != 0;
        bits[1] = (idx & 2) != 0;
        bits[2] = (idx & 1) != 0;
        return bits;
    }

    function _parseSigninSubjectParams(
        uint8 templateIndex,
        bytes[] memory subjectParams
    )
        private
        view
        returns (string memory username, uint256 nonce, uint256 expiry, TokenAllowance[] memory tokenAllowances)
    {
        bool[3] memory bits = _decomposeTo3Bits(templateIndex);
        username = abi.decode(subjectParams[0], (string));
        nonce = abi.decode(subjectParams[1], (uint256));
        uint8 numTokenAllowances = 2 * uint8(bits[1] ? 1 : 0) + uint8(bits[2] ? 1 : 0);
        tokenAllowances = new TokenAllowance[](numTokenAllowances);
        uint256 lastSubjectParamIdx = 2;
        if (bits[0]) {
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
        require(lastSubjectParamIdx == subjectParams.length, "invalid subjectParams length");
    }
}
