// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {EmailWalletCore} from "../EmailWalletCore.sol";
import {Wallet} from "../Wallet.sol";
import "../interfaces/Types.sol";
import {OauthExtensionBase} from "./OauthExtensionBase.sol";
import {IOauth} from "../interfaces/IOauth.sol";
import "forge-std/console.sol";

contract OauthSigninExtension is OauthExtensionBase {
    string[][] public templates;

    constructor() {
        _disableInitializers();
    }

    function initialize(address coreAddr) public override initializer {
        super.initialize(coreAddr);
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
        string memory username = abi.decode(subjectParams[0], (string));
        (uint256 nonce, uint256 expiry, TokenAllowance[] memory tokenAllowances) = _parseSigninSubjectParams(
            templateIndex,
            subjectParams
        );
        bytes memory data = abi.encodeWithSignature(
            "signin(string,uint256,uint256,(address,uint256)[])",
            username,
            nonce,
            expiry,
            tokenAllowances
        );
        core.executeAsExtension(address(oauthCore), data);
    }
}
