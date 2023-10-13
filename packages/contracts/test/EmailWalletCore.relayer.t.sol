// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "./helpers/EmailWalletCoreTestHelper.sol";

contract RelayerTest is EmailWalletCoreTestHelper {
    function testRegisterRelayer() public {
        bytes32 randHash = keccak256(abi.encodePacked(uint(1001)));

        vm.startPrank(relayer);
        core.registerRelayer(randHash, "relayer@domain.com", "relayer.xyz");
        vm.stopPrank();

        (bytes32 deployedRandHash, , ) = core.relayers(relayer);
        assertTrue(deployedRandHash == randHash);
    }

    // Same relayer wallet registering twice with differend randHash
    function testRevertWhenRegisteringRelayerTwice() public {
        bytes32 randHash = keccak256(abi.encodePacked(uint(1001)));
        bytes32 randHash2 = keccak256(abi.encodePacked(uint(1002)));

        vm.startPrank(relayer);
        core.registerRelayer(randHash, "relayer@domain.com", "relayer.xyz");
        vm.expectRevert("relayer already registered");
        core.registerRelayer(randHash2, "relayer2@domain.com", "relayer2.xyz");
        vm.stopPrank();
    }

    // Different relayer registering with same randHash
    function testRevertWhenRegisteringRelayerRandHashTwice() public {
        bytes32 randHash = keccak256(abi.encodePacked(uint(1001)));

        vm.startPrank(relayer);
        core.registerRelayer(randHash, "relayer@domain.com", "relayer.xyz");
        vm.stopPrank();

        vm.startPrank(vm.addr(3));
        vm.expectRevert("randHash already registered");
        core.registerRelayer(randHash, "relayer2@domain.com", "relayer2.xyz");
        vm.stopPrank();
    }

    // Different relayer registering with same emailAddr
    function testRevertWhenRegisteringRelayerEmailAddrTwice() public {
        bytes32 randHash = keccak256(abi.encodePacked(uint(1001)));
        bytes32 randHash2 = keccak256(abi.encodePacked(uint(1002)));

        vm.startPrank(relayer);
        core.registerRelayer(randHash, "relayer@domain.com", "relayer.xyz");
        vm.stopPrank();

        vm.startPrank(vm.addr(3));
        vm.expectRevert("emailAddr already registered");
        core.registerRelayer(randHash2, "relayer@domain.com", "relayer2.xyz");
        vm.stopPrank();
    }

    // Update relayer hostname
    function testUpdateRelayerHostname() public {
        bytes32 randHash = keccak256(abi.encodePacked(uint(1001)));

        vm.startPrank(relayer);
        core.registerRelayer(randHash, "relayer@domain.com", "relayer.xyz");
        core.updateRelayerConfig("newdomain.xyz");
        vm.stopPrank();

        (, , string memory hostname) = core.relayers(relayer);
        assertTrue(Strings.equal(hostname, "newdomain.xyz"));
    }
}
