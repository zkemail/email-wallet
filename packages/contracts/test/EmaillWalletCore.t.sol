// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "@openzeppelin/contracts-upgradeable/utils/Create2Upgradeable.sol";
import "@zk-email/contracts/DKIMRegistry.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "../src/EmailWalletCore.sol";
import "../src/utils/TokenRegistry.sol";
import "../src/utils/UniswapTWAPOracle.sol";
import "./mock/TestVerifier.sol";

contract EmailWalletCoreTest is Test {
    EmailWalletCore core;
    address deployer;
    address relayer;

    function setUp() public {
        deployer = vm.addr(1);
        relayer = vm.addr(2);

        vm.startPrank(deployer);

        address implementation = address(new EmailWalletCore());
        TestVerifier verifier = new TestVerifier();
        TokenRegistry tokenRegistry = new TokenRegistry();
        DKIMRegistry dkimRegistry = new DKIMRegistry();
        IPriceOracle priceOracle = new UniswapTWAPOracle(address(0), address(0));

        bytes memory data = abi.encodeCall(
            EmailWalletCore.initialize,
            (
                address(verifier),
                address(tokenRegistry),
                address(dkimRegistry),
                address(priceOracle),
                10 ** 10,
                0.0001 ether,
                30 days
            )
        );

        core = EmailWalletCore(address(new ERC1967Proxy(implementation, data)));

        vm.stopPrank();
    }

    function testRegisterRelayerSuccessfully() public {
        bytes32 randHash = keccak256(abi.encodePacked("relayer"));

        vm.startPrank(relayer);
        core.registerRelayer(randHash, "relayer@domain.com", "relayer.xyz");
        vm.stopPrank();

        (bytes32 deployedRandHash, , ) = core.relayers(relayer);
        assertTrue(deployedRandHash == randHash);
    }

    // Same relayer wallet registering twice with differend randHash
    function testRevertWhenRegisteringRelayerTwice() public {
        bytes32 randHash = keccak256(abi.encodePacked("relayer"));
        bytes32 randHash2 = keccak256(abi.encodePacked("relayer2"));

        vm.startPrank(relayer);
        core.registerRelayer(randHash, "relayer@domain.com", "relayer.xyz");
        vm.expectRevert("relayer already registered");
        core.registerRelayer(randHash2, "relayer2@domain.com", "relayer2.xyz");
        vm.stopPrank();
    }

    // Different relayer registering with same randHash
    function testRevertWhenRegisteringRelayerRandHashTwice() public {
        bytes32 randHash = keccak256(abi.encodePacked("relayer"));
        
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
        bytes32 randHash = keccak256(abi.encodePacked("relayer"));
        bytes32 randHash2 = keccak256(abi.encodePacked("relayer2"));

        vm.startPrank(relayer);
        core.registerRelayer(randHash, "relayer@domain.com", "relayer.xyz");
        vm.stopPrank();

        vm.startPrank(vm.addr(3));
        vm.expectRevert("emailAddr already registered");
        core.registerRelayer(randHash2, "relayer@domain.com", "relayer2.xyz");
        vm.stopPrank();
    }

}
