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
// import "./mocks/TestERC20.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
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

    // TestERC20 wethToken;
    ERC20 daiToken;
    ERC20 usdcToken;

    address constant WETH_ADDR = 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
    address constant DAI_ADDR = 0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1;
    address constant USDC_ADDR = 0xaf88d065e77c8cC2239327C5EDb3A432268e5831;
    address constant UNISWAP_V3_FACTORY = 0x1F98431c8aD98523631AE4a59f267346ea31F984;
    address constant UNISWAP_V3_ROUTER = 0xE592427A0AEce92De3Edee1F18E0157C05861564;

    uint256 constant DOMAIN_FIELDS = 9;
    uint256 constant SUBJECT_FIELDS = 17;

    uint256 maxFeePerGas = 10 ** 9;
    uint256 emailValidityDuration = 36500 days;
    uint256 unclaimedFundClaimGas = 1000000;
    uint256 unclaimedStateClaimGas = 1000000;
    uint256 unclaimedFundExpirationDuration = 36500 days;

    address deployer;
    address relayer1;
    address relayer2;
    bytes32 relayer1Rand = 0x05f5b4f85b25760c2ee168c67c856afd371308a291de9d4c36a6e1c1c2a71693;
    bytes32 relayer1RandHash = 0x0029b17c2ee64b5a9762387d37e2b3614d9e59879edb15cc2fd3122c959116e3;
    bytes32 relayer2Rand = 0x11a036998ca261fcd981225b1cdcaa581d0861d476ff0491258bef3c88146b01;
    bytes32 relayer2RandHash = 0x2451956F89B22A433050F391776B5B00E53616CEED3313C0C3E1754D3F1D9A50;

    UserTestConfig user1 = UserTestConfig({
        emailAddr: "suegamisora@gmail.com",
        accountKey: 0x01eb9b204cc24c3baee11accc37d253a9c53e92b1a2cc07763475c135d575b76,
        emailAddrPointer: bytes32(0)
    });
    UserTestConfig user2 = UserTestConfig({
        emailAddr: "emaiwallet.bob@gmail.com",
        accountKey: 0x1e2ead4231d73a3c85b1ff883f212d998c41cc9d2a8bac238f6d351ff2c57249,
        emailAddrPointer: bytes32(0)
    });
    // bytes32 user1EmailAddrPointer = 0x1ff706660702f76a0daa706d68b15ea04fb6145fb5f4e54823ae80fa386e1b3f;
    // bytes32 user1AccountKeyCommit = 0x0136e61d55558414797fa9e8acccac39c52ef0b2c9b3fda0ef2d858a111333df;
    // bytes32 user1WalletSalt = 0x16dc3fd3780b524ba792b9e19ec9f7cbeb931912462a9b028cecfdff0eb29d28;


    

    function setUp() public virtual {
        vm.createSelectFork("https://arb1.arbitrum.io/rpc");
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
        priceOracle = new UniswapTWAPOracle(UNISWAP_V3_FACTORY, WETH_ADDR);
        // weth = new WETH9();
        weth = WETH9(payable(WETH_ADDR));

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
        core.registerRelayer(relayer1RandHash, "emailwallet.relayer@gmail.com", "emailwallet.com");
        vm.stopPrank();
        vm.startPrank(relayer2);
        core.registerRelayer(relayer2RandHash, "emailwallet.relayer2@gmail.com", "emailwallet2.com");
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

    function testIntegration_Account_Transport() public {
        vm.startPrank(relayer1);
        (bytes32 relayerHash, bytes32 emailAddrPointer) = accountCreation(user1.emailAddr, relayer1Rand, user1.accountKey);
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        user1.emailAddrPointer = emailAddrPointer;
        string memory projectRoot = vm.projectRoot();
        (relayerHash, emailAddrPointer) = accountInit(string.concat(projectRoot,"/test/emails/account_transport_test1.eml"), relayer1Rand, "gmail.com");
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        require(emailAddrPointer == user1.emailAddrPointer, "Email address pointer mismatch");
        vm.stopPrank();
        vm.startPrank(relayer2);
        (bytes32 newRelayerHash, bytes32 newEmailAddrPointer) = accountTransport(relayer1RandHash, core.accountKeyCommitOfPointer(user1.emailAddrPointer), string.concat(projectRoot,"/test/emails/account_init_test1.eml"), "gmail.com", "suegamisora@gmail.com", relayer2Rand, user1.accountKey);
        user1.emailAddrPointer = newEmailAddrPointer;
        require(newRelayerHash == relayer2RandHash, "Relayer hash mismatch");
        vm.stopPrank();
    }

    function testIntegration_Transfer_ETH_To_Internal() public {
        vm.startPrank(relayer1);
        (bytes32 relayerHash, bytes32 emailAddrPointer) = accountCreation(user1.emailAddr, relayer1Rand, user1.accountKey);
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        user1.emailAddrPointer = emailAddrPointer;
        string memory projectRoot = vm.projectRoot();
        (relayerHash, emailAddrPointer) = accountInit(string.concat(projectRoot,"/test/emails/account_init_test1.eml"), relayer1Rand, "gmail.com");
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        require(emailAddrPointer == user1.emailAddrPointer, "Email address pointer mismatch");
        (,,,,bytes32 walletSalt,) = core.infoOfAccountKeyCommit(core.accountKeyCommitOfPointer(user1.emailAddrPointer));
        address user1Wallet = core.getWalletOfSalt(walletSalt);
        vm.stopPrank();
        vm.startPrank(user1Wallet);
        deal(user1Wallet, 0.15 ether);
        weth.deposit{value: 0.15 ether}();
        require(weth.balanceOf(user1Wallet) == 0.15 ether, "User1 wallet balance before the transaction mismatch");
        vm.stopPrank();
        vm.startPrank(relayer1);
        (EmailOp memory emailOp, bytes32 emailAddrRand) = genEmailOpPartial(string.concat(projectRoot,"/test/emails/token_transfer_test1.eml"), relayer1Rand, "Send", "Send 0.1 ETH to ", "gmail.com", "ETH");
        emailOp.walletParams.tokenName = "ETH";
        emailOp.walletParams.amount = 0.1 ether;
        deal(relayer1, core.unclaimedFundClaimGas() * core.maxFeePerGas());
        (bool success, bytes memory reason) = core.handleEmailOp{value: core.unclaimedFundClaimGas() * core.maxFeePerGas()}(emailOp);
        assertEq(success, true, string(reason));
        require(weth.balanceOf(user1Wallet) < 0.05 ether, "User1 wallet balance after the transaction is too large");
        require(weth.balanceOf(address(core)) == 0.1 ether, "Core contract weth balance mismatch");
        require(address(core).balance == core.unclaimedFundClaimGas() * core.maxFeePerGas(), "Core contract eth balance mismatch");
        (relayerHash, emailAddrPointer) = accountCreation(user2.emailAddr, relayer1Rand, user2.accountKey);
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        user2.emailAddrPointer = emailAddrPointer;
        (relayerHash, emailAddrPointer) = accountInit(string.concat(projectRoot,"/test/emails/account_init_test2.eml"), relayer1Rand, "gmail.com");
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        require(emailAddrPointer == user2.emailAddrPointer, "Email address pointer mismatch");
        (,,,, walletSalt,) = core.infoOfAccountKeyCommit(core.accountKeyCommitOfPointer(user2.emailAddrPointer));
        address user2Wallet = core.getWalletOfSalt(walletSalt);
        require(weth.balanceOf(user2Wallet) == 0, "User2 wallet balance mismatch");
        claimFund(user2.emailAddr, relayer1Rand, emailAddrRand);
        require(weth.balanceOf(user2Wallet) == 0.1 ether, "User2 wallet balance mismatch");
        require(weth.balanceOf(address(core)) == 0, "Core contract balance mismatch");
        vm.stopPrank();
    }

    function testIntegration_Transfer_ETH_To_External() public {
        vm.startPrank(relayer1);
        (bytes32 relayerHash, bytes32 emailAddrPointer) = accountCreation(user1.emailAddr, relayer1Rand, user1.accountKey);
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        user1.emailAddrPointer = emailAddrPointer;
        string memory projectRoot = vm.projectRoot();
        (relayerHash, emailAddrPointer) = accountInit(string.concat(projectRoot,"/test/emails/account_init_test1.eml"), relayer1Rand, "gmail.com");
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        require(emailAddrPointer == user1.emailAddrPointer, "Email address pointer mismatch");
        (,,,,bytes32 walletSalt,) = core.infoOfAccountKeyCommit(core.accountKeyCommitOfPointer(user1.emailAddrPointer));
        address user1Wallet = core.getWalletOfSalt(walletSalt);
        vm.stopPrank();
        vm.startPrank(user1Wallet);
        deal(user1Wallet, 0.3 ether);
        weth.deposit{value: 0.3 ether}();
        vm.stopPrank();
        vm.startPrank(relayer1);
        address recipient = vm.addr(4);
        (EmailOp memory emailOp,) = genEmailOpPartial(string.concat(projectRoot,"/test/emails/token_transfer_test2.eml"), relayer1Rand, "Send", string.concat("Send 0.25 ETH to ",recipient.toHexString()), "gmail.com", "ETH");
        emailOp.walletParams.tokenName = "ETH";
        emailOp.walletParams.amount = 0.25 ether;
        emailOp.recipientETHAddr = recipient;
        (bool success, bytes memory reason) = core.handleEmailOp{value: 0}(emailOp);
        assertEq(success, true, string(reason));
        require(weth.balanceOf(user1Wallet) < 0.05 ether, "User1 wallet balance is too large");
        require(recipient.balance == 0.25 ether, "Recipient address balance mismatch");
        require(weth.balanceOf(address(core)) == 0, "Core contract balance mismatch");
        vm.stopPrank();
    }

    function testIntegration_Transfers_Random() public {
        vm.startPrank(relayer1);
        (bytes32 relayerHash, bytes32 emailAddrPointer) = accountCreation(user1.emailAddr, relayer1Rand, user1.accountKey);
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        user1.emailAddrPointer = emailAddrPointer;
        (relayerHash, emailAddrPointer) = accountInit(string.concat(vm.projectRoot(),"/test/emails/account_init_test1.eml"), relayer1Rand, "gmail.com");
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        require(emailAddrPointer == user1.emailAddrPointer, "Email address pointer mismatch");
        (,,,,bytes32 walletSalt,) = core.infoOfAccountKeyCommit(core.accountKeyCommitOfPointer(user1.emailAddrPointer));
        address user1Wallet = core.getWalletOfSalt(walletSalt);
        (relayerHash, emailAddrPointer) = accountCreation(user2.emailAddr, relayer1Rand, user2.accountKey);
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        user2.emailAddrPointer = emailAddrPointer;
        (relayerHash, emailAddrPointer) = accountInit(string.concat(vm.projectRoot(),"/test/emails/account_init_test2.eml"), relayer1Rand, "gmail.com");
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        require(emailAddrPointer == user2.emailAddrPointer, "Email address pointer mismatch");
        (,,,, walletSalt,) = core.infoOfAccountKeyCommit(core.accountKeyCommitOfPointer(user2.emailAddrPointer));
        address user2Wallet = core.getWalletOfSalt(walletSalt);
        // address recipient = vm.addr(4);
        vm.stopPrank();
        vm.startPrank(user1Wallet);
        deal(user1Wallet, 20 ether);
        weth.deposit{value: 20 ether}();
        deal(address(daiToken), user1Wallet, 20*10000 ether);
        deal(address(usdcToken), user1Wallet, 20*10000*(10**6));
        vm.stopPrank();
        vm.startPrank(user2Wallet);
        deal(user2Wallet, 20 ether);
        weth.deposit{value: 20 ether}();
        deal(address(daiToken), user2Wallet, 20*10000 ether);
        deal(address(usdcToken), user2Wallet, 20*10000*(10**6));
        vm.stopPrank();

        vm.startPrank(relayer1);
        bytes32 randomHash = keccak256(abi.encode(blockhash(block.number - 1)));
        randomHash.logBytes32();
        string[3] memory amountStrs = ["1", "0.2", "0.03"];
        string[3] memory tokens = ["ETH", "DAI", "USDC"];
        UserTestConfig[2] memory users = [user1, user2];
        bool[3][3][2] memory usedEmail;
        uint idx = 0;
        while(idx < 8) {
            idx.logUint();
            randomHash = keccak256(abi.encode(randomHash));
            uint amountSelector = uint(randomHash) % 3;
            randomHash = keccak256(abi.encode(randomHash));
            uint tokenSelector = uint(randomHash) % 3;
            randomHash = keccak256(abi.encode(randomHash));
            uint senderSelector = uint(randomHash) % 2;
            randomHash = keccak256(abi.encode(randomHash));
            uint feeSelector = uint(randomHash) % 3;
            amountSelector.logUint();
            tokenSelector.logUint();
            senderSelector.logUint();
            feeSelector.logUint();
            if(usedEmail[senderSelector][tokenSelector][amountSelector]) {
                continue;
            }
            usedEmail[senderSelector][tokenSelector][amountSelector] = true;
            idx ++;

            string.concat(vm.projectRoot(),"/test/emails/random_test/",amountSelector.toString(),"_",tokens[tokenSelector],"_",senderSelector.toString(),"_",(1-senderSelector).toString(),".eml").logString();
            (EmailOp memory emailOp, bytes32 emailAddrRand) = genEmailOpPartial(string.concat(vm.projectRoot(),"/test/emails/random_test/",amountSelector.toString(),"_",tokens[tokenSelector],"_",senderSelector.toString(),"_",(1-senderSelector).toString(),".eml"), relayer1Rand, "Send", string.concat("Send ",amountStrs[amountSelector]," ",tokens[tokenSelector]," to "), "gmail.com", tokens[feeSelector]);
            emailOp.walletParams.tokenName = tokens[tokenSelector];
            if(tokenSelector == 0 || tokenSelector == 1) {
                emailOp.walletParams.amount = [uint(1 ether), uint(0.2 ether), uint(0.03 ether)][amountSelector];
            } else {
                emailOp.walletParams.amount = [uint(1 * (10**6)), uint(0.2 * (10**6)), uint(0.03 * (10**6))][amountSelector];
            }
            deal(relayer1, core.unclaimedFundClaimGas() * core.maxFeePerGas());
            (bool success, bytes memory reason) = core.handleEmailOp{value: core.unclaimedFundClaimGas() * core.maxFeePerGas()}(emailOp);
            success.logBool();
            assertEq(success, true, string(reason));
            claimFund(users[1-senderSelector].emailAddr, relayer1Rand, emailAddrRand);
        }
        vm.stopPrank();
    }

    function testIntegration_Swap_Tokens() public {
         vm.startPrank(relayer1);
        (bytes32 relayerHash, bytes32 emailAddrPointer) = accountCreation(user1.emailAddr, relayer1Rand, user1.accountKey);
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        user1.emailAddrPointer = emailAddrPointer;
        (relayerHash, emailAddrPointer) = accountInit(string.concat(vm.projectRoot(),"/test/emails/account_init_test1.eml"), relayer1Rand, "gmail.com");
        require(relayerHash == relayer1RandHash, "Relayer hash mismatch");
        require(emailAddrPointer == user1.emailAddrPointer, "Email address pointer mismatch");
        (,,,,bytes32 walletSalt,) = core.infoOfAccountKeyCommit(core.accountKeyCommitOfPointer(user1.emailAddrPointer));
        address user1Wallet = core.getWalletOfSalt(walletSalt);
        // address recipient = vm.addr(4);
        vm.stopPrank();
        vm.startPrank(user1Wallet);
        deal(user1Wallet, 20 ether);
        weth.deposit{value: 20 ether}();
        deal(address(daiToken), user1Wallet, 20*10000 ether);
        deal(address(usdcToken), user1Wallet, 20*10000*(10**6));
        vm.stopPrank();

        vm.startPrank(relayer1);
        (EmailOp memory emailOp,) = genEmailOpPartial(string.concat(vm.projectRoot(),"/test/emails/uniswap_test1.eml"), relayer1Rand, "Swap", "Swap 0.2 ETH to DAI", "gmail.com", "ETH");
        bytes[] memory extensionBytes = new bytes[](2);
        extensionBytes[0] = abi.encode(uint256(0.2 ether), "ETH");
        extensionBytes[1] = abi.encode("DAI");
        emailOp.extensionParams = ExtensionParams(0, extensionBytes);
        deal(relayer1, core.unclaimedFundClaimGas() * core.maxFeePerGas());
        (bool success, bytes memory reason) = core.handleEmailOp{value: core.unclaimedFundClaimGas() * core.maxFeePerGas()}(emailOp);
        require(success, string(reason));
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

    function accountTransport(bytes32 oldRelayerRandHash, bytes32 oldAccountKeyCommit, string memory emailFile, string memory emailDomain, string memory emailAddr, bytes32 newRelayerRand, bytes32 accountKey) internal returns (bytes32 newRelayerHash, bytes32 newEmailAddrPointer) {
        EmailProof memory transportEmailProof = genAccountTransportProof(oldRelayerRandHash, emailFile, emailDomain, newRelayerRand);

        string[] memory inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(vm.projectRoot(),"/test/bin/account_creation.sh");
        inputGenerationInput[1] = emailAddr;
        inputGenerationInput[2] = uint256(newRelayerRand).toHexString(32);
        inputGenerationInput[3] = uint256(accountKey).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(string.concat(vm.projectRoot(), "/test/build_integration/account_creation_public.json"));
        inputGenerationInput = abi.decode(vm.parseJson(publicInputFile), (string[]));
        newRelayerHash = bytes32(vm.parseUint(inputGenerationInput[0]));
        newEmailAddrPointer = bytes32(vm.parseUint(inputGenerationInput[1]));
        bytes32 newAccountKeyCommit = bytes32(vm.parseUint(inputGenerationInput[2]));
        bytes memory newPSIPoint = abi.encode(bytes32(vm.parseUint(inputGenerationInput[4])), bytes32(vm.parseUint(inputGenerationInput[5])));
        bytes memory accountCreationProof = proofToBytes(string.concat(vm.projectRoot(), "/test/build_integration/account_creation_proof.json"));
        core.transportAccount(oldAccountKeyCommit, newEmailAddrPointer, newAccountKeyCommit, newPSIPoint, transportEmailProof, accountCreationProof);
    }    

    function genAccountTransportProof(bytes32 oldRelayerRandHash, string memory emailFile, string memory emailDomain, bytes32 newRelayerRand) private returns (EmailProof memory) {
        string memory projectRoot = vm.projectRoot();
        string[] memory inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(projectRoot,"/test/bin/account_transport.sh");
        inputGenerationInput[1] = emailFile;
        inputGenerationInput[2] = uint256(oldRelayerRandHash).toHexString(32);
        inputGenerationInput[3] = uint256(newRelayerRand).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(string.concat(projectRoot, "/test/build_integration/account_transport_public.json"));
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        EmailProof memory transportEmailProof;
        transportEmailProof.nullifier = bytes32(vm.parseUint(pubSignals[DOMAIN_FIELDS + 1]));
        transportEmailProof.timestamp = vm.parseUint(pubSignals[DOMAIN_FIELDS + 5]);
        transportEmailProof.domain = emailDomain;
        transportEmailProof.proof = proofToBytes(string.concat(projectRoot, "/test/build_integration/account_transport_proof.json"));
        return transportEmailProof;
    } 

    function genEmailOpPartial(string memory emailFile, bytes32 relayerRand, string memory command, string memory maskedSubject, string memory emailDomain, string memory feeTokenName) internal returns (EmailOp memory emailOp, bytes32 emailAddrRand) {
        string[] memory inputGenerationInput = new string[](3);
        inputGenerationInput[0] = string.concat(vm.projectRoot(),"/test/bin/email_sender.sh");
        inputGenerationInput[1] = emailFile;
        inputGenerationInput[2] = uint256(relayerRand).toHexString(32);
        vm.ffi(inputGenerationInput);
        inputGenerationInput = new string[](2);
        inputGenerationInput[0] = string.concat(vm.projectRoot(),"/test/bin/extract_sign_rand.sh");
        inputGenerationInput[1] = emailFile;
        vm.ffi(inputGenerationInput);
        emailAddrRand = vm.parseBytes32(vm.readFile(string.concat(vm.projectRoot(), "/test/build_integration/sign_rand.txt")));

        string memory publicInputFile = vm.readFile(string.concat(vm.projectRoot(), "/test/build_integration/email_sender_public.json"));
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        emailOp.command = command;
        emailOp.emailDomain = emailDomain;
        emailOp.maskedSubject = maskedSubject;
        emailOp.feeTokenName = feeTokenName;
        emailOp.feePerGas = core.maxFeePerGas();
        emailOp.emailProof = proofToBytes(string.concat(vm.projectRoot(), "/test/build_integration/email_sender_proof.json"));
        emailOp.emailAddrPointer = bytes32(vm.parseUint(pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 3]));
        emailOp.hasEmailRecipient = vm.parseUint(pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 4]) == 1;
        emailOp.recipientEmailAddrCommit = bytes32(vm.parseUint(pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 5]));
        emailOp.emailNullifier = bytes32(vm.parseUint(pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 2]));
        emailOp.timestamp = vm.parseUint(pubSignals[SUBJECT_FIELDS + DOMAIN_FIELDS + 6]);
    }    

    function claimFund(string memory emailAddr, bytes32 relayerRand, bytes32 emailAddrRand) internal returns (bytes32 newRelayerHash, bytes32 newEmailAddrPointer) {
        string[] memory inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(vm.projectRoot(),"/test/bin/claim.sh");
        inputGenerationInput[1] = emailAddr;
        inputGenerationInput[2] = uint256(relayerRand).toHexString(32);
        inputGenerationInput[3] = uint256(emailAddrRand).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(string.concat(vm.projectRoot(), "/test/build_integration/claim_public.json"));
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        bytes32 recipientEmailAddrPointer = bytes32(vm.parseUint(pubSignals[1]));
        bytes32 emailAddrCommit = bytes32(vm.parseUint(pubSignals[2]));
        bytes memory proof = proofToBytes(string.concat(vm.projectRoot(), "/test/build_integration/claim_proof.json"));
        core.claimUnclaimedFund(emailAddrCommit, recipientEmailAddrPointer, proof);
    } 

    function claimState(string memory emailAddr, bytes32 relayerRand, bytes32 emailAddrRand) internal returns (bytes32 newRelayerHash, bytes32 newEmailAddrPointer) {
        string[] memory inputGenerationInput = new string[](4);
        inputGenerationInput[0] = string.concat(vm.projectRoot(),"/test/bin/claim.sh");
        inputGenerationInput[1] = emailAddr;
        inputGenerationInput[2] = uint256(relayerRand).toHexString(32);
        inputGenerationInput[3] = uint256(emailAddrRand).toHexString(32);
        vm.ffi(inputGenerationInput);

        string memory publicInputFile = vm.readFile(string.concat(vm.projectRoot(), "/test/build_integration/claim_public.json"));
        string[] memory pubSignals = abi.decode(vm.parseJson(publicInputFile), (string[]));
        bytes32 recipientEmailAddrPointer = bytes32(vm.parseUint(pubSignals[1]));
        bytes32 emailAddrCommit = bytes32(vm.parseUint(pubSignals[2]));
        bytes memory proof = proofToBytes(string.concat(vm.projectRoot(), "/test/build_integration/claim_proof.json"));
        core.claimUnclaimedState(emailAddrCommit, recipientEmailAddrPointer, proof);
    } 

    function proofToBytes(string memory proofPath) internal view returns (bytes memory) {
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
