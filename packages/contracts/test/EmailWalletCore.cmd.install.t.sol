// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";
import {TestExtension} from "./mocks/TestExtension.sol";

contract InstallExtensionCommandTest is EmailWalletCoreTestHelper {
    address extensionAddr;
    string[][] tempaltes = new string[][](1);
    string extensionName = "TestSwap";

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _createTestAccount();
    }

    function _publishExtension() internal {
        address extensionDev = vm.addr(3);

        Extension ext = new TestExtension(address(core), address(daiToken), address(tokenRegistry));
        extensionAddr = address(ext);

        string[4] memory temp = ["Swap", "{tokenAmount}", "to", "{string}"];
        tempaltes[0] = temp;

        vm.startPrank(extensionDev);
        extensionHandler.publishExtension(extensionName, extensionAddr, tempaltes, 0.1 ether);
        vm.stopPrank();
    }

    function test_InstallCommand() public {
        _publishExtension();

        string memory subject = string.concat("Install extension ", extensionName);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.maskedSubject = subject;
        emailOp.extensionName = extensionName;

        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
        assertEq(extensionHandler.userExtensionOfCommand(walletAddr, "Swap"), extensionAddr, "didnt install extension");
    }

    function test_InstallShouldOverrideDefaultExtensions() public {
        // Publish a new extension with the same command as default extension
        Extension ext = new TestExtension(address(core), address(daiToken), address(tokenRegistry));
        string[][] memory dummyTemplates = new string[][](1);
        dummyTemplates[0] = new string[](2);
        dummyTemplates[0][0] = "DEF_EXT"; // Same command of default extension installed in EmailWalletCoreTestHelper
        dummyTemplates[0][1] = "Not default";
        extensionHandler.publishExtension("Custom", address(ext), dummyTemplates, 0.1 ether);

        // Should be default extension
        assertEq(
            extensionHandler.getExtensionForCommand(walletAddr, "DEF_EXT"),
            defaultExtAddr,
            "defaultExtAddr not set"
        );

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.maskedSubject = "Install extension Custom";
        emailOp.extensionName = "Custom";

        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
        assertEq(extensionHandler.getExtensionForCommand(walletAddr, "DEF_EXT"), address(ext), "extension not changed");
    }

    function test_RevertIf_ExtensionNotRegistered() public {
        string memory subject = string.concat("Install extension ", extensionName);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.maskedSubject = subject;
        emailOp.extensionName = extensionName;

        vm.startPrank(relayer);
        vm.expectRevert("extension not registered");
        core.handleEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_UninstallCommand() public {
        _publishExtension();

        EmailOp memory emailOpInstall = _getBaseEmailOp();
        emailOpInstall.command = Commands.INSTALL_EXTENSION;
        emailOpInstall.maskedSubject = string.concat("Install extension ", extensionName);
        emailOpInstall.extensionName = extensionName;

        EmailOp memory emailOpUninstall = _getBaseEmailOp();
        emailOpUninstall.command = Commands.UNINSTALL_EXTENSION;
        emailOpUninstall.maskedSubject = string.concat("Uninstall extension ", extensionName);
        emailOpUninstall.extensionName = extensionName;
        emailOpUninstall.emailNullifier = bytes32(uint256(93845));

        vm.startPrank(relayer);
        core.handleEmailOp(emailOpInstall);
        (bool success, , , ) = core.handleEmailOp(emailOpUninstall);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
        assertEq(extensionHandler.getExtensionForCommand(walletAddr, "Swap"), address(0), "didnt uninstall extension");
    }

    function test_RevertIf_UnistallExtensionNotInstalled() public {
        _publishExtension();

        EmailOp memory emailOpUninstall = _getBaseEmailOp();
        emailOpUninstall.command = Commands.UNINSTALL_EXTENSION;
        emailOpUninstall.maskedSubject = string.concat("Uninstall extension ", extensionName);
        emailOpUninstall.extensionName = extensionName;

        vm.startPrank(relayer);
        vm.expectRevert("extension not installed");
        core.handleEmailOp(emailOpUninstall);
        vm.stopPrank();
    }
}
