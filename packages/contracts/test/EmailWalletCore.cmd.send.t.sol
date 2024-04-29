// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

contract TransferTest is EmailWalletCoreTestHelper {
    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _createTestAccount();
    }

    function test_ValidateSendingToEthAddress() public {
        address recipient = vm.addr(5);
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Send";
        emailOp.maskedSubject = string.concat("Send 1 DAI to ", SubjectUtils.addressToChecksumHexString(recipient));
        emailOp.recipientETHAddr = recipient;
        emailOp.walletParams.amount = 1 ether;
        emailOp.walletParams.tokenName = "DAI";

        vm.startPrank(relayer);
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function testFuzz_ValidateSendingToEthAddress(address recipient) public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Send";
        emailOp.maskedSubject = string.concat("Send 1 DAI to ", SubjectUtils.addressToChecksumHexString(recipient));
        emailOp.recipientETHAddr = recipient;
        emailOp.walletParams.amount = 1 ether;
        emailOp.walletParams.tokenName = "DAI";

        vm.startPrank(relayer);
        vm.assume(recipient != address(0));
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    // We only support addres in checksum format in the subject (not all lowercase)
    function test_Revert_SendingToEthAddress_WithNonChecksumAddress() public {
        address recipient = vm.addr(5);
        daiToken.freeMint(walletAddr, 1 ether);
        // vm.addr(5) in lowecase = 0xe1ab8145f7e55dc933d51a18c793f901a3a0b276
        string memory subject = string.concat("Send 1 DAI to 0xe1ab8145f7e55dc933d51a18c793f901a3a0b276");

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Send";
        emailOp.maskedSubject = subject;
        emailOp.recipientETHAddr = recipient;
        emailOp.walletParams.amount = 1 ether;
        emailOp.walletParams.tokenName = "DAI";

        vm.startPrank(relayer);
        vm.expectRevert("subject != Send 1 DAI to 0xe1AB8145F7E55DC933d51a18c793F901A3A0b276");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_ValidateSendingToEmail() public {
        daiToken.freeMint(walletAddr, 2 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Send";
        emailOp.maskedSubject = "Send 2 DAI to ";
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = bytes32(uint256(123));
        emailOp.walletParams.amount = 2 ether;
        emailOp.walletParams.tokenName = "DAI";

        vm.startPrank(relayer);
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_TokenNameIsNotSupported() public {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Send";
        emailOp.maskedSubject = "Send 2 JUNK to ";
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = bytes32(uint256(123));
        emailOp.walletParams.amount = 2 ether;
        emailOp.walletParams.tokenName = "JUNK";

        vm.startPrank(relayer);
        vm.expectRevert("token not supported");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_BalanceIsInsufficient() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Send";
        emailOp.maskedSubject = "Send 2 DAI to ";
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = bytes32(uint256(123));
        emailOp.walletParams.amount = 2 ether;
        emailOp.walletParams.tokenName = "DAI";

        vm.startPrank(relayer);
        vm.expectRevert("insufficient balance");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_ValidateForSendingDecimalAmounts() public {
        daiToken.freeMint(walletAddr, 2 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Send";
        emailOp.maskedSubject = "Send 1.5 DAI to ";
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = bytes32(uint256(123));
        emailOp.walletParams.amount = 1.5 ether;
        emailOp.walletParams.tokenName = "DAI";

        vm.startPrank(relayer);
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_SendTokenToEOA() public {
        address recipient = vm.addr(5);
        string memory subject = string.concat("Send 100 DAI to ", SubjectUtils.addressToChecksumHexString(recipient));

        // Mint 150 DAI to sender wallet (will send 100 DAI to recipient)
        daiToken.freeMint(walletAddr, 150 ether);

        // Create EmailOp
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 100 ether;
        emailOp.recipientETHAddr = recipient;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertEq(success, true, "handleEmailOp failed");
        assertEq(daiToken.balanceOf(recipient), 100 ether, "recipient did not receive 100 DAI");
        assertEq(daiToken.balanceOf(walletAddr), 50 ether, "sender did not have 50 DAI left");
    }

    function test_SendTokenToEOA_WithDecimals() public {
        address recipient = vm.addr(5);
        string memory subject = string.concat("Send 10.52 DAI to ", SubjectUtils.addressToChecksumHexString(recipient));

        daiToken.freeMint(walletAddr, 20 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 10.52 ether;
        emailOp.recipientETHAddr = recipient;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertEq(success, true, "handleEmailOp failed");
        assertEq(daiToken.balanceOf(recipient), 10.52 ether, "recipient did not receive 10.52 DAI");
        assertEq(daiToken.balanceOf(walletAddr), 9.48 ether, "sender did not have 9.48 DAI left");
    }

    function test_SendTokenToEmail() public {
        string memory subject = "Send 65.4 DAI to ";
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        // this need to be send to handleEmailOp for registering unclaimed funds
        vm.deal(relayer, unclaimedFundClaimGas * maxFeePerGas);

        // Mint 150 DAI to sender wallet (will send 100 DAI to recipient)
        daiToken.freeMint(walletAddr, 100 ether);

        usdcToken.freeMint(walletAddr, 100 ether); // for gas reimbursemt of unclaimed funds

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 65.4 ether;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.maskedSubject = subject;
        emailOp.feeTokenName = "USDC";

        vm.startPrank(relayer);
        (bool success, , , uint256 registeredUnclaimId) = core.handleEmailOp{
            value: unclaimedFundClaimGas * maxFeePerGas
        }(emailOp);
        vm.stopPrank();

        assertEq(success, true, "handleEmailOp failed");
        assertEq(daiToken.balanceOf(walletAddr), 34.6 ether, "sender did not have correct DAI left");

        // Should register unclaimed funds
        (, , , address tokenAddr, uint256 amount, ) = unclaimsHandler.unclaimedFundOfId(registeredUnclaimId);
        assertEq(tokenAddr, address(daiToken), "tokenName mismatch");
        assertEq(amount, 65.4 ether, "amount mismatch");
    }

    function test_SendTokenToEmailWithSubjectPrefix() public {
        string memory subject = "Re: Send 65.4 DAI to ";
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        // this need to be send to handleEmailOp for registering unclaimed funds
        vm.deal(relayer, unclaimedFundClaimGas * maxFeePerGas);

        // Mint 150 DAI to sender wallet (will send 100 DAI to recipient)
        daiToken.freeMint(walletAddr, 100 ether);

        usdcToken.freeMint(walletAddr, 100 ether); // for gas reimbursemt of unclaimed funds

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 65.4 ether;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.maskedSubject = subject;
        emailOp.feeTokenName = "USDC";
        emailOp.skipSubjectPrefix = 4;

        vm.startPrank(relayer);
        (bool success, , , uint256 registeredUnclaimId) = core.handleEmailOp{
            value: unclaimedFundClaimGas * maxFeePerGas
        }(emailOp);
        vm.stopPrank();

        assertEq(success, true, "handleEmailOp failed");
        assertEq(daiToken.balanceOf(walletAddr), 34.6 ether, "sender did not have correct DAI left");

        // Should register unclaimed funds
        (, , , address tokenAddr, uint256 amount, ) = unclaimsHandler.unclaimedFundOfId(registeredUnclaimId);
        assertEq(tokenAddr, address(daiToken), "tokenName mismatch");
        assertEq(amount, 65.4 ether, "amount mismatch");
    }

    function test_RevertIf_PrefixIsWrong() public {
        string memory subject = "Re: Send 65.4 DAI to ";
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        // this need to be send to handleEmailOp for registering unclaimed funds
        vm.deal(relayer, unclaimedFundClaimGas * maxFeePerGas);

        // Mint 150 DAI to sender wallet (will send 100 DAI to recipient)
        daiToken.freeMint(walletAddr, 100 ether);

        usdcToken.freeMint(walletAddr, 100 ether); // for gas reimbursemt of unclaimed funds

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 65.4 ether;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.maskedSubject = subject;
        emailOp.feeTokenName = "USDC";
        emailOp.skipSubjectPrefix = 3;

        vm.startPrank(relayer);
        vm.expectRevert("subject != Send 65.4 DAI to ");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_ConvertWethToEthOnExternalTransfer() public {
        address recipient = vm.addr(5);
        string memory subject = string.concat("Send 50 ETH to ", SubjectUtils.addressToChecksumHexString(recipient));

        // Mint 100 WETH to sender wallet; we only send 50
        vm.deal(walletAddr, 100 ether);
        vm.startPrank(walletAddr);
        weth.deposit{value: 100 ether}();
        vm.stopPrank();

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "ETH";
        emailOp.walletParams.amount = 50 ether;
        emailOp.recipientETHAddr = recipient;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        (bool success, bytes memory reason, , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertEq(success, true, string(reason));
        assertEq(recipient.balance, 50 ether, "recipient did not receive 50 ETH");
        assertEq(weth.balanceOf(walletAddr), 50 ether, "sender did not have 50 WETH left");
        assertEq(weth.balanceOf(recipient), 0 ether, "recipient received weth");
    }
}
