// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./EmailWalletCoreTestHelper.sol";

contract ExecuteTestContract {
    function process(uint256 num) public returns (uint256) {
        return num + 1;
    }
}

contract ExecuteCommandTest is EmailWalletCoreTestHelper {
    address testContractAddr;

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _registerAndInitializeAccount();

        testContractAddr = address(new ExecuteTestContract());
    }

    function test_Execute() public {
        address recipient = vm.addr(5);

        bytes memory erc20Calldata = abi.encodeWithSignature("process(uint256)", 90001);
        bytes memory emailOpCalldata = abi.encode(testContractAddr, 0, erc20Calldata);

        string memory subject = string.concat("Execute 0x", BytesUtils.bytesToHexString(emailOpCalldata));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        (bool success, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
    }

    function test_RevertIf_ExecuteTargetIsWallet() public {
        address recipient = vm.addr(5);

        bytes memory emailOpCalldata = abi.encode(walletAddr, 0, "");
        string memory subject = string.concat("Execute 0x", BytesUtils.bytesToHexString(emailOpCalldata));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        vm.expectRevert("cannot execute on wallet");
        (bool success, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
    }

    function test_RevertIf_ExecuteTargetIsCore() public {
        address recipient = vm.addr(5);

        bytes memory emailOpCalldata = abi.encode(address(core), 0, "");
        string memory subject = string.concat("Execute 0x", BytesUtils.bytesToHexString(emailOpCalldata));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        vm.expectRevert("cannot execute on core");
        (bool success, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
    }

    function test_RevertIf_ExecuteTargetIsToken() public {
        address recipient = vm.addr(5);

        bytes memory emailOpCalldata = abi.encode(address(daiToken), 0, abi.encodeWithSignature("transfer(uint256)", 1 ether));
        string memory subject = string.concat("Execute 0x", BytesUtils.bytesToHexString(emailOpCalldata));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        vm.expectRevert("cannot execute on token");
        (bool success, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
    }
}
