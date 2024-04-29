// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";
import "../src/libraries/SubjectUtils.sol";

contract ExecuteTestContract {
    function process(uint256 num) public pure returns (uint256) {
        return num + 1;
    }
}

contract ExecuteCommandTest is EmailWalletCoreTestHelper {
    address testContractAddr;

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _createTestAccount();

        testContractAddr = address(new ExecuteTestContract());
    }

    function test_ExecuteCommand() public {
        bytes memory targetCalldata = abi.encodeWithSignature("process(uint256)", 90001);
        bytes memory emailOpCalldata = abi.encode(testContractAddr, 0, targetCalldata);

        string memory subject = string.concat("Execute 0x", SubjectUtils.bytesToHexString(emailOpCalldata));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
    }

    // function testFail_ExecuteCommandAfterTimeLimit() public {
    //     vm.warp(1701388800);
    //     bytes memory targetCalldata = abi.encodeWithSignature("process(uint256)", 90001);
    //     bytes memory emailOpCalldata = abi.encode(testContractAddr, 0, targetCalldata);

    //     string memory subject = string.concat("Execute 0x", SubjectUtils.bytesToHexString(emailOpCalldata));

    //     EmailOp memory emailOp = _getBaseEmailOp();
    //     emailOp.command = Commands.EXECUTE;
    //     emailOp.executeCallData = emailOpCalldata;
    //     emailOp.maskedSubject = subject;

    //     vm.startPrank(relayer);
    //     core.handleEmailOp(emailOp);
    //     vm.stopPrank();
    // }


    function test_ExecuteFailureShouldNotRevert() public {
        // Calling an invalid function
        bytes memory targetCalldata = abi.encodeWithSignature("invalid(uint256)", 90001);
        bytes memory emailOpCalldata = abi.encode(testContractAddr, 0, targetCalldata);

        string memory subject = string.concat("Execute 0x", SubjectUtils.bytesToHexString(emailOpCalldata));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = subject;

        // Should not revert, but return false as this is not a validation error
        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(!success, "handleEmailOp succeded");
    }

    function test_RevertIf_ExecuteTargetIsWallet() public {
        bytes memory emailOpCalldata = abi.encode(walletAddr, 0, "");
        string memory subject = string.concat("Execute 0x", SubjectUtils.bytesToHexString(emailOpCalldata));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        vm.expectRevert("cannot execute on wallet");
        core.handleEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_ExecuteTargetIsCore() public {
        bytes memory emailOpCalldata = abi.encode(address(core), 0, "");
        string memory subject = string.concat("Execute 0x", SubjectUtils.bytesToHexString(emailOpCalldata));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        vm.expectRevert("cannot execute on core or handlers");
        core.handleEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_ExecuteTargetIsAHandler() public {
        bytes memory emailOpCalldata = abi.encode(address(accountHandler), 0, "");
        string memory subject = string.concat("Execute 0x", SubjectUtils.bytesToHexString(emailOpCalldata));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        vm.expectRevert("cannot execute on core or handlers");
        core.handleEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_ExecuteTargetIsToken() public {
        bytes memory emailOpCalldata = abi.encode(
            address(daiToken),
            0,
            abi.encodeWithSignature("transfer(uint256)", 1 ether)
        );
        string memory subject = string.concat("Execute 0x", SubjectUtils.bytesToHexString(emailOpCalldata));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        vm.expectRevert("cannot execute on token");
        core.handleEmailOp(emailOp);
        vm.stopPrank();
    }
}
