// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

contract RelayerTest is EmailWalletCoreTestHelper {
    function test_RegisterRelayer() public {
        string memory emailAddr = "relayer@domain.com";
        string memory hostname = "relayer.xyz";

        vm.startPrank(relayer);
        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.RelayerRegistered(relayer, emailAddr, hostname);

        relayerHandler.registerRelayer(emailAddr, hostname);
        vm.stopPrank();

        (string memory deployedEmailAddr, ) = relayerHandler.relayers(relayer);
        assertTrue(Strings.equal(deployedEmailAddr, emailAddr));
    }

    // randHash is never used
    // // Same relayer wallet registering twice with differend randHash
    // function test_RevertWhen_RegisteringRelayerTwice() public {
    //     bytes32 randHash = keccak256(abi.encodePacked(uint(1001)));
    //     bytes32 randHash2 = keccak256(abi.encodePacked(uint(1002)));

    //     vm.startPrank(relayer);
    //     relayerHandler.registerRelayer("relayer@domain.com", "relayer.xyz");
    //     vm.expectRevert("relayer already registered");
    //     relayerHandler.registerRelayer("relayer2@domain.com", "relayer2.xyz");
    //     vm.stopPrank();
    // }

    // randHash is never used
    // // Different relayer registering with same randHash
    // function test_RevertWhen_RegisteringRelayerRandHashTwice() public {
    //     bytes32 randHash = keccak256(abi.encodePacked(uint(1001)));

    //     vm.startPrank(relayer);
    //     relayerHandler.registerRelayer("relayer@domain.com", "relayer.xyz");
    //     vm.stopPrank();

    //     vm.startPrank(vm.addr(3));
    //     vm.expectRevert("randHash already registered");
    //     relayerHandler.registerRelayer("relayer2@domain.com", "relayer2.xyz");
    //     vm.stopPrank();
    // }

    // Different relayer registering with same emailAddr
    function test_RevertWhen_RegisteringRelayerEmailAddrTwice() public {
        // bytes32 randHash = keccak256(abi.encodePacked(uint(1001)));
        // bytes32 randHash2 = keccak256(abi.encodePacked(uint(1002)));

        vm.startPrank(relayer);
        relayerHandler.registerRelayer("relayer@domain.com", "relayer.xyz");
        vm.stopPrank();

        vm.startPrank(vm.addr(3));
        vm.expectRevert("emailAddr already registered");
        relayerHandler.registerRelayer("relayer@domain.com", "relayer2.xyz");
        vm.stopPrank();
    }

    // Update relayer hostname
    function test_UpdateRelayerHostname() public {
        // bytes32 randHash = keccak256(abi.encodePacked(uint(1001)));
        string memory newHostname = "newdomain.xyz";

        vm.startPrank(relayer);
        relayerHandler.registerRelayer("relayer@domain.com", "relayer.xyz");

        vm.expectEmit(true, true, true, true);
        emit EmailWalletEvents.RelayerConfigUpdated(relayer, newHostname);
        relayerHandler.updateRelayerConfig(newHostname);
        vm.stopPrank();

        (, string memory hostname) = relayerHandler.relayers(relayer);

        assertTrue(Strings.equal(hostname, newHostname));
    }

    function testUpgradeability() public {
        RelayerHandler implV2 = new RelayerHandler();

        vm.startPrank(deployer);
        relayerHandler.upgradeTo(address(implV2));
        vm.stopPrank();
    }
}
