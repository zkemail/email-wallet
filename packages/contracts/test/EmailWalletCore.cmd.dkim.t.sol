// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";
import {TestDKIMRegistry} from "./mocks/TestDKIMRegistry.sol";

contract DKIMRegistryCommandTest is EmailWalletCoreTestHelper {
    address extensionAddr;
    string[][] tempaltes = new string[][](1);
    string extensionName = "TestSwap";

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _createTestAccount();
    }

    function test_SetCustomDKIMRegistry() public {
        address dkimRegistryAddr = address(new TestDKIMRegistry());
        string memory subject = string.concat(
            "DKIM registry set to ",
            SubjectUtils.addressToChecksumHexString(dkimRegistryAddr)
        );

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.DKIM;
        emailOp.newDkimRegistry = dkimRegistryAddr;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "emailOp failed");
        assertEq(accountHandler.dkimRegistryOfWalletSalt(walletSalt), dkimRegistryAddr, "didnt set DKIM registry");
    }

    function test_SetCustomDKIMRegistryAlthoughAfterTimeLimit() public {
        vm.warp(1701388800);
        address dkimRegistryAddr = address(new TestDKIMRegistry());
        string memory subject = string.concat(
            "DKIM registry set to ",
            SubjectUtils.addressToChecksumHexString(dkimRegistryAddr)
        );

        EmailOp memory emailOp = _getBaseEmailOp();
        emailOp.command = Commands.DKIM;
        emailOp.newDkimRegistry = dkimRegistryAddr;
        emailOp.maskedSubject = subject;

        vm.startPrank(relayer);
        (bool success, , , ) = core.handleEmailOp(emailOp);
        vm.stopPrank();

        assertTrue(success, "emailOp failed");
        assertEq(accountHandler.dkimRegistryOfWalletSalt(walletSalt), dkimRegistryAddr, "didnt set DKIM registry");
    }
}
