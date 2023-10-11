// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "../src/utils/verifiers/AccountCreationVerifier.sol";
import "../src/utils/verifiers/AccountInitVerifier.sol";
import "../src/utils/verifiers/AccountTransportVerifier.sol";
import "../src/utils/verifiers/ClaimVerifier.sol";
import "../src/utils/verifiers/EmailSenderVerifier.sol";
import "../src/utils/Verifier.sol";

contract VerifierTest is Test {
    AllVerifiers verifier;

    function setUp() public {
        address accountCreationVerifier = address(new AccountCreationVerifier());
        address accountInitVerifier = address(new AccountInitVerifier());
        address accountTransportVerifier = address(new AccountTransportVerifier());
        address claimVerifier = address(new ClaimVerifier());
        address emailSenderVerifier = address(new EmailSenderVerifier());
        verifier = new AllVerifiers(
            accountCreationVerifier,
            accountInitVerifier,
            accountTransportVerifier,
            claimVerifier,
            emailSenderVerifier
        );
    }

    function testAccountCreationProof() public view {
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
                0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3, 0x1ff706660702f76a0daa706d68b15ea04fb6145fb5f4e54823ae80fa386e1b3f, 0x0136e61d55558414797fa9e8acccac39c52ef0b2c9b3fda0ef2d858a111333df, 0x16dc3fd3780b524ba792b9e19ec9f7cbeb931912462a9b028cecfdff0eb29d28, psiPoint, proof
            ),
            "Account creation proof verification failed"
        );
    }

    function testAccountInitializaionProof() public view {
        uint256[2] memory pA = [
            0x1f27b6ed716c5c33bc9b9c2de60f9779dac4bc7f59ecf4ce53335bcac85ddd2d,
            0x120ba1546141ad60378a422493cd0ba774e18c98801c941478390c81e7232c7c
        ];
        uint256[2][2] memory pB = [
            [
                0x28ba814548824b2eb9e60597ec8cfd0d8b6bbc482a8cab824b24d71027c52d97,
                0x03fe9c013545b570557118282be7f4c2f6045deb9c94bf4bdab768677409d540
            ],
            [
                0x0392b3493b67e1e6dfe4a7cab74220c226ce36f46e3b3cd3c2b5ca4b36e4123e,
                0x292c269db7f5f4b07f85fe97cbd8974a0e7ab8ca81537a18b46f70a2001118d4
            ]
        ];
        uint256[2] memory pC = [
            0x28d1ee7c82f420ca261484d31ed34cc1388a4d513630a3746ef8fe772e944d5c,
            0x07f2e661abb87e238a66328f680a60b31ea9687c813472435f34418e66237f1b
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAccountInitializaionProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                1694979179,
                0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3,
                0x1ff706660702f76a0daa706d68b15ea04fb6145fb5f4e54823ae80fa386e1b3f,
                0x1fa82a6a28ee9ba350cdf1cedb3af78c9d90c9f22307f61bcd6ff5c61ba66fc8,
                0x253145f8b7a1c17fd656038997e74cafa4dc812495d7352dd2aaa9184ab2527c,
                proof
            ),
            "Account init proof verification failed"
        );
    }

   function testAccountTransportProof() public view {
        uint256[2] memory pA = [
            0x1fca161fdbbe869f775877b9639546f446336aa6d660472f03df34639565cf91,
            0x09e1b9a33d884b1fe45928ffc4ff8401da7ad3a48113de4f1cf751062780bd00
        ];
        uint256[2][2] memory pB = [
            [
                0x270988b388e2aa3c73c16816204960b4d010f943e514891f0f514a56e2d68a0e,
                0x00a5e3174398d53865cc81418c144ec9b165bb4a4c13bd5190a485efd35b2063
            ],
            [
                0x250ea7a72d92ccf5fb49b0cbb6f30e5718bee85d3e8c27f20ef4730b9750b04b,
                0x2ff46693bc550a57dd26b762f6b6e1c49f2e82cfd213e29a2fccf309a0b9cca2
            ]
        ];
        uint256[2] memory pC = [
            0x0e80054d27ccacba6202680693ee33ffe742f1358b35a23c88895bbd0695d854,
            0x2aaf17287fc50da051a39eb920dae79d1431ac0e67e11f1580eaabbb52cee0c6
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAccountTransportProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                1694979179,
                0x253145f8b7a1c17fd656038997e74cafa4dc812495d7352dd2aaa9184ab2527c,
                0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3,
                0x26bd42ec6664604d0a82eb62d1b374fe13fa35e165e6f3cf8d727b421826cc3d,
                0x1fa82a6a28ee9ba350cdf1cedb3af78c9d90c9f22307f61bcd6ff5c61ba66fc8,
                0x02975ab1ed4e0d54633a65cb97ef54ae982dfa4e0ae29b069bf776418ffe1317,
                proof
            ),
            "Account init proof verification failed"
        );
    }

    function testClaimFundProof() public view {
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
            "Account init proof verification failed"
        );
    }

    function testEmailSenderProof() public view {
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
            verifier.verifyEmailProof(
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
            "Account init proof verification failed"
        );
    }

}
