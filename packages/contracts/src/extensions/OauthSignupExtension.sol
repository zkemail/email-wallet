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

contract OauthSignupExtension is Extension, Initializable, UUPSUpgradeable, OwnableUpgradeable {
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
        templates = new string[][](1);
        templates[0] = ["Sign-up", "{string}"];
    }

    function execute(
        uint8 templateIndex,
        bytes[] memory subjectParams,
        address wallet,
        bool hasEmailRecipient,
        address,
        bytes32
    ) external override onlyCore {
        require(templateIndex == 0, "invalid templateIndex");
        require(!hasEmailRecipient, "recipient is not supported");

        IOauth oauthCore = Wallet(payable(wallet)).getOauth();
        require(subjectParams.length == 1, "invalid subjectParams length");
        string memory username = abi.decode(subjectParams[0], (string));
        bytes memory data = abi.encodeWithSignature("signup(string)", username);
        core.executeAsExtension(address(oauthCore), data);
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
