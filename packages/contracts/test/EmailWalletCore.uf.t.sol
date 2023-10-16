// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

contract UnclaimedFundTest is EmailWalletCoreTestHelper {
    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _registerAndInitializeAccount();
    }

    // Internally means that the unclaimed fund is registered by handleEmailOp (send tokent to email)
    function test_RegisterUnclaimedFunds_OnTokenTransfer() public {
        string memory subject = "Send 100 DAI to ";
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        // this need to be send to handleEmailOp for registering unclaimed funds
        vm.deal(relayer, unclaimedFundClaimGas * maxFeePerGas);

        // Mint 150 DAI to sender wallet (will send 100 DAI to recipient)
        daiToken.freeMint(walletAddr, 100 ether);
        usdcToken.freeMint(walletAddr, 50 ether); // for fee reimbursement

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 100 ether;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.maskedSubject = subject;
        emailOp.feeTokenName = "USDC";

        vm.startPrank(relayer);
        vm.expectEmit();
        emit UnclaimedFundRegistered(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            walletAddr, // walletAddr should be sender
            block.timestamp + unclaimedFundExpirationDuration, // default expiry
            0,
            ""
        );
        (bool success, ) = core.handleEmailOp{value: unclaimedFundClaimGas * maxFeePerGas}(emailOp);
        vm.stopPrank();

        assertEq(success, true, "handleEmailOp failed");

        (bytes32 emailAddrCommit, address sender, address tokenAddr, uint256 amount, uint256 expiryTime) = core
            .unclaimedFundOfEmailAddrCommit(recipientEmailAddrCommit);

        assertEq(emailAddrCommit, recipientEmailAddrCommit, "emailAddrCommit mismatch");
        assertEq(sender, walletAddr, "sender mismatch");
        assertEq(tokenAddr, address(daiToken), "tokenName mismatch");
        assertEq(amount, 100 ether, "amount mismatch");

        // Should use default expiry
        assertEq(expiryTime, block.timestamp + unclaimedFundExpirationDuration, "expiryTime mismatch");

        // Core contract should have the balance
        assertEq(daiToken.balanceOf(address(core)), 100 ether, "core contract didnt receive the tokens");
    }

    function test_RevertIf_RegisterUnclaimedFundsInternal_NotEnoughFee() public {
        string memory subject = "Send 100 DAI to ";
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        // this need to be send to handleEmailOp for registering unclaimed funds
        vm.deal(relayer, unclaimedFundClaimGas * maxFeePerGas);

        // Mint 150 DAI to sender wallet (will send 100 DAI to recipient)
        daiToken.freeMint(walletAddr, 100 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.SEND;
        emailOp.walletParams.tokenName = "DAI";
        emailOp.walletParams.amount = 100 ether;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        vm.expectRevert("incorrect ETH sent for unclaimed fund");
        core.handleEmailOp{value: 0}(emailOp);
        vm.stopPrank();
    }

    function test_RegisterUnclaimedFundsExternal() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether); // Add allowance to core so it can transfer tokens

        vm.expectEmit();
        emit UnclaimedFundRegistered(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            sender,
            block.timestamp + unclaimedFundExpirationDuration, // default expiry
            0,
            ""
        );
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        (bytes32 emailAddrCommit, address ufSender, address tokenAddr, uint256 amount, uint256 expiryTime) = core
            .unclaimedFundOfEmailAddrCommit(recipientEmailAddrCommit);

        assertEq(emailAddrCommit, recipientEmailAddrCommit, "emailAddrCommit mismatch");
        assertEq(ufSender, sender, "sender mismatch");
        assertEq(tokenAddr, address(daiToken), "tokenName mismatch");
        assertEq(amount, 100 ether, "amount mismatch");
        assertEq(expiryTime, block.timestamp + unclaimedFundExpirationDuration, "expiryTime mismatch");
        assertEq(daiToken.balanceOf(address(core)), 100 ether, "core contract didnt receive the tokens");
    }

    function test_RegisterUnclaimedFundsExternal_WithCustomExpiry() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        uint256 expiry = block.timestamp + 1 days;

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            expiry,
            0,
            ""
        );
        vm.stopPrank();

        (, , , , uint256 expiryTime) = core.unclaimedFundOfEmailAddrCommit(recipientEmailAddrCommit);

        assertEq(expiryTime, expiry, "expiryTime mismatch");
    }

    function test_RevertIf_RegisterUnclaimedFundsExternal_NotEnoughFee() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);

        vm.expectRevert("invalid unclaimed fund fee");
        core.registerUnclaimedFund{value: 0}(recipientEmailAddrCommit, address(daiToken), 100 ether, 0, 0, "");
        vm.stopPrank();
    }

    function test_RegisterUnclaimedFundsExternal_WithAnnouncement() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        uint256 commitmentRand = uint256(198273198237);
        string memory emailAddr = "recipient@test.com";

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        vm.expectEmit();
        emit UnclaimedFundRegistered(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            sender,
            block.timestamp + unclaimedFundExpirationDuration, // default expiry
            commitmentRand,
            emailAddr
        );
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            commitmentRand,
            emailAddr
        );
        vm.stopPrank();
    }

    function test_RevertIf_RegisteringAlreadyExpired() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        uint256 commitmentRand = uint256(198273198237);
        string memory emailAddr = "recipient@test.com";

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        vm.expectRevert("invalid expiry time");
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            1,
            commitmentRand,
            emailAddr
        );
        vm.stopPrank();
    }

    function test_RevertIf_RegisteringExistingCommitment() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        vm.deal(sender, 2 * unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);
        usdcToken.freeMint(sender, 50 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );

        // Register another with same commitment
        usdcToken.approve(address(core), 50 ether);
        vm.expectRevert("unclaimed fund exists");
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(usdcToken),
            50 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();
    }

    function test_ClaimUnclaimedFund_ToExistingAccount() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);

        // vm.expectEmit();
        // emit UnclaimedFundClaimed(recipientEmailAddrCommit, address(daiToken), 100 ether, walletAddr);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        // Relayer claim the unclaimed fund to account
        vm.startPrank(relayer);
        core.claimUnclaimedFund(recipientEmailAddrCommit, emailAddrPointer, mockProof);
        vm.stopPrank();

        assertEq(daiToken.balanceOf(walletAddr), 100 ether, "recipient didnt receive tokens");
        (, , , uint256 amt, ) = core.unclaimedFundOfEmailAddrCommit(recipientEmailAddrCommit);
        assertEq(amt, 0, "unclaimed fund not cleared");
    }

    function test_ClaimUnclaimedFund_ToNewlyCreatedAccount() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes32 newEmailAddrPointer = bytes32(uint256(2001));
        bytes32 newAccountKeyCommit = bytes32(uint256(2002));
        bytes32 newWalletSalt = bytes32(uint256(2003));
        bytes memory newPSIPoint = abi.encodePacked(uint256(2003));
        address relayer2 = vm.addr(3);

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        // New relayer should be able to create account and claim
        vm.startPrank(relayer2);
        core.registerRelayer(bytes32(uint256(980398)), "relayer3@test.com", "relayer3.com");
        core.createAccount(newEmailAddrPointer, newAccountKeyCommit, newWalletSalt, newPSIPoint, mockProof);
        core.initializeAccount(newEmailAddrPointer, emailDomain, block.timestamp, emailNullifier2, mockProof);

        core.claimUnclaimedFund(recipientEmailAddrCommit, newEmailAddrPointer, mockProof);
        vm.stopPrank();

        assertEq(daiToken.balanceOf(core.getWalletOfSalt(newWalletSalt)), 100 ether, "recipient didnt receive tokens");
    }

    function test_ClaimUnclaimedFund_MultipleToNewlyCreatedAccount() public {
        address sender = vm.addr(7);
        address sender2 = vm.addr(3);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes32 recipientEmailAddrCommit2 = bytes32(uint256(5345345));
        bytes32 newEmailAddrPointer = bytes32(uint256(2001));
        bytes32 newAccountKeyCommit = bytes32(uint256(2002));
        bytes32 newWalletSalt = bytes32(uint256(2003));
        bytes memory newPSIPoint = abi.encodePacked(uint256(2003));
        address newRelayer = vm.addr(8);

        vm.deal(sender, 2 * unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);
        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        vm.deal(sender2, 2 * unclaimedFundClaimGas * maxFeePerGas);
        usdcToken.freeMint(sender2, 50 ether);
        vm.startPrank(sender2);
        usdcToken.approve(address(core), 50 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit2,
            address(usdcToken),
            50 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        // New relayer should be able to create account and claim both
        vm.startPrank(newRelayer);
        core.registerRelayer(bytes32(uint256(980398)), "relayer3@test.com", "relayer3.com");
        core.createAccount(newEmailAddrPointer, newAccountKeyCommit, newWalletSalt, newPSIPoint, mockProof);
        core.initializeAccount(newEmailAddrPointer, emailDomain, block.timestamp, emailNullifier2, mockProof);

        core.claimUnclaimedFund(recipientEmailAddrCommit, newEmailAddrPointer, mockProof);
        core.claimUnclaimedFund(recipientEmailAddrCommit2, newEmailAddrPointer, mockProof);
        vm.stopPrank();

        assertEq(daiToken.balanceOf(core.getWalletOfSalt(newWalletSalt)), 100 ether, "recipient didnt receive tokens");
        assertEq(usdcToken.balanceOf(core.getWalletOfSalt(newWalletSalt)), 50 ether, "recipient didnt receive tokens");
    }

    function test_ClaimUnclaimedFund_ToTransportedAccount() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes32 newEmailAddrPointer = bytes32(uint256(2001));
        bytes32 newAccountKeyCommit = bytes32(uint256(2002));
        bytes memory newPSIPoint = abi.encodePacked(uint256(2003));
        address newRelayer = vm.addr(8);

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        // New relayer should be able to claim for existing unclaied funds
        vm.startPrank(newRelayer);
        core.registerRelayer(bytes32(uint256(980398)), "relayer3@test.com", "relayer3.com");
        core.transportAccount(
            accountKeyCommit,
            newEmailAddrPointer,
            newAccountKeyCommit,
            newPSIPoint,
            EmailProof({nullifier: emailNullifier2, domain: emailDomain, timestamp: block.timestamp, proof: mockProof}),
            mockProof
        );

        core.claimUnclaimedFund(recipientEmailAddrCommit, newEmailAddrPointer, mockProof);
        vm.stopPrank();
    }

    function test_ClaimUnclaimedFund_ShouldSendClaimFeeToRelayer() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        // Relayer claim the unclaimed fund to account
        vm.startPrank(relayer);
        core.claimUnclaimedFund(recipientEmailAddrCommit, emailAddrPointer, mockProof);
        vm.stopPrank();

        assertEq(relayer.balance, unclaimedFundClaimGas * maxFeePerGas, "recipient didnt receive claim fee");
    }

    function test_RevertIf_ClaimUnclaimedFund_CalledByNonRelayer() public {
        address sender = vm.addr(7);
        address newRelayer = vm.addr(8);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        // Register a new relayer and call unclaim; but not the relayer of account (which is the `relayer` in EmailWalletHelper)
        vm.startPrank(newRelayer);
        core.registerRelayer(bytes32(uint256(980398)), "relayer3@test.com", "relayer3.com");
        vm.expectRevert("invalid relayer for account");
        core.claimUnclaimedFund(recipientEmailAddrCommit, emailAddrPointer, mockProof);
        vm.stopPrank();
    }

    function test_RevertIf_ClaimUnclaimedFund_IsExpired() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.warp(1000); // Set time to 1000

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        vm.warp(1000 + 31 days); // Expiry time is 30 days (set in EmailWalletCoreTestHelper)

        vm.startPrank(relayer);
        vm.expectRevert("unclaimed fund expired");
        core.claimUnclaimedFund(recipientEmailAddrCommit, emailAddrPointer, mockProof);
        vm.stopPrank();
    }

    function test_RevertIf_ClaimUnclaimedFund_ToUninitializedAccount() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes32 newEmailAddrPointer = bytes32(uint256(32334));
        bytes32 newAccountKeyCommit = bytes32(uint256(32335));
        bytes32 newWalletSalt = bytes32(uint256(32336));
        bytes memory newPSI = abi.encodePacked(uint256(32337));

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        // Relayer claim the unclaimed fund to a newly created account, but not initialized
        vm.startPrank(relayer);
        core.createAccount(newEmailAddrPointer, newAccountKeyCommit, newWalletSalt, newPSI, mockProof);
        vm.expectRevert("account not initialized");
        core.claimUnclaimedFund(recipientEmailAddrCommit, newEmailAddrPointer, mockProof);
        vm.stopPrank();
    }

    function test_VoidUnclaimedFund() public {
        address sender = vm.addr(7);
        address voidUser = vm.addr(4);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        vm.warp(block.timestamp + 31 days); // Expiry time is 30 days (set in EmailWalletCoreTestHelper)

        vm.startPrank(voidUser);
        vm.expectEmit();
        emit UnclaimedFundVoided(recipientEmailAddrCommit, address(daiToken), 100 ether, sender);
        core.voidUnclaimedFund(recipientEmailAddrCommit);
        vm.stopPrank();

        assertEq(daiToken.balanceOf(address(core)), 0, "core contract still have tokens");
        assertEq(daiToken.balanceOf(sender), 100 ether, "sender didnt receive tokens");
        assertEq(voidUser.balance + sender.balance, unclaimedFundClaimGas * maxFeePerGas, "claim fee not returned correctly");

        (, , , uint256 amt, ) = core.unclaimedFundOfEmailAddrCommit(recipientEmailAddrCommit);
        assertEq(amt, 0, "unclaimed fund not cleared");
    }

    function test_RevertIf_VoidUnclaimedFund_NotExpired() public {
        address sender = vm.addr(7);
        address voidUser = vm.addr(4);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        vm.deal(sender, unclaimedFundClaimGas * maxFeePerGas);
        daiToken.freeMint(sender, 100 ether);

        vm.startPrank(sender);
        daiToken.approve(address(core), 100 ether);
        core.registerUnclaimedFund{value: unclaimedFundClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(daiToken),
            100 ether,
            0,
            0,
            ""
        );
        vm.stopPrank();

        vm.startPrank(voidUser);
        vm.expectRevert("unclaimed fund not expired");
        core.voidUnclaimedFund(recipientEmailAddrCommit);
        vm.stopPrank();
    }
}
