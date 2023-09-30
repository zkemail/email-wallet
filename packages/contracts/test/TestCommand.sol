// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./EmailWalletCoreTestHelper.sol";

contract ExecuteCommandTest is EmailWalletCoreTestHelper {
    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _registerAndInitializeAccount();
    }

    function testExecute() public {
        // We will send 5 USDC to recipient by passing calldata
        // This could be done using the "Send" command, but we want to test the "Execute" command
        usdcToken.freeMint(walletAddr, 20 ether);
        address recipient = vm.addr(5);

        bytes memory erc20Calldata = abi.encodeWithSignature("transfer(address,uint256)", recipient, 5 ether);
        bytes memory emailOpCalldata = abi.encode(address(usdcToken), 0, erc20Calldata);

        string memory subject = string.concat("Execute 0x", core.bytesToHexString(emailOpCalldata));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        (bool success, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertEq(success, true, "handleEmailOp failed");
        assertEq(usdcToken.balanceOf(recipient), 5 ether, "recipient did not receive 5 USDC");
        assertEq(usdcToken.balanceOf(walletAddr), 15 ether, "sender did not have 15 USDC left");
    }
}
