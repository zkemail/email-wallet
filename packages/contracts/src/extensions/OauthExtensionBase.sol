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

abstract contract OauthExtensionBase is Extension, Initializable, UUPSUpgradeable, OwnableUpgradeable {
    using StringUtils for *;

    EmailWalletCore public core;

    modifier onlyCore() {
        require((msg.sender == address(core)) || (msg.sender == address(core.unclaimsHandler())), "invalid sender");
        _;
    }

    constructor() {
        _disableInitializers();
    }

    function initialize(address coreAddr) public virtual initializer {
        __Ownable_init();
        core = EmailWalletCore(payable(coreAddr));
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    function _decomposeTo3Bits(uint8 idx) internal pure returns (bool[3] memory) {
        bool[3] memory bits;
        bits[0] = (idx & 4) != 0;
        bits[1] = (idx & 2) != 0;
        bits[2] = (idx & 1) != 0;
        return bits;
    }

    function _parseSigninSubjectParams(
        uint8 templateIndex,
        bytes[] memory subjectParams
    ) internal view returns (uint256 nonce, uint256 expiry, TokenAllowance[] memory tokenAllowances) {
        bool[3] memory bits = _decomposeTo3Bits(templateIndex);
        nonce = abi.decode(subjectParams[1], (uint256));
        uint256 lastSubjectParamIdx = 2;
        uint8 numTokenAllowances = 2 * uint8(bits[1] ? 1 : 0) + uint8(bits[2] ? 1 : 0);
        tokenAllowances = new TokenAllowance[](numTokenAllowances);
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
            (tokenAmount, tokenName) = abi.decode(subjectParams[lastSubjectParamIdx], (uint256, string));
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
