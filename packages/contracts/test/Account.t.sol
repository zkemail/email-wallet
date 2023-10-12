// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./EmailWalletCoreTestHelper.sol";

contract AccountTest is EmailWalletCoreTestHelper {
    function setUp() public override {
        super.setUp();
        _registerRelayer();
    }

    function testCreateAccount() public {
        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        vm.stopPrank();

        Wallet wallet = Wallet(payable(core.getWalletOfSalt(walletSalt)));
        assertEq(wallet.owner(), address(core), "wallet owner is not walletAddr");

        assertEq(core.accountKeyCommitOfPointer(emailAddrPointer), accountKeyCommit);
        (address akRelayer, bool initialized, bool nullified, bool akWalletSaltSet, bytes32 akWalletSalt, ) = core
            .infoOfAccountKeyCommit(accountKeyCommit);
        assertEq(akWalletSalt, walletSalt);
        assertTrue(akWalletSaltSet);
        assertEq(akRelayer, relayer);
        assertEq(core.pointerOfPSIPoint(psiPoint), emailAddrPointer);
        assertTrue(!initialized);
        assertTrue(!nullified);
    }

    function testRevertCreateAccountWhenRelayerIsNotRegistered() public {
        vm.expectRevert("relayer not registered");
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
    }

    function testRevertIfPointerIsAlreadyRegistered() public {
        bytes32 accountKeyCommit2 = bytes32(uint256(2));
        bytes32 walletSalt2 = bytes32(uint256(3));
        bytes memory psiPoint2 = abi.encodePacked(uint256(41121));

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        vm.expectRevert("pointer exists");
        core.createAccount(emailAddrPointer, accountKeyCommit2, walletSalt2, psiPoint2, mockProof);
        vm.stopPrank();
    }

    function testRevertIfPSIPointIsAlreadyRegistered() public {
        bytes32 emailAddrPointer2 = bytes32(uint256(2));
        bytes32 accountKeyCommit2 = bytes32(uint256(2));
        bytes32 walletSalt2 = bytes32(uint256(3));

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        vm.expectRevert("PSI point exists");
        core.createAccount(emailAddrPointer2, accountKeyCommit2, walletSalt2, psiPoint, mockProof);
        vm.stopPrank();
    }

    function testRevertIfAccountKeyCommitAlreadyHasAnotherWalletSalt() public {
        bytes32 emailAddrPointer2 = bytes32(uint256(2));
        bytes32 walletSalt2 = bytes32(uint256(2));
        bytes memory psiPoint2 = abi.encodePacked(uint256(4));

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        vm.expectRevert("walletSalt exists");
        core.createAccount(emailAddrPointer2, accountKeyCommit, walletSalt2, psiPoint2, mockProof);
        vm.stopPrank();
    }

    function testCreateWalletWithPredeterministicAddress() public {
        address predictedAddr = core.getWalletOfSalt(walletSalt);

        vm.startPrank(relayer);
        address walletAddr = address(
            core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof)
        );
        vm.stopPrank();

        assertEq(walletAddr, predictedAddr);
    }

    function testRevertWhenPredeterministicWalletIsAlreadyDeployed() public {
        address predictedAddr = core.getWalletOfSalt(walletSalt);
        deployCodeTo("WETH9.sol", abi.encode(address(weth)), predictedAddr);

        vm.startPrank(relayer);
        vm.expectRevert("wallet already deployed");
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        vm.stopPrank();
    }

    function testAccountInitailization() public {
        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();

        (, bool initialized, , , , ) = core.infoOfAccountKeyCommit(accountKeyCommit);
        assertTrue(initialized);
    }

    function testRevertAccountInitializationIfAccountNotRegistered() public {
        vm.startPrank(relayer);
        vm.expectRevert("account not registered");
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();
    }

    function testAccountTransport() public {
        bytes32 newEmailAddrPointer = bytes32(uint256(2001));
        bytes32 newAccountKeyCommit = bytes32(uint256(2002));
        bytes memory newPSIPoint = abi.encodePacked(uint256(2003));
        address relayer2 = vm.addr(3);
        bytes32 relayer2RandHash = bytes32(uint256(311));

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();

        vm.startPrank(relayer2);
        core.registerRelayer(relayer2RandHash, "mail@relayer2", "relayer2.com");
        core.transportAccount(
            accountKeyCommit,
            newEmailAddrPointer,
            newAccountKeyCommit,
            newPSIPoint,
            EmailProof({nullifier: emailNullifier2, domain: emailDomain, timestamp: block.timestamp, proof: mockProof}),
            mockProof
        );
        vm.stopPrank();

        (, , bool nullified, , bytes32 akWalletSaltSet, ) = core.infoOfAccountKeyCommit(accountKeyCommit);
        assertEq(akWalletSaltSet, bytes32(0));
        assertTrue(nullified); // old accountKeyCommit is nullified
        assertEq(core.accountKeyCommitOfPointer(newEmailAddrPointer), newAccountKeyCommit);
        (address newAkRelayer, bool newAkInitialized, bool newAkNullified, , bytes32 newAkWalletSaltSet, ) = core
            .infoOfAccountKeyCommit(newAccountKeyCommit);
        assertEq(newAkRelayer, relayer2);
        assertEq(newAkWalletSaltSet, walletSalt); // should not change
        assertTrue(!newAkNullified);
        assertTrue(newAkInitialized);
        assertEq(core.pointerOfPSIPoint(newPSIPoint), newEmailAddrPointer);
    }

    function testTransportMultipleTimes() public {
        address relayer2 = vm.addr(3);
        bytes32 relayer2RandHash = bytes32(uint256(311));
        bytes32 relayer2Pointer = bytes32(uint256(2001));
        bytes32 relayer2AccountKeyCommit = bytes32(uint256(2002));
        bytes memory relayer2PSIPoint = abi.encodePacked(uint256(2003));

        address relayer3 = vm.addr(4);
        bytes32 relayer3RandHash = bytes32(uint256(411));
        bytes32 relayer3Pointer = bytes32(uint256(3001));
        bytes32 relayer3AccountKeyCommit = bytes32(uint256(3002));
        bytes memory relayer3PSIPoint = abi.encodePacked(uint256(3003));

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();

        // Transporting will nullify the accountKeyCommit of relayer1
        vm.startPrank(relayer2);
        core.registerRelayer(relayer2RandHash, "mail@relayer2", "relayer2.com");
        core.transportAccount(
            accountKeyCommit,
            relayer2Pointer,
            relayer2AccountKeyCommit,
            relayer2PSIPoint,
            EmailProof({nullifier: emailNullifier2, domain: emailDomain, timestamp: block.timestamp, proof: mockProof}),
            mockProof
        );
        vm.stopPrank();

        // Transporting to relayer3 with relayer2AccountKeyCommit - most recent relayer should used as "old"
        vm.startPrank(relayer3);
        core.registerRelayer(relayer3RandHash, "mail@relayer3", "relayer3.com");
        core.transportAccount(
            relayer2AccountKeyCommit,
            relayer3Pointer,
            relayer3AccountKeyCommit,
            relayer3PSIPoint,
            EmailProof({nullifier: emailNullifier3, domain: emailDomain, timestamp: block.timestamp, proof: mockProof}),
            mockProof
        );
        vm.stopPrank();

        // Relayer 1 and 2 should be nullified, but 3 should work
        (, bool r1Initialized, bool r1Nullified, , , ) = core.infoOfAccountKeyCommit(accountKeyCommit);
        assertTrue(!r1Initialized, "relayer1 account is still initialized");
        assertTrue(r1Nullified, "relayer1 account is not nullified");

        (, bool r2Initialized, bool r2Nullified, , , ) = core.infoOfAccountKeyCommit(relayer2AccountKeyCommit);
        assertTrue(!r2Initialized, "relayer2 account is still initialized");
        assertTrue(r2Nullified, "relayer2 account is not nullified");

        (, bool r3Initialized, bool r3Nullified, , , ) = core.infoOfAccountKeyCommit(relayer3AccountKeyCommit);
        assertTrue(r3Initialized, "relayer3 account is not initialized");
        assertTrue(!r3Nullified, "relayer3 account is nullified");

        assertEq(core.accountKeyCommitOfPointer(relayer3Pointer), relayer3AccountKeyCommit);
    }

    function testRevertIfTransportedAccountIsNotInitialized() public {
        address relayer2 = vm.addr(3);
        bytes32 relayer2RandHash = bytes32(uint256(311));
        bytes32 relayer2Pointer = bytes32(uint256(2001));
        bytes32 relayer2AccountKeyCommit = bytes32(uint256(2002));
        bytes memory relayer2PSIPoint = abi.encodePacked(uint256(2003));

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        vm.stopPrank();

        vm.startPrank(relayer2);
        core.registerRelayer(relayer2RandHash, "mail@relayer2", "relayer2.com");
        vm.expectRevert("account not initialized");
        core.transportAccount(
            accountKeyCommit,
            relayer2Pointer,
            relayer2AccountKeyCommit,
            relayer2PSIPoint,
            EmailProof({nullifier: emailNullifier, domain: emailDomain, timestamp: block.timestamp, proof: mockProof}),
            mockProof
        );
        vm.stopPrank();
    }

    function testRevertAccountInitializationIfTransported() public {
        address relayer2 = vm.addr(3);
        bytes32 relayer2RandHash = bytes32(uint256(311));
        bytes32 relayer2Pointer = bytes32(uint256(2001));
        bytes32 relayer2AccountKeyCommit = bytes32(uint256(2002));
        bytes memory relayer2PSIPOint = abi.encodePacked(uint256(2003));

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, bytes32(uint256(101)), mockProof);
        vm.stopPrank();

        vm.startPrank(relayer2);
        core.registerRelayer(relayer2RandHash, "mail@relayer2", "relayer2.com");
        core.transportAccount(
            accountKeyCommit,
            relayer2Pointer,
            relayer2AccountKeyCommit,
            relayer2PSIPOint,
            EmailProof({
                nullifier: bytes32(uint256(102)),
                domain: emailDomain,
                timestamp: block.timestamp,
                proof: mockProof
            }),
            mockProof
        );
        vm.stopPrank();

        // Transport will nullify account with relayer - Re initialization should fail
        vm.startPrank(relayer);
        vm.expectRevert("account is nullified");
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();
    }

    // Relayer can transport account even if the pointer was registered previously but not initialized
    function testAccountTransportForExistingPointer() public {
        address relayer2 = vm.addr(3);
        bytes32 relayer2RandHash = bytes32(uint256(311121));
        bytes32 relayer2Pointer = bytes32(uint256(2001232));
        bytes32 relayer2InitialAccountKeyCommit = bytes32(uint256(12012302));
        bytes32 relayer2NewAccountKeyCommit = bytes32(uint256(12012302));
        bytes32 relayer2WalletSalt = bytes32(uint256(2123123002));
        bytes memory relayer2PSIPoint = abi.encodePacked(uint256(20434303));

        // Register and initialize with relayer 1
        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();

        // Register wtih relayer 2 (dont initialized), then transport from relayer 1 to relayer 2
        vm.startPrank(relayer2);
        core.registerRelayer(relayer2RandHash, "mail@relayer2", "relayer2.com");
        core.createAccount(
            relayer2Pointer,
            relayer2InitialAccountKeyCommit,
            relayer2WalletSalt,
            relayer2PSIPoint,
            mockProof
        );
        core.transportAccount(
            accountKeyCommit,
            relayer2Pointer, // Pointer will be same as relayer2 has already created the account for email
            relayer2NewAccountKeyCommit, // Different accountKeyCommitment as AK is the one used had with relayer1
            relayer2PSIPoint,
            EmailProof({nullifier: emailNullifier2, domain: emailDomain, timestamp: block.timestamp, proof: mockProof}),
            mockProof
        );
        vm.stopPrank();

        (, bool r1Initialized, bool r1Nullified, , , ) = core.infoOfAccountKeyCommit(accountKeyCommit);
        assertTrue(!r1Initialized, "relayer1 account is still initialized");
        assertTrue(r1Nullified, "relayer1 account is not nullified");

        assertEq(core.accountKeyCommitOfPointer(relayer2Pointer), relayer2NewAccountKeyCommit);

        (, bool r2Initialized, bool r2Nullified, , , ) = core.infoOfAccountKeyCommit(relayer2NewAccountKeyCommit);
        assertTrue(r2Initialized, "transported account not initialized");
        assertTrue(!r2Nullified, "transported account is nullified");
    }

    function testAccountTransportBackToPreviousRelayer() public {
        address relayer2 = vm.addr(3);
        bytes32 relayer2RandHash = bytes32(uint256(311121));
        bytes32 relayer2Pointer = bytes32(uint256(202201232));
        bytes32 relayer2AccountKeyCommit = bytes32(uint256(12012302));
        bytes32 relayer2WalletSalt = bytes32(uint256(2123123002));
        bytes memory relayer2PSIPoint = abi.encodePacked(uint256(20434303));

        // Register and initialize with relayer 1
        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();

        // Transport from relayer 1 to relayer 2
        vm.startPrank(relayer2);
        core.registerRelayer(relayer2RandHash, "mail@relayer2", "relayer2.com");
        core.transportAccount(
            accountKeyCommit,
            relayer2Pointer,
            relayer2AccountKeyCommit,
            relayer2PSIPoint,
            EmailProof({nullifier: emailNullifier2, domain: emailDomain, timestamp: block.timestamp, proof: mockProof}),
            mockProof
        );
        vm.stopPrank();

        // Transport from relayer 2 to relayer 1
        vm.startPrank(relayer);
        core.transportAccount(
            relayer2AccountKeyCommit,
            emailAddrPointer,
            accountKeyCommit, // newAccountKeyCommit is the first (relayer1) accountKeyCommit
            psiPoint,
            EmailProof({nullifier: emailNullifier3, domain: emailDomain, timestamp: block.timestamp, proof: mockProof}),
            mockProof
        );
        vm.stopPrank();

        (, bool initialized, bool nullified, , , ) = core.infoOfAccountKeyCommit(accountKeyCommit);
        assertTrue(initialized, "transported account not initialized");
        assertTrue(!nullified, "transported account is nullified");

        assertEq(core.accountKeyCommitOfPointer(emailAddrPointer), accountKeyCommit);
    }
}
