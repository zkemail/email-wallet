// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "@zk-email/contracts/DKIMRegistry.sol";
import "../src/EmailWalletCore.sol";
import "../src/utils/TokenRegistry.sol";
import "../src/utils/UniswapTWAPOracle.sol";
import "../src/libraries/BytesUtils.sol";
import "./mocks/TestERC20.sol";
import "./helpers/WETH9.sol";
import "../src/verifier/AccountCreationVerifier.sol";
import "../src/verifier/AccountInitVerifier.sol";
import "../src/verifier/AccountTransportVerifier.sol";
import "../src/verifier/ClaimVerifier.sol";
import "../src/verifier/EmailSenderVerifier.sol";
import "../src/verifier/Verifier.sol";

contract IntegrationTest is Test {
    using Strings for *;
    using console for *;

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

    TestERC20 wethToken;
    TestERC20 daiToken;
    TestERC20 usdcToken;

    uint256 public constant DOMAIN_FIELDS = 9;
    uint256 public constant SUBJECT_FIELDS = 17;

    uint256 maxFeePerGas = 10 ** 10;
    uint256 emailValidityDuration = 1 hours;
    uint256 unclaimedFundClaimGas = 0.0002 ether;
    uint256 unclaimedStateClaimGas = 0.0002 ether;
    uint256 unclaimedFundExpirationDuration = 30 days;

    address deployer;
    address relayer1;
    address relayer2;
    bytes32 relayer1Rand = 0x05f5b4f85b25760c2ee168c67c856afd371308a291de9d4c36a6e1c1c2a71693;
    bytes32 relayer1RandHash = 0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3;

    UserTestConfig user1 = UserTestConfig({
        emailAddr: "suegamisora@gmail.com",
        accountKey: 0x01eb9b204cc24c3baee11accc37d253a9c53e92b1a2cc07763475c135d575b76,
        emailAddrPointer: bytes32(0)
    });
    // bytes32 user1EmailAddrPointer = 0x1ff706660702f76a0daa706d68b15ea04fb6145fb5f4e54823ae80fa386e1b3f;
    // bytes32 user1AccountKeyCommit = 0x0136e61d55558414797fa9e8acccac39c52ef0b2c9b3fda0ef2d858a111333df;
    // bytes32 user1WalletSalt = 0x16dc3fd3780b524ba792b9e19ec9f7cbeb931912462a9b028cecfdff0eb29d28;


    

    function setUp() public virtual {
        deployer = vm.addr(1);
        relayer1 = vm.addr(2);
        relayer2 = vm.addr(3);

        vm.startPrank(deployer);

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
        tokenRegistry = new TokenRegistry();
        dkimRegistry = new DKIMRegistry();
        priceOracle = new UniswapTWAPOracle(address(0), address(0));
        weth = new WETH9();

        dkimRegistry.setDKIMPublicKeyHash("gmail.com", 0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788);

        // Deploy core contract as proxy
        address implementation = address(
            new EmailWalletCore(
                address(verifier),
                address(tokenRegistry),
                address(dkimRegistry),
                address(priceOracle),
                address(weth),
                maxFeePerGas,
                emailValidityDuration,
                unclaimedFundClaimGas,
                unclaimedStateClaimGas,
                unclaimedFundExpirationDuration
            )
        );
        bytes memory data = abi.encodeCall(EmailWalletCore.initialize, ());
        core = EmailWalletCore(payable(new ERC1967Proxy(implementation, data)));

        // Deploy some ERC20 test tokens and add them to registry
        wethToken = new TestERC20("WETH", "WETH");
        daiToken = new TestERC20("DAI", "DAI");
        usdcToken = new TestERC20("USDC", "USDC");
        tokenRegistry.setTokenAddress("WETH", address(wethToken));
        tokenRegistry.setTokenAddress("DAI", address(daiToken));
        tokenRegistry.setTokenAddress("USDC", address(usdcToken));
        vm.stopPrank();
        vm.startPrank(relayer1);
        core.registerRelayer(relayer1RandHash, "emailwallet.relayer@gmail.com", "emailwallet.com");
        vm.stopPrank();
    }

    // function accountCreation1(address relayer) internal {
    //     vm.startPrank(relayer);
    //     bytes memory psiPoint = abi.encode(
    //         0x174143d11cc953c0477683692a4a655f75954b36d01662728ba5ce1e5c78d28d,
    //         0x0e7662bbbc708e0f6371dedca8fdc916baa5d0bbd50e737f5a8a3d127d821cd7
    //     );
        
    //     core.createAccount(user1EmailAddrPointer, user1AccountKeyCommit, user1WalletSalt, psiPoint, proof);
    // }

    function testIntegration_Account_Creation() public {
        vm.startPrank(relayer1);
        (bytes32 relayerHash, bytes32 emailAddrPointer) = accountCreation(user1.emailAddr, relayer1Rand, user1.accountKey);
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        user1.emailAddrPointer = emailAddrPointer;
        vm.stopPrank();
    }

     function testIntegration_Account_Init() public {
        vm.startPrank(relayer1);
        (bytes32 relayerHash, bytes32 emailAddrPointer) = accountCreation(user1.emailAddr, relayer1Rand, user1.accountKey);
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        user1.emailAddrPointer = emailAddrPointer;
        string memory projectRoot = vm.projectRoot();
        (relayerHash, emailAddrPointer) = accountInit(string.concat(projectRoot,"/test/emails/account_init_test1.eml"), relayer1Rand, "gmail.com");
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        require(emailAddrPointer == user1.emailAddrPointer, "Email address pointer mismatch");
        vm.stopPrank();
    }


    function accountCreation(string memory emailAddr, bytes32 relayerRand, bytes32 accountKey) internal returns (bytes32 relayerHash, bytes32 emailAddrPointer) {
        string memory projectRoot = vm.projectRoot();
        string[] memory inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(projectRoot,"/test/bin/account_creation.sh");
        inputGenerationInput[1] = emailAddr;
        inputGenerationInput[2] = uint256(relayerRand).toHexString(32);
        inputGenerationInput[3] = uint256(accountKey).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(string.concat(projectRoot, "/test/build_integration/account_creation_public.json"));
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        relayerHash = bytes32(vm.parseUint(pubSignals[0]));
        emailAddrPointer = bytes32(vm.parseUint(pubSignals[1]));
        bytes32 accountKeyCommit = bytes32(vm.parseUint(pubSignals[2]));
        accountKeyCommit.logBytes32();
        bytes32 walletSalt = bytes32(vm.parseUint(pubSignals[3]));
        bytes32 x = bytes32(vm.parseUint(pubSignals[4]));
        bytes32 y = bytes32(vm.parseUint(pubSignals[5]));
        bytes memory psiPoint = abi.encode(x, y);
        bytes memory proof = proofToBytes(string.concat(projectRoot, "/test/build_integration/account_creation_proof.json"));
        core.createAccount(emailAddrPointer, accountKeyCommit, walletSalt, psiPoint, proof);
    }

    function accountInit(string memory emailFile, bytes32 relayerRand, string memory emailDomain) internal returns (bytes32 relayerHash, bytes32 emailAddrPointer) {
        string memory projectRoot = vm.projectRoot();
        string[] memory inputGenerationInput = new string[](3);
        inputGenerationInput[0] = string.concat(projectRoot,"/test/bin/account_init.sh");
        inputGenerationInput[1] = emailFile;
        inputGenerationInput[2] = uint256(relayerRand).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(string.concat(projectRoot, "/test/build_integration/account_init_public.json"));
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        relayerHash = bytes32(vm.parseUint(pubSignals[DOMAIN_FIELDS + 1]));
        bytes32 emailNullifier = bytes32(vm.parseUint(pubSignals[DOMAIN_FIELDS + 2]));
        emailAddrPointer = bytes32(vm.parseUint(pubSignals[DOMAIN_FIELDS + 3]));
        uint emailTimestamp = vm.parseUint(pubSignals[DOMAIN_FIELDS + 5]);
        bytes memory proof = proofToBytes(string.concat(projectRoot, "/test/build_integration/account_init_proof.json"));
        core.initializeAccount(emailAddrPointer, emailDomain, emailTimestamp, emailNullifier, proof);
    }

    function accountTransport(bytes32 oldAccountKeyCommit, string memory emailFile, string memory emailDomain, string memory emailAddr, bytes32 newRelayerRand, bytes32 accountKey) internal returns (bytes32 newRelayerHash, bytes32 newEmailAddrPointer) {
        string memory projectRoot = vm.projectRoot();
        string[] memory inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(projectRoot,"/test/bin/account_transport.sh");
        inputGenerationInput[1] = emailFile;
        inputGenerationInput[2] = uint256(oldAccountKeyCommit).toHexString(32);
        inputGenerationInput[3] = uint256(newRelayerRand).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(string.concat(projectRoot, "/test/build_integration/account_transport_public.json"));
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        bytes32 emailNullifier = bytes32(vm.parseUint(pubSignals[DOMAIN_FIELDS + 1]));
        uint emailTimestamp = vm.parseUint(pubSignals[DOMAIN_FIELDS + 5]); 
        bytes memory proof = proofToBytes(string.concat(projectRoot, "/test/build_integration/account_init_proof.json"));
        EmailProof memory transportEmailProof;
        transportEmailProof.nullifier = emailNullifier;
        transportEmailProof.timestamp = emailTimestamp;
        transportEmailProof.domain = emailDomain;
        transportEmailProof.proof = proof;

        inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(projectRoot,"/test/bin/account_creation.sh");
        inputGenerationInput[1] = emailAddr;
        inputGenerationInput[2] = uint256(newRelayerRand).toHexString(32);
        inputGenerationInput[3] = uint256(accountKey).toHexString(32);
        vm.ffi(inputGenerationInput);

        publicInputFile = vm.readFile(string.concat(projectRoot, "/test/build_integration/account_creation_public.json"));
        pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        newRelayerHash = bytes32(vm.parseUint(pubSignals[0]));
        newEmailAddrPointer = bytes32(vm.parseUint(pubSignals[1]));
        bytes32 newAccountKeyCommit = bytes32(vm.parseUint(pubSignals[2]));
        bytes32 x = bytes32(vm.parseUint(pubSignals[4]));
        bytes32 y = bytes32(vm.parseUint(pubSignals[5]));
        bytes memory newPSIPoint = abi.encode(x, y);
        bytes memory accountCreationProof = proofToBytes(string.concat(projectRoot, "/test/build_integration/account_creation_proof.json"));
        core.transportAccount(oldAccountKeyCommit, newEmailAddrPointer, newAccountKeyCommit, newPSIPoint, transportEmailProof, accountCreationProof);
    }    

    function proofToBytes(string memory proofPath) internal returns (bytes memory) {
        string memory proofFile = vm.readFile(proofPath);
        string[] memory pi_a = abi.decode(vm.parseJson(proofFile,"pi_a"), (string[]));
        uint256[2] memory pA = [
            vm.parseUint(pi_a[0]),
            vm.parseUint(pi_a[1])
        ];
        string[][] memory pi_b = abi.decode(vm.parseJson(proofFile,"pi_b"), (string[][]));
        uint256[2][2] memory pB = [
            [
                vm.parseUint(pi_b[0][1]),
                vm.parseUint(pi_b[0][0])
            ],
            [
                vm.parseUint(pi_b[1][1]),
                vm.parseUint(pi_b[1][0])
            ]
        ];
        string[] memory pi_c = abi.decode(vm.parseJson(proofFile,"pi_c"), (string[]));
        uint256[2] memory pC = [
            vm.parseUint(pi_c[0]),
            vm.parseUint(pi_c[1])
        ];
        bytes memory proof = abi.encode(pA, pB, pC);
        return proof;
    }

}
