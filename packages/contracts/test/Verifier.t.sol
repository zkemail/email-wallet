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
        verifier = new AllVerifiers();
    }

    function testVerifier_AccountCreation() public view {
        uint256[2] memory pA = [
            0x19d2869e21e075dc2c7fe066ceffd60968945add2609bd07c803b33f9b53e637,
            0x04527f7bdfa7e32f3c793f2839725b021874704ed08ed44cae290c36081d57cf
        ];
        uint256[2][2] memory pB = [
            [
                0x02f4116d2bafbff0ae697b230189880d54e0024f2adff0f4b57f475b9b5a3690,
                0x0d2d03aa68defc3454ec72eb9dab7e729e44cb28d8c6b44ce2c3ab87b049a46a
            ],
            [
                0x0f499dc807c29dde76ce12793612e965894b7b3c1e2afb6800ef0cbb8721b14e,
                0x1ce7fdff24b89696988c01731198dfdad5bc63af8246d74d3b56009368d4c08e
            ]
        ];
        uint256[2] memory pC = [
            0x08b95f31ca3149e75338e8a6cd382a0517981a7408bd33a552aff89e78b2d79f,
            0x2cd0f5da1fbf0db490632f002f28e72de64b4b733035f1bf70b1e8b970d96ab7
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        bytes memory psiPoint = abi.encode(
            0x137f37195be2846f471ca4c2ebe71d1d2aeb83b01e1a25356bc0430d5e974f1b,
            0x0024d47abc04548c24a7dae9e7a23d8f791266162467c425fda80d5fad023290
        );
        require(
            verifier.verifyAccountCreationProof(
                0x0796930f14ad1b325aca059bb26fa26179ff2cd0fa6cd063bda7572886194b1f,
                0x1665ce2f746c92ddc98c3b0db45a6615793b1afe0192d5d140479c21cae435c3,
                0x2b205cde32d2644a3aefaf9d1af1886900584371e4735ae2f5230e0deb06e747,
                0x217e9ff6eeeda73c130fab0fa603f61225e50860e57024fce75d4a6994a884f0,
                psiPoint,
                proof
            ),
            "Account creation proof verification failed"
        );
    }

    function testVerifier_AccountInit() public view {
        uint256[2] memory pA = [
            0x260162e5d4f1174f10f99fd6f52e8cd2d6fa8d46f21cede71ad8c36910a0c663,
            0x1a8e5622e6b8491202830e5df6f38b9008c5c3f3ad73ffee3698f8f7b1cd05fe
        ];
        uint256[2][2] memory pB = [
            [
                0x030ed667fdf75632ec85576df8f8d3007d495b5dee375e69616ebc6d352819e5,
                0x0d3f2f8c9232f27b68fd28dc0dda1dc1e4efe072d3203e87e4e7cabecf29fcbf
            ],
            [
                0x08a22365700fb07bf7b6e8e2df310f45d7ee1f05adcfff35826db081a1b1a61d,
                0x29ac833c21c79e0e24480f75581cd69ff1e984415dba59404074b7e1914f0678
            ]
        ];
        uint256[2] memory pC = [
            0x29aee29376de95eb6e96a9f3c9c017bea24a35b945b9a7791243743d53d991ba,
            0x1c5f3939ed70b71deec5ac151b277f16ed16aae97dce33b1eb2ec5383e8c2400
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAccountInitializaionProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                0x0000000000000000000000000000000000000000000000000000000065298ddf,
                0x0796930f14ad1b325aca059bb26fa26179ff2cd0fa6cd063bda7572886194b1f,
                0x1665ce2f746c92ddc98c3b0db45a6615793b1afe0192d5d140479c21cae435c3,
                0x1a72c1328a61e638924582dac6feb46443ba20572a3d91cac2b91a84c738c7ac,
                0x268bc70efa64d6289086c536bedad66017705a768a7e9114ff4bf0bababb26e6,
                proof
            ),
            "Account init proof verification failed"
        );
    }

    function testVerifier_AccountTransport() public view {
        uint256[2] memory pA = [
            0x2a0e46ff1f4d6eefc8f6e6a3142480a7f3827fbcc5a89797278f1324befbfc35,
            0x1dc308920dff6c28a04f3001cb6cc7de05f7649564080a46b806584c60d8057e
        ];
        uint256[2][2] memory pB = [
            [
                0x20315c3a8edd6365b031d112197ad3b75636180d6c77ed34e01aff12579981fe,
                0x03d422941eead7d3e70d0f174d4881c084122aef3b06462c8549b4500bf25a83
            ],
            [
                0x2eefef260826a5530a12a81f75a92dce8264be5159c62ec61bc2fb74ac9d4502,
                0x2b2b196ecac9bae12528388aaf6f20673b73504ee6d3b2b62c7f69224401c977
            ]
        ];
        uint256[2] memory pC = [
            0x037f69cb6a68597312ab4592efd7d916c2deab9ce697cd3ec57176cd3541be60,
            0x2dbd06d6e5ffb293e756884f26eaaef952d183c47def797ff3248419749563c4
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAccountTransportProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                0x0000000000000000000000000000000000000000000000000000000065298ddf,
                0x268bc70efa64d6289086c536bedad66017705a768a7e9114ff4bf0bababb26e6,
                0x0796930f14ad1b325aca059bb26fa26179ff2cd0fa6cd063bda7572886194b1f,
                0x1a3ce78210397eb208c5f921f20dfa9394cc441776e3deeb0d7179a331a7dd29,
                0x1a72c1328a61e638924582dac6feb46443ba20572a3d91cac2b91a84c738c7ac,
                0x2e482e8bab2f3a566f8278e67b74a91cca5259a8cf2f411987b4bfe62619de26,
                proof
            ),
            "Account transport proof verification failed"
        );
    }

    function testVerifier_ClaimFund() public view {
        uint256[2] memory pA = [
            0x270164d21f7a359ee6400e60d60edac28738f652b72fcd053ffb326d96a4bb1d,
            0x0467d07106ea9014c256b23bf2ec4aac3581a14a52612195cf09c5ee2802ec3f
        ];
        uint256[2][2] memory pB = [
            [
                0x29cbb258c4138535ead7a3a361fe9f72ea5bf92d04063bad0fabaa3bc2f81b2a,
                0x2feb3d4d40ef40172734b83fe960c134d3111b6f2fd1ecbb9f71bb1c44696ec4
            ],
            [
                0x0462c0f124946fbc5799f9283c527524605fefd67be8ee515ab7af7edc5366a3,
                0x0e87de99bd7b2863de0e982b8e9550b42e97cbe0c063ff407cd526f5462f0645
            ]
        ];
        uint256[2] memory pC = [
            0x154795b56267a7b44ce4e75adc18182d21a824ec4917b48c958a6057aba40938,
            0x2df52c049e105fc281f8dd6b7a589039ea4a9f322358393fc710c89d49332bf7
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyClaimFundProof(
                0x0796930f14ad1b325aca059bb26fa26179ff2cd0fa6cd063bda7572886194b1f,
                0x1665ce2f746c92ddc98c3b0db45a6615793b1afe0192d5d140479c21cae435c3,
                0x0a7f26d0547d1c8ba1ac9bc91dfdb52b2e0361a6990e9ffb0a90c02738e612eb,
                proof
            ),
            "Claim fund proof verification failed"
        );
    }

    function testVerifier_EmailSenderProof() public view {
        uint256[2] memory pA = [
            0x1114225c176fea1d4365722901aea59eeffacd9b754b8d20cbc06d8628d145ef,
            0x0e437f855823f2a38ca675b2a2d27c32ecc04ac72b5ac8aa97946ae20257d4ff
        ];
        uint256[2][2] memory pB = [
            [
                0x221d505f28dbff0a59baa7e2a92651446e68a7c0926ef75b5d7f8cb56a12ac52,
                0x2ece94e2e0d2224c85bc78a6fe08d6d7fe766cbd37f558a09d2811e3c6ec4835
            ],
            [
                0x1e2a8aaa9bb05b9dc2b03a34f1212b7b0f15e89e9e4707456d2431cccabf7994,
                0x0c5d73902114ade8e32c59cc47b0388b712c2dda434443566ec4b971d90fd5ba
            ]
        ];
        uint256[2] memory pC = [
            0x0d7ecf32203975e26a4708cf56d3e55b84137be3aa9bdbd1c7538ee4f2a5955e,
            0x1a406b747cbada94cc0849f296bb384f0691a939b4d8559b33e651d1b18b9397
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyEmailOpProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                0x0000000000000000000000000000000000000000000000000000000065077df4,
                "Send 0.1 ETH to ",
                0x00a83fce3d4b1c9ef0f600644c1ecc6c8115b57b1596e0e3295e2c5105fbfd8a,
                0x0796930f14ad1b325aca059bb26fa26179ff2cd0fa6cd063bda7572886194b1f,
                0x1665ce2f746c92ddc98c3b0db45a6615793b1afe0192d5d140479c21cae435c3,
                true,
                0x1c4df0cda9c4060167d46362872a85eed361b4eec2e148ed5ef30788ce27b32e,
                proof
            ),
            "Email sender proof verification failed"
        );
    }

    function testVerifier_Announcement() public view {
        uint256[2] memory pA = [
            0x070fda625542e9868ad7a36cb29a6b654813184a57c1ea1d102d66308fe2605d,
            0x2f5fc68df2d5ea9b3ba522407fb428874b9a371c4e6d1c33c2283728ca94d6ea
        ];
        uint256[2][2] memory pB = [
            [
                0x06b32882a37b339ef0d5620aef0b853ef2d13cf798729bc5383ffe2eca574502,
                0x21e821d0b4dc8acb3e8c831764f468383b4b9f4dee161b5f6f6207ad7baa5767
            ],
            [
                0x2cf3fb47a2b94c849eb2623e501974085e34fdb193b3d25d86b3a2bad3e4c3cc,
                0x2c42cae8fcc4501eb2eff5f19b69c370bfb96b827fb4343175eaae5d56127a36
            ]
        ];
        uint256[2] memory pC = [
            0x1dbd4733b624b97f208820b4115906c32c01a0808a7e1d3f0d8d7b2c455211eb,
            0x14c84f50af2a9cbfb0cfd9be941a03316241b5b3441cdb82a7af947040d30e94
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAnnouncementProof(
                "suegamisora@gmail.com",
                0x15984bbf2019df84ffa89e17cd965b28ec1af5ed066b156d3cb06e7fadf98deb,
                0x0a7f26d0547d1c8ba1ac9bc91dfdb52b2e0361a6990e9ffb0a90c02738e612eb,
                proof
            ),
            "Claim fund proof verification failed"
        );
    }
}
