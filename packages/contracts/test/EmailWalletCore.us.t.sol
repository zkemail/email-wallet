// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";
import "../src/extensions/NFTExtension.sol";
import "./mocks/TestExtension.sol";
import "./mocks/DummyNFT.sol";

contract UnclaimedStateTest is EmailWalletCoreTestHelper {
    NFTExtension nftExtension;
    TestExtension mockExtension;
    DummyNFT dummyNFT;
    string[][] public nftTempates = new string[][](1);
    string[][] public mockExtTemplates = new string[][](7);

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _createTestAccount();

        // Publish and install two extension - we will use them to test unclaimed state
        {
            NFTExtension nftExtensionImpl = new NFTExtension();
            ERC1967Proxy proxy = new ERC1967Proxy(address(nftExtensionImpl), abi.encodeCall(nftExtensionImpl.initialize, (
                address(core)
            )));
            nftExtension = NFTExtension(payable(address(proxy)));

            dummyNFT = new DummyNFT();
            nftExtension.setNFTAddress("APE", address(dummyNFT));

            nftTempates[0] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"];
            extensionHandler.publishExtension("NFT Wallet", address(nftExtension), nftTempates, 0.1 ether);
        }

        mockExtension = new TestExtension(address(core), address(daiToken), address(tokenRegistry));
        mockExtTemplates[0] = ["Test", "Register Unclaimed State"];
        mockExtTemplates[1] = ["Test", "Register Unclaimed State Twice"];
        mockExtTemplates[2] = ["Test", "Register Empty Unclaimed State"];
        mockExtTemplates[3] = ["Test", "Register Unclaimed State to", "{address}"];
        mockExtTemplates[4] = ["Test", "Request Token", "{tokenAmount}"];
        mockExtTemplates[5] = ["Test", "Deposit Token", "{tokenAmount}"];
        mockExtTemplates[6] = ["Test", "Execute on", "{address}"];
        extensionHandler.publishExtension("TestExtension", address(mockExtension), mockExtTemplates, 0.1 ether);

        EmailOp memory emailOpNFT = _getBaseEmailOp();
        emailOpNFT.command = Commands.INSTALL_EXTENSION;
        emailOpNFT.extensionName = "NFT Wallet";
        emailOpNFT.maskedSubject = "Install extension NFT Wallet";
        emailOpNFT.emailNullifier = bytes32(uint256(93845));

        EmailOp memory emailOpTestExt = _getBaseEmailOp();
        emailOpTestExt.command = Commands.INSTALL_EXTENSION;
        emailOpTestExt.extensionName = "TestExtension";
        emailOpTestExt.maskedSubject = "Install extension TestExtension";
        emailOpTestExt.emailNullifier = bytes32(uint256(4234));

        vm.startPrank(relayer);
        core.handleEmailOp(emailOpNFT);
        core.handleEmailOp(emailOpTestExt);
        vm.stopPrank();
    }

    function test_RegisterUnclaimedState_FromEmailOp() public {
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "NFT";
        emailOp.maskedSubject = string.concat("NFT Send 55 of APE to ");
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.feeTokenName = "DAI";
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(55);
        emailOp.extensionParams.subjectParams[1] = abi.encode("APE");

        bytes memory expectedState = abi.encode(address(dummyNFT), 55);

        dummyNFT.freeMint(walletAddr, 55); // Mint a NFT with tokenId 55 to walletAddr
        daiToken.freeMint(walletAddr, 20 ether); // For fee reibursement

        vm.deal(relayer, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(relayer);
        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.UnclaimedStateRegistered(
            0,
            recipientEmailAddrCommit,
            address(nftExtension),
            walletAddr,
            block.timestamp + unclaimsExpiryDuration, // should be the default expiration time
            expectedState,
            0, // dont announce randomness and email
            ""
        );
        (bool success, , , uint256 registeredUnclaimId) = core.handleEmailOp{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(emailOp);
        vm.stopPrank();

        (
            uint256 foundId,
            bytes32 emailAddrCommit,
            address extensionAddr,
            address sender,
            bytes memory state,
            uint256 expiryTime
        ) = unclaimsHandler.unclaimedStateOfId(registeredUnclaimId);

        assertEq(foundId, registeredUnclaimId, "foundId mismatch");
        assertEq(success, true, "handleEmailOp should succeed");
        assertEq(emailAddrCommit, recipientEmailAddrCommit, "emailAddrCommit mismatch");
        assertEq(extensionAddr, address(nftExtension), "extensionAddr mismatch");
        assertEq(sender, walletAddr, "sender mismatch");
        assertEq(state, expectedState, "state mismatch");
        assertEq(expiryTime, block.timestamp + unclaimsExpiryDuration, "expiryTime mismatch");

        // Handler should have received the fee
        assertEq(
            address(unclaimsHandler).balance,
            unclaimedStateClaimGas * maxFeePerGas,
            "unclaimsHandler didnt receive ETH"
        );
    }

    function test_RegisterUnclaimedState_ToAnotherExtension_FromEmailOp() public {
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        // We will deploy same TestExtension as another extension
        // Dummy subject templates used, as we only care about registerUnclaimedState method (which wont revert)
        TestExtension anotherExtension = new TestExtension(address(core), address(daiToken), address(tokenRegistry));
        string[][] memory anotherExtTemplates = new string[][](1);
        anotherExtTemplates[0] = new string[](1);
        anotherExtTemplates[0][0] = "Another";
        extensionHandler.publishExtension(
            "AnotherExtension",
            address(anotherExtension),
            anotherExtTemplates,
            0.1 ether
        );

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = string.concat(
            "Test Register Unclaimed State to ",
            SubjectUtils.addressToChecksumHexString(address(anotherExtension))
        );
        emailOp.extensionParams.subjectTemplateIndex = 3;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.feeTokenName = "DAI";
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(address(anotherExtension));

        daiToken.freeMint(walletAddr, 20 ether); // For fee reibursement
        vm.deal(relayer, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(relayer);
        (bool success, bytes memory reason, , uint256 registeredUnclaimId) = core.handleEmailOp{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(emailOp);
        vm.stopPrank();

        (, bytes32 emailAddrCommit, address extensionAddr, , bytes memory state, ) = unclaimsHandler.unclaimedStateOfId(
            registeredUnclaimId
        );

        assertEq(success, true, string.concat("handleEmailOp failed", string(reason)));
        assertEq(emailAddrCommit, recipientEmailAddrCommit, "emailAddrCommit mismatch");
        assertEq(extensionAddr, address(anotherExtension), "extensionAddr mismatch");
        assertEq(state, abi.encode("test"), "state mismatch");
    }

    function test_RevertIf_ExtensionDontRegisterUnclaimedState_FromEmailOp() public {
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "NFT";
        emailOp.maskedSubject = string.concat("NFT Send 55 of APE to ");
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.feeTokenName = "DAI";
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(55);
        emailOp.extensionParams.subjectParams[1] = abi.encode("APE");

        vm.deal(relayer, unclaimedStateClaimGas * maxFeePerGas);
        daiToken.freeMint(walletAddr, unclaimedStateClaimGas * maxFeePerGas); // For fee reibursement

        vm.startPrank(relayer);
        (bool success, bytes memory reason, , ) = core.handleEmailOp{value: unclaimedStateClaimGas * maxFeePerGas}(
            emailOp
        );
        vm.stopPrank();

        assertEq(success, false, "handleEmailOp should fail");
        assertEq(string(reason), "ERC721: invalid token ID", "reason mismatch");
    }

    function test_RevertIf_RegisterUnclaimedState_NotEnoughFee() public {
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "NFT";
        emailOp.maskedSubject = string.concat("NFT Send 55 of APE to ");
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.feeTokenName = "DAI";
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(55);
        emailOp.extensionParams.subjectParams[1] = abi.encode("APE");

        dummyNFT.freeMint(walletAddr, 55); // Mint a NFT with tokenId 55 to walletAddr

        vm.startPrank(relayer);
        vm.expectRevert("incorrect ETH sent for unclaimed state");
        core.handleEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_UnclaimedStateExists_Internal() public {
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "NFT";
        emailOp.maskedSubject = string.concat("NFT Send 55 of APE to ");
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.feeTokenName = "DAI";
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(55);
        emailOp.extensionParams.subjectParams[1] = abi.encode("APE");

        EmailOp memory emailOp2 = _getBaseEmailOp();
        emailOp2.command = "NFT";
        emailOp2.maskedSubject = string.concat("NFT Send 22 of APE to ");
        emailOp2.extensionParams.subjectTemplateIndex = 0;
        emailOp2.hasEmailRecipient = true;
        emailOp2.feeTokenName = "DAI";
        emailOp2.recipientEmailAddrCommit = recipientEmailAddrCommit; // Use same emailAddrCommit
        emailOp2.extensionParams.subjectParams = new bytes[](2);
        emailOp2.extensionParams.subjectParams[0] = abi.encode(22);
        emailOp2.extensionParams.subjectParams[1] = abi.encode("APE");
        emailOp2.emailNullifier = bytes32(uint256(1212123));

        dummyNFT.freeMint(walletAddr, 55);
        vm.deal(relayer, 2 * unclaimedStateClaimGas * maxFeePerGas);
        daiToken.freeMint(walletAddr, 10 ether); // For fee reibursement

        vm.startPrank(relayer);
        (, , , uint256 registeredUnclaimId) = core.handleEmailOp{value: unclaimedStateClaimGas * maxFeePerGas}(emailOp);
        require(registeredUnclaimId == 0, "the first registeredUnclaimId mismatch");
        dummyNFT.freeMint(walletAddr, 22);
        bool success;
        daiToken.freeMint(walletAddr, 10 ether);
        (success, , , registeredUnclaimId) = core.handleEmailOp{value: unclaimedStateClaimGas * maxFeePerGas}(emailOp2);
        require(success, "handleEmailOp failed");
        require(registeredUnclaimId == 1, "the second registeredUnclaimId mismatch");
        vm.stopPrank();
    }

    function test_RevertWhen_MultipleUnclaimedRegisteredInOneEmailOp() public {
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test"; // We are using the TestExtension to make multiple unclaimed state registration
        emailOp.maskedSubject = "Test Register Unclaimed State Twice";
        emailOp.extensionParams.subjectTemplateIndex = 1;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.feeTokenName = "DAI";

        daiToken.freeMint(walletAddr, 20 ether); // For fee reibursement
        vm.deal(relayer, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(relayer);
        (bool success, bytes memory reason, , ) = core.handleEmailOp{value: unclaimedStateClaimGas * maxFeePerGas}(
            emailOp
        );
        vm.stopPrank();

        assertEq(success, false, "handleEmailOp didnt fail");
        assertEq(string(reason), "unclaimed state exists", "reason mismatch");
    }

    function test_RevertIf_EmptyUnclaimedRegistered_Internally() public {
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = "Test Register Empty Unclaimed State";
        emailOp.extensionParams.subjectTemplateIndex = 2;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.feeTokenName = "DAI";

        daiToken.freeMint(walletAddr, 20 ether); // For fee reibursement
        vm.deal(relayer, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(relayer);
        (bool success, bytes memory reason, , ) = core.handleEmailOp{value: unclaimedStateClaimGas * maxFeePerGas}(
            emailOp
        );
        vm.stopPrank();

        assertEq(success, false, "handleEmailOp didnt fail");
        assertEq(string(reason), "state cannot be empty", "reason mismatch");
    }

    function test_RegisterUnclaimedState_External() public {
        address sender = vm.addr(5);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes memory state = abi.encode(address(dummyNFT), 23);

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        vm.stopPrank();

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        vm.expectEmit(true, true, true, true);

        emit EmailWalletEvents.UnclaimedStateRegistered(
            0,
            recipientEmailAddrCommit,
            address(nftExtension),
            sender,
            block.timestamp + unclaimsExpiryDuration, // should be the default expiration time
            state,
            0, // dont announce randomness and email
            ""
        );
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit, address(nftExtension), state, 0, 0, "");
        vm.stopPrank();

        (
            uint256 foundId,
            bytes32 emailAddrCommit,
            address extensionAddr,
            address ufSender,
            bytes memory usState,
            uint256 expiryTime
        ) = unclaimsHandler.unclaimedStateOfId(registeredUnclaimId);

        assertEq(foundId, registeredUnclaimId, "foundId mismatch");
        assertEq(emailAddrCommit, recipientEmailAddrCommit, "emailAddrCommit mismatch");
        assertEq(extensionAddr, address(nftExtension), "extensionAddr mismatch");
        assertEq(sender, ufSender, "sender mismatch");
        assertEq(usState, state, "state mismatch");
        assertEq(expiryTime, block.timestamp + unclaimsExpiryDuration, "expiryTime mismatch");
    }

    function test_RegisterUnclaimedState_ExternalWithCustomExpiry() public {
        address sender = vm.addr(5);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes memory state = abi.encode(address(dummyNFT), 23);
        uint256 expiryTime = block.timestamp + 1000;

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        vm.stopPrank();

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit, address(nftExtension), state, expiryTime, 0, "");
        vm.stopPrank();

        (, , , , , uint256 usExpiry) = unclaimsHandler.unclaimedStateOfId(registeredUnclaimId);

        assertEq(expiryTime, usExpiry, "expiryTime mismatch");
    }

    function test_RegisterUnclaimedState_ExternalWithAnnouncement() public {
        address sender = vm.addr(5);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes memory state = abi.encode(address(dummyNFT), 23);
        string memory emailAddr = "test@test.com";
        uint256 commitmentRand = uint256(93845);

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        vm.stopPrank();

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.UnclaimedStateRegistered(
            0,
            recipientEmailAddrCommit,
            address(nftExtension),
            sender,
            block.timestamp + unclaimsExpiryDuration, // should be the default expiration time
            state,
            commitmentRand,
            emailAddr
        );
        unclaimsHandler.registerUnclaimedState{value: unclaimedStateClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(nftExtension),
            state,
            0,
            commitmentRand,
            emailAddr
        );
        vm.stopPrank();
    }

    function test_RevertIf_ExtensionDontRegisterUnclaimedState_External() public {
        address sender = vm.addr(5);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes memory state = abi.encode(address(dummyNFT), 23);

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        vm.expectRevert("unclaimed state reg err: ERC721: invalid token ID");
        unclaimsHandler.registerUnclaimedState{value: unclaimedStateClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(nftExtension),
            state,
            0,
            0,
            ""
        );
        vm.stopPrank();
    }

    function test_RevertIf_RegisterEmptyUnclaimedState_External() public {
        address sender = vm.addr(5);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        vm.expectRevert("state cannot be empty");
        unclaimsHandler.registerUnclaimedState{value: unclaimedStateClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(nftExtension),
            bytes(""),
            0,
            0,
            ""
        );
        vm.stopPrank();
    }

    function test_RevertIf_RegisterExpiredUnclaimedState_External() public {
        address sender = vm.addr(5);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        vm.warp(10000000);
        uint expiry = block.timestamp - 1 days;

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        vm.expectRevert("invalid expiry time");
        unclaimsHandler.registerUnclaimedState{value: unclaimedStateClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(nftExtension),
            abi.encode(address(dummyNFT), 23),
            expiry,
            0,
            ""
        );
        vm.stopPrank();
    }

    function test_RevertIf_UnclaimedStateExists_External() public {
        address sender = vm.addr(5);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        dummyNFT.freeMint(sender, 33); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 33);
        vm.stopPrank();

        vm.deal(sender, 2 * unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit, address(nftExtension), abi.encode(address(dummyNFT), 23), 0, 0, "");
        require(registeredUnclaimId == 0, "the first registeredUnclaimId mismatch");

        registeredUnclaimId = unclaimsHandler.registerUnclaimedState{value: unclaimedStateClaimGas * maxFeePerGas}(
            recipientEmailAddrCommit,
            address(nftExtension),
            abi.encode(address(dummyNFT), 33),
            0,
            0,
            ""
        );
        require(registeredUnclaimId == 1, "the second registeredUnclaimId mismatch");
        vm.stopPrank();
    }

    function test_ClaimUnclaimedState_CreatedFromEmailOp() public {
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "NFT";
        emailOp.maskedSubject = string.concat("NFT Send 55 of APE to ");
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.feeTokenName = "DAI";
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(55);
        emailOp.extensionParams.subjectParams[1] = abi.encode("APE");

        bytes memory expectedState = abi.encode(address(dummyNFT), 55);

        dummyNFT.freeMint(walletAddr, 55); // Mint a NFT with tokenId 55 to walletAddr
        daiToken.freeMint(walletAddr, 20 ether); // For fee reibursement

        vm.deal(relayer, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(relayer);
        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.UnclaimedStateRegistered(
            0,
            recipientEmailAddrCommit,
            address(nftExtension),
            walletAddr,
            block.timestamp + unclaimsExpiryDuration, // should be the default expiration time
            expectedState,
            0, // dont announce randomness and email
            ""
        );
        (bool success, , , uint256 registeredUnclaimId) = core.handleEmailOp{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");

        assertEq(
            address(unclaimsHandler).balance,
            unclaimedStateClaimGas * maxFeePerGas,
            "unclaimsHandler didnt receive ETH"
        );

        // Relayer claim the unclaimed state to account
        vm.startPrank(relayer);
        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.UnclaimedStateClaimed(registeredUnclaimId, recipientEmailAddrCommit, walletAddr);

        unclaimsHandler.claimUnclaimedState(registeredUnclaimId, walletSalt, mockProof);
        vm.stopPrank();

        assertEq(dummyNFT.ownerOf(55), walletAddr, "NFT not transferred to account");
        (, , , , bytes memory st, ) = unclaimsHandler.unclaimedStateOfId(registeredUnclaimId);
        assertEq(st.length, 0, "unclaimed state not cleared");

        assertEq(address(relayer).balance, unclaimedStateClaimGas * maxFeePerGas, "relayer didnt receive claim fee");
        assertEq(address(unclaimsHandler).balance, 0, "unclaimsHandler still have ETH");
    }

    function test_ClaimUnclaimedState_CreatedExternally() public {
        address sender = vm.addr(5);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes memory state = abi.encode(address(dummyNFT), 23);

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        vm.stopPrank();

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit, address(nftExtension), state, 0, 0, "");
        vm.stopPrank();

        assertEq(
            address(unclaimsHandler).balance,
            unclaimedStateClaimGas * maxFeePerGas,
            "unclaimsHandler didnt receive ETH"
        );

        // Relayer claim the unclaimed state to account
        vm.startPrank(relayer);
        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.UnclaimedStateClaimed(registeredUnclaimId, recipientEmailAddrCommit, walletAddr);

        unclaimsHandler.claimUnclaimedState(registeredUnclaimId, walletSalt, mockProof);
        vm.stopPrank();

        assertEq(dummyNFT.ownerOf(23), walletAddr, "NFT not transferred to account");
        (, , , , bytes memory st, ) = unclaimsHandler.unclaimedStateOfId(registeredUnclaimId);
        assertEq(st.length, 0, "unclaimed state not cleared");

        assertEq(address(relayer).balance, unclaimedStateClaimGas * maxFeePerGas, "relayer didnt receive claim fee");
        assertEq(address(unclaimsHandler).balance, 0, "unclaimsHandler still have ETH");
    }

    function test_ClaimUnclaimedState_ToNewlyCreatedAccount() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes32 newWalletSalt = bytes32(uint256(2003));
        address newWalletAddr = accountHandler.getWalletOfSalt(newWalletSalt);
        bytes memory newPSIPoint = abi.encodePacked(uint256(2003));
        address relayer2 = vm.addr(3);
        bytes memory state = abi.encode(address(dummyNFT), 23);

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        vm.stopPrank();

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit, address(nftExtension), state, 0, 0, "");
        vm.stopPrank();

        // New relayer should be able to create account and claim
        vm.startPrank(relayer2);
        relayerHandler.registerRelayer("relayer3@test.com", "relayer3.com");
        accountHandler.createAccount(
            newWalletSalt, 
            newPSIPoint, 
            EmailProof({
                nullifier: emailNullifier2,
                proof: mockProof,
                dkimPublicKeyHash: mockDKIMHash,
                domain: emailDomain,
                timestamp: block.timestamp
            })
        );

        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.UnclaimedStateClaimed(registeredUnclaimId, recipientEmailAddrCommit, newWalletAddr);

        unclaimsHandler.claimUnclaimedState(registeredUnclaimId, newWalletSalt, mockProof);
        vm.stopPrank();

        assertEq(dummyNFT.ownerOf(23), newWalletAddr, "NFT not transferred to account");
        (, , , , bytes memory st, ) = unclaimsHandler.unclaimedStateOfId(registeredUnclaimId);
        assertEq(st.length, 0, "unclaimed state not cleared");
    }

    function test_ClaimUnclaimedState_MultipleToNewlyCreatedAccount() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes32 recipientEmailAddrCommit2 = bytes32(uint256(34));
        bytes32 newWalletSalt = bytes32(uint256(2003));
        address newWalletAddr = accountHandler.getWalletOfSalt(newWalletSalt);
        bytes memory newPSIPoint = abi.encodePacked(uint256(2003));
        address relayer2 = vm.addr(3);

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        dummyNFT.freeMint(sender, 33); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 33);
        vm.stopPrank();

        vm.deal(sender, 2 * unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        uint256 registeredUnclaimId1 = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit, address(nftExtension), abi.encode(address(dummyNFT), 23), 0, 0, "");
        uint256 registeredUnclaimId2 = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit2, address(nftExtension), abi.encode(address(dummyNFT), 33), 0, 0, "");
        vm.stopPrank();

        // New relayer should be able to create account and claim
        vm.startPrank(relayer2);
        relayerHandler.registerRelayer("relayer3@test.com", "relayer3.com");
        accountHandler.createAccount(
            newWalletSalt, 
            newPSIPoint, 
            EmailProof({
                nullifier: emailNullifier2,
                proof: mockProof,
                dkimPublicKeyHash: mockDKIMHash,
                domain: emailDomain,
                timestamp: block.timestamp
            })
        );

        unclaimsHandler.claimUnclaimedState(registeredUnclaimId1, newWalletSalt, mockProof);
        unclaimsHandler.claimUnclaimedState(registeredUnclaimId2, newWalletSalt, mockProof);
        vm.stopPrank();

        assertEq(dummyNFT.ownerOf(23), newWalletAddr, "NFT 23 didnt transfer to account");
        assertEq(dummyNFT.ownerOf(33), newWalletAddr, "NFT 33 didnt transfer to account");
    }

    // function test_ClaimUnclaimedState_ToTransportedAccount() public {
    //     address sender = vm.addr(7);
    //     bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
    //     bytes32 newEmailAddrPointer = bytes32(uint256(2001));
    //     bytes32 newAccountKeyCommit = bytes32(uint256(2002));
    //     bytes memory newPSIPoint = abi.encodePacked(uint256(2003));
    //     address relayer2 = vm.addr(3);
    //     bytes memory state = abi.encode(address(dummyNFT), 23);

    //     vm.startPrank(sender);
    //     dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
    //     dummyNFT.approve(address(nftExtension), 23);
    //     vm.stopPrank();

    //     vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

    //     vm.startPrank(sender);
    //     uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
    //         value: unclaimedStateClaimGas * maxFeePerGas
    //     }(recipientEmailAddrCommit, address(nftExtension), state, 0, 0, "");
    //     vm.stopPrank();

    //     // New relayer should be able to claim for existing unclaied States
    //     vm.startPrank(relayer2);
    //     relayerHandler.registerRelayer(bytes32(uint256(980398)), "relayer3@test.com", "relayer3.com");
    //     accountHandler.transportAccount(
    //         accountKeyCommit,
    //         newEmailAddrPointer,
    //         newAccountKeyCommit,
    //         newPSIPoint,
    //         EmailProof({
    //             nullifier: emailNullifier2,
    //             domain: emailDomain,
    //             dkimPublicKeyHash: mockDKIMHash,
    //             timestamp: block.timestamp,
    //             proof: mockProof
    //         }),
    //         mockProof
    //     );

    //     unclaimsHandler.claimUnclaimedState(registeredUnclaimId, newEmailAddrPointer, mockProof);
    //     vm.stopPrank();
    // }

    function test_RevertIf_ClaimUnclaimedState_CalledByNonRelayer() public {
        address newRelayer = vm.addr(8);
        address sender = vm.addr(5);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes memory state = abi.encode(address(dummyNFT), 23);

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        vm.stopPrank();

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit, address(nftExtension), state, 0, 0, "");
        vm.stopPrank();

        // Register a new relayer and call unclaim; but not the relayer of account (which is the `relayer` in EmailWalletHelper)
        vm.startPrank(newRelayer);
        // relayerHandler.registerRelayer("relayer3@test.com", "relayer3.com");
        vm.expectRevert("caller is not a relayer");
        unclaimsHandler.claimUnclaimedState(registeredUnclaimId, emailAddr, mockProof);
        vm.stopPrank();
    }

    function test_RevertIf_ClaimUnclaimedState_IsExpired() public {
        address sender = vm.addr(5);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes memory state = abi.encode(address(dummyNFT), 23);

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        vm.stopPrank();

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit, address(nftExtension), state, 0, 0, "");
        vm.stopPrank();

        vm.warp(1000 + 31 days); // Expiry time is 30 days (set in EmailWalletCoreTestHelper)

        vm.startPrank(relayer);
        vm.expectRevert("unclaimed state expired");
        unclaimsHandler.claimUnclaimedState(registeredUnclaimId, emailAddr, mockProof);
        vm.stopPrank();
    }

    function test_RevertIf_ClaimUnclaimedState_ToUninitializedAccount() public {
        address sender = vm.addr(7);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes32 newEmailAddrPointer = bytes32(uint256(32334));
        // bytes32 newAccountKeyCommit = bytes32(uint256(32335));
        bytes32 newWalletSalt = bytes32(uint256(32336));
        bytes memory newPSI = abi.encodePacked(uint256(32337));
        bytes memory state = abi.encode(address(dummyNFT), 23);

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        vm.stopPrank();

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit, address(nftExtension), state, 0, 0, "");
        vm.stopPrank();

        // Relayer claim the unclaimed State to a newly created account, but not initialized
        vm.startPrank(relayer);
        accountHandler.createAccount(
            newWalletSalt, 
            newPSI,
            EmailProof({
                nullifier: emailNullifier2,
                proof: mockProof,
                dkimPublicKeyHash: mockDKIMHash,
                domain: emailDomain,
                timestamp: block.timestamp
            })
        );

        // vm.expectRevert("account not initialized");
        unclaimsHandler.claimUnclaimedState(registeredUnclaimId, newEmailAddrPointer, mockProof);
        vm.stopPrank();
    }

    function test_VoidUnclaimedState() public {
        address sender = vm.addr(7);
        address voidUser = vm.addr(4);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes memory state = abi.encode(address(dummyNFT), 23);

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        vm.stopPrank();

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit, address(nftExtension), state, 0, 0, "");
        vm.stopPrank();

        vm.warp(block.timestamp + 31 days); // Expiry time is 30 days (set in EmailWalletCoreTestHelper)

        vm.startPrank(voidUser);
        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.UnclaimedStateVoided(registeredUnclaimId, recipientEmailAddrCommit, sender);
        unclaimsHandler.voidUnclaimedState(registeredUnclaimId);
        vm.stopPrank();

        assertEq(dummyNFT.ownerOf(23), sender, "NFT not returned to sender");
        assertEq(
            voidUser.balance + sender.balance,
            unclaimedStateClaimGas * maxFeePerGas,
            "claim fee not returned correctly"
        );

        (, , , , bytes memory st, ) = unclaimsHandler.unclaimedStateOfId(registeredUnclaimId);
        assertEq(st.length, 0, "unclaimed state not cleared");
    }

    function test_RevertIf_VoidUnclaimedState_NotExpired() public {
        address sender = vm.addr(7);
        address voidUser = vm.addr(4);
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));
        bytes memory state = abi.encode(address(dummyNFT), 23);

        vm.startPrank(sender);
        dummyNFT.freeMint(sender, 23); // Mint a NFT with tokenId 23 to walletAddr
        dummyNFT.approve(address(nftExtension), 23);
        vm.stopPrank();

        vm.deal(sender, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(sender);
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
            value: unclaimedStateClaimGas * maxFeePerGas
        }(recipientEmailAddrCommit, address(nftExtension), state, 0, 0, "");
        vm.stopPrank();

        vm.startPrank(voidUser);
        vm.expectRevert("unclaimed state not expired");
        unclaimsHandler.voidUnclaimedState(registeredUnclaimId);
        vm.stopPrank();
    }
}
