// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@zk-email/contracts/DKIMRegistry.sol";
import "../../src/EmailWalletCore.sol";
import "../../src/utils/TokenRegistry.sol";
import "../../src/utils/UniswapTWAPOracle.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "./WETH9.sol";
import "../../src/verifier/AccountCreationVerifier.sol";
import "../../src/verifier/AccountInitVerifier.sol";
import "../../src/verifier/AccountTransportVerifier.sol";
import "../../src/verifier/ClaimVerifier.sol";
import "../../src/verifier/EmailSenderVerifier.sol";
import "../../src/verifier/AnnouncementVerifier.sol";
import "../../src/verifier/Verifier.sol";
import {EmailWalletEvents} from "../../src/interfaces/Events.sol";
import "../../src/extensions/UniswapExtension.sol";
import "../../src/extensions/NFTExtension.sol";
import "../mocks/DummyNFT.sol";

abstract contract IntegrationTestHelper is Test {
    using Strings for *;
    using console for *;
    using stdStorage for StdStorage;

    struct Groth16ProofJson {
        uint256[2] pi_a;
        uint256[2][3] pi_b;
        uint256[3] pi_c;
        string y;
        string z;
    }

    struct UserTestConfig {
        string emailAddr;
        bytes32 accountKey;
        bytes32 emailAddrPointer;
    }

    EmailWalletCore core;
    AllVerifiers verifier;
    TokenRegistry tokenRegistry;
    DKIMRegistry dkimRegistry;
    IPriceOracle priceOracle;
    WETH9 weth;
    UniswapExtension uniExtension;
    NFTExtension nftExtension;

    RelayerHandler relayerHandler;
    AccountHandler accountHandler;
    UnclaimsHandler unclaimsHandler;
    ExtensionHandler extensionHandler;

    // TestERC20 wethToken;
    ERC20 daiToken;
    ERC20 usdcToken;

    bytes32 mockDKIMHash = bytes32(uint256(123));

    address constant WETH_ADDR = 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
    address constant DAI_ADDR = 0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1;
    address constant USDC_ADDR = 0xaf88d065e77c8cC2239327C5EDb3A432268e5831;
    address constant UNISWAP_V3_FACTORY = 0x1F98431c8aD98523631AE4a59f267346ea31F984;
    address constant UNISWAP_V3_ROUTER = 0xE592427A0AEce92De3Edee1F18E0157C05861564;

    uint256 constant DOMAIN_FIELDS = 9;
    uint256 constant SUBJECT_FIELDS = 17;
    uint256 constant EMAIL_ADDR_FIELDS = 9;

    uint256 maxFeePerGas = 10 ** 9;
    uint256 emailValidityDuration = 10 days;
    uint256 unclaimedFundClaimGas = 1000000;
    uint256 unclaimedStateClaimGas = 1000000;
    uint256 unclaimsExpiryDuration = 30 days;

    address deployer;
    address relayer1;
    address relayer2;
    bytes32 relayer1Rand = 0x05f5b4f85b25760c2ee168c67c856afd371308a291de9d4c36a6e1c1c2a71693;
    bytes32 relayer1RandHash = 0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3;
    bytes32 relayer2Rand = 0x11a036998ca261fcd981225b1cdcaa581d0861d476ff0491258bef3c88146b01;
    bytes32 relayer2RandHash = 0x2451956F89B22A433050F391776B5B00E53616CEED3313C0C3E1754D3F1D9A50;

    UserTestConfig user1 =
        UserTestConfig({
            emailAddr: "suegamisora@gmail.com",
            accountKey: 0x01eb9b204cc24c3baee11accc37d253a9c53e92b1a2cc07763475c135d575b76,
            emailAddrPointer: bytes32(0)
        });
    UserTestConfig user2 =
        UserTestConfig({
            emailAddr: "emaiwallet.bob@gmail.com",
            accountKey: 0x1e2ead4231d73a3c85b1ff883f212d998c41cc9d2a8bac238f6d351ff2c57249,
            emailAddrPointer: bytes32(0)
        });

    string[][] subjectTemplates;

    function setUp() public virtual {
        vm.createSelectFork("https://arb1.arbitrum.io/rpc");
        vm.warp(1697222111);
        deployer = vm.addr(1);
        relayer1 = vm.addr(2);
        relayer2 = vm.addr(3);

        vm.startPrank(deployer);

        verifier = new AllVerifiers();

        {
            TokenRegistry tokenRegistryImpl = new TokenRegistry();
            ERC1967Proxy proxy = new ERC1967Proxy(address(tokenRegistryImpl), abi.encodeCall(tokenRegistryImpl.initialize, ()));
            tokenRegistry = TokenRegistry(payable(address(proxy)));
        }

        dkimRegistry = new DKIMRegistry();
        priceOracle = new UniswapTWAPOracle(UNISWAP_V3_FACTORY, WETH_ADDR);
        // weth = new WETH9();
        weth = WETH9(payable(WETH_ADDR));

        Wallet walletImp = new Wallet(address(weth));

        dkimRegistry.setDKIMPublicKeyHash(
            "gmail.com",
            0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788
        );

        {
            RelayerHandler relayerHandlerImpl = new RelayerHandler();
            ERC1967Proxy proxy = new ERC1967Proxy(address(relayerHandlerImpl), abi.encodeCall(relayerHandlerImpl.initialize, ()));
            relayerHandler = RelayerHandler(payable(address(proxy)));
        }
        
        {
            ExtensionHandler extensionHandlerImpl = new ExtensionHandler();
            ERC1967Proxy proxy = new ERC1967Proxy(address(extensionHandlerImpl), abi.encodeCall(extensionHandlerImpl.initialize, ()));
            extensionHandler = ExtensionHandler(payable(address(proxy)));
        }

        {
            AccountHandler accountHandlerImpl = new AccountHandler();
            ERC1967Proxy proxy = new ERC1967Proxy(address(accountHandlerImpl), abi.encodeCall(accountHandlerImpl.initialize, (
                address(relayerHandler),
                address(dkimRegistry),
                address(verifier),
                address(walletImp),
                emailValidityDuration
            )));
            accountHandler = AccountHandler(payable(address(proxy)));
        }
        
        {
            UnclaimsHandler unclaimsHandlerImpl = new UnclaimsHandler();
            ERC1967Proxy proxy = new ERC1967Proxy(address(unclaimsHandlerImpl), abi.encodeCall(unclaimsHandlerImpl.initialize, (
                address(relayerHandler),
                address(accountHandler),
                address(verifier),
                unclaimedFundClaimGas,
                unclaimedStateClaimGas,
                unclaimsExpiryDuration,
                maxFeePerGas
            )));
            unclaimsHandler = UnclaimsHandler(payable(address(proxy)));
        }

        {
            EmailWalletCore coreImpl = new EmailWalletCore();
            ERC1967Proxy proxy = new ERC1967Proxy(address(coreImpl), abi.encodeCall(coreImpl.initialize, (
                address(relayerHandler),
                address(accountHandler),
                address(unclaimsHandler),
                address(extensionHandler),
                address(verifier),
                address(tokenRegistry),
                address(priceOracle),
                address(weth),
                maxFeePerGas,
                emailValidityDuration,
                unclaimedFundClaimGas,
                unclaimedStateClaimGas
            )));
            core = EmailWalletCore(payable(address(proxy)));
        }

        relayerHandler.transferOwnership(address(core));
        accountHandler.transferOwnership(address(core));
        unclaimsHandler.transferOwnership(address(core));
        extensionHandler.transferOwnership(address(core));

        // Deploy some ERC20 test tokens and add them to registry
        // wethToken = new TestERC20("WETH", "WETH");
        // daiToken = new TestERC20("DAI", "DAI");
        daiToken = ERC20(DAI_ADDR);
        // usdcToken = new TestERC20("USDC", "USDC");
        usdcToken = ERC20(USDC_ADDR);
        tokenRegistry.setTokenAddress("WETH", address(weth));
        tokenRegistry.setTokenAddress("DAI", address(daiToken));
        tokenRegistry.setTokenAddress("USDC", address(usdcToken));
        vm.stopPrank();
        vm.startPrank(relayer1);
        relayerHandler.registerRelayer(relayer1RandHash, "emailwallet.relayer@gmail.com", "emailwallet.com");
        vm.stopPrank();
        vm.startPrank(relayer2);
        relayerHandler.registerRelayer(relayer2RandHash, "emailwallet.relayer2@gmail.com", "emailwallet2.com");
        vm.stopPrank();

        address extensionDev = vm.addr(3);
        vm.startPrank(extensionDev);
        {
            UniswapExtension uniExtensionImpl = new UniswapExtension();
            ERC1967Proxy proxy = new ERC1967Proxy(address(uniExtensionImpl), abi.encodeCall(uniExtensionImpl.initialize, (
                address(core),
                address(tokenRegistry),
                UNISWAP_V3_ROUTER,
                UNISWAP_V3_FACTORY
            )));
            uniExtension = UniswapExtension(payable(address(proxy)));
        }

        {
            NFTExtension nftExtensionImpl = new NFTExtension();
            ERC1967Proxy proxy = new ERC1967Proxy(address(nftExtensionImpl), abi.encodeCall(nftExtensionImpl.initialize, (
                address(core)
            )));
            nftExtension = NFTExtension(payable(address(proxy)));
            
            DummyNFT apeNFT = new DummyNFT();
            nftExtension.setNFTAddress("APE", address(apeNFT));
        }

        uint256 maxExecutionGas = 10 ** 6;
        string[][] memory templates = _getUniswapSubjectTemplates();
        extensionHandler.publishExtension("Uniswap", address(uniExtension), templates, maxExecutionGas);
        templates = _getNFTSubjectTemplates();
        extensionHandler.publishExtension("NFT", address(nftExtension), templates, maxExecutionGas);
        vm.stopPrank();
    }

    function accountCreation(
        string memory emailAddr,
        bytes32 relayerRand,
        bytes32 accountKey
    ) internal returns (bytes32 relayerHash, bytes32 emailAddrPointer) {
        string memory projectRoot = vm.projectRoot();
        string[] memory inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(projectRoot, "/test/bin/account_creation.sh");
        inputGenerationInput[1] = emailAddr;
        inputGenerationInput[2] = uint256(relayerRand).toHexString(32);
        inputGenerationInput[3] = uint256(accountKey).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(
            string.concat(projectRoot, "/test/build_integration/account_creation_public.json")
        );
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        relayerHash = bytes32(vm.parseUint(pubSignals[0]));
        emailAddrPointer = bytes32(vm.parseUint(pubSignals[1]));
        bytes32 accountKeyCommit = bytes32(vm.parseUint(pubSignals[2]));
        bytes32 walletSalt = bytes32(vm.parseUint(pubSignals[3]));
        bytes32 x = bytes32(vm.parseUint(pubSignals[4]));
        bytes32 y = bytes32(vm.parseUint(pubSignals[5]));
        bytes memory psiPoint = abi.encode(x, y);
        bytes memory proof = proofToBytes(
            string.concat(projectRoot, "/test/build_integration/account_creation_proof.json")
        );
        accountHandler.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, proof);
    }

    function accountInit(
        string memory emailFile,
        bytes32 relayerRand,
        string memory emailDomain
    ) internal returns (bytes32 relayerHash, bytes32 emailAddrPointer) {
        string memory projectRoot = vm.projectRoot();
        string[] memory inputGenerationInput = new string[](3);
        inputGenerationInput[0] = string.concat(projectRoot, "/test/bin/account_init.sh");
        inputGenerationInput[1] = emailFile;
        inputGenerationInput[2] = uint256(relayerRand).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(
            string.concat(projectRoot, "/test/build_integration/account_init_public.json")
        );
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        relayerHash = bytes32(vm.parseUint(pubSignals[DOMAIN_FIELDS + 1]));
        bytes32 emailNullifier = bytes32(vm.parseUint(pubSignals[DOMAIN_FIELDS + 2]));
        emailAddrPointer = bytes32(vm.parseUint(pubSignals[DOMAIN_FIELDS + 3]));
        uint emailTimestamp = vm.parseUint(pubSignals[DOMAIN_FIELDS + 5]);
        bytes32 publicKeyHash = bytes32(vm.parseUint(pubSignals[DOMAIN_FIELDS + 0]));
        bytes memory proof = proofToBytes(
            string.concat(projectRoot, "/test/build_integration/account_init_proof.json")
        );
        accountHandler.initializeAccount(
            emailAddrPointer,
            emailDomain,
            emailTimestamp,
            emailNullifier,
            publicKeyHash,
            proof
        );
    }

    function accountTransport(
        bytes32 oldRelayerRandHash,
        bytes32 oldAccountKeyCommit,
        string memory emailFile,
        string memory emailDomain,
        string memory emailAddr,
        bytes32 newRelayerRand,
        bytes32 accountKey
    ) internal returns (bytes32 newRelayerHash, bytes32 newEmailAddrPointer) {
        EmailProof memory transportEmailProof = genAccountTransportProof(
            oldRelayerRandHash,
            emailFile,
            emailDomain,
            newRelayerRand
        );

        string[] memory inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(vm.projectRoot(), "/test/bin/account_creation.sh");
        inputGenerationInput[1] = emailAddr;
        inputGenerationInput[2] = uint256(newRelayerRand).toHexString(32);
        inputGenerationInput[3] = uint256(accountKey).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(
            string.concat(vm.projectRoot(), "/test/build_integration/account_creation_public.json")
        );
        inputGenerationInput = abi.decode(vm.parseJson(publicInputFile), (string[]));
        newRelayerHash = bytes32(vm.parseUint(inputGenerationInput[0]));
        newEmailAddrPointer = bytes32(vm.parseUint(inputGenerationInput[1]));
        bytes32 newAccountKeyCommit = bytes32(vm.parseUint(inputGenerationInput[2]));
        bytes memory newPSIPoint = abi.encode(
            bytes32(vm.parseUint(inputGenerationInput[4])),
            bytes32(vm.parseUint(inputGenerationInput[5]))
        );
        bytes memory accountCreationProof = proofToBytes(
            string.concat(vm.projectRoot(), "/test/build_integration/account_creation_proof.json")
        );
        accountHandler.transportAccount(
            oldAccountKeyCommit,
            newEmailAddrPointer,
            newAccountKeyCommit,
            newPSIPoint,
            transportEmailProof,
            accountCreationProof
        );
    }

    function genAccountTransportProof(
        bytes32 oldRelayerRandHash,
        string memory emailFile,
        string memory emailDomain,
        bytes32 newRelayerRand
    ) private returns (EmailProof memory) {
        string memory projectRoot = vm.projectRoot();
        string[] memory inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(projectRoot, "/test/bin/account_transport.sh");
        inputGenerationInput[1] = emailFile;
        inputGenerationInput[2] = uint256(oldRelayerRandHash).toHexString(32);
        inputGenerationInput[3] = uint256(newRelayerRand).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(
            string.concat(projectRoot, "/test/build_integration/account_transport_public.json")
        );
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        EmailProof memory transportEmailProof;
        transportEmailProof.nullifier = bytes32(vm.parseUint(pubSignals[DOMAIN_FIELDS + 1]));
        transportEmailProof.timestamp = vm.parseUint(pubSignals[DOMAIN_FIELDS + 5]);
        transportEmailProof.domain = emailDomain;
        transportEmailProof.dkimPublicKeyHash = bytes32(vm.parseUint(pubSignals[DOMAIN_FIELDS + 0]));
        transportEmailProof.proof = proofToBytes(
            string.concat(projectRoot, "/test/build_integration/account_transport_proof.json")
        );
        return transportEmailProof;
    }

    function genEmailOpPartial(
        string memory emailFile,
        bytes32 relayerRand,
        string memory command,
        string memory maskedSubject,
        string memory emailDomain,
        string memory feeTokenName
    ) internal returns (EmailOp memory emailOp, bytes32 emailAddrRand) {
        string[] memory inputGenerationInput = new string[](3);
        inputGenerationInput[0] = string.concat(vm.projectRoot(), "/test/bin/email_sender.sh");
        inputGenerationInput[1] = emailFile;
        inputGenerationInput[2] = uint256(relayerRand).toHexString(32);
        vm.ffi(inputGenerationInput);
        inputGenerationInput = new string[](2);
        inputGenerationInput[0] = string.concat(vm.projectRoot(), "/test/bin/extract_sign_rand.sh");
        inputGenerationInput[1] = emailFile;
        vm.ffi(inputGenerationInput);
        emailAddrRand = vm.parseBytes32(
            vm.readFile(string.concat(vm.projectRoot(), "/test/build_integration/sign_rand.txt"))
        );

        string memory publicInputFile = vm.readFile(
            string.concat(vm.projectRoot(), "/test/build_integration/email_sender_public.json")
        );
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        emailOp.command = command;
        emailOp.emailDomain = emailDomain;
        emailOp.maskedSubject = maskedSubject;
        emailOp.feeTokenName = feeTokenName;
        emailOp.feePerGas = core.maxFeePerGas();
        emailOp.emailProof = proofToBytes(
            string.concat(vm.projectRoot(), "/test/build_integration/email_sender_proof.json")
        );
        emailOp.emailAddrPointer = bytes32(vm.parseUint(pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 3]));
        emailOp.hasEmailRecipient = vm.parseUint(pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 4]) == 1;
        emailOp.recipientEmailAddrCommit = bytes32(vm.parseUint(pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 5]));
        emailOp.emailNullifier = bytes32(vm.parseUint(pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 2]));
        emailOp.dkimPublicKeyHash = bytes32(vm.parseUint(pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 0]));
        emailOp.timestamp = vm.parseUint(pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 6]);
    }

    function claimFund(
        uint256 registeredUnclaimId,
        string memory emailAddr,
        bytes32 relayerRand,
        bytes32 emailAddrRand
    ) internal returns (bytes32 newRelayerHash, bytes32 newEmailAddrPointer) {
        newRelayerHash;
        newEmailAddrPointer;

        string[] memory inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(vm.projectRoot(), "/test/bin/claim.sh");
        inputGenerationInput[1] = emailAddr;
        inputGenerationInput[2] = uint256(relayerRand).toHexString(32);
        inputGenerationInput[3] = uint256(emailAddrRand).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(
            string.concat(vm.projectRoot(), "/test/build_integration/claim_public.json")
        );
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        bytes32 recipientEmailAddrPointer = bytes32(vm.parseUint(pubSignals[1]));
        bytes memory proof = proofToBytes(string.concat(vm.projectRoot(), "/test/build_integration/claim_proof.json"));
        UnclaimsHandler(core.unclaimsHandler()).claimUnclaimedFund(
            registeredUnclaimId,
            recipientEmailAddrPointer,
            proof
        );
    }

    function claimState(
        uint256 registeredUnclaimId,
        string memory emailAddr,
        bytes32 relayerRand,
        bytes32 emailAddrRand
    ) internal returns (bytes32 newRelayerHash, bytes32 newEmailAddrPointer) {
        newRelayerHash;
        newEmailAddrPointer;

        string[] memory inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(vm.projectRoot(), "/test/bin/claim.sh");
        inputGenerationInput[1] = emailAddr;
        inputGenerationInput[2] = uint256(relayerRand).toHexString(32);
        inputGenerationInput[3] = uint256(emailAddrRand).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(
            string.concat(vm.projectRoot(), "/test/build_integration/claim_public.json")
        );
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        bytes32 recipientEmailAddrPointer = bytes32(vm.parseUint(pubSignals[1]));
        bytes memory proof = proofToBytes(string.concat(vm.projectRoot(), "/test/build_integration/claim_proof.json"));
        UnclaimsHandler(core.unclaimsHandler()).claimUnclaimedState(
            registeredUnclaimId,
            recipientEmailAddrPointer,
            proof
        );
    }

    function genAnnouncement(
        string memory emailAddr,
        bytes32 emailAddrRand
    ) internal returns (bytes32 emailAddrCommit, bytes memory proof) {
        string[] memory inputGenerationInput = new string[](3);
        inputGenerationInput[0] = string.concat(vm.projectRoot(), "/test/bin/announcement.sh");
        inputGenerationInput[1] = emailAddr;
        inputGenerationInput[2] = uint256(emailAddrRand).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(
            string.concat(vm.projectRoot(), "/test/build_integration/announcement_public.json")
        );
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        emailAddrCommit = bytes32(vm.parseUint(pubSignals[EMAIL_ADDR_FIELDS]));
        proof = proofToBytes(string.concat(vm.projectRoot(), "/test/build_integration/announcement_proof.json"));
    }

    function proofToBytes(string memory proofPath) internal view returns (bytes memory) {
        string memory proofFile = vm.readFile(proofPath);
        string[] memory pi_a = abi.decode(vm.parseJson(proofFile, ".pi_a"), (string[]));
        uint256[2] memory pA = [vm.parseUint(pi_a[0]), vm.parseUint(pi_a[1])];
        string[][] memory pi_b = abi.decode(vm.parseJson(proofFile, ".pi_b"), (string[][]));
        uint256[2][2] memory pB = [
            [vm.parseUint(pi_b[0][1]), vm.parseUint(pi_b[0][0])],
            [vm.parseUint(pi_b[1][1]), vm.parseUint(pi_b[1][0])]
        ];
        string[] memory pi_c = abi.decode(vm.parseJson(proofFile, ".pi_c"), (string[]));
        uint256[2] memory pC = [vm.parseUint(pi_c[0]), vm.parseUint(pi_c[1])];
        bytes memory proof = abi.encode(pA, pB, pC);
        return proof;
    }

    function _getUniswapSubjectTemplates() internal returns (string[][] memory) {
        delete subjectTemplates;
        subjectTemplates = new string[][](4);
        subjectTemplates[0] = ["Swap", "{tokenAmount}", "to", "{string}"];
        subjectTemplates[1] = ["Swap", "{tokenAmount}", "to", "{string}", "with", "{amount}", "slippage"];
        subjectTemplates[2] = ["Swap", "{tokenAmount}", "to", "{string}", "under", "{uint}", "sqrt", "price", "limit"];
        subjectTemplates[3] = [
            "Swap",
            "{tokenAmount}",
            "to",
            "{string}",
            "with",
            "{amount}",
            "slippage",
            "under",
            "{uint}",
            "sqrt",
            "price",
            "limit"
        ];

        return subjectTemplates;
    }

    function _getNFTSubjectTemplates() internal returns (string[][] memory) {
        delete subjectTemplates;
        subjectTemplates = new string[][](2);
        subjectTemplates[0] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"];
        subjectTemplates[1] = ["NFT", "Approve", "{recipient}", "for", "{uint}", "of", "{string}"];
        return subjectTemplates;
    }
}
