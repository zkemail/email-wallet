// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

contract InstallExtensionCommandTest is EmailWalletCoreTestHelper {
    string[][] tempaltes = new string[][](1);

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _registerAndInitializeAccount();
    }

    function _publishExtension() internal {
        Extension ext = new TextExtension();

        string[4] memory temp = ["Swap", "{tokenAmount}", "to", "{string}"];
        tempaltes[0] = temp;

        vm.startPrank(relayer);
        core.publishExtension(address(ext), "TestExtension", tempaltes, 1000000);
        vm.stopPrank();
    }

    function test_InstallCommand() public {
        _publishExtension();

        string memory subject = "Install extension for Swap as TestExtension";

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.extManagerParams.command = "Swap";
        emailOp.extManagerParams.extensionName = "TestExtension";

        vm.startPrank(relayer);
        (bool success, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
    }
}
