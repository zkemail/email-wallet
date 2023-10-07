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
        (address akRelayer, bool initialized, bool nullified, bool akWalletSaltSet, bytes32 akWalletSalt) = core
            .infoOfAccountKeyCommit(accountKeyCommit);
        assertEq(akWalletSalt, walletSalt);
        assertTrue(akWalletSaltSet);
        assertEq(akRelayer, relayer);
        assertEq(core.pointerOfPSIPoint(psiPoint), emailAddrPointer);
        assertFalse(initialized);
        assertFalse(nullified);
    }

    function testRevertCreateAccountWhenRelayerIsNotRegistered() public {
        vm.expectRevert("relayer not registered");
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
    }

    function testRevertIfPointerIsAlreadyRegistered() public {
        bytes32 accountKeyCommit2 = bytes32(uint256(2));
        bytes32 walletSalt2 = bytes32(uint256(3));
        bytes memory psiPoint2 = abi.encodePacked(uint256(4));

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
        deployCodeTo("TestWallet.sol", abi.encode(address(weth)), predictedAddr);

        vm.startPrank(relayer);
        vm.expectRevert("wallet already deployed");
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        vm.stopPrank();
    }

    function testAccountInitailization() public {
        bytes32 emailNullifier = bytes32(uint256(101));

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();

        (, bool initialized, , , ) = core.infoOfAccountKeyCommit(accountKeyCommit);
        assertTrue(initialized);
    }

    function testRevertAccountInitializationIfAccountNotRegistered() public {
        bytes32 emailNullifier = bytes32(uint256(101));

        vm.startPrank(relayer);
        vm.expectRevert("account not registered");
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();
    }

    function testAccountTransport() public {
        bytes32 emailNullifier = bytes32(uint256(101));
        bytes32 emailNullifier2 = bytes32(uint256(102));
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

        (, , bool nullified, , bytes32 akWalletSaltSet) = core.infoOfAccountKeyCommit(accountKeyCommit);
        assertEq(akWalletSaltSet, bytes32(0));
        assertTrue(nullified); // old accountKeyCommit is nullified
        assertEq(core.accountKeyCommitOfPointer(newEmailAddrPointer), newAccountKeyCommit);
        (address newAkRelayer, bool newAkInitialized, bool newAkNullified, , bytes32 newAkWalletSaltSet) = core
            .infoOfAccountKeyCommit(newAccountKeyCommit);
        assertEq(newAkRelayer, relayer2);
        assertEq(newAkWalletSaltSet, walletSalt); // should not change
        assertFalse(newAkNullified);
        assertTrue(newAkInitialized);
        assertEq(core.pointerOfPSIPoint(newPSIPoint), newEmailAddrPointer);
    }

    function testRevertIfTransportedAccountIsNullified() public {
        bytes32 emailNullifier = bytes32(uint256(101));
        bytes32 emailNullifier2 = bytes32(uint256(102));
        bytes32 newEmailAddrPointer = bytes32(uint256(2001));
        bytes32 newAccountKeyCommit = bytes32(uint256(2002));
        bytes memory newPSIPoint = abi.encodePacked(uint256(2003));
        address relayer2 = vm.addr(3);
        bytes32 relayer2RandHash = bytes32(uint256(311));
        address relayer3 = vm.addr(4);
        bytes32 relayer3RandHash = bytes32(uint256(411));

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();

        // Transporting will nullify the accountKeyCommit
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

        vm.startPrank(relayer3);
        core.registerRelayer(relayer3RandHash, "mail@relayer3", "relayer3.com");
        vm.expectRevert("account is nullified");
        core.transportAccount(
            accountKeyCommit, // old accountKeyCommit
            bytes32(uint256(20011)), // newEmailAddrPointer
            bytes32(uint256(20021)), // newAccountKeyCommit
            abi.encodePacked(uint256(20031)), // newPSIPoint
            EmailProof({
                nullifier: bytes32(uint256(1021)), // emailNullifier
                domain: emailDomain,
                timestamp: block.timestamp,
                proof: mockProof
            }),
            mockProof
        );
        vm.stopPrank();
    }

    function testRevertIfTransportedAccountIsNotInitialized() public {
        bytes32 emailNullifier = bytes32(uint256(101));
        bytes32 newEmailAddrPointer = bytes32(uint256(2001));
        bytes32 newAccountKeyCommit = bytes32(uint256(2002));
        bytes memory newPSIPoint = abi.encodePacked(uint256(2003));
        address relayer2 = vm.addr(3);
        bytes32 relayer2RandHash = bytes32(uint256(311));

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        vm.stopPrank();

        vm.startPrank(relayer2);
        core.registerRelayer(relayer2RandHash, "mail@relayer2", "relayer2.com");
        vm.expectRevert("account not initialized");
        core.transportAccount(
            accountKeyCommit,
            newEmailAddrPointer,
            newAccountKeyCommit,
            newPSIPoint,
            EmailProof({
                nullifier: emailNullifier,
                domain: emailDomain,
                timestamp: block.timestamp,
                proof: mockProof
            }),
            mockProof
        );
        vm.stopPrank();
    }

    function testRevertIfTransportedAccountAlreadyExists() public {
        bytes32 emailNullifier = bytes32(uint256(101));
        bytes32 emailNullifier2 = bytes32(uint256(102));
        bytes32 newEmailAddrPointer = bytes32(uint256(2001));
        bytes32 newAccountKeyCommit = bytes32(uint256(2002));
        bytes32 walletSalt2 = bytes32(uint256(2002));
        bytes memory newPSIPoint = abi.encodePacked(uint256(2003));
        address relayer2 = vm.addr(3);
        bytes32 relayer2RandHash = bytes32(uint256(311));

        vm.startPrank(relayer);
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, mockProof);
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();

        vm.startPrank(relayer2);
        core.registerRelayer(relayer2RandHash, "mail@relayer2", "relayer2.com");
        core.createAccount(newEmailAddrPointer, newAccountKeyCommit, walletSalt2, newPSIPoint, mockProof);
        vm.expectRevert("new pointer already exist");
        core.transportAccount(
            accountKeyCommit,
            newEmailAddrPointer,
            newAccountKeyCommit,
            newPSIPoint,
            EmailProof({nullifier: emailNullifier2, domain: emailDomain, timestamp: block.timestamp, proof: mockProof}),
            mockProof
        );
        vm.stopPrank();
    }

    function testRevertAccountInitializationIfTransported() public {
        bytes32 emailNullifier = bytes32(uint256(101));
        bytes32 emailNullifier2 = bytes32(uint256(102));
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

        // Transport will nullify account with relayer - Re initialization should fail
        vm.startPrank(relayer);
        vm.expectRevert("account is nullified");
        core.initializeAccount(emailAddrPointer, emailDomain, block.timestamp, emailNullifier, mockProof);
        vm.stopPrank();
    }
}
