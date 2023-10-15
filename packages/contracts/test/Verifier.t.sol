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

    function test_AccountCreationProof() public view {
        uint256[2] memory pA = [
            0x14e40fffa1254bda8689b14ff658800bf4d34d3df4cadd5762be0bc9fe7a4494,
            0x2d4f10039746aeab37204f711427d2e545a03d7a46d1071b02fa0ccd98eef26e
        ];
        uint256[2][2] memory pB = [
            [
                0x07e9dc5c7823fe0ff1c6f73ba8c7ae1fd1c67803d2db2b12ae53463a750efaf5,
                0x2c4a5eae66862ea669329e3fea99afbee4c2455fde7f5419b223de30148b107c
            ],
            [
                0x0d07b2e019c56d563a11e953abea1254ad039c481cfbde908e29f9584537aca0,
                0x1e35d4a4d6e2ad03bd59d92ec3e01cd24b2f455543c33022551349a4a4c80e66
            ]
        ];
        uint256[2] memory pC = [
            0x1d445cda10b886fd47be0166307fedfcb5e63f768c41bc06820576bc95c74a5c,
            0x0ad78f41c65594fae5fa23f9b6b9d113d615da6bc7086acd85dbb367a961f9d0
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

    function test_AccountInitializaionProof() public view {
        uint256[2] memory pA = [
            0x28ac18e1851045419f98794259d8ddc9cef2a248d0b6af5a27b7eaadcfe88264,
            0x1d944f52201b986a0d2f7b9485ffe8c40e5e2f8a0d500078a6c0f677699a36cd
        ];
        uint256[2][2] memory pB = [
            [
                0x1d5d68f5b54933e98ab0b5cf6e8daea518228dba83686c02f2e8ef56bf9df574,
                0x285105876c2e1a653ea9d3a86172eb685ccecdcb00adc11395f330993d48e370
            ],
            [
                0x167b6fceb01fdc9d49313df4c9ee8265b062c833a5bbbe22b1c5b42bc81641d1,
                0x07b251da9a31e93f314e400547cbb67674acd836a2700b045b96658077b3bdbf
            ]
        ];
        uint256[2] memory pC = [
            0x221bf660be30f3d549464b317d1d6c95394b1dd8562e29f6afe6d50e8b4fa8d5,
            0x091fb33116368efcad1b94d28f16f4dbd1d634fab7d31b2be8a2e29fddfb6a8c
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAccountInitializaionProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                0x0000000000000000000000000000000000000000000000000000000065298ddf,
                0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3,
                0x1ff706660702f76a0daa706d68b15ea04fb6145fb5f4e54823ae80fa386e1b3f,
                0x0136E61D55558414797FA9E8ACCCAC39C52EF0B2C9B3FDA0EF2D858A111333DF,
                0x268bc70efa64d6289086c536bedad66017705a768a7e9114ff4bf0bababb26e6,
                proof
            ),
            "Account init proof verification failed"
        );
    }

    function test_AccountTransportProof() public view {
        uint256[2] memory pA = [
            0x11607df54da9e5a79f41da80a68b7ea382b5c73ce65e354460b129043ea72d1e,
            0x103eeefecd4e441dbe8c6004fc295dcf5e16e95dc40474a742292a0f2a5ac4a7
        ];
        uint256[2][2] memory pB = [
            [
                0x26e31a31267ee34962f3bfaf123a19961f835303ed0df1aee6034dddd2fcb37c,
                0x2f0365051ff73b8e91e8f80d7ceb09ed4164d98d9833d9128d7b9f2d34c05c6a
            ],
            [
                0x18813bf231ba9a42da09d1390d84a1de87b2b42f0c6dc5d4f336674c705ffe0a,
                0x01fbd925d2b25ab45b8ca146780cfa0d32fe9b8c976b1f49440223078e3845f3
            ]
        ];
        uint256[2] memory pC = [
            0x085cda370e4b1b4b779ec6d26a61a7adbac58ff216701587d95ff60828e4f270,
            0x04e3087ce1b0581a3de266bd72d223febcd58db3c04c6dd24d0dd2a26c31ccad
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

    function test_ClaimFundProof() public view {
        uint256[2] memory pA = [
            0x03a4bf5a5a281882fb7883333aa07df6b871665f16ff2d5dd4562c0d58be3199,
            0x24e7db0cdf1b72707d6a05640ed3965adaaae3e50a124e7f2140a60e63032a7d
        ];
        uint256[2][2] memory pB = [
            [
                0x2b46562382d2fd156870e87444da255e9175ec4cf002d2278afb2ce190a028ea,
                0x2ac83502dc35796c69fad25af5e56f8ddb45643e206c4ad3eb492fec656169cd
            ],
            [
                0x2bf37df7b8a669dc0b7b9b6d45240289e23dff0e8c0b0104402c85f33ae9ab1a,
                0x291d544b94cab2fe59b728fc4f988d7c0fc1cbc2650af6d19448b3f44f62fbe5
            ]
        ];
        uint256[2] memory pC = [
            0x2d1f72f65aae6aad27dd1fd48ae653a6ece5b6bb1bcc4c948ab3ad1ebd0077ed,
            0x11a23efc4c52508b948f779e03af94096494e77ee5e9c8087924dacad1361e8e
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyClaimFundProof(
                0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3,
                0x1ff706660702f76a0daa706d68b15ea04fb6145fb5f4e54823ae80fa386e1b3f,
                0x3056878dae6c22b97f04938d2c5962b4cf4c06ae857e162f56f1e195f65cce62,
                proof
            ),
            "Claim fund proof verification failed"
        );
    }

    function test_EmailSenderProof() public view {
        uint256[2] memory pA = [
            0x2efc74fdcd39cae75580dd5760a5bc68a350548ead19042da58681de8ad11657,
            0x1a0aa04d4ac1dd939a29a8a124278fde41a13aa2d640072d9c5d805e131d8722
        ];
        uint256[2][2] memory pB = [
            [
                0x28861232eddcf644190d37db0b8d6b3abf889f329db52ec14cee6b8fe736a17a,
                0x2066c954f873783e50410dfb287c2bf2f1ab159565564b852e7b230ca89a95bd
            ],
            [
                0x1edfd634cbd2ec9483d8c3f3c41b82d943b0fc04b835ed04527c32c7bdfdf14d,
                0x0032e997076beb5444e65413377d7499e26e2180cf4e1586a088466bef3dc0b4
            ]
        ];
        uint256[2] memory pC = [
            0x134342d5802c0e371bc53c152860620c1e4ec911fca0a9d457cfd58437427505,
            0x266cd14bec62aeb9f912b4df60a0618432e5a265176a949a085d0d0dec07ec8a
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyEmailOpProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                1694989812,
                "Send 0.1 ETH to ",
                0x00a83fce3d4b1c9ef0f600644c1ecc6c8115b57b1596e0e3295e2c5105fbfd8a,
                0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3,
                0x1ff706660702f76a0daa706d68b15ea04fb6145fb5f4e54823ae80fa386e1b3f,
                true,
                0x1c4df0cda9c4060167d46362872a85eed361b4eec2e148ed5ef30788ce27b32e,
                proof
            ),
            "Email sender proof verification failed"
        );
    }
}
