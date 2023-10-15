// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

// Tests for extension publishing
// Tests for installing and uninstalling extension are in EmailWalletCore.cmd.install.t.sol
// Tests for EmailOp with extension are in EmailWalletCore.cmd.extension.t.sol
// Tests for Unclaimed States are in EmailWalletCore.us.sol
contract ExtensionTest is EmailWalletCoreTestHelper {
    Extension testExtension;
    address testExtensionAddr;
    string[][] subjectTemplates;

    function setUp() public override {
        super.setUp();
        testExtension = new TestExtension();
        testExtensionAddr = address(testExtension);
    }

    function _getSampleSubjectTemplates() internal returns (string[][] memory) {
        delete subjectTemplates;
        subjectTemplates = new string[][](2);
        subjectTemplates[0] = ["Swap", "{tokenAmount}", "to", "{string}"];
        subjectTemplates[1] = ["Swap", "all", "{tokenAmount}", "to", "{string}"];
        return subjectTemplates;
    }

    function test_PublishExtension() public {
        address extensionDev = vm.addr(3);
        string memory extensionName = "testSwap";
        uint256 maxExecutionGas = 0.1 ether;
        string[][] memory subjectTemplates = _getSampleSubjectTemplates();

        vm.startPrank(extensionDev);
        vm.expectEmit();
        emit ExtensionPublished(extensionName, testExtensionAddr, subjectTemplates, maxExecutionGas);
        core.publishExtension(extensionName, testExtensionAddr, subjectTemplates, maxExecutionGas);
        vm.stopPrank();

        assertEq(core.addressOfExtension(extensionName), testExtensionAddr);
        assertEq(core.maxGasOfExtension(testExtensionAddr), maxExecutionGas);

        for (uint8 i = 0; i < 2; i++) {
            for (uint8 j = 0; j < subjectTemplates[i].length; j++) {
                assertEq(
                    core.subjectTemplatesOfExtension(testExtensionAddr, i, j),
                    subjectTemplates[i][j],
                    "subject mismatch"
                );
            }
        }
    }

    function test_RevertIf_ExtensionNameAlreadyUsed() public {
        address extensionDev = vm.addr(3);
        string memory extensionName = "testSwap";
        string[][] memory subjectTemplates = _getSampleSubjectTemplates();

        TestExtension testExtension2 = new TestExtension();

        vm.startPrank(extensionDev);
        core.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
        vm.expectRevert("extension name already used");
        core.publishExtension(extensionName, address(testExtension2), subjectTemplates, 1 ether);
        vm.stopPrank();
    }

    function test_RevertIf_ExtensionAddressAlreadyUsed() public {
        address extensionDev = vm.addr(3);
        string memory extensionName = "testSwap";
        string[][] memory subjectTemplates = _getSampleSubjectTemplates();

        vm.startPrank(extensionDev);
        core.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
        vm.expectRevert("extension already published");
        core.publishExtension("testSwap2", testExtensionAddr, subjectTemplates, 1 ether);
        vm.stopPrank();
    }

    // Command mean first word of the extension
    function test_RevertIf_TemplatesDontUseSameCommand() public {
        address extensionDev = vm.addr(3);
        string memory extensionName = "testSwap";
        string[][] memory subjectTemplates = _getSampleSubjectTemplates();
        subjectTemplates[1][0] = "Exchange"; // Alter one of the sample's first word

        vm.startPrank(extensionDev);
        vm.expectRevert("subjectTemplates must have same command");
        core.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
        vm.stopPrank();
    }

    // First item of the template array is considered as command, and should be word (cannot contain space)
    function test_RevertIf_CommandIsNotOneWord() public {
        address extensionDev = vm.addr(3);
        string memory extensionName = "testSwap";
        string[][] memory subjectTemplates = _getSampleSubjectTemplates();

        // Alter both sample template and use two words as command
        subjectTemplates[0][0] = "Exchange all";
        subjectTemplates[1][0] = "Exchange all";

        vm.startPrank(extensionDev);
        vm.expectRevert("command should be one word");
        core.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
        vm.stopPrank();
    }

    function test_RevertIf_CommandIsReserved() public {
        address extensionDev = vm.addr(3);
        string memory extensionName = "testSwap";
        string[][] memory subjectTemplates = _getSampleSubjectTemplates();

        // Install is a reserverd command for installing extension
        subjectTemplates[0][0] = "Install";
        subjectTemplates[1][0] = "Install";

        vm.startPrank(extensionDev);
        vm.expectRevert("command cannot be a reserved name");
        core.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
        vm.stopPrank();
    }

    function test_RevertIf_CommandIsTemplateMatcher() public {
        address extensionDev = vm.addr(3);
        string memory extensionName = "testSwap";
        string[][] memory subjectTemplates = _getSampleSubjectTemplates();

        subjectTemplates[0][0] = "{tokenAmount}";
        subjectTemplates[1][0] = "{tokenAmount}";

        vm.startPrank(extensionDev);
        vm.expectRevert("command cannot be a template matcher");
        core.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
        vm.stopPrank();
    }
}
