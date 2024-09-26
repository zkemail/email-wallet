// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

contract EmailWalletCoreTest is EmailWalletCoreTestHelper {

    function setUp() public override {
        super.setUp();
        _registerRelayer();
        _createTestAccount();
    }

    function testUpgradeability() public {
        EmailWalletCore implV2 = new EmailWalletCore();

        vm.startPrank(deployer);
        core.upgradeToAndCall(address(implV2), new bytes(0));
        vm.stopPrank();
    }
}
