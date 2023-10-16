// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "../src/dkimRegistryOwners/ECDSAOwner.sol";

contract ECDSAOwnerTest is Test {
    ECDSAOwner owner;
    uint256 constant EXPECTED_PUBKEY_HASH = 0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788;
    using console for *;

    function setUp() public {
        owner = new ECDSAOwner(0xD47297cD18b82685cae80442d74E46Fe66430cFB);
    }

    function test_SetDKIMPublicKeyHash() public {
        vm.chainId(1);
        owner.setDKIMPublicKeyHash(
            "20230601",
            "gmail.com",
            EXPECTED_PUBKEY_HASH,
            vm.parseBytes(
                "0xe8acbbb17d3c3a91984d0fc9bf90d78ce17ea402609d8d67b03e4e6d5cf09e5a2c4a9debd51b2a45a37864db2d30e45faf48a1052ae15dd2d601e12d57302a471c"
            )
        );
        require(owner.getDKIMPublicKeyHash("gmail.com") == EXPECTED_PUBKEY_HASH, "Invalid public key hash");
        owner.setDKIMPublicKeyHash(
            "20230601",
            "gmail.com",
            EXPECTED_PUBKEY_HASH,
            vm.parseBytes(
                "0x3d2fd57094a004d496b36ec79f1d80817db75e9a25e029236548b4c0c1577cd50a6266cf8ae57f937c0e2dd672c748e7e4155dd9f3497cf513e0952afec1ad0f1b"
            )
        );
        require(owner.getDKIMPublicKeyHash("gmail.com") == EXPECTED_PUBKEY_HASH, "Invalid public key hash");
        owner.setDKIMPublicKeyHash(
            "20230601",
            "gmail.com",
            EXPECTED_PUBKEY_HASH,
            vm.parseBytes(
                "0x4409648a32c4860528f8c476005205a59079fe38d12cc814c552efb0d25960ef08e6ba93efc1b54ac5860260b4463b8492c4f2bbb1d4de908c4aacd182e6eaab1b"
            )
        );
        require(owner.getDKIMPublicKeyHash("gmail.com") == EXPECTED_PUBKEY_HASH, "Invalid public key hash");
        owner.setDKIMPublicKeyHash(
            "20230601",
            "gmail.com",
            EXPECTED_PUBKEY_HASH,
            vm.parseBytes(
                "0xcf1b5d25cfa35ae446fd73ab687f570e3bc534e65c3325339daac5f0fe4ac79a4608169cf39a324fd62d68a24a44a3dad731340bbeb3993a141b0d36699a775f1c"
            )
        );
        require(owner.getDKIMPublicKeyHash("gmail.com") == EXPECTED_PUBKEY_HASH, "Invalid public key hash");
    }
}
