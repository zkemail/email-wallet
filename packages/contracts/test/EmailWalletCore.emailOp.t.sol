// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/StdStorage.sol";
import "./helpers/EmailWalletCoreTestHelper.sol";
import "../src/libraries/SubjectUtils.sol";

// Generic EmailOp validations - command specific validations are in respective command test file
contract EmailOpValidationTest is EmailWalletCoreTestHelper {
    using stdStorage for StdStorage;

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _createTestAccount();
    }

    function test_RevertIf_EmailExpired() public {
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

    function test_RevertIf_DomainUnsupported() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.emailDomain = "random.com";
        emailOp.command = "Send 1 ETH to someone@sample.com";

        vm.startPrank(relayer);
        vm.expectRevert("invalid DKIM public key");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_SenderIsNotRelayer() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();

        vm.startPrank(vm.addr(5)); // Random addr
        // relayerHandler.registerRelayer("r2@rel.com", "rel.com");
        vm.expectRevert("relayer not registered");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    // function test_RevertIf_AccountIsNotInitialized() public {
    //     daiToken.freeMint(walletAddr, 1 ether);

    //     bytes32 emailAddrPointer = bytes32(uint256(37465));
    //     bytes32 accountKeyCommit = bytes32(uint256(3434));
    //     bytes32 walletSalt = bytes32(uint256(3434));
    //     bytes memory psiPoint = abi.encodePacked(uint256(3434));

    //     EmailOp memory emailOp = _getTokenSendingEmailOp();
    //     emailOp.emailAddrPointer = emailAddrPointer;

    //     vm.startPrank(relayer);
    //     accountHandler.createAccount(emailAddrPointer, walletSalt, psiPoint, mockProof);
    //     vm.expectRevert("account not initialized");
    //     core.validateEmailOp(emailOp);
    //     vm.stopPrank();
    // }

    function test_RevertIf_NullifierIsUsed() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.emailNullifier = emailNullifier; // This nullifier already used for account initialization

        vm.startPrank(relayer);
        vm.expectRevert("email nullified");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_FeeTokenIsNotSupported() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.feeTokenName = "UNI";
        emailOp.command = "Send";

        vm.startPrank(relayer);
        vm.expectRevert("unsupported fee token");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_FeePerGasIsHigherThanMax() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.feePerGas = 10 gwei; // maxFeePerGas is 5 gwei in EmailWalletCoreTestHelper
        emailOp.command = "Send";

        vm.startPrank(relayer);
        vm.expectRevert("fee per gas too high");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_BothRecipientTypeExist() public {
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

    function test_RevertIf_RecipientCommittmentNotFound() public {
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = bytes32(0);

        vm.startPrank(relayer);
        vm.expectRevert("recipientEmailAddrCommit not found");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_RecipientCommittmentFound_ForSubjectWithoutEmailAddr() public {
        address recipient = vm.addr(5);
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.hasEmailRecipient = false;
        emailOp.recipientEmailAddrCommit = bytes32(uint256(123));
        emailOp.maskedSubject = string.concat("Send 1 DAI to ", SubjectUtils.addressToChecksumHexString(recipient));

        vm.startPrank(relayer);
        vm.expectRevert("recipientEmailAddrCommit not allowed");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_ProofIsNotValid() public {
        bytes memory proof = abi.encodePacked(bytes1(0x02));
        daiToken.freeMint(walletAddr, 1 ether);

        EmailOp memory emailOp = _getTokenSendingEmailOp();
        emailOp.emailProof = proof; // this proof is not valid as per TestVerifier.sol

        vm.startPrank(relayer);
        vm.expectRevert("invalid email proof");
        core.validateEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_ShouldReturnFeeIfUnclaimsNotRegistered() public {
        address recipient = vm.addr(5);
        string memory subject = string.concat("Send 100 DAI to ", SubjectUtils.addressToChecksumHexString(recipient));

        daiToken.freeMint(walletAddr, 150 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 100 ether;
        emailOp.recipientETHAddr = recipient;
        emailOp.maskedSubject = subject;

        vm.deal(relayer, 1 ether);

        vm.startPrank(relayer);
        // Send 1 ETH to handleEmail
        (bool success, , , ) = core.handleEmailOp{value: 1 ether}(emailOp);
        vm.stopPrank();

        assertTrue(success, "emailOp failed");

        assertEq(relayer.balance, 1 ether, "relayer didnt receive unused fee");
        assertEq(address(core).balance, 0, "core balance should be 0");
    }

    function test_RelayerGasReimbursement_WhenUserPaysInETH() public {
        address recipient = vm.addr(5);
        string memory subject = string.concat("Send 100 DAI to ", SubjectUtils.addressToChecksumHexString(recipient));

        daiToken.freeMint(walletAddr, 150 ether);

        // Mint 100 weth to walletAddr - will use it to pay fee
        vm.startPrank(walletAddr);
        vm.deal(walletAddr, 100 ether);
        weth.deposit{value: 100 ether}();
        vm.stopPrank();

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 100 ether;
        emailOp.recipientETHAddr = recipient;
        emailOp.maskedSubject = subject;
        emailOp.feeTokenName = "ETH"; // User will pay in WETH
        emailOp.feePerGas = maxFeePerGas;

        vm.startPrank(relayer);
        (bool success, , uint256 totalFee, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        uint256 expectedReimbursement = totalFee; // ETH = WETH

        assertTrue(success, "emailOp failed");
        assertEq(weth.balanceOf(relayer), expectedReimbursement, "relayer didnt receive reimbursement");
        assertEq(weth.balanceOf(walletAddr), 100 ether - expectedReimbursement, "wallet didnt send weth");
    }

    function test_RelayerGasReimbursement_WhenUserPaysInToken() public {
        address recipient = vm.addr(5);
        string memory subject = string.concat("Send 100 DAI to ", SubjectUtils.addressToChecksumHexString(recipient));

        daiToken.freeMint(walletAddr, 150 ether);
        usdcToken.freeMint(walletAddr, 50 ether); // For gas  reimbursement

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 100 ether;
        emailOp.recipientETHAddr = recipient;
        emailOp.maskedSubject = subject;
        emailOp.feeTokenName = "USDC";
        emailOp.feePerGas = maxFeePerGas;

        vm.startPrank(relayer);
        (bool success, , uint256 totalFee, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        uint256 expectedReimbursement = totalFee * 1500; // 1 ETH = 1500 USDC set in TestOracle.sol

        assertTrue(success, "emailOp failed");
        assertEq(usdcToken.balanceOf(relayer), expectedReimbursement, "relayer didnt receive reimbursement");
        assertEq(usdcToken.balanceOf(walletAddr), 50 ether - expectedReimbursement, "wallet didnt send weth");
    }

    function test_RelayerGasReimbursement_WithCustomFeePerGas() public {
        address recipient = vm.addr(5);
        string memory subject = string.concat("Send 100 DAI to ", SubjectUtils.addressToChecksumHexString(recipient));
        uint256 feePerGas = 3 gwei;

        daiToken.freeMint(walletAddr, 150 ether);
        usdcToken.freeMint(walletAddr, 50 ether); // For gas  reimbursement

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 100 ether;
        emailOp.recipientETHAddr = recipient;
        emailOp.maskedSubject = subject;
        emailOp.feeTokenName = "USDC";
        emailOp.feePerGas = feePerGas; // Custom fee per gas < maxFeePerGas

        vm.startPrank(relayer);
        (bool success, , uint256 totalFee, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        uint256 expectedReimbursement = totalFee * 1500; // 1 ETH = 1500 USDC set in TestOracle.sol

        assertTrue(success, "emailOp failed");
        assertEq(usdcToken.balanceOf(relayer), expectedReimbursement, "relayer didnt receive reimbursement");
        assertEq(usdcToken.balanceOf(walletAddr), 50 ether - expectedReimbursement, "wallet didnt send weth");
    }

    function test_RelayerGasReimbursement_WithUnclaimedFunds() public {
        string memory subject = string.concat("Send 100 DAI to ");

        daiToken.freeMint(walletAddr, 150 ether);
        usdcToken.freeMint(walletAddr, 50 ether); // For gas  reimbursement

        // Relayer need ETH to pay gas cost
        vm.deal(relayer, 1 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 100 ether;
        emailOp.recipientEmailAddrCommit = bytes32(uint256(552323));
        emailOp.hasEmailRecipient = true;
        emailOp.maskedSubject = subject;
        emailOp.feeTokenName = "USDC";
        emailOp.feePerGas = maxFeePerGas;

        uint256 unclaimFee = unclaimedFundClaimGas * maxFeePerGas;

        vm.startPrank(relayer);
        (bool success, , uint256 totalFee, ) = core.handleEmailOp{value: unclaimFee}(emailOp);
        vm.stopPrank();

        uint256 expectedReimbursement = totalFee * 1500; // 1 ETH = 1500 USDC set in TestOracle.sol

        assertTrue(success, "emailOp failed");
        assertTrue(totalFee > unclaimFee, "totalFee should be greater than unclaimFee");
        assertEq(usdcToken.balanceOf(relayer), expectedReimbursement, "relayer didnt receive reimbursement");
        assertEq(usdcToken.balanceOf(walletAddr), 50 ether - expectedReimbursement, "wallet didnt send weth");
    }

    function test_RelayerShouldGetGasReimbursement_EvenIfExecutionFails() public {
        usdcToken.freeMint(walletAddr, 50 ether); // For gas  reimbursement

        // Calling an invalid function
        bytes memory targetCalldata = abi.encodeWithSignature("invalid(uint256)", 90001);
        bytes memory emailOpCalldata = abi.encode(address(this), 0, targetCalldata); // target = testRunner

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.EXECUTE;
        emailOp.executeCallData = emailOpCalldata;
        emailOp.maskedSubject = string.concat("Execute 0x", SubjectUtils.bytesToHexString(emailOpCalldata));

        // Should not revert, but return false as this is not a validation error
        vm.startPrank(relayer);
        (bool success, , uint256 totalFee, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(!success, "handleEmailOp succeded"); // Execution fails

        uint256 expectedReimbursement = totalFee * 1500; // 1 ETH = 1500 USDC set in TestOracle.sol

        assertEq(usdcToken.balanceOf(relayer), expectedReimbursement, "relayer didnt receive reimbursement");
        assertEq(usdcToken.balanceOf(walletAddr), 50 ether - expectedReimbursement, "wallet didnt send weth");
    }

    function test_RevertIf_RelayerGasReimbursementFails() public {
        address recipient = vm.addr(5);
        string memory subject = string.concat("Send 100 DAI to ", SubjectUtils.addressToChecksumHexString(recipient));

        daiToken.freeMint(walletAddr, 150 ether);
        usdcToken.freeMint(walletAddr, 100 wei); // This is not enough for gas reimbursement

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 100 ether;
        emailOp.recipientETHAddr = recipient;
        emailOp.maskedSubject = subject;
        emailOp.feeTokenName = "USDC";
        emailOp.feePerGas = maxFeePerGas;

        vm.startPrank(relayer);
        vm.expectRevert("fee reimbursement failed: ERC20: transfer amount exceeds balance");
        core.handleEmailOp(emailOp);
        vm.stopPrank();
    }
}
