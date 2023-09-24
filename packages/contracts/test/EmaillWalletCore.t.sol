// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "@openzeppelin/contracts-upgradeable/utils/Create2Upgradeable.sol";
import "@zk-email/contracts/DKIMRegistry.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "../src/EmailWalletCore.sol";
import "../src/utils/TokenRegistry.sol";
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

        bytes memory data = abi.encodeCall(
            EmailWalletCore.initialize,
            (address(verifier), address(tokenRegistry), address(dkimRegistry), 10 ** 10, 0.0001 ether, 30 days)
        );

        core = EmailWalletCore(address(new ERC1967Proxy(implementation, data)));

        vm.stopPrank();
    }

    function testRegisterRelayer() public {
        vm.startPrank(relayer);

        bytes32 randHash = keccak256(abi.encodePacked("relayer"));
        core.registerRelayer(randHash, "relayer@domain.com", "relayer.xyz");

        vm.stopPrank();

        (bytes32 deployedRandHash, ,) = core.relayers(relayer);
        assertTrue(deployedRandHash == randHash);
    }
}
