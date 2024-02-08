// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";
import {TestExtension} from "./mocks/TestExtension.sol";

// Tests for extension publishing
// Tests for installing and uninstalling extension are in EmailWalletCore.cmd.install.t.sol
// Tests for EmailOp with extension are in EmailWalletCore.cmd.extension.t.sol
// Tests for Unclaimed States are in EmailWalletCore.us.sol
contract ExtensionTest is EmailWalletCoreTestHelper {
    Extension testExtension;
    address testExtensionAddr;

    function setUp() public override {
        super.setUp();
        testExtension = new TestExtension(address(core), address(daiToken), address(tokenRegistry));
        testExtensionAddr = address(testExtension);
    }

    string[][] _subjectTemplates;

    function _getSampleSubjectTemplates() internal returns (string[][] memory) {
        delete _subjectTemplates;
        _subjectTemplates = new string[][](2);
        _subjectTemplates[0] = ["Swap", "{tokenAmount}", "to", "{string}"];
        _subjectTemplates[1] = ["Swap", "all", "{tokenAmount}", "to", "{string}"];
        return _subjectTemplates;
    }

    function test_DefaultExtensions() public {
        // Default extension is already deployed in EmailWalletCoreTestHelper
        // Just testing it was deployed properly here
        address adddr = extensionHandler.defaultExtensionOfCommand("DEF_EXT");

        assertTrue(defaultExtAddr != address(0), "defaultExtAddr not set");
        assertEq(adddr, defaultExtAddr, "extension not set");
        assertEq(extensionHandler.addressOfExtensionName("DEF_EXT_NAME"), adddr, "ext name mismatch");
        assertEq(extensionHandler.maxGasOfExtension(adddr), 1 ether, "maxGas not set"); // set during core deployment
        assertEq(extensionHandler.subjectTemplatesOfExtension(adddr, 0, 1), "NOOP", "subject mismatch");

        // getExtensionForCommand method should return same for all wallet addr
        address randomAddr = vm.addr(3);
        address getExtensionAddr = extensionHandler.getExtensionForCommand(randomAddr, "DEF_EXT");

        assertEq(getExtensionAddr, adddr, "extension not set for all users");
    }

    function test_PublishExtension() public {
        address extensionDev = vm.addr(3);
        string memory extensionName = "testSwap";
        uint256 maxExecutionGas = 0.1 ether;
        string[][] memory subjectTemplates = _getSampleSubjectTemplates();

        vm.startPrank(extensionDev);
        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.ExtensionPublished(extensionName, testExtensionAddr, subjectTemplates, maxExecutionGas);
        extensionHandler.publishExtension(extensionName, testExtensionAddr, subjectTemplates, maxExecutionGas);
        vm.stopPrank();

        assertEq(extensionHandler.addressOfExtensionName(extensionName), testExtensionAddr);
        assertEq(extensionHandler.maxGasOfExtension(testExtensionAddr), maxExecutionGas);

        for (uint8 i = 0; i < 2; i++) {
            for (uint8 j = 0; j < subjectTemplates[i].length; j++) {
                assertEq(
                    extensionHandler.subjectTemplatesOfExtension(testExtensionAddr, i, j),
                    subjectTemplates[i][j],
                    "subject mismatch"
                );
            }
        }
    }

    // function testFail_PublishExtension() public {
    //     vm.warp(1701388800);
    //     address extensionDev = vm.addr(3);
    //     string memory extensionName = "testSwap";
    //     uint256 maxExecutionGas = 0.1 ether;
    //     string[][] memory subjectTemplates = _getSampleSubjectTemplates();

    //     vm.startPrank(extensionDev);
    //     vm.expectEmit(true, true, true, true);
    //     emit EmailWalletEvents.ExtensionPublished(extensionName, testExtensionAddr, subjectTemplates, maxExecutionGas);
    //     extensionHandler.publishExtension(extensionName, testExtensionAddr, subjectTemplates, maxExecutionGas);
    //     vm.stopPrank();
    // }

    function test_RevertIf_ExtensionNameAlreadyUsed() public {
        address extensionDev = vm.addr(3);
        string memory extensionName = "testSwap";
        string[][] memory subjectTemplates = _getSampleSubjectTemplates();

        TestExtension testExtension2 = new TestExtension(address(core), address(daiToken), address(tokenRegistry));

        vm.startPrank(extensionDev);
        extensionHandler.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
        vm.expectRevert("extension name already used");
        extensionHandler.publishExtension(extensionName, address(testExtension2), subjectTemplates, 1 ether);
        vm.stopPrank();
    }

    function test_RevertIf_ExtensionAddressAlreadyUsed() public {
        address extensionDev = vm.addr(3);
        string memory extensionName = "testSwap";
        string[][] memory subjectTemplates = _getSampleSubjectTemplates();

        vm.startPrank(extensionDev);
        extensionHandler.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
        vm.expectRevert("extension already published");
        extensionHandler.publishExtension("testSwap2", testExtensionAddr, subjectTemplates, 1 ether);
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
        extensionHandler.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
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
        extensionHandler.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
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
        extensionHandler.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
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
        extensionHandler.publishExtension(extensionName, testExtensionAddr, subjectTemplates, 1 ether);
        vm.stopPrank();
    }

    function testUpgradeability() public {
        ExtensionHandler implV2 = new ExtensionHandler();

        vm.startPrank(deployer);
        extensionHandler.upgradeTo(address(implV2));
        vm.stopPrank();
    }
}
