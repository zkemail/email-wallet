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

contract OauthExtension is Extension, Initializable, UUPSUpgradeable, OwnableUpgradeable {
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
        address,
        bytes32
    ) external override onlyCore {
        require(templateIndex < 17, "invalid templateIndex");
        require(!hasEmailRecipient, "recipient is not supported");

        IOauth oauthCore = Wallet(payable(wallet)).oauth();
        if (templateIndex == 0) {
            require(subjectParams.length == 1, "invalid subjectParams length");
            core.executeAsExtension(address(oauthCore), abi.encodeWithSignature("signup(string)", subjectParams[0]));
        } else {
            (
                string memory username,
                uint256 nonce,
                bool isSudo,
                uint256 expiry,
                TokenAllowance[] memory tokenAllowances
            ) = _parseSigninSubjectParams(templateIndex, subjectParams);
            bytes memory data = abi.encodeWithSignature(
                "signin(string,uint256,uint256,(address,uint256)[],bool)",
                username,
                nonce,
                expiry,
                tokenAllowances,
                isSudo
            );
            core.executeAsExtension(address(oauthCore), data);
        }
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    function _decomposeTo4Bits(uint8 idx) public pure returns (bool[4] memory) {
        bool[4] memory bits;
        bits[0] = (idx & 8) != 0;
        bits[1] = (idx & 4) != 0;
        bits[2] = (idx & 2) != 0;
        bits[3] = (idx & 1) != 0;
        return bits;
    }

    function _parseSigninSubjectParams(
        uint8 templateIndex,
        bytes[] memory subjectParams
    )
        private
        view
        returns (
            string memory username,
            uint256 nonce,
            bool isSudo,
            uint256 expiry,
            TokenAllowance[] memory tokenAllowances
        )
    {
        bool[4] memory bits = _decomposeTo4Bits(templateIndex - 1);
        username = abi.decode(subjectParams[0], (string));
        nonce = abi.decode(subjectParams[1], (uint256));
        isSudo = bits[0] && bits[1];
        uint8 numTokenAllowances = 2 * uint8(bits[2] ? 1 : 0) + uint8(bits[3] ? 1 : 0);
        tokenAllowances = new TokenAllowance[](numTokenAllowances);
        uint256 lastSubjectParamIdx = 2;
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
        require(lastSubjectParamIdx == subjectParams.length, "invalid subjectParams length");
    }
}
