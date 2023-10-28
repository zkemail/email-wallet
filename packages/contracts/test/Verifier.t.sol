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
            0x0bb1a492f4f502af3c4c977f29131673c1896111fd472cb6047847977ca76d95,
            0x26ef4d88d1dbb63ef18fd87570d368d65b374d6ebcfe8a8ac0a9fb9d7a8ea2fb
        ];
        uint256[2][2] memory pB = [
            [
                0x26b4acee0c4487c88320c20f9c86d4f231845bc64e0247defd9fa07c92059bb2,
                0x262204badb669678be732aa9ffde9d2eb96da4da8314fba444890152a87a415a
            ],
            [
                0x27c1ecdb3dee0417e93f515de6d9d61f727dac866a645e798199af958fccab90,
                0x033781924be4e18e066ca8a642577fbad6e33d75aadd729c4d0b7fe62f7c36fa
            ]
        ];
        uint256[2] memory pC = [
            0x225a539d0e026aeff4d3842d881b54bed93b834f4c1151928dc5c6ecf16095b6,
            0x1086c24dec0bf5c42fe21a15b584b1f77bc87e5bf3752f880c55185d8ac64330
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        bytes memory psiPoint = abi.encode(
            0x2af2a2b0a227b2140e26e566374921dacbe55b09e4554a4b83659ee2c3cc34ac,
            0x215d506ba975c642b07284acc7ba631c56fb5888df65c0e4b7ff4bbe3fbaff47
        );
        require(
            verifier.verifyAccountCreationProof(
                0x29c62c9ffd65f342646caeeebe6b62ecdb96062d4b203b3852a1cbdd8b1109fe,
                0x061fe8ef6eaa7d1f25c256e8c7ac69af6900f6d99eec1b40b763ff04fbef465c,
                0x04390282bc61eb88ec0077a286cea2fc77b768f43422828859cc4a00fa58074a,
                0x2faf230d2774f4824a15a9cb07fdb8a704000925e63bcd5d63d212965b1e3f61,
                psiPoint,
                proof
            ),
            "Account creation proof verification failed"
        );
    }

    function testVerifier_AccountInit() public view {
        uint256[2] memory pA = [
            0x09c56aed96e41876d384c7ab9cd29335aef3e262ca54de100edd099c6f48dfe2,
            0x0ff231bb98f4371bae6405a4935dd59dbfd63b0f5ed22bd28843d832b665ad41
        ];
        uint256[2][2] memory pB = [
            [
                0x096510a544f5df10542c9eb7b0e5e9f5da8ae9cc82c96c7d4ba4f638a68d5d75,
                0x038a28fc9ae89440517e4d7f9892bd1a6d293c6280d1db8c63217185d38dcacf
            ],
            [
                0x0e683de4acecb70309e80e9660b8db596df8c60ff22cfffd8be99f0fe13d0d1c,
                0x2e636039c8fd383d6b724e4a65f81029f2eb04212d908732dff299a05c51e8e8
            ]
        ];
        uint256[2] memory pC = [
            0x126f922b00446720ec92a1966691a59606cfbff4cb1853de3d9c3ce319083c23,
            0x18dabe3e31604d2601f18e7174b8ebb80555d05762bb95ad0a18cfb24873b1f6
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAccountInitializaionProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                0x0000000000000000000000000000000000000000000000000000000065298ddf,
                0x29c62c9ffd65f342646caeeebe6b62ecdb96062d4b203b3852a1cbdd8b1109fe,
                0x061fe8ef6eaa7d1f25c256e8c7ac69af6900f6d99eec1b40b763ff04fbef465c,
                0x05bfa5728308c74f18ea0126fbb0a5d92fc7b0480dbde536bf98040168de315f,
                0x268bc70efa64d6289086c536bedad66017705a768a7e9114ff4bf0bababb26e6,
                proof
            ),
            "Account init proof verification failed"
        );
    }

    function testVerifier_AccountTransport() public view {
        uint256[2] memory pA = [
            0x131a81a99a69f52990a1abc5a8f27457d81a144e788261bcecafe0380fd110c4,
            0x1840dd019a2e252c3cf9be800a2387caf59e9f863b9b082ba053869e9e2cb99b
        ];
        uint256[2][2] memory pB = [
            [
                0x159ed1c9e515b9b2df9331b3da6a15132f918229633899412503eb4263e3ec8c,
                0x044c773c3e11ead1f172ed02046faf717f8a068c4fe9e9d4df51f3aac417d080
            ],
            [
                0x0cc11c8c9b05f4d9c2ada455c91763fed138bc5f42f3b01608c4bc92cbf1223e,
                0x06e493b1f75bc3b353d59cc9c6a1ef6ef5affab0726891bc21514cf287193979
            ]
        ];
        uint256[2] memory pC = [
            0x2f84ef2184ad3befd9a9843fdaef8b8ac70b8b67631ae9e28bb1fac9f9d628c6,
            0x28389f723197a423bf4535ac67c22ac0fc1c58ac92b3d13cacfeee990dd6a8fc
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAccountTransportProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                0x0000000000000000000000000000000000000000000000000000000065298ddf,
                0x268bc70efa64d6289086c536bedad66017705a768a7e9114ff4bf0bababb26e6,
                0x29c62c9ffd65f342646caeeebe6b62ecdb96062d4b203b3852a1cbdd8b1109fe,
                0x16620737ea2a3ce070a334cf76e874842e0a2e2cfbc52ebdce72cb614c6bc624,
                0x05bfa5728308c74f18ea0126fbb0a5d92fc7b0480dbde536bf98040168de315f,
                0x028b3c9c743d32453c84c27c990b08f3b6b56e21e901151b9414eae5a17ac806,
                proof
            ),
            "Account transport proof verification failed"
        );
    }

    function testVerifier_ClaimFund() public view {
        uint256[2] memory pA = [
            0x15c61dfd4db6f252c9bfc6fb6cbe7a21a3e902d0cd1231147f5b0d54b7ea7e78,
            0x2d8a674a4b4aac6750e0885d9f569d0df2ece489f85844b0b8eec097e5c9be94
        ];
        uint256[2][2] memory pB = [
            [
                0x3004c2d2b7ab606a909f552f28a203dd1b605f2101b3a1c5197154c2e274f12e,
                0x2d4a964777c8ff4cefdfbca28405aab41cb0a50424b3a8cb9171e6a8145a3f7c
            ],
            [
                0x246e4aab654b07dba499717762368b28988ff6c81e4d70ad144a471aa5aa9bf8,
                0x11592e5378fd0a112408e1866888772a4def947416f2370151332525e8f5ffb9
            ]
        ];
        uint256[2] memory pC = [
            0x2e8520bbbd57d34513cf6a94933deb2523770e31c81be64375dca4ce7249e642,
            0x174797a3e33b4a4ba6940ce01227acd12ca1327587cad7acc55b0a4b503d3c62
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyClaimFundProof(
                0x29c62c9ffd65f342646caeeebe6b62ecdb96062d4b203b3852a1cbdd8b1109fe,
                0x061fe8ef6eaa7d1f25c256e8c7ac69af6900f6d99eec1b40b763ff04fbef465c,
                0x0c7ac0f7f39ad116c8d7424bbbcd215d7ff35b549d27291e5b9c8c12eb796984,
                proof
            ),
            "Claim fund proof verification failed"
        );
    }

    function testVerifier_EmailSenderProof() public view {
        uint256[2] memory pA = [
            0x089ad9cc6e6aa2e6a86b748a515d4d4d46e814562f32eca6203a34906c83e3ba,
            0x111ea5325fee4f7a843e5671774cd55ae77d9ff46e18deea1f527ba340af0f64
        ];
        uint256[2][2] memory pB = [
            [
                0x02c5d28cf18552c85453b1988eabec184468b98f442ba73da08d35a72e6fe850,
                0x1b660cc943d83a1e1751943877cfb163bc49b013e94d7127a4df1b609e0e940e
            ],
            [
                0x111c7263f0c9817eca3dafa26a64e31781cdb9f28ab73e8bffd33e8bdcb29e5b,
                0x0c4ccc9f0bfec10c81c2341f5ebe8264fd64a3328276cf7a960d0b00d4b4cb6e
            ]
        ];
        uint256[2] memory pC = [
            0x09c79cfcbdbb877e8e3d35332ee022689b738227d3d229ee90e7abfd53875056,
            0x021157b68dea7be7a12296a7b65a1de114b43bb10be648471768e9e65f22d49c
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyEmailOpProof(
                "gmail.com",
                0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788,
                0x0000000000000000000000000000000000000000000000000000000065077df4,
                "Send 0.1 ETH to ",
                0x00a83fce3d4b1c9ef0f600644c1ecc6c8115b57b1596e0e3295e2c5105fbfd8a,
                0x29c62c9ffd65f342646caeeebe6b62ecdb96062d4b203b3852a1cbdd8b1109fe,
                0x061fe8ef6eaa7d1f25c256e8c7ac69af6900f6d99eec1b40b763ff04fbef465c,
                true,
                0x1c4df0cda9c4060167d46362872a85eed361b4eec2e148ed5ef30788ce27b32e,
                proof
            ),
            "Email sender proof verification failed"
        );
    }

    function testVerifier_Announcement() public view {
        uint256[2] memory pA = [
            0x26811c26694e3a778e118a3005b41ae6033bbf924bcb97449724cc47606d56cb,
            0x2d4583d366efca410a72bf929a8b7513b719b6c97f487ad7192b8a11f581a40f
        ];
        uint256[2][2] memory pB = [
            [
                0x1a1d58816e75a423839f51a75100f0589ad5e7b47bfecb4f4382666e7696e772,
                0x15384a67cd743ee59e2b26556607db1ece7c3fe570fcc6862b43051bff3d07ab
            ],
            [
                0x1b333dbbe4564fbaf90ec47141e35cdd9272a61038b0c34578eeebe2168a56d4,
                0x1b2383922ad22fc834e03b04476b426776ee0bd4b3ecb636dd9a07333056db62
            ]
        ];
        uint256[2] memory pC = [
            0x070fc365509c8468f255bc5b868ffbc5bb8724dc3979f024fefcb851b1169066,
            0x1fad99979f555c068c8972e00411fa1749fe6a716860d0848ccafbf99ffcc579
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        require(
            verifier.verifyAnnouncementProof(
                "suegamisora@gmail.com",
                0x12a122dabd5ad76e27c3474d2b6015035b5c684a1ba75b7e41f07c4ad6063c7c,
                0x0c7ac0f7f39ad116c8d7424bbbcd215d7ff35b549d27291e5b9c8c12eb796984,
                proof
            ),
            "Claim fund proof verification failed"
        );
    }
}
