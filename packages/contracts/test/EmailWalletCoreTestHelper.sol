// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "@zk-email/contracts/DKIMRegistry.sol";
import "../src/EmailWalletCore.sol";
import "../src/utils/TokenRegistry.sol";
import "../src/utils/UniswapTWAPOracle.sol";
import "./mock/TestVerifier.sol";

contract EmailWalletCoreTestHelper is Test {
    EmailWalletCore core;
    address deployer;
    address relayer;

    bytes mockProof = abi.encodePacked(bytes1(0x01));

    // Relayer details
    uint256 relayerRand = 10001;
    bytes32 randHash = keccak256(abi.encodePacked(relayerRand));

    // User details (sender) - for when sender failure is not expected
    // Computing hashes to resemble the actual process
    string senderEmail = "sender@test.com";
    string emailDomain = "test.com";
    uint256 viewingKey = 2001;
    bytes32 emailAddrPointer = keccak256(abi.encodePacked(relayerRand, senderEmail));
    bytes32 viewingKeyCommitment = keccak256(abi.encodePacked(viewingKey, senderEmail, randHash));
    bytes32 walletSalt = keccak256(abi.encodePacked(viewingKeyCommitment, uint(0)));
    bytes psiPoint = abi.encodePacked(uint(1004));

    function setUp() public virtual {
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

        dkimRegistry.setDKIMPublicKeyHash(emailDomain, uint256(111122223333));

        vm.stopPrank();
    }

    function _registerRelayer() internal {
        vm.startPrank(relayer);
        core.registerRelayer(randHash, "relayer@relayer.xyz", "relayer.xyz");
        vm.stopPrank();
    }
}
