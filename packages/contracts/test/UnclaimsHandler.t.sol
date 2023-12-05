// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

contract UnclaimsTest is EmailWalletCoreTestHelper {
    function setUp() public override {
        super.setUp();
        _registerRelayer();
    }

    function testUpgradeability() public {
        UnclaimsHandler implV2 = new UnclaimsHandler();

        vm.startPrank(deployer);
        unclaimsHandler.upgradeTo(address(implV2));
        vm.stopPrank();
    }
}
