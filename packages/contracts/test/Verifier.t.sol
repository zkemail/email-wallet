// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "../src/verifier/AccountCreationVerifier.sol";
import "../src/verifier/AccountInitVerifier.sol";
import "../src/verifier/AccountTransportVerifier.sol";
import "../src/verifier/ClaimVerifier.sol";
import "../src/verifier/EmailSenderVerifier.sol";
import "../src/verifier/AnnouncementVerifier.sol";
import "../src/verifier/Verifier.sol";

contract VerifierTest is Test {
    AllVerifiers verifier;

    function setUp() public {
        address accountCreationVerifier = address(new AccountCreationVerifier());
        address accountInitVerifier = address(new AccountInitVerifier());
        address accountTransportVerifier = address(new AccountTransportVerifier());
        address claimVerifier = address(new ClaimVerifier());
        address emailSenderVerifier = address(new EmailSenderVerifier());
        address announcementVerifier = address(new AnnouncementVerifier());
        verifier = new AllVerifiers(
            accountCreationVerifier,
            accountInitVerifier,
            accountTransportVerifier,
            claimVerifier,
            emailSenderVerifier,
            announcementVerifier
        );
    }

    function testVerifier_AccountCreation() public view {
        uint256[2] memory pA = [
            0x1dd253dc6a8d47cc17357e9f06ee03b34e7b1c84b91eff5fd176582953286220,
            0x1f731de17322110b1a1cb23be3c62b6336b74a8f9d9786686c3f9d3a4e6193c8
        ];
        uint256[2][2] memory pB = [
            [
                0x17087c46e4b2bc49207a88e5ea771df114251ddb6fb34c7e1fbee919996fbacd,
                0x0abc9cbd05f082ce30b4e60301ae9a1fab204c02d33f9ec615a96b4a63617a09
            ],
            [
                0x134592c00f8cac17886d59c0bb3ac1866026cd36f394377cc49625aa7f0ac244,
                0x1a719f68203f719eab47b2f35798cab8f04b61211bf86ecbfae8fb7388795192
            ]
        ];
        uint256[2] memory pC = [
            0x0101bb968cf2240eba1ad458ee758c8bd5ffc31cc9f8b81e9ddd1a09cffa6e2a,
            0x12e7fd86bf0fe2bb3ed68b74a8accf086bf59481fc36982891af2ab240eed2b4
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        bytes memory psiPoint = abi.encode(
            0x174143d11cc953c0477683692a4a655f75954b36d01662728ba5ce1e5c78d28d,
            0x0e7662bbbc708e0f6371dedca8fdc916baa5d0bbd50e737f5a8a3d127d821cd7
        );
        require(
            verifier.verifyAccountCreationProof(
                0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3,
                0x1ff706660702f76a0daa706d68b15ea04fb6145fb5f4e54823ae80fa386e1b3f,
                0x0136e61d55558414797fa9e8acccac39c52ef0b2c9b3fda0ef2d858a111333df,
                0x16dc3fd3780b524ba792b9e19ec9f7cbeb931912462a9b028cecfdff0eb29d28,
                psiPoint,
                proof
            ),
            "Account creation proof verification failed"
        );
    }

    function testVerifier_AccountInit() public view {
        uint256[2] memory pA = [
            0x2956893e19f52ce1c9f74ca09f98289349c580853ebf62f285eb1943e5f05bb6,
            0x138d32d31cc1839664c968966c056006f9a018f1bf86df82c43671b88b64bf26
        ];
        uint256[2][2] memory pB = [
            [
                0x292bbe08f38f0bed03539488d63f50b11bda4910ddef0201b2d72a9947737bc6,
                0x2f91572c0357e2a9090a641e2cd9934a49f0f2672b892492c3cd32f59a36b6ff
            ],
            [
                0x0bf55031b123fd344ce3951a4c2abf10556bbbba454706ee2deb87dd73d519c6,
                0x0dfaf17d22dfb48ab4f39c9e45d2f361a329d5b0c2655b235b05584e0d788283
            ]
        ];
        uint256[2] memory pC = [
            0x284acbfdb235ce85480ee28db3459f5071d3fc6ffd646905784edd37e678f737,
            0x0444cf3b416c62ef640f98817eceedde0b663f7cd0b0bdd91887149aa071967e
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAccountInitializaionProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                0x0000000000000000000000000000000000000000000000000000000065298ddf,
                0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3,
                0x1ff706660702f76a0daa706d68b15ea04fb6145fb5f4e54823ae80fa386e1b3f,
                0x0136e61d55558414797fa9e8acccac39c52ef0b2c9b3fda0ef2d858a111333df,
                0x268bc70efa64d6289086c536bedad66017705a768a7e9114ff4bf0bababb26e6,
                proof
            ),
            "Account init proof verification failed"
        );
    }

    function testVerifier_AccountTransport() public view {
        uint256[2] memory pA = [
            0x0a221982427a514d01d7c2325814f8af820a31fbc3012f0f47a4ac4d34724e77,
            0x2d12a18533cbce0ac3bf9c320528dddd16ca3f8bb4defe34cdb6788f1829f22a
        ];
        uint256[2][2] memory pB = [
            [
                0x00b9fb23f3aed28f17700876efdaebfbb639126a33af3806e5500672630e7eb6,
                0x2af2e5a04a2f584f2e9a0c9c27eb79c08ccaf7240e9d1cca53ce37ccd6800ffb
            ],
            [
                0x0aa2ea1483714ce3ef44bef09dc02246635d6ce657897038c95e7dfbe3937681,
                0x0e9873cfb8a129f42d214ad892f7afa3c4fe282b4531ee83f15046716e9377d5
            ]
        ];
        uint256[2] memory pC = [
            0x1f2cf420351c58301e8d86812f6dee9a2d130d382f1ab6bc355dd246b1f01ec8,
            0x2a58235f6c9599c5608dd03aa94a752532bcc5224a76972a6eb0d0554373d0c7
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAccountTransportProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                0x0000000000000000000000000000000000000000000000000000000065298ddf,
                0x268bc70efa64d6289086c536bedad66017705a768a7e9114ff4bf0bababb26e6,
                0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3,
                0x2451956f89b22a433050f391776b5b00e53616ceed3313c0c3e1754d3f1d9a50,
                0x0136e61d55558414797fa9e8acccac39c52ef0b2c9b3fda0ef2d858a111333df,
                0x2f666c5540d60627d08b48ec2ac8a328b1e842f298b259d818fec166baf920d9,
                proof
            ),
            "Account transport proof verification failed"
        );
    }

    function testVerifier_ClaimFund() public view {
        uint256[2] memory pA = [
            0x1faa2cfd93ef8f3d9a3fa26782812e569a246e62163773a16edd0b8d84383558,
            0x118bcf03234571c4b87fba6064b5d5f5da293dba20fab7eda3dfe292d748810d
        ];
        uint256[2][2] memory pB = [
            [
                0x16ca0458e79b8b7c675c506ccd8c2cbb4c0a88c289328215c562df54bf4437f2,
                0x20ecef84449cd965c6dc939f979ee8d13a32a36dbcb93b27dc30dab5c4300b3d
            ],
            [
                0x182c6dd275aa1a0cef8c96d561bef6f4b217eee8167c26fd7f49d8c2854212a0,
                0x10637f3d0b66a7322898fa1417263f294ef3ac57f7687ef58824a9aa63aac6e3
            ]
        ];
        uint256[2] memory pC = [
            0x0f9bbc90a2b3bb3dbae282cfa36ee3bb7feff395eb8d4ca7abcc41e083ebf5ca,
            0x22bc64aeb1d7497ae6d9c97f8bc0e7e2de57a9c5dbd61509c7b6f226c4f11d74
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyClaimFundProof(
                0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3,
                0x1ff706660702f76a0daa706d68b15ea04fb6145fb5f4e54823ae80fa386e1b3f,
                0x0dc9e2309f2f09c15b3bc05870142bd23e570e5fd3365160ded4067f9178ccec,
                proof
            ),
            "Claim fund proof verification failed"
        );
    }

    function testVerifier_EmailSenderProof() public view {
        uint256[2] memory pA = [
            0x208d493fa45d015a3e49ad0011a1fda0d5bb301db5a71c8bf52172591e95d63f,
            0x2dda94a6dd2426bc72f6d8e8f835778ef67005ee6823b854b55d6268cf8773fe
        ];
        uint256[2][2] memory pB = [
            [
                0x0ec212e956931579a14c636e68e37a6e479375b17dde9adc777a21ebe3ef45db,
                0x0250ba4e5f224dd7c292a555ec44be8386e8e7eebab51b754c86bc83e4e8426e
            ],
            [
                0x12645ac2fca0bea83b8eddbdc0f8cbdd6ef5567164011f2d28a655f83a093985,
                0x0e99330918fa4be57a54aafeb8182069bbf3aed244658a5227f8c11261c40111
            ]
        ];
        uint256[2] memory pC = [
            0x0065e32a836565f07e24d9a1253e298dc41cfb9ce64a8d570dc932c0af4ce44b,
            0x033150ec19a4d78c3035115e815900e5b804de3fac1bb5573a29a3de1d8348bc
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyEmailOpProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                0x00000000000000000000000000000000000000000000000000000000652a82f3,
                "Send 0.1 ETH to ",
                0x2debe3e9a496b64e9e731a9ef99e675548e43c89e34fc8b3f5e1b4128bd6f8a8,
                0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3,
                0x1ff706660702f76a0daa706d68b15ea04fb6145fb5f4e54823ae80fa386e1b3f,
                true,
                0x002f64d1db29c693326f0fac440e19c79f96a1bbd5bc5a8bcd4f098301a4228b,
                proof
            ),
            "Email sender proof verification failed"
        );
    }

    function testVerifier_Announcement() public view {
        uint256[2] memory pA = [
            0x0f2238ddff681bb841bae41c39c8f8510a447712d739451891502b28e2cb18f2,
            0x2f087697c98398fed929be6d7a59136e21398c40f6fd770763c3e1284f9ae851
        ];
        uint256[2][2] memory pB = [
            [
                0x0d994f8335c3bcfb7997d8a84507beacffde5cfece0805bf3d61442397761ba0,
                0x1d12dbe8abb3f82fc22d74c3b940ff7533a2bec2847f167ff6194d69915e8388
            ],
            [
                0x1108d40e175b4bd24f613c5da19f3fdef1f3901357695a1c0eff44725c0be36b,
                0x070cd28f8233830d0d42c008eebc05dd187edf45c148aefb986e650046ccc6fb
            ]
        ];
        uint256[2] memory pC = [
            0x0a09ac304dfeb251879b8dc2831b1b6d5ba09c143f1586cfc3202dc955dcfe3c,
            0x1e13e025d730ee56e525f51bdfaa20b665a43cdda600e4f45e26ef1f4af22eff
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAnnouncementProof(
                "suegamisora@gmail.com",
                0x24b937a8b8ce44c9ae130d08ad77bd4456697b9ebf563b622a74448ab0fb8ca2,
                0x0dc9e2309f2f09c15b3bc05870142bd23e570e5fd3365160ded4067f9178ccec,
                proof
            ),
            "Claim fund proof verification failed"
        );
    }

}
