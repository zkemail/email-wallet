// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {Extension} from "../interfaces/Extension.sol";
import {EmailWalletCore} from "../EmailWalletCore.sol";
import "../interfaces/Types.sol";
import {StringUtils} from "../libraries/StringUtils.sol";

contract Safe2FAExtension is Extension, Initializable, UUPSUpgradeable, OwnableUpgradeable {
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
        __Ownable_init(msg.sender);
        core = EmailWalletCore(payable(coreAddr));
        templates = new string[][](4);
        templates[0] = ["Safe", "Transaction:", "Approve", "{string}", "from", "{address}"];
    }

    // Safe Transaction: Approve {txhash}
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}

    function execute(
        uint8 templateIndex,
        bytes[] memory subjectParams,
        address wallet,
        bool hasEmailRecipient,
        address recipientETHAddr,
        bytes32 emailNullifier
    ) external override onlyCore {
        wallet;
        recipientETHAddr;
        emailNullifier;
        require(templateIndex == 0, "invalid templateIndex");
        require(!hasEmailRecipient, "recipient is not supported");
        require(subjectParams.length == 2, "invalid subjectParams length");

        string memory txHashHex = abi.decode(subjectParams[0], (string));
        bytes32 txHash = txHashHex.hexToBytes32();
        address safeAccountAddr = abi.decode(subjectParams[1], (address));
        bytes memory data = abi.encodeWithSignature("approveHash(bytes32)", txHash);
        core.executeAsExtension(safeAccountAddr, data);
    }
}
