// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {EmailWalletCore} from "../../src/EmailWalletCore.sol";
import {Extension} from "../../src/interfaces/Extension.sol";
import {TokenRegistry} from "../../src/utils/TokenRegistry.sol";
import "../../src/interfaces/Types.sol";

contract TestExtension is Extension {
    EmailWalletCore core;
    ERC20 sampleToken;
    TokenRegistry tokenRegistry;

    constructor(address coreAddr, address tokenAddr, address tokenRegistryAddr) {
        core = EmailWalletCore(payable(coreAddr));
        sampleToken = ERC20(payable(tokenAddr));
        tokenRegistry = TokenRegistry(tokenRegistryAddr);
    }

    function execute(
        uint8 templateIndex,
        bytes[] memory subjectParams,
        address wallet,
        bool hasEmailRecipient,
        address recipientETHAddr,
        bytes32 emailNullifier
    ) external override {
        subjectParams;
        wallet;
        hasEmailRecipient;
        recipientETHAddr;
        emailNullifier;

        // templates[0] = ["Test", "Register Unclaimed State"];
        // templates[1] = ["Test", "Register Unclaimed State Twice"];
        // templates[2] = ["Test", "Register Empty Unclaimed State"];
        // templates[3] = ["Test", "Register Unclaimed State to", "{address}"];
        // templates[4] = ["Test", "Request Token", "{tokenAmount}"];
        // templates[5] = ["Test", "Request Token Twice", "{tokenAmount}"];
        // templates[6] = ["Test", "Deposit Token", "{tokenAmount}"];
        // templates[7] = ["Test", "Execute on", "{address}"];

        // Register
        if (templateIndex == 0) {
            core.registerUnclaimedStateAsExtension(address(this), abi.encode("test"));
        }

        // Register twice
        if (templateIndex == 1) {
            core.registerUnclaimedStateAsExtension(address(this), abi.encode("test"));
            core.registerUnclaimedStateAsExtension(address(this), abi.encode("test2"));
        }

        // Register empty
        if (templateIndex == 2) {
            bytes memory empty;
            core.registerUnclaimedStateAsExtension(address(this), empty);
        }

        // Register to address
        if (templateIndex == 3) {
            address addr = abi.decode(subjectParams[0], (address));
            core.registerUnclaimedStateAsExtension(addr, abi.encode("test"));
        }

        // Request token
        if (templateIndex == 4) {
            (uint256 amount, string memory tokenName) = abi.decode(subjectParams[0], (uint256, string));
            address tokenAddr = tokenRegistry.getTokenAddress(tokenName);

            core.requestTokenAsExtension(tokenAddr, amount);
        }

        // Request token twice
        if (templateIndex == 5) {
            (uint256 amount, string memory tokenName) = abi.decode(subjectParams[0], (uint256, string));
            address tokenAddr = tokenRegistry.getTokenAddress(tokenName);

            core.requestTokenAsExtension(tokenAddr, amount);
            core.requestTokenAsExtension(tokenAddr, amount);
        }

        // Desposit token
        if (templateIndex == 6) {
            (uint256 amount, string memory tokenName) = abi.decode(subjectParams[0], (uint256, string));
            address tokenAddr = tokenRegistry.getTokenAddress(tokenName);

            core.depositTokenAsExtension(tokenAddr, amount);
        }

        // Execute on
        if (templateIndex == 7) {
            address addr = abi.decode(subjectParams[0], (address));
            core.executeAsExtension(addr, "");
        }
    }

    function registerUnclaimedState(UnclaimedState memory unclaimedState, bool) public pure override {
        unclaimedState;
    }

    function claimUnclaimedState(UnclaimedState memory unclaimedState, address wallet) external pure override {
        unclaimedState;
        wallet;
    }

    function voidUnclaimedState(UnclaimedState memory unclaimedState) external pure override {
        unclaimedState;
    }
}
