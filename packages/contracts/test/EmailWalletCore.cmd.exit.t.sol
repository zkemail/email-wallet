// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

contract ExitCommandTest is EmailWalletCoreTestHelper {
    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _createTestAccount();
    }

    function test_ExitAndTransferOwnership() public {
        address newOwner = vm.addr(5);
        string memory subject = string.concat(
            "Exit Email Wallet. Change ownership to ",
            SubjectUtils.addressToChecksumHexString(newOwner)
        );
        Wallet wallet = Wallet(payable(walletAddr));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXIT_EMAIL_WALLET;
        emailOp.newWalletOwner = newOwner;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
        assertEq(wallet.owner(), newOwner, "wallet owner not changed");

        // Core contract should not be able to do any operations now (testing a Send EmailOp)
        daiToken.freeMint(walletAddr, 150 ether);
        address recipient = vm.addr(7);

        EmailOp memory sendEmailOp = _getBaseEmailOp();
        string memory sendSubject = string.concat(
            "Send 100 DAI to ",
            SubjectUtils.addressToChecksumHexString(recipient)
        );
        sendEmailOp.command = Commands.SEND;
        sendEmailOp.walletParams.tokenName = "DAI";
        sendEmailOp.walletParams.amount = 100 ether;
        sendEmailOp.recipientETHAddr = recipient;
        sendEmailOp.maskedSubject = sendSubject;
        sendEmailOp.emailNullifier = bytes32(uint(9853094850));

        vm.startPrank(relayer);
        (bool sendSuccess, bytes memory errData, , ) = core.handleEmailOp(sendEmailOp);
        vm.stopPrank();

        assertFalse(sendSuccess, "handleEmailOp succeeded after exit");
        assertTrue(Strings.equal(string(errData), "only owner or self"), "wrong error message");
        assertEq(daiToken.balanceOf(walletAddr), 150 ether, "EmailOp changed sender balance");
        assertEq(daiToken.balanceOf(recipient), 0 ether, "EmailOp changed recipient balance");

        // New owner should be able to execute (testing same token send)
        vm.startPrank(newOwner);
        wallet.execute(
            address(daiToken),
            0,
            abi.encodeWithSignature("transfer(address,uint256)", recipient, 100 ether)
        );
        vm.stopPrank();

        assertEq(daiToken.balanceOf(recipient), 100 ether, "recipient did not receive 100 DAI");
        assertEq(daiToken.balanceOf(walletAddr), 50 ether, "sender did not have 50 DAI left");
    }

    function test_ExitAndTransferOwnershipAlthoughAfterTimeLimit() public {
        vm.warp(1701388800);
        address newOwner = vm.addr(5);
        string memory subject = string.concat(
            "Exit Email Wallet. Change ownership to ",
            SubjectUtils.addressToChecksumHexString(newOwner)
        );
        Wallet wallet = Wallet(payable(walletAddr));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXIT_EMAIL_WALLET;
        emailOp.newWalletOwner = newOwner;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");
        assertEq(wallet.owner(), newOwner, "wallet owner not changed");
    }
}
