// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

contract InstallExtensionCommandTest is EmailWalletCoreTestHelper {
    address testExtensionAddr;
    string[][] tempaltes = new string[][](1);
    string extensionName = "TestSwap";

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _registerAndInitializeAccount();
    }

    function _publishExtension() internal {
        address extensionDev = vm.addr(3);

        Extension ext = new TestExtension();
        testExtensionAddr = address(ext);

        string[4] memory temp = ["Swap", "{tokenAmount}", "to", "{string}"];
        tempaltes[0] = temp;

        vm.startPrank(extensionDev);
        core.publishExtension( extensionName, testExtensionAddr, tempaltes, 0.1 ether);
        vm.stopPrank();
    }

    function test_InstallCommand() public {
        _publishExtension();

        string memory subject = string.concat("Install extension ", extensionName);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.maskedSubject = subject;
        emailOp.extManagerParams.extensionName = extensionName;

        vm.startPrank(relayer);
        (bool success, bytes memory reason) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
        assertEq(core.userExtensionOfCommand(walletAddr, "Swap"), testExtensionAddr, "didnt install extension");
    }

    function test_RevertIf_ExtensionNotRegistered() public {
        string memory subject = string.concat("Install extension ", extensionName);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.maskedSubject = subject;
        emailOp.extManagerParams.extensionName = extensionName;

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
        emailOpInstall.extManagerParams.extensionName = extensionName;

        EmailOp memory emailOpUninstall = _getBaseEmailOp();
        emailOpUninstall.command = Commands.UNINSTALL_EXTENSION;
        emailOpUninstall.maskedSubject = string.concat("Uninstall extension ", extensionName);
        emailOpUninstall.extManagerParams.extensionName = extensionName;
        emailOpUninstall.emailNullifier = bytes32(uint256(93845));

        vm.startPrank(relayer);
        core.handleEmailOp(emailOpInstall);
        (bool success, ) = core.handleEmailOp(emailOpUninstall);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
        assertEq(core.userExtensionOfCommand(walletAddr, "Swap"), address(0), "didnt uninstall extension");
    }

    function test_RevertIf_UnistallExtensionNotInstalled() public {
        _publishExtension();

        EmailOp memory emailOpUninstall = _getBaseEmailOp();
        emailOpUninstall.command = Commands.UNINSTALL_EXTENSION;
        emailOpUninstall.maskedSubject = string.concat("Uninstall extension ", extensionName);
        emailOpUninstall.extManagerParams.extensionName = extensionName;

        vm.startPrank(relayer);
        vm.expectRevert("extension not installed");
        core.handleEmailOp(emailOpUninstall);
        vm.stopPrank();
    }


}
