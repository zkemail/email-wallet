// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";
import "./mocks/TestNFTExtension.sol";

// Testing extension functionality using TestNFTExtension
contract ExtensionCommandTest is EmailWalletCoreTestHelper {
    address testExtensionAddr;
    TestNFTExtension ext;
    DummyApes dummyApes;
    string[][] public templates = new string[][](2);

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _registerAndInitializeAccount();

        // Publish and install extension
        ext = new TestNFTExtension(address(core));
        testExtensionAddr = address(ext);
        dummyApes = DummyApes(ext.addressOfNFTName("APE"));

        templates[0] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"];

        // A dummy template to test the subject matchers that are not above
        // TestNFTExtension dont do anything with this template
        templates[1] = [
            "NFT",
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

        core.publishExtension("NFT Wallet", testExtensionAddr, templates, 0.1 ether);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.extManagerParams.extensionName = "NFT Wallet";
        emailOp.maskedSubject = "Install extension NFT Wallet";
        emailOp.emailNullifier = bytes32(uint256(93845));

        vm.startPrank(relayer);
        core.handleEmailOp(emailOp);
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
        dummyApes.freeMint(walletAddr, 22); // Mint a NFT with tokenId 22 to walletAddr
        vm.stopPrank();

        vm.startPrank(relayer);
        core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertEq(dummyApes.ownerOf(22), recipient, "NFT not sent to recipient");
    }

    function test_SubjectWithEmailRecipient() public {
        bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "NFT";
        emailOp.maskedSubject = string.concat("NFT Send 55 of APE to ");
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = true;
        emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
        emailOp.extensionParams.subjectParams = new bytes[](2);
        emailOp.extensionParams.subjectParams[0] = abi.encode(55);
        emailOp.extensionParams.subjectParams[1] = abi.encode("APE");

        vm.startPrank(walletAddr);
        dummyApes.freeMint(walletAddr, 55); // Mint a NFT with tokenId 55 to walletAddr
        vm.stopPrank();

        vm.deal(relayer, unclaimedStateClaimGas * maxFeePerGas);

        vm.startPrank(relayer);
        core.handleEmailOp{value: unclaimedStateClaimGas * maxFeePerGas}(emailOp);
        vm.stopPrank();

        (, , , bytes memory state, ) = core.unclaimedStateOfEmailAddrCommit(recipientEmailAddrCommit);
        assertTrue(state.length > 0, "unclaimed state should not be empty");
    }

    function test_SubjectWithMultipleMatchers() public {
        address randomAddress = vm.addr(3);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "NFT";
        emailOp.maskedSubject = string.concat(
            "NFT Sell for 23.2 DAI if 4.5 is between -5 and 10 then send to ",
            Strings.toHexString(uint160(randomAddress), 20)
        );
        emailOp.extensionParams.subjectTemplateIndex = 1;
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
            "NFT Sell for 23 DAI if 4.5 is between -5 and 10 then send to ",
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
        emailOp.command = "NFT";
        emailOp.maskedSubject = string.concat(
            "NFT Sell for 23 DAI if 4.5 is between -5 and 10 then send to ",
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
}
