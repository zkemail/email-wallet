// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

contract AccountTest is EmailWalletCoreTestHelper {
    function setUp() public override {
        super.setUp();
        _registerRelayer();
    }

    function test_CreateAccount() public {
        vm.startPrank(relayer);
        vm.expectEmit(true, true, true, true);

        EmailProof memory emailProof = EmailProof({
            emailDomain: emailDomain,
            dkimPublicKeyHash: mockDKIMHash,
            timestamp: block.timestamp,
            emailNullifier: emailNullifier,
            maskedSubject: "",
            accountSalt: accountSalt,
            isCodeExist: true,
            hasEmailRecipient: false,
            recipientEmailAddrCommit: bytes32(0),
            proof: mockProof
        });

        emit EmailWalletEvents.AccountCreated(accountSalt, psiPoint);

        accountHandler.createAccount(emailProof, psiPoint, mockProof);
        vm.stopPrank();

        Wallet wallet = Wallet(payable(accountHandler.getWalletOfSalt(accountSalt)));
        assertEq(wallet.owner(), address(core), "wallet owner is not accountHandler");
        assertEq(accountHandler.accountSaltOfPSIPoint(psiPoint), accountSalt);
    }

    // function testFail_CreateAccount() public {
    //     vm.warp(1701388800);
    //     vm.startPrank(relayer);
    //     vm.expectEmit(true, true, true, true);
    //     emit EmailWalletEvents.AccountCreated(emailAddrPointer, accountCodeCommit, accountSalt, psiPoint);

    //     accountHandler.createAccount(emailAddrPointer, accountCodeCommit, accountSalt, psiPoint, mockProof);
    //     vm.stopPrank();

    //     Wallet wallet = Wallet(payable(accountHandler.getWalletOfSalt(accountSalt)));
    //     assertEq(wallet.owner(), address(core), "wallet owner is not accountHandler");

    //     assertEq(accountHandler.accountCodeCommitOfPointer(emailAddrPointer), accountCodeCommit);

    //     accountHandler.infoOfaccountCodeCommit(accountCodeCommit);
    // }

    // We dont't use a pointer anymore.
    // TODO: Remove this commented function later.
    // "relayer not registered" is not used anymore
    // function test_RevertWhen_CreateAccountRelayerIsNotRegistered() public {
    //     vm.expectRevert("relayer not registered");
    //     accountHandler.createAccount(emailAddrPointer, accountSalt, psiPoint, mockProof);
    // }

    // We dont't use a pointer anymore.
    // TODO: Remove this commented function later.
    // function test_RevertIf_PointerIsAlreadyRegistered() public {
    //     bytes32 accountSalt2 = bytes32(uint256(3));
    //     bytes memory psiPoint2 = abi.encodePacked(uint256(41121));
    //     vm.startPrank(relayer);
    //     accountHandler.createAccount(
    //         emailAddr,
    //         accountSalt,
    //         psiPoint,
    //         emailDomain,
    //         block.timestamp,
    //         emailNullifier,
    //         mockDKIMHash,
    //         mockProof
    //     );
    //     vm.expectRevert("pointer exists");
    //     accountHandler.createAccount(
    //         emailAddr,
    //         accountSalt2,
    //         psiPoint2,
    //         emailDomain,
    //         block.timestamp,
    //         emailNullifier,
    //         mockDKIMHash,
    //         mockProof
    //     );
    //     vm.stopPrank();
    // }

    function test_RevertIf_PSIPointIsAlreadyRegistered() public {
        bytes32 accountSalt2 = bytes32(uint256(3));

        vm.startPrank(relayer);
        EmailProof memory emailProof = EmailProof({
            emailDomain: emailDomain,
            dkimPublicKeyHash: mockDKIMHash,
            timestamp: block.timestamp,
            emailNullifier: emailNullifier,
            maskedSubject: "",
            accountSalt: accountSalt,
            isCodeExist: true,
            hasEmailRecipient: false,
            recipientEmailAddrCommit: bytes32(0),
            proof: mockProof
        });
        accountHandler.createAccount(emailProof, psiPoint, mockProof);
        vm.expectRevert("PSI point exists for another wallet salt");
        emailProof = EmailProof({
            emailDomain: emailDomain,
            dkimPublicKeyHash: mockDKIMHash,
            timestamp: block.timestamp,
            emailNullifier: emailNullifier,
            maskedSubject: "",
            accountSalt: accountSalt2,
            isCodeExist: true,
            hasEmailRecipient: false,
            recipientEmailAddrCommit: bytes32(0),
            proof: mockProof
        });
        accountHandler.createAccount(emailProof, psiPoint, mockProof);
        vm.stopPrank();
    }

    // account key commit is not used anymore
    // function test_RevertIf_AccountCodeCommitAlreadyHasAnotherAccountSalt() public {
    //     bytes32 emailAddrPointer2 = bytes32(uint256(2));
    //     bytes32 accountSalt2 = bytes32(uint256(2));
    //     bytes memory psiPoint2 = abi.encodePacked(uint256(4));

    //     vm.startPrank(relayer);
    //     accountHandler.createAccount(emailAddrPointer, accountSalt, psiPoint, mockProof);
    //     vm.expectRevert("accountSalt exists");
    //     accountHandler.createAccount(emailAddrPointer2, accountSalt2, psiPoint2, mockProof);
    //     vm.stopPrank();
    // }

    function test_CreateWalletWithPredeterministicAddress() public {
        address predictedAddr = accountHandler.getWalletOfSalt(accountSalt);

        vm.startPrank(relayer);
        EmailProof memory emailProof = EmailProof({
            emailDomain: emailDomain,
            dkimPublicKeyHash: mockDKIMHash,
            timestamp: block.timestamp,
            emailNullifier: emailNullifier,
            maskedSubject: "",
            accountSalt: accountSalt,
            isCodeExist: true,
            hasEmailRecipient: false,
            recipientEmailAddrCommit: bytes32(0),
            proof: mockProof
        });
        address walletAddr = address(accountHandler.createAccount(emailProof, psiPoint, mockProof));
        vm.stopPrank();

        assertEq(walletAddr, predictedAddr);
    }

    function test_RevertWhen_PredeterministicWalletIsAlreadyDeployed() public {
        address predictedAddr = accountHandler.getWalletOfSalt(accountSalt);
        deployCodeTo("WETH9.sol", abi.encode(address(weth)), predictedAddr);

        vm.startPrank(relayer);
        vm.expectRevert("wallet already deployed");
        EmailProof memory emailProof = EmailProof({
            emailDomain: emailDomain,
            dkimPublicKeyHash: mockDKIMHash,
            timestamp: block.timestamp,
            emailNullifier: emailNullifier,
            maskedSubject: "",
            accountSalt: accountSalt,
            isCodeExist: true,
            hasEmailRecipient: false,
            recipientEmailAddrCommit: bytes32(0),
            proof: mockProof
        });
        accountHandler.createAccount(emailProof, psiPoint, mockProof);
        vm.stopPrank();
    }

    // function test_AccountInitailization() public {
    //     vm.startPrank(relayer);
    //     accountHandler.createAccount(
    //         emailAddrPointer,
    //         accountSalt,
    //         psiPoint,
    //         mockProof
    //     );

    //     vm.expectEmit(true, true, true, true);
    //     emit EmailWalletEvents.AccountInitialized(emailAddrPointer, accountSalt, accountSalt);

    //     accountHandler.initializeAccount(
    //         emailAddrPointer,
    //         emailDomain,
    //         block.timestamp,
    //         emailNullifier,
    //         mockDKIMHash,
    //         mockProof
    //     );
    //     vm.stopPrank();

    //     (, bool initialized, ) = accountHandler.infoOfEmailAddrPointer(accountSalt);
    //     assertTrue(initialized);
    // }

    // function testFail_AccountInitailization() public {
    //     vm.warp(1701388800);
    //     vm.startPrank(relayer);
    //     accountHandler.createAccount(emailAddrPointer, accountCodeCommit, accountSalt, psiPoint, mockProof);

    //     vm.expectEmit(true, true, true, true);
    //     emit EmailWalletEvents.AccountInitialized(emailAddrPointer, accountCodeCommit, accountSalt);

    //     accountHandler.initializeAccount(
    //         emailAddrPointer,
    //         emailDomain,
    //         block.timestamp,
    //         emailNullifier,
    //         mockDKIMHash,
    //         mockProof
    //     );
    //     vm.stopPrank();

    //     accountHandler.infoOfAccountCodeCommit(accountCodeCommit);
    // }

    // initializeAccount function is not called from outside anymore
    // TODO: Remove this commented function later.
    // function test_RevertIf_InitializingAccountNotRegistered() public {
    //     vm.startPrank(relayer);
    //     vm.expectRevert("account not registered");
    //     accountHandler.initializeAccount(
    //         emailAddr,
    //         emailDomain,
    //         block.timestamp,
    //         emailNullifier,
    //         mockDKIMHash,
    //         mockProof
    //     );
    //     vm.stopPrank();
    // }

    // function test_AccountTransport() public {
    //     bytes32 newEmailAddrPointer = bytes32(uint256(2001));
    //     bytes32 newAccountCodeCommit = bytes32(uint256(2002));
    //     bytes memory newPSIPoint = abi.encodePacked(uint256(2003));
    //     address relayer2 = vm.addr(3);
    //     bytes32 relayer2RandHash = bytes32(uint256(311));

    //     vm.startPrank(relayer);
    //     accountHandler.createAccount(emailAddrPointer, accountSalt, psiPoint, mockProof);
    //     accountHandler.initializeAccount(
    //         emailAddrPointer,
    //         emailDomain,
    //         block.timestamp,
    //         emailNullifier,
    //         mockDKIMHash,
    //         mockProof
    //     );
    //     vm.stopPrank();

    //     vm.startPrank(relayer2);
    //     relayerHandler.registerRelayer("mail@relayer2", "relayer2.com");

    //     vm.expectEmit(true, true, true, true);
    //     emit EmailWalletEvents.AccountTransported(
    //         accountCodeCommit,
    //         newEmailAddrPointer,
    //         newAccountCodeCommit,
    //         newPSIPoint
    //     );

    //     // accountHandler.transportAccount(
    //     //     accountCodeCommit,
    //     //     newEmailAddrPointer,
    //     //     newAccountCodeCommit,
    //     //     newPSIPoint,
    //     //     EmailProof({
    //     //         nullifier: emailNullifier2,
    //     //         domain: emailDomain,
    //     //         dkimPublicKeyHash: mockDKIMHash,
    //     //         timestamp: block.timestamp,
    //     //         proof: mockProof
    //     //     }),
    //     //     mockProof
    //     // );
    //     vm.stopPrank();

    //     (, bool initializedOld, ) = accountHandler.infoOfAccountCodeCommit(accountCodeCommit);
    //     assertTrue(initializedOld); // old accountCodeCommit should still be initialized
    //     assertEq(accountHandler.accountCodeCommitOfPointer(newEmailAddrPointer), newAccountCodeCommit);
    //     (address newAkRelayer, bool newAkInitialized, bytes32 newAccountSalt) = accountHandler.infoOfAccountCodeCommit(
    //         newAccountCodeCommit
    //     );
    //     assertEq(newAkRelayer, relayer2);
    //     assertEq(newAccountSalt, accountSalt); // should not change
    //     assertTrue(newAkInitialized);
    //     assertEq(accountHandler.pointerOfPSIPoint(newPSIPoint), newEmailAddrPointer);
    // }

    // function test_AccountTransport_MultipleTimes() public {
    //     address relayer2 = vm.addr(3);
    //     bytes32 relayer2RandHash = bytes32(uint256(311));
    //     bytes32 relayer2Pointer = bytes32(uint256(2001));
    //     bytes32 relayer2AccountCodeCommit = bytes32(uint256(2002));
    //     bytes memory relayer2PSIPoint = abi.encodePacked(uint256(2003));

    //     address relayer3 = vm.addr(4);
    //     bytes32 relayer3RandHash = bytes32(uint256(411));
    //     bytes32 relayer3Pointer = bytes32(uint256(3001));
    //     bytes32 relayer3AccountCodeCommit = bytes32(uint256(3002));
    //     bytes memory relayer3PSIPoint = abi.encodePacked(uint256(3003));

    //     vm.startPrank(relayer);
    //     accountHandler.createAccount(emailAddrPointer, accountSalt, psiPoint, mockProof);
    //     accountHandler.initializeAccount(
    //         emailAddrPointer,
    //         emailDomain,
    //         block.timestamp,
    //         emailNullifier,
    //         mockDKIMHash,
    //         mockProof
    //     );
    //     vm.stopPrank();

    //     // Transporting will nullify the accountCodeCommit of relayer1
    //     vm.startPrank(relayer2);
    //     relayerHandler.registerRelayer("mail@relayer2", "relayer2.com");
    //     // accountHandler.transportAccount(
    //     //     accountCodeCommit,
    //     //     relayer2Pointer,
    //     //     relayer2AccountCodeCommit,
    //     //     relayer2PSIPoint,
    //     //     EmailProof({
    //     //         dkimPublicKeyHash: mockDKIMHash,
    //     //         nullifier: emailNullifier2,
    //     //         domain: emailDomain,
    //     //         timestamp: block.timestamp,
    //     //         proof: mockProof
    //     //     }),
    //     //     mockProof
    //     // );
    //     vm.stopPrank();

    //     // Transporting to relayer3 with relayer2AccountCodeCommit - most recent relayer should used as "old"
    //     vm.startPrank(relayer3);
    //     relayerHandler.registerRelayer("mail@relayer3", "relayer3.com");
    //     // accountHandler.transportAccount(
    //     //     relayer2AccountCodeCommit,
    //     //     relayer3Pointer,
    //     //     relayer3AccountCodeCommit,
    //     //     relayer3PSIPoint,
    //     //     EmailProof({
    //     //         dkimPublicKeyHash: mockDKIMHash,
    //     //         nullifier: emailNullifier3,
    //     //         domain: emailDomain,
    //     //         timestamp: block.timestamp,
    //     //         proof: mockProof
    //     //     }),
    //     //     mockProof
    //     // );
    //     vm.stopPrank();

    //     // Relayer 1 and 2 should be nullified, but 3 should work
    //     (, bool r1Initialized, ) = accountHandler.infoOfAccountCodeCommit(accountCodeCommit);
    //     assertTrue(r1Initialized, "relayer1 account should be initialized");

    //     (, bool r2Initialized, ) = accountHandler.infoOfAccountCodeCommit(relayer2AccountCodeCommit);
    //     assertTrue(r2Initialized, "relayer2 account should be initialized");

    //     (, bool r3Initialized, ) = accountHandler.infoOfAccountCodeCommit(relayer3AccountCodeCommit);
    //     assertTrue(r3Initialized, "relayer3 account should be initialized");

    //     assertEq(accountHandler.accountCodeCommitOfPointer(relayer3Pointer), relayer3AccountCodeCommit);
    // }

    // function test_RevertIf_TransportedAccountIsNotInitialized() public {
    //     address relayer2 = vm.addr(3);
    //     bytes32 relayer2RandHash = bytes32(uint256(311));
    //     bytes32 relayer2Pointer = bytes32(uint256(2001));
    //     bytes32 relayer2AccountCodeCommit = bytes32(uint256(2002));
    //     bytes memory relayer2PSIPoint = abi.encodePacked(uint256(2003));

    //     vm.startPrank(relayer);
    //     accountHandler.createAccount(emailAddrPointer, accountCodeCommit, accountSalt, psiPoint, mockProof);
    //     vm.stopPrank();

    //     vm.startPrank(relayer2);
    //     relayerHandler.registerRelayer(relayer2RandHash, "mail@relayer2", "relayer2.com");
    //     vm.expectRevert("account not initialized");
    //     accountHandler.transportAccount(
    //         accountCodeCommit,
    //         relayer2Pointer,
    //         relayer2AccountCodeCommit,
    //         relayer2PSIPoint,
    //         EmailProof({
    //             dkimPublicKeyHash: mockDKIMHash,
    //             nullifier: emailNullifier,
    //             domain: emailDomain,
    //             timestamp: block.timestamp,
    //             proof: mockProof
    //         }),
    //         mockProof
    //     );
    //     vm.stopPrank();
    // }

    // Relayer can transport account even if the pointer was registered previously but not initialized
    // function test_AccountTransport_RelayerWithExistingPointer() public {
    //     address relayer2 = vm.addr(3);
    //     bytes32 relayer2RandHash = bytes32(uint256(311121));
    //     bytes32 relayer2Pointer = bytes32(uint256(2001232));
    //     bytes32 relayer2InitialAccountCodeCommit = bytes32(uint256(12012302));
    //     bytes32 relayer2NewAccountCodeCommit = bytes32(uint256(12012302));
    //     bytes32 relayer2AccountSalt = bytes32(uint256(2123123002));
    //     bytes memory relayer2PSIPoint = abi.encodePacked(uint256(20434303));

    //     // Register and initialize with relayer 1
    //     vm.startPrank(relayer);
    //     accountHandler.createAccount(emailAddrPointer, accountCodeCommit, accountSalt, psiPoint, mockProof);
    //     accountHandler.initializeAccount(
    //         emailAddrPointer,
    //         emailDomain,
    //         block.timestamp,
    //         emailNullifier,
    //         mockDKIMHash,
    //         mockProof
    //     );
    //     vm.stopPrank();

    //     // Register wtih relayer 2 (dont initialized), then transport from relayer 1 to relayer 2
    //     vm.startPrank(relayer2);
    //     relayerHandler.registerRelayer(relayer2RandHash, "mail@relayer2", "relayer2.com");
    //     accountHandler.createAccount(
    //         relayer2Pointer,
    //         relayer2InitialAccountCodeCommit,
    //         relayer2AccountSalt,
    //         relayer2PSIPoint,
    //         mockProof
    //     );
    //     accountHandler.transportAccount(
    //         accountCodeCommit,
    //         relayer2Pointer, // Pointer will be same as relayer2 has already created the account for email
    //         relayer2NewAccountCodeCommit, // Different accountCodeCommitment as AK is the one used had with relayer1
    //         relayer2PSIPoint,
    //         EmailProof({
    //             dkimPublicKeyHash: mockDKIMHash,
    //             nullifier: emailNullifier2,
    //             domain: emailDomain,
    //             timestamp: block.timestamp,
    //             proof: mockProof
    //         }),
    //         mockProof
    //     );
    //     vm.stopPrank();

    //     (, bool r1Initialized, ) = accountHandler.infoOfAccountCodeCommit(accountCodeCommit);
    //     assertTrue(r1Initialized, "old relayer should still be initialized");

    //     assertEq(accountHandler.accountCodeCommitOfPointer(relayer2Pointer), relayer2NewAccountCodeCommit);

    //     (, bool r2Initialized, ) = accountHandler.infoOfAccountCodeCommit(relayer2NewAccountCodeCommit);
    //     assertTrue(r2Initialized, "new relayer account not initialized");
    // }

    // function test_RevertIf_AccountTransport_BackToOriginalRelayer() public {
    //     address relayer2 = vm.addr(3);
    //     bytes32 relayer2RandHash = bytes32(uint256(311121));
    //     bytes32 relayer2Pointer = bytes32(uint256(202201232));
    //     bytes32 relayer2AccountCodeCommit = bytes32(uint256(12012302));
    //     bytes memory relayer2PSIPoint = abi.encodePacked(uint256(20434303));

    //     // Register and initialize with relayer 1
    //     vm.startPrank(relayer);
    //     accountHandler.createAccount(emailAddrPointer, accountCodeCommit, accountSalt, psiPoint, mockProof);
    //     accountHandler.initializeAccount(
    //         emailAddrPointer,
    //         emailDomain,
    //         block.timestamp,
    //         emailNullifier,
    //         mockDKIMHash,
    //         mockProof
    //     );
    //     vm.stopPrank();

    //     // Transport from relayer 1 to relayer 2
    //     vm.startPrank(relayer2);
    //     relayerHandler.registerRelayer(relayer2RandHash, "mail@relayer2", "relayer2.com");
    //     accountHandler.transportAccount(
    //         accountCodeCommit,
    //         relayer2Pointer,
    //         relayer2AccountCodeCommit,
    //         relayer2PSIPoint,
    //         EmailProof({
    //             dkimPublicKeyHash: mockDKIMHash,
    //             nullifier: emailNullifier2,
    //             domain: emailDomain,
    //             timestamp: block.timestamp,
    //             proof: mockProof
    //         }),
    //         mockProof
    //     );
    //     vm.stopPrank();

    //     // Transport from relayer 2 to relayer 1
    //     vm.startPrank(relayer);
    //     vm.expectRevert("new account is already initialized");
    //     accountHandler.transportAccount(
    //         relayer2AccountCodeCommit,
    //         emailAddrPointer,
    //         accountCodeCommit, // newAccountCodeCommit is the first (relayer1) accountCodeCommit
    //         psiPoint,
    //         EmailProof({
    //             dkimPublicKeyHash: mockDKIMHash,
    //             nullifier: emailNullifier3,
    //             domain: emailDomain,
    //             timestamp: block.timestamp,
    //             proof: mockProof
    //         }),
    //         mockProof
    //     );
    //     vm.stopPrank();

    //     (, bool initialized, ) = accountHandler.infoOfAccountCodeCommit(accountCodeCommit);
    //     assertTrue(initialized, "transported account not initialized");

    //     assertEq(accountHandler.accountCodeCommitOfPointer(emailAddrPointer), accountCodeCommit);
    // }

    function testUpgradeability() public {
        AccountHandler implV2 = new AccountHandler();

        vm.startPrank(deployer);
        accountHandler.upgradeTo(address(implV2));
        vm.stopPrank();
    }
}
