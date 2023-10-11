// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "@openzeppelin/contracts-upgradeable/utils/Create2Upgradeable.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "../src/Wallet.sol";
import "./mock/WETH9.sol";

contract TestWallet is Wallet {
    constructor(address wethAddress) Wallet(wethAddress) {}

    function getName() public pure returns (string memory) {
        return "Test";
    }
}

contract WalletTest is Test {
    WETH9 weth;
    Wallet public walletImplementation;

    // Below methods are used for deploying upgradeable deterministic wallets
    // They are the similar to the code used in EmailWalletCore
    function _deployWallet(bytes32 salt) internal returns (Wallet wallet) {
        wallet = Wallet(
            payable(
                new ERC1967Proxy{salt: bytes32(salt)}(
                    address(walletImplementation),
                    abi.encodeCall(Wallet.initialize, ())
                )
            )
        );
    }

    function _getWalletOfSalt(bytes32 salt) internal view returns (address) {
        return
            Create2Upgradeable.computeAddress(
                salt,
                keccak256(
                    abi.encodePacked(
                        type(ERC1967Proxy).creationCode,
                        abi.encode(address(walletImplementation), abi.encodeCall(Wallet.initialize, ()))
                    )
                )
            );
    }

    function setUp() public {
        weth = new WETH9();
        walletImplementation = new Wallet(address(weth));
    }

    function testWalletDeploy() public {
        bytes32 salt = bytes32(uint(1001));
        Wallet wallet = _deployWallet(salt);

        assertEq(_getWalletOfSalt(salt), address(wallet));
        assertEq(wallet.owner(), address(this)); // Verify deployed (test contract) is owner
    }

    function testWalletExecution() public {
        bytes32 salt = bytes32(uint(1002));
        Wallet wallet = _deployWallet(salt);

        wallet.execute(address(wallet), 0, ""); // Should be able to execute as owner
    }

    function testCannotExecuteAsNonOwner() public {
        bytes32 salt = bytes32(uint(1003));
        Wallet wallet = _deployWallet(salt);

        vm.startPrank(vm.addr(10));
        vm.expectRevert();
        wallet.execute(address(wallet), 0, ""); // Should not be able to execute as non-owner
        vm.stopPrank();
    }

    function testWalletOwnershipChange() public {
        bytes32 salt = bytes32(uint(1002));
        address newOwner = vm.addr(2);
        Wallet wallet = _deployWallet(salt);
        wallet.transferOwnership(newOwner);

        assertEq(wallet.owner(), newOwner);

        vm.startPrank(newOwner);
        wallet.execute(address(wallet), 0, ""); // Should be able to execute as new owner
        vm.stopPrank();
    }

    function testWalletUpgrade() public {
        bytes32 salt = bytes32(uint(1003));
        Wallet wallet = _deployWallet(salt);

        TestWallet newImplementation = new TestWallet(address(weth));
        wallet.upgradeTo(address(newImplementation)); // Upgrade to new test implementation

        TestWallet wallet2 = TestWallet(payable(address(wallet)));
        assertEq(wallet2.getName(), "Test"); // Test function from new implementation
    }

    function testETHDepositFromEOAShouldConvertToWETH() public {
        bytes32 salt = bytes32(uint(1004));
        Wallet wallet = _deployWallet(salt);

        address eoa = vm.addr(10);
        vm.deal(eoa, 1 ether);

        vm.startPrank(eoa);
        (bool success, ) = address(wallet).call{value: 1 ether}(""); // Send ETH
        vm.stopPrank();

        assertEq(success, true, "send ETH failed");
        assertEq(weth.balanceOf(address(wallet)), 1 ether, "weth balance is not 1 ether");
        assertEq(eoa.balance, 0, "eoa balance is not zero");
        assertEq(address(wallet).balance, 0, "wallet balance is not zero");
    }

    function testETHDepositFromContractShouldConvertToWETH() public {
        bytes32 salt = bytes32(uint(1004));
        Wallet wallet = _deployWallet(salt);

        vm.deal(address(this), 1 ether);
        (bool success, ) = address(wallet).call{value: 1 ether}(""); // Send ETH

        assertEq(success, true, "send ETH failed");
        assertEq(weth.balanceOf(address(wallet)), 1 ether, "weth balance is not 1 ether");
        assertEq(address(this).balance, 0, "eoa balance is not zero");
        assertEq(address(wallet).balance, 0, "wallet balance is not zero");
    }
}
