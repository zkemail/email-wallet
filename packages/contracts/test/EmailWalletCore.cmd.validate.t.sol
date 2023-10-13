// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

// Generic EmailOp validations - command specific validations are in respective command test file
contract EmailOpValidationTest is EmailWalletCoreTestHelper {
    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _registerAndInitializeAccount();
    }

    function testRevertOnExpiredEmail() public {
        daiToken.freeMint(walletAddr, 1 ether);

        vm.warp(1641070800);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.timestamp = block.timestamp - 1 days; // Core contract accept only emails up to 1 hour
        emailOp.command = "Send 1 ETH to someone@sample.com";

        vm.startPrank(relayer);
        vm.expectRevert("email expired");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function testRevertForUnsupportedDomain() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.emailDomain = "random.com";
        emailOp.command = "Send 1 ETH to someone@sample.com";

        vm.startPrank(relayer);
        vm.expectRevert("cannot find DKIM for domain");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function testRevertIfSenderIsNotAccountRelayer() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();

        vm.startPrank(vm.addr(5)); // Random addr
        core.registerRelayer(bytes32(uint256(123)), "r2@rel.com", "rel.com");
        vm.expectRevert("invalid relayer");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function testRevertIfAccountIsNotInitialized() public {
        daiToken.freeMint(walletAddr, 1 ether);

        bytes32 emailAddrPointer = bytes32(uint256(37465));
        bytes32 accountKeyCommit = bytes32(uint256(3434));
        bytes32 walletSalt = bytes32(uint256(3434));
        bytes memory psiPoint = abi.encodePacked(uint256(3434));

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.emailAddrPointer = emailAddrPointer;

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        vm.expectRevert("account not initialized");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function testRevertIfNullifierIsUsed() public {
        daiToken.freeMint(walletAddr, 1 ether);
        bytes32 nullifier = bytes32(uint256(123));

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.emailNullifier = nullifier; // This nullifier already used for account initialization

        vm.startPrank(relayer);
        vm.expectRevert("email nullifier already used");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function testRevertIfFeeTokenIsNotSupported() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.feeTokenName = "UNI";
        emailOp.command = "Send";

        vm.startPrank(relayer);
        vm.expectRevert("unsupported fee token");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function testRevertIfFeePerGasIsHigherThanMax() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.feePerGas = 10 ** 11; // We used 10 ** 10 as maxFeePerGas when deploying core
        emailOp.command = "Send";

        vm.startPrank(relayer);
        vm.expectRevert("fee per gas too high");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function testRevertIfRecipientCommittmentNotFoundWhenSubjectHasEmailRecipient() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = bytes32(0);

        vm.startPrank(relayer);
        vm.expectRevert("recipientEmailAddrCommit not found");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function testRevertCannotHaveBothRecipientTypes() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.hasEmailRecipient = true;
        emailOp.recipientETHAddr = vm.addr(5);
        emailOp.recipientEmailAddrCommit = bytes32(uint256(123));

        vm.startPrank(relayer);
        vm.expectRevert("cannot have both recipient types");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function testRevertIfEmailCommitmentIsPresentWhenSubjectDontHaveEmail() public {
        address recipient = vm.addr(5);
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.hasEmailRecipient = false;
        emailOp.recipientEmailAddrCommit = bytes32(uint256(123));
        emailOp.maskedSubject = string.concat("Send 1 DAI to ", Strings.toHexString(uint256(uint160(recipient)), 20));

        vm.startPrank(relayer);
        vm.expectRevert("recipientEmailAddrCommit not allowed");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function testRevertIfProofIsNotValid() public {
        bytes memory proof = abi.encodePacked(bytes1(0x02));
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.emailProof = proof; // this proof is not valid as per TestVerifier.sol

        vm.startPrank(relayer);
        vm.expectRevert("invalid email proof");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }
}
