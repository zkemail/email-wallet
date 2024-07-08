// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";
import {IOauth} from "../src/interfaces/IOauth.sol";
import {OauthCore} from "../src/utils/OauthCore.sol";
import {OauthSignupExtension} from "../src/extensions/OauthSignupExtension.sol";
import {OauthSigninExtension} from "../src/extensions/OauthSigninExtension.sol";
import {StringUtils} from "../src/libraries/StringUtils.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "./mocks/TestERC20.sol";

// Testing extension functionality for Safe2FA extension
contract OauthExtensionCommandTest is EmailWalletCoreTestHelper {
    using StringUtils for *;

    OauthSignupExtension oauthUpExtension;
    string[][] public oauthUpExtTemplates = new string[][](9);
    OauthSigninExtension oauthInExtension;
    string[][] public oauthInExtTemplates = new string[][](8);
    uint256 ephePrivKey = 777;
    address epheAddr = vm.addr(ephePrivKey);
    string username = "alice";

    fallback() external {
        // For one test below to call this contract with empty calldata
    }

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _createTestAccount();

        // Publish and install extension
        {
            OauthSignupExtension oauthUpExtensionImpl = new OauthSignupExtension();
            ERC1967Proxy proxy = new ERC1967Proxy(
                address(oauthUpExtensionImpl),
                abi.encodeCall(oauthUpExtensionImpl.initialize, (address(core)))
            );
            oauthUpExtension = OauthSignupExtension(payable(address(proxy)));
            oauthUpExtTemplates[0] = ["Sign-up", "{string}"];
            // (0,0) = 0
            oauthUpExtTemplates[1] = ["Sign-up", "{string}", "on", "device", "{uint}"];
            // (0,1) = 1
            oauthUpExtTemplates[2] = ["Sign-up", "{string}", "on", "device", "{uint}", "for", "{tokenAmount}"];
            // (0,2) = 2
            oauthUpExtTemplates[3] = [
                "Sign-up",
                "{string}",
                "on",
                "device",
                "{uint}",
                "for",
                "{tokenAmount}",
                "{tokenAmount}"
            ];
            // (0,3) = 3
            oauthUpExtTemplates[4] = [
                "Sign-up",
                "{string}",
                "on",
                "device",
                "{uint}",
                "for",
                "{tokenAmount}",
                "{tokenAmount}",
                "{tokenAmount}"
            ];
            // (1,0) = 4
            oauthUpExtTemplates[5] = ["Sign-up", "{string}", "on", "device", "{uint}", "until", "timestamp", "{uint}"];
            // (1,1) = 4 + 1 = 5
            oauthUpExtTemplates[6] = [
                "Sign-up",
                "{string}",
                "on",
                "device",
                "{uint}",
                "until",
                "timestamp",
                "{uint}",
                "for",
                "{tokenAmount}"
            ];
            // (1,2) = 4 + 2 = 6
            oauthUpExtTemplates[7] = [
                "Sign-up",
                "{string}",
                "on",
                "device",
                "{uint}",
                "until",
                "timestamp",
                "{uint}",
                "for",
                "{tokenAmount}",
                "{tokenAmount}"
            ];
            // (1,3) = 4 + 3 = 7
            oauthUpExtTemplates[8] = [
                "Sign-up",
                "{string}",
                "on",
                "device",
                "{uint}",
                "until",
                "timestamp",
                "{uint}",
                "for",
                "{tokenAmount}",
                "{tokenAmount}",
                "{tokenAmount}"
            ];
            extensionHandler.publishExtension("OauthSignup", address(oauthUpExtension), oauthUpExtTemplates, 0.1 ether);

            OauthSigninExtension oauthInExtensionImpl = new OauthSigninExtension();
            proxy = new ERC1967Proxy(
                address(oauthInExtensionImpl),
                abi.encodeCall(oauthInExtensionImpl.initialize, (address(core)))
            );
            oauthInExtension = OauthSigninExtension(payable(address(proxy)));
            // (0,0) = 0
            oauthInExtTemplates[0] = ["Sign-in", "{string}", "on", "device", "{uint}"];
            // (0,1) = 1
            oauthInExtTemplates[1] = ["Sign-in", "{string}", "on", "device", "{uint}", "for", "{tokenAmount}"];
            // (0,2) = 2
            oauthInExtTemplates[2] = [
                "Sign-in",
                "{string}",
                "on",
                "device",
                "{uint}",
                "for",
                "{tokenAmount}",
                "{tokenAmount}"
            ];
            // (0,3) = 3
            oauthInExtTemplates[3] = [
                "Sign-in",
                "{string}",
                "on",
                "device",
                "{uint}",
                "for",
                "{tokenAmount}",
                "{tokenAmount}",
                "{tokenAmount}"
            ];
            // (1,0) = 4
            oauthInExtTemplates[4] = ["Sign-in", "{string}", "on", "device", "{uint}", "until", "timestamp", "{uint}"];
            // (1,1) = 4 + 1 = 5
            oauthInExtTemplates[5] = [
                "Sign-in",
                "{string}",
                "on",
                "device",
                "{uint}",
                "until",
                "timestamp",
                "{uint}",
                "for",
                "{tokenAmount}"
            ];
            // (1,2) = 4 + 2 = 6
            oauthInExtTemplates[6] = [
                "Sign-in",
                "{string}",
                "on",
                "device",
                "{uint}",
                "until",
                "timestamp",
                "{uint}",
                "for",
                "{tokenAmount}",
                "{tokenAmount}"
            ];
            // (1,3) = 4 + 3 = 7
            oauthInExtTemplates[7] = [
                "Sign-in",
                "{string}",
                "on",
                "device",
                "{uint}",
                "until",
                "timestamp",
                "{uint}",
                "for",
                "{tokenAmount}",
                "{tokenAmount}",
                "{tokenAmount}"
            ];
            extensionHandler.publishExtension("OauthSignin", address(oauthInExtension), oauthInExtTemplates, 0.1 ether);
        }

        vm.startPrank(relayer);

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.extensionName = "OauthSignup";
        emailOp.maskedSubject = "Install extension OauthSignup";
        emailOp.emailNullifier = bytes32(uint256(93844));
        (bool success, , , ) = core.handleEmailOp(emailOp);
        require(success, "installing OauthSignup failed");

        emailOp = _getBaseEmailOp();
        emailOp.command = Commands.INSTALL_EXTENSION;
        emailOp.extensionName = "OauthSignin";
        emailOp.maskedSubject = "Install extension OauthSignin";
        emailOp.emailNullifier = bytes32(uint256(93845));
        (success, , , ) = core.handleEmailOp(emailOp);
        require(success, "installing OauthSignup failed");

        vm.stopPrank();
    }

    function test_Oauth_WETHTransferInSignup() public {
        vm.startPrank(walletAddr);
        deal(address(walletAddr), 10 ether);
        weth.deposit{value: 10 ether}();
        vm.stopPrank();

        vm.startPrank(relayer);
        console.log("wallet of username", oauthCore.walletOfUsername(username));
        _registerEpheAddr(walletAddr, epheAddr);
        uint nonce = oauthCore.nextNonceOfWallet(walletAddr) - 1;
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Sign-up";
        emailOp.maskedSubject = string.concat("Sign-up ", username, " on device ", nonce.toString(), " for 7 ETH");
        emailOp.extensionName = "OauthSignup";
        emailOp.extensionParams.subjectTemplateIndex = 2;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](3);
        emailOp.extensionParams.subjectParams[0] = abi.encode(username);
        emailOp.extensionParams.subjectParams[1] = abi.encode(nonce);
        emailOp.extensionParams.subjectParams[2] = abi.encode(uint256(7 ether), "ETH");
        emailOp.emailNullifier = bytes32(uint256(93847));
        (bool success, , , ) = core.handleEmailOp(emailOp);
        assertTrue(success, "emailOp failed");

        address recipient = vm.addr(110);
        EphemeralTx memory txData = EphemeralTx({
            walletAddr: walletAddr,
            txNonce: 0,
            target: address(weth),
            ethValue: 0,
            tokenAmount: 7 ether,
            data: abi.encodeWithSignature("transfer(address,uint256)", recipient, 7 ether),
            epheAddr: epheAddr,
            epheAddrNonce: nonce,
            signature: new bytes(0)
        });
        bytes32 txHash = Wallet(payable(walletAddr)).hashEphemeralTx(txData);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ephePrivKey, ECDSA.toEthSignedMessageHash(txHash));
        txData.signature = abi.encodePacked(r, s, v);
        Wallet(payable(walletAddr)).executeEphemeralTx(txData);
        require(WETH9(weth).balanceOf(recipient) == 7 ether, "invalid recipient balance");
        require(WETH9(weth).balanceOf(walletAddr) == 3 ether, "invalid sender balance");
        vm.stopPrank();
    }

    function test_Oauth_WETHTransfer() public {
        vm.startPrank(walletAddr);
        deal(address(walletAddr), 10 ether);
        weth.deposit{value: 10 ether}();
        vm.stopPrank();

        vm.startPrank(relayer);
        _signUp(username);
        console.log("wallet of username", oauthCore.walletOfUsername(username));
        _registerEpheAddr(walletAddr, epheAddr);
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Sign-in";
        uint nonce = oauthCore.nextNonceOfWallet(walletAddr) - 1;
        emailOp.maskedSubject = string.concat("Sign-in ", username, " on device ", nonce.toString(), " for 7 ETH");
        emailOp.extensionName = "OauthSignin";
        emailOp.extensionParams.subjectTemplateIndex = 1;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](3);
        emailOp.extensionParams.subjectParams[0] = abi.encode(username);
        emailOp.extensionParams.subjectParams[1] = abi.encode(nonce);
        emailOp.extensionParams.subjectParams[2] = abi.encode(uint256(7 ether), "ETH");
        emailOp.emailNullifier = bytes32(uint256(93847));
        (bool success, , , ) = core.handleEmailOp(emailOp);
        assertTrue(success, "emailOp failed");

        address recipient = vm.addr(110);
        EphemeralTx memory txData = EphemeralTx({
            walletAddr: walletAddr,
            txNonce: 0,
            target: address(weth),
            ethValue: 0,
            tokenAmount: 7 ether,
            data: abi.encodeWithSignature("transfer(address,uint256)", recipient, 7 ether),
            epheAddr: epheAddr,
            epheAddrNonce: nonce,
            signature: new bytes(0)
        });
        bytes32 txHash = Wallet(payable(walletAddr)).hashEphemeralTx(txData);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ephePrivKey, ECDSA.toEthSignedMessageHash(txHash));
        txData.signature = abi.encodePacked(r, s, v);
        Wallet(payable(walletAddr)).executeEphemeralTx(txData);
        require(WETH9(weth).balanceOf(recipient) == 7 ether, "invalid recipient balance");
        require(WETH9(weth).balanceOf(walletAddr) == 3 ether, "invalid sender balance");
        vm.stopPrank();
    }

    function test_Oauth_DAITransfer() public {
        assertTrue(true); // TODO
    }

    function test_Oauth_USDCApprove() public {
        assertTrue(true); // TODO
    }

    function test_Oauth_USDCAllowance() public {
        assertTrue(true); // TODO
    }

    function test_Oauth_USDCTransfer() public {
        assertTrue(true); // TODO
    }

    function test_Oauth_USDCTransferFrom() public {
        assertTrue(true); // TODO
    }

    function test_RevertIf_Oauth_WETHTransferExceedAllowance() public {
        vm.startPrank(walletAddr);
        deal(address(walletAddr), 10 ether);
        weth.deposit{value: 10 ether}();
        vm.stopPrank();

        vm.startPrank(relayer);
        _signUp(username);
        console.log("wallet of username", oauthCore.walletOfUsername(username));
        _registerEpheAddr(walletAddr, epheAddr);
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Sign-in";
        uint nonce = oauthCore.nextNonceOfWallet(walletAddr) - 1;
        emailOp.maskedSubject = string.concat("Sign-in ", username, " on device ", nonce.toString(), " for 7 ETH");
        emailOp.extensionName = "OauthSignin";
        emailOp.extensionParams.subjectTemplateIndex = 1;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](3);
        emailOp.extensionParams.subjectParams[0] = abi.encode(username);
        emailOp.extensionParams.subjectParams[1] = abi.encode(nonce);
        emailOp.extensionParams.subjectParams[2] = abi.encode(uint256(7 ether), "ETH");
        emailOp.emailNullifier = bytes32(uint256(93847));
        (bool success, , , ) = core.handleEmailOp(emailOp);
        assertTrue(success, "emailOp failed");

        address recipient = vm.addr(110);
        EphemeralTx memory txData = EphemeralTx({
            walletAddr: walletAddr,
            txNonce: 0,
            target: address(weth),
            ethValue: 0,
            tokenAmount: 10 ether,
            data: abi.encodeWithSignature("transfer(address,uint256)", recipient, 10 ether),
            epheAddr: epheAddr,
            epheAddrNonce: nonce,
            signature: new bytes(0)
        });
        bytes32 txHash = Wallet(payable(walletAddr)).hashEphemeralTx(txData);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ephePrivKey, ECDSA.toEthSignedMessageHash(txHash));
        txData.signature = abi.encodePacked(r, s, v);
        vm.expectRevert("insufficient token allowance");
        Wallet(payable(walletAddr)).executeEphemeralTx(txData);
        require(WETH9(weth).balanceOf(recipient) == 0 ether, "invalid recipient balance");
        require(WETH9(weth).balanceOf(walletAddr) == 10 ether, "invalid sender balance");
        vm.stopPrank();
    }

    function test_RevertIf_Oauth_WETHTransferInvalidTokenAmountInTx() public {
        vm.startPrank(walletAddr);
        deal(address(walletAddr), 10 ether);
        weth.deposit{value: 10 ether}();
        vm.stopPrank();

        vm.startPrank(relayer);
        _signUp(username);
        console.log("wallet of username", oauthCore.walletOfUsername(username));
        _registerEpheAddr(walletAddr, epheAddr);
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Sign-in";
        uint nonce = oauthCore.nextNonceOfWallet(walletAddr) - 1;

        emailOp.maskedSubject = string.concat("Sign-in ", username, " on device ", nonce.toString(), " for 7 ETH");
        emailOp.extensionName = "OauthSignin";
        emailOp.extensionParams.subjectTemplateIndex = 1;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](3);
        emailOp.extensionParams.subjectParams[0] = abi.encode(username);
        emailOp.extensionParams.subjectParams[1] = abi.encode(nonce);
        emailOp.extensionParams.subjectParams[2] = abi.encode(uint256(7 ether), "ETH");
        emailOp.emailNullifier = bytes32(uint256(93847));
        (bool success, , , ) = core.handleEmailOp(emailOp);
        assertTrue(success, "emailOp failed");
        assertEq(oauthCore.walletOfUsername(username), walletAddr);

        address recipient = vm.addr(110);
        EphemeralTx memory txData = EphemeralTx({
            walletAddr: walletAddr,
            txNonce: 0,
            target: address(weth),
            ethValue: 0,
            tokenAmount: 7 ether,
            data: abi.encodeWithSignature("transfer(address,uint256)", recipient, 10 ether),
            epheAddr: epheAddr,
            epheAddrNonce: nonce,
            signature: new bytes(0)
        });
        bytes32 txHash = Wallet(payable(walletAddr)).hashEphemeralTx(txData);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ephePrivKey, ECDSA.toEthSignedMessageHash(txHash));
        txData.signature = abi.encodePacked(r, s, v);
        vm.expectRevert("invalid amount set");
        Wallet(payable(walletAddr)).executeEphemeralTx(txData);
        require(WETH9(weth).balanceOf(recipient) == 0 ether, "invalid recipient balance");
        require(WETH9(weth).balanceOf(walletAddr) == 10 ether, "invalid sender balance");
        vm.stopPrank();
    }

    function _signUp(string memory _username) private {
        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = "Sign-up";
        emailOp.maskedSubject = string.concat("Sign-up ", _username);
        emailOp.extensionName = "OauthSignup";
        emailOp.extensionParams.subjectTemplateIndex = 0;
        emailOp.hasEmailRecipient = false;
        emailOp.extensionParams.subjectParams = new bytes[](1);
        emailOp.extensionParams.subjectParams[0] = abi.encode(_username);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        assertTrue(success, "emailOp failed");
    }

    function _registerEpheAddr(address _wallet, address _epheAddr) private {
        oauthCore.registerEpheAddr(_wallet, _epheAddr);
    }
    // function test_Safe2FAExtension_AuthETHTransferByEOA() public {
    //     deal(address(safeAccount), 10 ether);
    //     assertEq(address(safeAccount).balance, 10 ether, "failed to deposit to safe account");
    //     address recipient = vm.addr(110);
    //     bytes32 txHash = safeAccount.getTransactionHash(
    //         recipient,
    //         1 ether,
    //         bytes(""),
    //         Enum.Operation.Call,
    //         0,
    //         0,
    //         0,
    //         address(0),
    //         address(safeAccount),
    //         safeAccount.nonce()
    //     );
    //     bytes memory signature = _signTxByEOA(safeEOAOwner1Pk, txHash);
    //     safeAccount.execTransaction(
    //         recipient,
    //         1 ether,
    //         bytes(""),
    //         Enum.Operation.Call,
    //         0,
    //         0,
    //         0,
    //         address(0),
    //         payable(address(safeAccount)),
    //         signature
    //     );
    //     assertEq(address(safeAccount).balance, 9 ether, "failed to transfer from safe account");
    // }

    // function test_Safe2FAExtension_AuthETHTransferByEmail() public {
    //     deal(address(safeAccount), 10 ether);
    //     assertEq(address(safeAccount).balance, 10 ether, "failed to deposit to safe account");

    //     _addEmailOwner();
    //     address recipient = vm.addr(110);
    //     bytes32 txHash = safeAccount.getTransactionHash(
    //         recipient,
    //         1 ether,
    //         bytes(""),
    //         Enum.Operation.Call,
    //         0,
    //         0,
    //         0,
    //         address(0),
    //         address(safeAccount),
    //         safeAccount.nonce()
    //     );
    //     bytes memory signatureEOA = _signTxByEOA(safeEOAOwner1Pk, txHash);
    //     _authTxByEmail(txHash);
    //     bytes memory signatureEmail = _signTxByEmail(walletAddr);
    //     bytes memory signature;
    //     if (safeEOAOwner1 < walletAddr) {
    //         signature = abi.encodePacked(signatureEOA, signatureEmail);
    //     } else {
    //         signature = abi.encodePacked(signatureEmail, signatureEOA);
    //     }
    //     safeAccount.execTransaction(
    //         recipient,
    //         1 ether,
    //         bytes(""),
    //         Enum.Operation.Call,
    //         0,
    //         0,
    //         0,
    //         address(0),
    //         payable(address(safeAccount)),
    //         signature
    //     );
    //     assertEq(address(safeAccount).balance, 9 ether, "failed to transfer from safe account");
    // }

    // function _addEmailOwner() private {
    //     vm.startPrank(address(safeAccount));
    //     safeAccount.addOwnerWithThreshold(address(walletAddr), 2);
    //     vm.stopPrank();
    // }

    // function _authTxByEmail(bytes32 txHash) private {
    //     vm.startPrank(relayer);
    //     string memory txHashStr = uint256(txHash).toHexString();
    //     // Build email op
    //     EmailOp memory emailOp = _getBaseEmailOp();
    //     emailOp.command = "Safe";
    //     emailOp.maskedSubject = string.concat(
    //         "Safe Transaction: Approve ",
    //         txHashStr,
    //         " from ",
    //         SubjectUtils.addressToChecksumHexString(address(safeAccount))
    //     );
    //     emailOp.extensionName = "Safe2FA";
    //     emailOp.extensionParams.subjectTemplateIndex = 0;
    //     emailOp.hasEmailRecipient = false;
    //     emailOp.extensionParams.subjectParams = new bytes[](2);
    //     emailOp.extensionParams.subjectParams[0] = abi.encode(txHashStr);
    //     emailOp.extensionParams.subjectParams[1] = abi.encode(address(safeAccount));
    //     (bool success, , , ) = core.handleEmailOp(emailOp);
    //     assertTrue(success, "emailOp failed");
    //     vm.stopPrank();
    // }

    // function _signTxByEOA(uint256 pk, bytes32 txHash) private pure returns (bytes memory) {
    //     (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, txHash);
    //     return abi.encodePacked(r, s, v);
    // }

    // function _signTxByEmail(address signer) private pure returns (bytes memory) {
    //     uint8 v = 1;
    //     bytes32 r = bytes32(uint256(uint160(signer)));
    //     bytes32 s = bytes32(0);
    //     return abi.encodePacked(r, s, v);
    // }
}
