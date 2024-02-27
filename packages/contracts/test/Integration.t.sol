// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "@zk-email/contracts/DKIMRegistry.sol";
import "../src/EmailWalletCore.sol";
import "../src/utils/TokenRegistry.sol";
import "../src/utils/UniswapTWAPOracle.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "./helpers/WETH9.sol";
import "../src/verifier/AccountCreationVerifier.sol";
// import "../src/verifier/AccountInitVerifier.sol";
// import "../src/verifier/AccountTransportVerifier.sol";
import "../src/verifier/ClaimVerifier.sol";
import "../src/verifier/EmailSenderVerifier.sol";
import "../src/verifier/AnnouncementVerifier.sol";
import "../src/verifier/Verifier.sol";
import {EmailWalletEvents} from "../src/interfaces/Events.sol";
import "../src/extensions/UniswapExtension.sol";
import "../src/extensions/NFTExtension.sol";
import "./mocks/DummyNFT.sol";
import "./helpers/IntegrationTestHelper.sol";
import "../src/Wallet.sol";

import "forge-std/console.sol";

contract IntegrationTest is IntegrationTestHelper {
    using Strings for *;
    using console for *;
    using SubjectUtils for *;

    function testIntegration_Account_Creation() public {
        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        vm.stopPrank();

        vm.startPrank(relayer1);
        Wallet user2WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test2.eml"),
            user2.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        vm.stopPrank();

    }

    function testIntegration_Transfer_ETH_To_Internal() public {
        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user1Wallet = address(user1WalletContract);
        vm.stopPrank();

        vm.startPrank(user1Wallet);
        deal(user1Wallet, 0.15 ether);
        weth.deposit{value: 0.15 ether}();
        require(weth.balanceOf(user1Wallet) == 0.15 ether, "User1 wallet balance before the transaction mismatch");
        vm.stopPrank();


        vm.startPrank(relayer1);
        (EmailOp memory emailOp, bytes32 emailAddrRand) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/token_transfer_test1.eml"),
            user1.accountKey,
            "Send",
            "Send 0.1 ETH to ",
            "gmail.com",
            "ETH"
        );
        emailOp.walletParams.tokenName = "ETH";
        emailOp.walletParams.amount = 0.1 ether;

        deal(relayer1, core.unclaimedFundClaimGas() * core.maxFeePerGas());
        (bool success, bytes memory reason, , uint256 registeredUnclaimId) = core.handleEmailOp{
            value: core.unclaimedFundClaimGas() * core.maxFeePerGas()
        }(emailOp);
        assertEq(success, true, string(reason));
        require(weth.balanceOf(user1Wallet) < 0.05 ether, "User1 wallet balance after the transaction is too large");
        require(weth.balanceOf(address(unclaimsHandler)) == 0.1 ether, "Core contract weth balance mismatch");
        require(
            address(unclaimsHandler).balance == core.unclaimedFundClaimGas() * core.maxFeePerGas(),
            "Core contract eth balance mismatch"
        );
        Wallet user2WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test2.eml"),
            user2.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user2Wallet = address(user2WalletContract);
        console.logString("Relayer1 address: ");
        console.logAddress(relayer1);
        console.logString("User1 wallet address: ");
        console.logAddress(user1Wallet);
        console.logString("User2 wallet address: ");
        console.logAddress(user2Wallet);
        require(weth.balanceOf(user2Wallet) == 0, "User2 wallet balance mismatch");
        claimFund(registeredUnclaimId, user2.emailAddr, emailAddrRand, user2.accountKey);
        require(weth.balanceOf(user2Wallet) == 0.1 ether, "User2 wallet balance mismatch");
        require(weth.balanceOf(address(unclaimsHandler)) == 0, "Core contract balance mismatch");
        vm.stopPrank();
    }

    function testIntegration_Transfer_ETH_To_External() public {
        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user1Wallet = address(user1WalletContract);
        vm.stopPrank();

        vm.startPrank(user1Wallet);
        deal(user1Wallet, 0.3 ether);
        weth.deposit{value: 0.3 ether}();
        vm.stopPrank();

        vm.startPrank(relayer1);
        address recipient = vm.addr(4);
        (EmailOp memory emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/token_transfer_test2.eml"),
            user1.accountKey,
            "Send",
            string.concat("Send 0.25 ETH to ", recipient.addressToChecksumHexString()),
            "gmail.com",
            "ETH"
        );
        emailOp.walletParams.tokenName = "ETH";
        emailOp.walletParams.amount = 0.25 ether;
        emailOp.recipientETHAddr = recipient;
        (bool success, bytes memory reason, , ) = core.handleEmailOp{value: 0}(emailOp);
        assertEq(success, true, string(reason));
        require(weth.balanceOf(user1Wallet) < 0.05 ether, "User1 wallet balance is too large");
        require(recipient.balance == 0.25 ether, "Recipient address balance mismatch");
        require(weth.balanceOf(address(unclaimsHandler)) == 0, "Core contract balance mismatch");
        vm.stopPrank();
    }

    function testIntegration_Transfers_Random() public {
        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user1Wallet = address(user1WalletContract);
        vm.stopPrank();

        vm.startPrank(relayer1);
        Wallet user2WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test2.eml"),
            user2.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user2Wallet = address(user2WalletContract);
        vm.stopPrank();

        vm.startPrank(user1Wallet);
        deal(user1Wallet, 20 ether);
        weth.deposit{value: 20 ether}();
        deal(address(daiToken), user1Wallet, 20 * 10000 ether);
        deal(address(usdcToken), user1Wallet, 20 * 10000 * (10 ** 6));
        vm.stopPrank();
        vm.startPrank(user2Wallet);
        deal(user2Wallet, 20 ether);
        weth.deposit{value: 20 ether}();
        deal(address(daiToken), user2Wallet, 20 * 10000 ether);
        deal(address(usdcToken), user2Wallet, 20 * 10000 * (10 ** 6));
        vm.stopPrank();
        
        vm.startPrank(relayer1);
        bytes32 randomHash = keccak256(abi.encode(blockhash(block.number - 1)));
        string[3] memory amountStrs = ["1", "0.2", "0.03"];
        // TODO: We can't send USDC because it's a native usdc token.
        // string[3] memory tokens = ["ETH", "DAI", "USDC"];
        string[2] memory tokens = ["ETH", "DAI"];
        UserTestConfig[2] memory users = [user1, user2];
        bool[3][3][3] memory usedEmail;
        uint idx = 0;
        while (idx < 8) {
            randomHash = keccak256(abi.encode(randomHash));
            uint amountSelector = uint(randomHash) % 3;
            randomHash = keccak256(abi.encode(randomHash));
            uint tokenSelector = uint(randomHash) % 2;
            randomHash = keccak256(abi.encode(randomHash));
            uint senderSelector = uint(randomHash) % 2;
            randomHash = keccak256(abi.encode(randomHash));
            uint feeSelector = uint(randomHash) % 2;
            if (usedEmail[senderSelector][tokenSelector][amountSelector]) {
                continue;
            }
            usedEmail[senderSelector][tokenSelector][amountSelector] = true;
            idx++;
            // senderSelector = 0;
            (EmailOp memory emailOp, bytes32 emailAddrRand) = genEmailOpPartial(
                string.concat(
                    vm.projectRoot(),
                    "/test/emails/random_test/",
                    amountSelector.toString(),
                    "_",
                    tokens[tokenSelector],
                    "_",
                    senderSelector.toString(),
                    "_",
                    (1 - senderSelector).toString(),
                    ".eml"
                ),
                users[senderSelector].accountKey,
                "Send",
                string.concat("Send ", amountStrs[amountSelector], " ", tokens[tokenSelector], " to "),
                "gmail.com",
                tokens[feeSelector]
            );
            emailOp.walletParams.tokenName = tokens[tokenSelector];
            if (tokenSelector == 0 || tokenSelector == 1) {
                emailOp.walletParams.amount = [uint(1 ether), uint(0.2 ether), uint(0.03 ether)][amountSelector];
            } else {
                emailOp.walletParams.amount = [uint(1 * (10 ** 6)), uint(0.2 * (10 ** 6)), uint(0.03 * (10 ** 6))][
                    amountSelector
                ];
            }
            deal(relayer1, core.unclaimedFundClaimGas() * core.maxFeePerGas());
            console.logString("function testIntegration_Transfers_Random");
            console.logString("users[senderSelector].accountKey");
            console.logBytes32(users[senderSelector].accountKey);
            console.logString("tokens[tokenSelector]");
            console.logString(tokens[tokenSelector]);
            console.logString("tokens[feeSelector]");
            console.logString(tokens[feeSelector]);
            
            (bool success, bytes memory reason, , uint256 registeredUnclaimId) = core.handleEmailOp{
                value: core.unclaimedFundClaimGas() * core.maxFeePerGas()
            }(emailOp);
            assertEq(success, true, string(reason));
            claimFund(registeredUnclaimId, users[1 - senderSelector].emailAddr, emailAddrRand, users[1 - senderSelector].accountKey);
        }
        vm.stopPrank();
    }

    function testIntegration_Swap_Tokens() public {
        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user1Wallet = address(user1WalletContract);
        vm.stopPrank();

        vm.startPrank(user1Wallet);
        deal(user1Wallet, 20 ether);
        weth.deposit{value: 20 ether}();
        deal(address(daiToken), user1Wallet, 20 * 10000 ether);
        deal(address(usdcToken), user1Wallet, 20 * 10000 * (10 ** 6));
        vm.stopPrank();

        vm.startPrank(relayer1);
        (EmailOp memory emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/install_uniswap.eml"),
            user1.accountKey,
            "Install",
            "Install extension Uniswap",
            "gmail.com",
            "ETH"
        );
        emailOp.extensionName = "Uniswap";
        (bool success, bytes memory reason, , ) = core.handleEmailOp(emailOp);
        require(success, string(reason));

        (emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/uniswap_test1.eml"),
            user1.accountKey,
            "Swap",
            "Swap 0.2 ETH to DAI",
            "gmail.com",
            "ETH"
        );
        bytes[] memory extensionBytes = new bytes[](2);
        extensionBytes[0] = abi.encode(uint256(0.2 ether), "ETH");
        extensionBytes[1] = abi.encode("DAI");
        emailOp.extensionParams = ExtensionParams(0, extensionBytes);
        uint preEthBalance = weth.balanceOf(user1Wallet);
        uint preDaiBalance = daiToken.balanceOf(user1Wallet);
        (success, reason, , ) = core.handleEmailOp(emailOp);
        require(success, string(reason));
        require(preEthBalance > weth.balanceOf(user1Wallet), "ETH balance does not decrease");
        require(preDaiBalance < daiToken.balanceOf(user1Wallet), "DAI balance does not increase");

        (emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/uniswap_test2.eml"),
            user1.accountKey,
            "Swap",
            "Swap 200 DAI to USDC",
            "gmail.com",
            "DAI"
        );
        extensionBytes = new bytes[](2);
        extensionBytes[0] = abi.encode(uint256(200 ether), "DAI");
        extensionBytes[1] = abi.encode("USDC");
        emailOp.extensionParams = ExtensionParams(0, extensionBytes);
        preDaiBalance = daiToken.balanceOf(user1Wallet);
        uint preUsdcNativeBalance = usdcNativeToken.balanceOf(user1Wallet);
        (success, reason, , ) = core.handleEmailOp(emailOp);
        require(success, string(reason));
        require(preDaiBalance > daiToken.balanceOf(user1Wallet), "DAI balance does not decrease");
        // TODO: What increases here is the usdcNativeToken.
        require(preUsdcNativeBalance < usdcNativeToken.balanceOf(user1Wallet), "USDC balance does not increase");

        // This test case is not working because we don't have 200 usdcNativeToken in the wallet.
        // (emailOp, ) = genEmailOpPartial(
        //     string.concat(vm.projectRoot(), "/test/emails/uniswap_test3.eml"),
        //     user1.accountKey,
        //     "Swap",
        //     "Swap 200 USDC to ETH",
        //     "gmail.com",
        //     "USDC"
        // );
        // extensionBytes = new bytes[](2);
        // extensionBytes[0] = abi.encode(uint256(200 * (10 ** 6)), "USDC");
        // extensionBytes[1] = abi.encode("ETH");
        // emailOp.extensionParams = ExtensionParams(0, extensionBytes);
        // preUsdcNativeBalance = usdcNativeToken.balanceOf(user1Wallet);
        // preEthBalance = weth.balanceOf(user1Wallet);
        // (success, reason, , ) = core.handleEmailOp(emailOp);
        // require(success, string(reason));
        // // TODO: What increases here is the usdcNativeToken.
        // require(preUsdcNativeBalance > usdcNativeToken.balanceOf(user1Wallet), "USDC balance does not decrease");
        // require(preEthBalance < weth.balanceOf(user1Wallet), "ETH balance does not increase");

        (emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/uniswap_test4.eml"),
            user1.accountKey,
            "Swap",
            "Swap 200 DAI to ETH",
            "gmail.com",
            "DAI"
        );
        extensionBytes = new bytes[](2);
        extensionBytes[0] = abi.encode(uint256(200 ether), "DAI");
        extensionBytes[1] = abi.encode("ETH");
        emailOp.extensionParams = ExtensionParams(0, extensionBytes);
        preDaiBalance = daiToken.balanceOf(user1Wallet);
        preEthBalance = weth.balanceOf(user1Wallet);
        (success, reason, , ) = core.handleEmailOp(emailOp);
        require(success, string(reason));
        require(preDaiBalance > daiToken.balanceOf(user1Wallet), "DAI balance does not decrease");
        require(preEthBalance < weth.balanceOf(user1Wallet), "ETH balance does not increase");

        // template index 1
        // Swap 0.2 ETH to DAI with 0.5 slippage
        (emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/uniswap_test5.eml"),
            user1.accountKey,
            "Swap",
            "Swap 0.2 ETH to DAI with 0.5 slippage",
            "gmail.com",
            "ETH"
        );
        extensionBytes = new bytes[](3);
        extensionBytes[0] = abi.encode(uint256(0.2 ether), "ETH");
        extensionBytes[1] = abi.encode("DAI");
        extensionBytes[2] = abi.encode(uint256(0.5 ether));
        emailOp.extensionParams = ExtensionParams(1, extensionBytes);
        preEthBalance = weth.balanceOf(user1Wallet);
        preDaiBalance = daiToken.balanceOf(user1Wallet);
        (success, reason, , ) = core.handleEmailOp(emailOp);
        require(success, string(reason));
        require(preEthBalance > weth.balanceOf(user1Wallet), "ETH balance does not decrease");
        require(preDaiBalance < daiToken.balanceOf(user1Wallet), "DAI balance does not increase");

        // This test case is not working because of the slippage changes frequently.
        // // template index 2
        // // Swap 0.2 ETH to DAI under 3627979510633925696217750990627 sqrt price limit
        // (emailOp, ) = genEmailOpPartial(
        //     string.concat(vm.projectRoot(), "/test/emails/uniswap_test6.eml"),
        //     relayer1Rand,
        //     "Swap",
        //     "Swap 0.2 ETH to DAI under 3627979510633925696217750990627 sqrt price limit",
        //     "gmail.com",
        //     "DAI"
        // );
        // extensionBytes = new bytes[](3);
        // extensionBytes[0] = abi.encode(uint256(0.2 ether), "ETH");
        // extensionBytes[1] = abi.encode("DAI");
        // extensionBytes[2] = abi.encode(uint(3627979510633925696217750990627));
        // emailOp.extensionParams = ExtensionParams(2, extensionBytes);
        // preEthBalance = weth.balanceOf(user1Wallet);
        // preDaiBalance = daiToken.balanceOf(user1Wallet);
        // (success, reason, ,) = core.handleEmailOp(emailOp);
        // require(success, string(reason));
        // require(preEthBalance > weth.balanceOf(user1Wallet), "ETH balance does not decrease");
        // require(preDaiBalance < daiToken.balanceOf(user1Wallet), "DAI balance does not increase");

        vm.stopPrank();
    }

    function testIntegration_Deposit_Transfer_Withdraw() public {
        address depositer = vm.addr(6);
        vm.startPrank(depositer);
        deal(depositer, 20 ether);
        weth.deposit{value: 20 ether}();
        bytes32 rand1 = 0x24b937a8b8ce44c9ae130d08ad77bd4456697b9ebf563b622a74448ab0fb8ca2;
        (bytes32 emailAddrCommit, bytes memory announcementProof) = genAnnouncement(user1.emailAddr, rand1);
        AllVerifiers verifier = AllVerifiers(address(core.verifier()));
        require(
            verifier.verifyAnnouncementProof(user1.emailAddr, rand1, emailAddrCommit, announcementProof),
            "invalid announcement proof"
        );
        deal(depositer, core.unclaimedFundClaimGas() * core.maxFeePerGas());
        require(weth.approve(address(unclaimsHandler), 0.5 ether), "approve failed");
        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.UnclaimedFundRegistered(
            0,
            emailAddrCommit,
            address(weth),
            0.5 ether,
            depositer,
            block.timestamp + unclaimsExpiryDuration,
            uint256(rand1),
            user1.emailAddr
        );
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedFund{
            value: core.unclaimedFundClaimGas() * core.maxFeePerGas()
        }(emailAddrCommit, address(weth), 0.5 ether, 0, uint256(rand1), user1.emailAddr);
        vm.stopPrank();

        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user1Wallet = address(user1WalletContract);
        claimFund(registeredUnclaimId, user1.emailAddr, rand1, user1.accountKey);
        require(
            weth.balanceOf(user1Wallet) == 0.5 ether,
            "User1 wallet balance after claiming unclaimed fund mismatch"
        );

        (EmailOp memory emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/token_transfer_test1.eml"),
            user1.accountKey,
            "Send",
            "Send 0.1 ETH to ",
            "gmail.com",
            "ETH"
        );
        emailOp.walletParams.tokenName = "ETH";
        emailOp.walletParams.amount = 0.1 ether;
        deal(relayer1, core.unclaimedFundClaimGas() * core.maxFeePerGas());
        (bool success, bytes memory reason, , ) = core.handleEmailOp{
            value: core.unclaimedFundClaimGas() * core.maxFeePerGas()
        }(emailOp);
        assertEq(success, true, string(reason));
        // weth.balanceOf(user1Wallet).logUint();
        require(
            weth.balanceOf(user1Wallet) < 0.4 ether,
            "User1 wallet balance after the first transaction is too large"
        );

        address recipient = vm.addr(4);
        (emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/token_transfer_test2.eml"),
            user1.accountKey,
            "Send",
            string.concat("Send 0.25 ETH to ", recipient.addressToChecksumHexString()),
            "gmail.com",
            "ETH"
        );
        emailOp.walletParams.tokenName = "ETH";
        emailOp.walletParams.amount = 0.25 ether;
        emailOp.recipientETHAddr = recipient;
        (success, reason, , ) = core.handleEmailOp{value: 0}(emailOp);
        assertEq(success, true, string(reason));
        require(
            weth.balanceOf(user1Wallet) < 0.15 ether,
            "User1 wallet balance after the second transaction is too large"
        );
        require(recipient.balance == 0.25 ether, "Recipient eth balance mismatch");
        require(weth.balanceOf(recipient) == 0, "Recipient weth balance must be zero");
        vm.stopPrank();
    }

    function testIntegration_Void_Transfer_Tokens() public {
        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user1Wallet = address(user1WalletContract);
        vm.stopPrank();

        vm.startPrank(user1Wallet);
        deal(user1Wallet, 0.15 ether);
        weth.deposit{value: 0.15 ether}();
        require(weth.balanceOf(user1Wallet) == 0.15 ether, "User1 wallet balance before the transaction mismatch");
        vm.stopPrank();

        vm.startPrank(relayer1);
        (EmailOp memory emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/token_transfer_test1.eml"),
            user1.accountKey,
            "Send",
            "Send 0.1 ETH to ",
            "gmail.com",
            "ETH"
        );
        emailOp.walletParams.tokenName = "ETH";
        emailOp.walletParams.amount = 0.1 ether;
        deal(relayer1, core.unclaimedFundClaimGas() * core.maxFeePerGas());
        (bool success, bytes memory reason, , uint256 registeredUnclaimId) = core.handleEmailOp{
            value: core.unclaimedFundClaimGas() * core.maxFeePerGas()
        }(emailOp);
        assertEq(success, true, string(reason));
        require(weth.balanceOf(user1Wallet) < 0.05 ether, "User1 wallet balance after the transaction is too large");
        require(weth.balanceOf(address(unclaimsHandler)) == 0.1 ether, "Core contract weth balance mismatch");
        require(
            address(unclaimsHandler).balance == core.unclaimedFundClaimGas() * core.maxFeePerGas(),
            "Core contract eth balance mismatch"
        );
        vm.stopPrank();

        address voider = vm.addr(7);
        vm.startPrank(voider);
        uint preWethBalance = weth.balanceOf(user1Wallet);
        (, , , , , uint256 expiryTime) = unclaimsHandler.unclaimedFundOfId(registeredUnclaimId);
        vm.warp(expiryTime + 1);
        unclaimsHandler.voidUnclaimedFund(registeredUnclaimId);
        require(
            weth.balanceOf(user1Wallet) - preWethBalance > 0.1 ether,
            "User 1 wallet balance after voiding the uf is too small"
        );
        require(
            voider.balance > 0 && voider.balance < core.unclaimedFundClaimGas() * core.maxFeePerGas(),
            "Voider ETH balance is incorrect"
        );
        vm.stopPrank();
    }

    function testIntegration_Transfer_NFT_To_Internal() public {
        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user1Wallet = address(user1WalletContract);
        vm.stopPrank();

        vm.startPrank(relayer1);
        DummyNFT ape = DummyNFT(nftExtension.addressOfNFTName("APE"));
        ape.freeMint(user1Wallet, 1);
        require(ape.ownerOf(1) == user1Wallet, "User1 wallet does not own APE");
        vm.stopPrank();
        
        vm.startPrank(user1Wallet);
        deal(user1Wallet, 0.15 ether);
        weth.deposit{value: 0.15 ether}();
        vm.stopPrank();

        vm.startPrank(relayer1);
        (EmailOp memory emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/install_nft.eml"),
            user1.accountKey,
            "Install",
            "Install extension NFT",
            "gmail.com",
            "ETH"
        );
        emailOp.extensionName = "NFT";
        (bool success, bytes memory reason, , ) = core.handleEmailOp(emailOp);
        require(success, string(reason));
        bytes32 emailAddrRand;
        (emailOp, emailAddrRand) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/nft_transfer_test1.eml"),
            user1.accountKey,
            "NFT",
            "NFT Send 1 of APE to ",
            "gmail.com",
            "ETH"
        );
        bytes[] memory extensionBytes = new bytes[](2);
        extensionBytes[0] = abi.encode(uint(1));
        extensionBytes[1] = abi.encode("APE");
        emailOp.extensionParams = ExtensionParams(0, extensionBytes);
        deal(relayer1, core.unclaimedStateClaimGas() * core.maxFeePerGas());
        uint256 registeredUnclaimId;
        (success, reason, , registeredUnclaimId) = core.handleEmailOp{
            value: core.unclaimedStateClaimGas() * core.maxFeePerGas()
        }(emailOp);
        require(success, string(reason));
        require(ape.ownerOf(1) == address(nftExtension), "Extension contract does not own APE");
        vm.stopPrank();

        vm.startPrank(relayer1);
        Wallet user2WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test2.eml"),
            user2.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user2Wallet = address(user2WalletContract);
        vm.stopPrank();

        vm.startPrank(relayer1);
        claimState(registeredUnclaimId, user2.emailAddr, emailAddrRand, user2.accountKey);
        require(ape.ownerOf(1) == user2Wallet, "User2 wallet does not own APE");
        vm.stopPrank();
    }

    function testIntegration_Transfer_NFT_To_External() public {
        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user1Wallet = address(user1WalletContract);
        vm.stopPrank();

        vm.startPrank(relayer1);
        DummyNFT ape = DummyNFT(nftExtension.addressOfNFTName("APE"));
        ape.freeMint(user1Wallet, 1);
        require(ape.ownerOf(1) == user1Wallet, "User1 wallet does not own APE");
        vm.stopPrank();

        vm.startPrank(user1Wallet);
        deal(user1Wallet, 0.15 ether);
        weth.deposit{value: 0.15 ether}();
        vm.stopPrank();

        vm.startPrank(relayer1);
        (EmailOp memory emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/install_nft.eml"),
            user1.accountKey,
            "Install",
            "Install extension NFT",
            "gmail.com",
            "ETH"
        );
        emailOp.extensionName = "NFT";
        (bool success, bytes memory reason, , ) = core.handleEmailOp(emailOp);
        require(success, string(reason));
        bytes32 emailAddrRand;
        address recipient = vm.addr(4);
        (emailOp, emailAddrRand) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/nft_transfer_test2.eml"),
            relayer1Rand,
            "NFT",
            string.concat("NFT Send 1 of APE to ", recipient.addressToChecksumHexString()),
            "gmail.com",
            "ETH"
        );
        bytes[] memory extensionBytes = new bytes[](2);
        extensionBytes[0] = abi.encode(uint(1));
        extensionBytes[1] = abi.encode("APE");
        emailOp.extensionParams = ExtensionParams(0, extensionBytes);
        emailOp.recipientETHAddr = recipient;
        deal(relayer1, core.unclaimedStateClaimGas() * core.maxFeePerGas());
        // (success, reason, , ) = core.handleEmailOp{value: core.unclaimedStateClaimGas() * core.maxFeePerGas()}(emailOp);
        // require(success, string(reason));
        // require(ape.ownerOf(1) == address(recipient), "Recipient does not own APE");
        vm.stopPrank();
    }

    function testIntegration_Deposit_NFT() public {
        address depositer = vm.addr(6);
        vm.startPrank(depositer);
        deal(depositer, 20 ether);
        weth.deposit{value: 20 ether}();
        bytes32 rand1 = 0x24b937a8b8ce44c9ae130d08ad77bd4456697b9ebf563b622a74448ab0fb8ca2;
        (bytes32 emailAddrCommit, bytes memory announcementProof) = genAnnouncement(user1.emailAddr, rand1);
        AllVerifiers verifier = AllVerifiers(address(core.verifier()));
        require(
            verifier.verifyAnnouncementProof(user1.emailAddr, rand1, emailAddrCommit, announcementProof),
            "invalid announcement proof"
        );
        deal(depositer, core.unclaimedStateClaimGas() * core.maxFeePerGas());
        DummyNFT ape = DummyNFT(nftExtension.addressOfNFTName("APE"));
        ape.freeMint(depositer, 1);
        ape.approve(address(nftExtension), 1);
        bytes memory unclaimedState = abi.encode(address(ape), 1);
        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.UnclaimedStateRegistered(
            0,
            emailAddrCommit,
            address(nftExtension),
            depositer,
            block.timestamp + unclaimsExpiryDuration,
            unclaimedState,
            uint256(rand1),
            user1.emailAddr
        );
        uint256 registeredUnclaimId = unclaimsHandler.registerUnclaimedState{
            value: core.unclaimedStateClaimGas() * core.maxFeePerGas()
        }(emailAddrCommit, address(nftExtension), unclaimedState, 0, uint256(rand1), user1.emailAddr);
        require(ape.ownerOf(1) == address(nftExtension), "Extension contract does not own APE");
        vm.stopPrank();

        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user1Wallet = address(user1WalletContract);
        vm.stopPrank();

        vm.startPrank(relayer1);
        claimState(registeredUnclaimId, user1.emailAddr, rand1, user1.accountKey);
        require(ape.ownerOf(1) == user1Wallet, "User1 wallet does not own APE");
        vm.stopPrank();
    }

    function testIntegration_Void_Transfer_NFT() public {
        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user1Wallet = address(user1WalletContract);
        vm.stopPrank();

        vm.startPrank(relayer1);
        DummyNFT ape = DummyNFT(nftExtension.addressOfNFTName("APE"));
        ape.freeMint(user1Wallet, 1);
        require(ape.ownerOf(1) == user1Wallet, "User1 wallet does not own APE");
        vm.stopPrank();

        vm.startPrank(user1Wallet);
        deal(user1Wallet, 0.15 ether);
        weth.deposit{value: 0.15 ether}();
        vm.stopPrank();

        vm.startPrank(relayer1);
        (EmailOp memory emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/install_nft.eml"),
            user1.accountKey,
            "Install",
            "Install extension NFT",
            "gmail.com",
            "ETH"
        );
        emailOp.extensionName = "NFT";
        (bool success, bytes memory reason, , ) = core.handleEmailOp(emailOp);
        require(success, string(reason));
        bytes32 emailAddrRand;
        (emailOp, emailAddrRand) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/nft_transfer_test1.eml"),
            user1.accountKey,
            "NFT",
            "NFT Send 1 of APE to ",
            "gmail.com",
            "ETH"
        );
        bytes[] memory extensionBytes = new bytes[](2);
        extensionBytes[0] = abi.encode(uint(1));
        extensionBytes[1] = abi.encode("APE");
        emailOp.extensionParams = ExtensionParams(0, extensionBytes);
        deal(relayer1, core.unclaimedStateClaimGas() * core.maxFeePerGas());
        uint256 registeredUnclaimId;
        (success, reason, , registeredUnclaimId) = core.handleEmailOp{
            value: core.unclaimedStateClaimGas() * core.maxFeePerGas()
        }(emailOp);
        require(success, string(reason));
        require(ape.ownerOf(1) == address(nftExtension), "Extension contract does not own APE");
        vm.stopPrank();

        address voider = vm.addr(7);
        vm.startPrank(voider);
        (, , , , , uint256 expiryTime) = unclaimsHandler.unclaimedStateOfId(registeredUnclaimId);
        vm.warp(expiryTime + 1);
        unclaimsHandler.voidUnclaimedState(registeredUnclaimId);
        require(ape.ownerOf(1) == user1Wallet, "User1 wallet does not own APE");
        require(
            voider.balance > 0 && voider.balance < core.unclaimedFundClaimGas() * core.maxFeePerGas(),
            "Voider ETH balance is incorrect"
        );
        vm.stopPrank();
    }

    function testIntegration_Approve_NFT() public {
        vm.startPrank(relayer1);
        Wallet user1WalletContract = accountCreation(
            string.concat(vm.projectRoot(), "/test/emails/account_creation_test1.eml"),
            user1.emailAddr,
            relayer1Rand,
            "gmail.com"
        );
        address user1Wallet = address(user1WalletContract);
        vm.stopPrank();

        vm.startPrank(relayer1);
        DummyNFT ape = DummyNFT(nftExtension.addressOfNFTName("APE"));
        ape.freeMint(user1Wallet, 1);
        require(ape.ownerOf(1) == user1Wallet, "User1 wallet does not own APE");
        vm.stopPrank();

        vm.startPrank(user1Wallet);
        deal(user1Wallet, 0.15 ether);
        weth.deposit{value: 0.15 ether}();
        vm.stopPrank();

        vm.startPrank(relayer1);
        (EmailOp memory emailOp, ) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/install_nft.eml"),
            user1.accountKey,
            "Install",
            "Install extension NFT",
            "gmail.com",
            "ETH"
        );
        emailOp.extensionName = "NFT";
        (bool success, bytes memory reason, , ) = core.handleEmailOp(emailOp);
        require(success, string(reason));
        bytes32 emailAddrRand;
        address recipient = vm.addr(4);
        (emailOp, emailAddrRand) = genEmailOpPartial(
            string.concat(vm.projectRoot(), "/test/emails/nft_approve_test1.eml"),
            user1.accountKey,
            "NFT",
            string.concat("NFT Approve ", recipient.addressToChecksumHexString(), " for 1 of APE"),
            "gmail.com",
            "ETH"
        );
        bytes[] memory extensionBytes = new bytes[](2);
        extensionBytes[0] = abi.encode(uint(1));
        extensionBytes[1] = abi.encode("APE");
        emailOp.extensionParams = ExtensionParams(1, extensionBytes);
        emailOp.recipientETHAddr = recipient;
        deal(relayer1, core.unclaimedStateClaimGas() * core.maxFeePerGas());
        (success, reason, , ) = core.handleEmailOp{value: core.unclaimedStateClaimGas() * core.maxFeePerGas()}(emailOp);
        require(success, string(reason));
        require(ape.ownerOf(1) == user1Wallet, "User1 wallet should still own APE");
        require(ape.getApproved(1) == recipient, "Recipient should be approved for APE");
        vm.stopPrank();
    }
}
