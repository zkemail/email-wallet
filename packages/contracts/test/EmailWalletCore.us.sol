// // SPDX-License-Identifier: MIT
// pragma solidity ^0.8.12;

// import "./helpers/EmailWalletCoreTestHelper.sol";

// con

// contract UnclaimedFundTest is EmailWalletCoreTestHelper {
//     function setUp() public override {
//         super.setUp();
//         _registerRelayer();
//         _registerAndInitializeAccount();
//     }

//     // Internally means that the unclaimed fund is registered by handleEmailOp (send tokent to email)
//     function test_UnclaimedStateInternal_RegisterByExtension() public {
//         string memory subject = "Send 100 DAI to ";
//         bytes32 recipientEmailAddrCommit = bytes32(uint256(32333));

//         // this need to be send to handleEmailOp for registering unclaimed funds
//         vm.deal(relayer, unclaimedFundClaimGas * maxFeePerGas);

//         // Mint 150 DAI to sender wallet (will send 100 DAI to recipient)
//         daiToken.freeMint(walletAddr, 100 ether);

//         EmailOp memory emailOp = _getBaseEmailOp();
//         emailOp.command = Commands.SEND;
//         emailOp.walletParams.tokenName = "DAI";
//         emailOp.walletParams.amount = 100 ether;
//         emailOp.hasEmailRecipient = true;
//         emailOp.recipientEmailAddrCommit = recipientEmailAddrCommit;
//         emailOp.maskedSubject = subject;

//         vm.startPrank(relayer);
//         vm.expectEmit();
//         emit UnclaimedFundRegistered(
//             recipientEmailAddrCommit,
//             address(daiToken),
//             100 ether,
//             walletAddr, // walletAddr should be sender
//             block.timestamp + unclaimedFundExpirationDuration, // default expiry
//             0,
//             ""
//         );
//         (bool success, ) = core.handleEmailOp{value: unclaimedFundClaimGas * maxFeePerGas}(emailOp);
//         vm.stopPrank();

//         assertEq(success, true, "handleEmailOp failed");

//         (bytes32 emailAddrCommit, address sender, address tokenAddr, uint256 amount, uint256 expiryTime) = core
//             .unclaimedFundOfEmailAddrCommit(recipientEmailAddrCommit);

//         assertEq(emailAddrCommit, recipientEmailAddrCommit, "emailAddrCommit mismatch");
//         assertEq(sender, walletAddr, "sender mismatch");
//         assertEq(tokenAddr, address(daiToken), "tokenName mismatch");
//         assertEq(amount, 100 ether, "amount mismatch");

//         // Should use default expiry
//         assertEq(expiryTime, block.timestamp + unclaimedFundExpirationDuration, "expiryTime mismatch");

//         // Core contract should have the balance
//         assertEq(daiToken.balanceOf(address(core)), 100 ether, "core contract didnt receive the tokens");
//     }
// }
