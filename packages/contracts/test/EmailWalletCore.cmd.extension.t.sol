// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";
// import "./mocks/TestNFTExtension.sol";
import "../src/extensions/NFTExtension.sol";
import "./mocks/TestExtension.sol";
import "./mocks/DummyNFT.sol";

// Testing extension functionality using TestNFTExtension
contract ExtensionCommandTest is EmailWalletCoreTestHelper {
    TestExtension mockExtension;
    NFTExtension nftExtension;
    DummyNFT dummyNFT;
    string[][] public nftExtTemplates = new string[][](1);
    string[][] public mockExtTemplates = new string[][](9);

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _registerAndInitializeAccount();

        // Publish and install extension
        nftExtension = new NFTExtension(address(core));
        dummyNFT = new DummyNFT();
        nftExtension.setNFTAddress("APE", address(dummyNFT));
        nftExtTemplates[0] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"];
        extensionHandler.publishExtension("NFT Wallet", address(nftExtension), nftExtTemplates, 0.1 ether);

        mockExtension = new TestExtension(address(core), address(usdcToken), address(tokenRegistry));
        mockExtTemplates[0] = ["Test", "Register Unclaimed State"];
        mockExtTemplates[1] = ["Test", "Register Unclaimed State Twice"];
        mockExtTemplates[2] = ["Test", "Register Empty Unclaimed State"];
        mockExtTemplates[3] = ["Test", "Register Unclaimed State to", "{address}"];
        mockExtTemplates[4] = ["Test", "Request Token", "{tokenAmount}"];
        mockExtTemplates[5] = ["Test", "Request Token Twice", "{tokenAmount}"];
        mockExtTemplates[6] = ["Test", "Deposit Token", "{tokenAmount}"];
        mockExtTemplates[7] = ["Test", "Execute on", "{address}"];
        // A dummy template to test the subject matchers that are not above
        // mockExtension has wont do anything with this template
        mockExtTemplates[8] = [
            "Test",
            "Sell for",
            "{tokenAmount}",
            "if",
            "{amount}",
            "is between",
            "{int}",
            "and",
            "{uint}",
            "then send to",
            "{address}"
        ];
        extensionHandler.publishExtension("mockExtension", address(mockExtension), mockExtTemplates, 0.1 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.extensionName = "NFT Wallet";
        emailOp.maskedSubject = "Install extension NFT Wallet";
        emailOp.emailNullifier = bytes32(uint256(93845));

        EmailOp memory emailOpTestExt = _getBaseEmailOp();
        emailOpTestExt.command = Commands.INSTALL_EXTENSION;
        emailOpTestExt.extensionName = "mockExtension";
        emailOpTestExt.maskedSubject = "Install extension mockExtension";
        emailOpTestExt.emailNullifier = bytes32(uint256(4234));

        vm.startPrank(relayer);
        core.handleEmailOp(emailOp);
        core.handleEmailOp(emailOpTestExt);
        vm.stopPrank();
    }

    function test_SubjectWithEthRecipient() public {
        address recipient = vm.addr(3);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "NFT";
        emailOp.maskedSubject = string.concat("NFT Send 22 of APE to ", Strings.toHexString(uint160(recipient), 20));
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = false;
        emailOp.recipientETHAddr = recipient;
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(22));
        emailOp.extensionParams.subjectParams[1] = abi.encode(string("APE"));

        vm.startPrank(walletAddr);
        dummyNFT.freeMint(walletAddr, 22); // Mint a NFT with tokenId 22 to walletAddr
        vm.stopPrank();

        vm.startPrank(relayer);
        core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertEq(dummyNFT.ownerOf(22), recipient, "NFT not sent to recipient");
    }

    function test_SubjectWithEmailRecipient() public {
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

        vm.startPrank(walletAddr);
        dummyNFT.freeMint(walletAddr, 55); // Mint a NFT with tokenId 55 to walletAddr
        vm.stopPrank();

        vm.deal(relayer, unclaimedStateClaimGas * maxFeePerGas);
        daiToken.freeMint(walletAddr, 100 ether); // for fee reimbursement

        vm.startPrank(relayer);
        core.handleEmailOp{value: unclaimedStateClaimGas * maxFeePerGas}(emailOp);
        vm.stopPrank();

        (, , , bytes memory state, ) = unclaimsHandler.unclaimedStateOfEmailAddrCommit(recipientEmailAddrCommit);
        assertTrue(state.length > 0, "unclaimed state should not be empty");
    }

    function test_SubjectWithMultipleMatchers() public {
        address randomAddress = vm.addr(3);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = string.concat(
            "Test Sell for 23.2 DAI if 4.5 is between -5 and 10 then send to ",
            Strings.toHexString(uint160(randomAddress), 20)
        );
        emailOp.extensionParams.subjectTemplateIndex = 8;
        emailOp.extensionParams.subjectParams = new bytes[](5);
        emailOp.extensionParams.subjectParams[0] = abi.encode(23.2 ether, "DAI"); // tokenAmount as (uint256,string)
        emailOp.extensionParams.subjectParams[1] = abi.encode(4.5 ether);
        emailOp.extensionParams.subjectParams[2] = abi.encode(-5);
        emailOp.extensionParams.subjectParams[3] = abi.encode(10);
        emailOp.extensionParams.subjectParams[4] = abi.encode(randomAddress);

        vm.startPrank(relayer);
        core.handleEmailOp(emailOp); // We only need to verify subjects match (i.e dont revert)
        vm.stopPrank();
    }

    function test_RevertIf_CommandIsInvalid() public {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "INVALID ";
        emailOp.maskedSubject = "INVALID to ";
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = false;
        emailOp.recipientETHAddr = vm.addr(3);
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(uint256(22));

        vm.startPrank(relayer);
        vm.expectRevert("invalid command or extension");
        core.handleEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_SubjectParamsAreInsufficient() public {
        address randomAddress = vm.addr(3);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "NFT";
        emailOp.maskedSubject = string.concat(
            "Test Sell for 23 DAI if 4.5 is between -5 and 10 then send to ",
            Strings.toHexString(uint160(randomAddress), 20)
        );
        emailOp.extensionParams.subjectTemplateIndex = 1;
        emailOp.extensionParams.subjectParams = new bytes[](4);
        emailOp.extensionParams.subjectParams[0] = abi.encode(23 ether, "DAI");
        emailOp.extensionParams.subjectParams[1] = abi.encode(4.5 ether);
        emailOp.extensionParams.subjectParams[2] = abi.encode(-5);
        emailOp.extensionParams.subjectParams[3] = abi.encode(10);
        // emailOp.extensionParams.subjectParams[4] = abi.encode(randomAddress); // Missing param

        vm.startPrank(relayer);
        vm.expectRevert(); // The foor loop will fail; couldn't match error message
        core.handleEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RevertIf_SubjectParamsAreExcess() public {
        address randomAddress = vm.addr(3);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = string.concat(
            "Test Sell for 23 DAI if 4.5 is between -5 and 10 then send to ",
            Strings.toHexString(uint160(randomAddress), 20)
        );
        emailOp.extensionParams.subjectTemplateIndex = 1;
        emailOp.extensionParams.subjectParams = new bytes[](6);
        emailOp.extensionParams.subjectParams[0] = abi.encode(23 ether, "DAI");
        emailOp.extensionParams.subjectParams[1] = abi.encode(4.5 ether);
        emailOp.extensionParams.subjectParams[2] = abi.encode(-5);
        emailOp.extensionParams.subjectParams[3] = abi.encode(10);
        emailOp.extensionParams.subjectParams[4] = abi.encode(randomAddress);
        emailOp.extensionParams.subjectParams[5] = abi.encode(8192); // This is extra param

        vm.startPrank(relayer);
        vm.expectRevert("invalid subject params length");
        core.handleEmailOp(emailOp);
        vm.stopPrank();
    }

    function test_RequestTokenAsExtension() public {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = "Test Request Token 25 USDC"; // This will trigger requestTokenAsExtension
        emailOp.extensionParams.subjectTemplateIndex = 4;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(25 ether, "USDC");

        usdcToken.freeMint(walletAddr, 25 ether); // For token request by extension

        vm.startPrank(relayer);
        (bool success, , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertEq(success, true, "handleEmailOp failed");
        assertEq(usdcToken.balanceOf(walletAddr), 0, "USDC still with user");
        assertEq(usdcToken.balanceOf(address(mockExtension)), 25 ether, "Extension didnt receive USDC");
    }

    function test_RevertIf_RequestTokenExceedAllowance() public {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = "Test Request Token Twice 25 USDC"; // This will trigger requestTokenAsExtention twice
        emailOp.extensionParams.subjectTemplateIndex = 5;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(25 ether, "USDC");

        usdcToken.freeMint(walletAddr, 25 ether); // For token request by extension; ext will request for 25 twice

        vm.startPrank(relayer);
        (bool success, bytes memory reason, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertEq(success, false, "handleEmailOp should have failed");
        assertEq(string(reason), "insufficient allowance", "wrong revert reason");
    }

    function test_RevertIf_RequestTokenWithoutEmailOp() public {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = "Test Request Token 25 USDC"; // This will trigger requestTokenAsExtension
        emailOp.extensionParams.subjectTemplateIndex = 4;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(25 ether, "USDC");

        usdcToken.freeMint(walletAddr, 26 ether);

        vm.startPrank(relayer);
        core.handleEmailOp(emailOp);
        vm.stopPrank();

        // Call requestTokenAsExtension directly should fail. i.e context should be cleared
        vm.startPrank(address(mockExtension));
        vm.expectRevert("caller not extension in context");
        core.requestTokenAsExtension(address(usdcToken), 1 ether);
        vm.stopPrank();
    }

    function test_DepositTokenAsExtension() public {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = "Test Deposit Token 25 USDC"; // This will trigger depositTokenAsExtesion
        emailOp.extensionParams.subjectTemplateIndex = 6;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(25 ether, "USDC");

        vm.startPrank(address(mockExtension));
        usdcToken.freeMint(25 ether); // For extenstion to deposit token
        usdcToken.approve(address(core), 25 ether); // For core to take tokens from extension to wallet
        vm.stopPrank();

        vm.startPrank(relayer);
        (bool success, bytes memory reason, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertEq(success, true, string.concat("handleEmailOp failed: ", string(reason)));
        assertEq(usdcToken.balanceOf(address(mockExtension)), 0, "Extension still has USDC");
        assertEq(usdcToken.balanceOf(walletAddr), 25 ether, "User didnt receive USDC");
        assertEq(usdcToken.balanceOf(address(core)), 0, "Core contract have USDC");
    }

    function test_RevertIf_DepositTokenWithoutEmailOp() public {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = "Test Deposit Token 25 USDC"; // This will trigger depositTokenAsExtesion
        emailOp.extensionParams.subjectTemplateIndex = 6;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(25 ether, "USDC");

        vm.startPrank(address(mockExtension));
        usdcToken.freeMint(27 ether); // For extenstion to deposit token
        usdcToken.approve(address(core), 26 ether); // For core to take tokens from extension to wallet
        vm.stopPrank();

        vm.startPrank(relayer);
        (bool success, , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "handleEmailOp failed");

        // Call depositTokenAsExtension directly should fail. i.e context should be cleared
        vm.startPrank(address(mockExtension));
        vm.expectRevert("caller not extension in context");
        core.depositTokenAsExtension(address(usdcToken), 1 ether);
        vm.stopPrank();
    }

    function test_ExecuteAsExtension() public {
        address randomAddress = vm.addr(3); // since execute is on a EOA it wont revert

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = string.concat("Test Execute on ", Strings.toHexString(uint160(randomAddress), 20));
        emailOp.extensionParams.subjectTemplateIndex = 7;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(randomAddress);

        vm.startPrank(relayer);
        (bool success, , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertEq(success, true, "handleEmailOp failed");
    }

    function test_RevertIf_ExecuteAsExtension_TargetIsCore() public {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = string.concat("Test Execute on ", Strings.toHexString(uint160(address(core)), 20));
        emailOp.extensionParams.subjectTemplateIndex = 7;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(address(core));

        vm.startPrank(relayer);
        (bool success, bytes memory reason, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(!success, "handleEmailOp should have failed");
        assertEq(string(reason), "target cannot be core or handlers", "invalid reason");
    }

    function test_RevertIf_ExecuteAsExtension_TargetIsHandler() public {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = string.concat(
            "Test Execute on ",
            Strings.toHexString(uint160(address(unclaimsHandler)), 20)
        );
        emailOp.extensionParams.subjectTemplateIndex = 7;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(address(unclaimsHandler));

        vm.startPrank(relayer);
        (bool success, bytes memory reason, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(!success, "handleEmailOp should have failed");
        assertEq(string(reason), "target cannot be core or handlers", "invalid reason");
    }

    function test_RevertIf_ExecuteAsExtension_TargetIsWallet() public {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = string.concat("Test Execute on ", Strings.toHexString(uint160(walletAddr), 20));
        emailOp.extensionParams.subjectTemplateIndex = 7;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(walletAddr);

        vm.startPrank(relayer);
        (bool success, bytes memory reason, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(!success, "handleEmailOp should have failed");
        assertEq(string(reason), "target cannot be wallet", "invalid reason");
    }

    function test_RevertIf_ExecuteAsExtension_TargetIsToken() public {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = string.concat("Test Execute on ", Strings.toHexString(uint160(address(usdcToken)), 20));
        emailOp.extensionParams.subjectTemplateIndex = 7;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(address(usdcToken));

        vm.startPrank(relayer);
        (bool success, bytes memory reason, ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(!success, "handleEmailOp should have failed");
        assertEq(string(reason), "target cannot be a token", "invalid reason");
    }

    // Testing extension cannot execute when not part of an emailOp
    function test_RevertIf_ExecuteAsExtension_WithoutEmailOp() public {
        address randomAddress = vm.addr(3);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Test";
        emailOp.maskedSubject = string.concat("Test Execute on ", Strings.toHexString(uint160(randomAddress), 20));
        emailOp.extensionParams.subjectTemplateIndex = 7;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(randomAddress);

        vm.startPrank(relayer);
        core.handleEmailOp(emailOp); // This will succeed
        vm.stopPrank();

        // Call executeAsExtension directly should fail. i.e context should be cleared
        vm.startPrank(address(mockExtension));
        vm.expectRevert("caller not extension in context");
        core.executeAsExtension(randomAddress, "");
        vm.stopPrank();
    }
}
