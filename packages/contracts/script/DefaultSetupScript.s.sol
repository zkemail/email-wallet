// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/verifier/Verifier.sol";
import "../src/utils/ECDSAOwnedDKIMRegistry.sol";
import "../src/utils/UniswapTWAPOracle.sol";
import "../src/extensions/NFTExtension.sol";
import "../src/extensions/UniswapExtension.sol";
import "../src/EmailWalletCore.sol";

contract TestERC20 is ERC20 {
    uint maxPerMint;

    constructor(string memory name, string memory tick, uint _maxPerMint) ERC20(name, tick) {
        maxPerMint = _maxPerMint;
    }

    function freeMint(uint256 amount) public {
        freeMint(msg.sender, amount);
    }

    function freeMint(address to, uint256 amount) public {
        require(amount <= maxPerMint, "amount exceeds maxPerMint");
        _mint(to, amount);
    }
}

contract Deploy is Script {
    TokenRegistry public tokenRegistry;
    TokenRegistry public tokenRegistryImpl;

    AllVerifiers verifierImpl;

    ECDSAOwnedDKIMRegistry dkim;

    Wallet walletImpl;

    RelayerHandler relayerHandler;
    RelayerHandler relayerHandlerImpl;

    ExtensionHandler extensionHandler;
    ExtensionHandler extensionHandlerImpl;

    AccountHandler accountHandler;
    AccountHandler accountHandlerImpl;

    UnclaimsHandler unclaimsHandler;
    UnclaimsHandler unclaimsHandlerImpl;

    UniswapTWAPOracle oracle;

    TestERC20 testToken;

    EmailWalletCore core;
    EmailWalletCore coreImpl;

    NFTExtension nftExt;
    NFTExtension nftExtImpl;

    UniswapExtension uniExt;
    UniswapExtension uniExtImpl;

    uint256 constant emailValidityDuration = 14 days;
    uint256 constant unclaimedFundClaimGas = 450000;
    uint256 constant unclaimedStateClaimGas = 500000;
    uint256 constant unclaimsExpiryDuration = 30 days;
    uint256 constant maxFeePerGas = 2 gwei;

    string[][] nftExtTemplates = new string[][](3);
    string[][] uniswapExtTemplates = new string[][](4);

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        address signer = vm.envAddress("SIGNER");
        if (signer == address(0)) {
            console.log("SIGNER env var not set");
            return;
        }

        address weth = vm.envAddress("WETH");
        if (weth == address(0)) {
            console.log("WETH env var not set.");
            return;
        }

        address uniswapFactory = vm.envAddress("UNISWAP_FACTORY");
        if (uniswapFactory == address(0)) {
            console.log("UNISWAP_FACTORY env var not set. Deploy UniswapTWAPOracle and set env var");
            return;
        }

        uint256 maxAmount = vm.envUint("MAX_AMOUNT");
        if (maxAmount == 0) {
            console.log("MAX_AMOUNT env var not set");
            return;
        }

        string memory chainName = vm.envString("CHAIN_NAME");
        
        uint256 chainId = vm.envUint("CHAIN_ID");

        string memory tokenName = vm.envString("TOKEN_NAME");

        vm.startBroadcast(deployerPrivateKey);

       {
            tokenRegistryImpl = new TokenRegistry();
            bytes memory data = abi.encodeWithSelector(
                TokenRegistry(tokenRegistryImpl).initialize.selector
            );
            ERC1967Proxy proxy = new ERC1967Proxy(address(tokenRegistryImpl), data);
            tokenRegistry = TokenRegistry(payable(address(proxy)));
       }

        verifierImpl = new AllVerifiers();

        dkim = new ECDSAOwnedDKIMRegistry(signer);

        walletImpl = new Wallet(address(weth));

        {
            relayerHandlerImpl = new RelayerHandler();
            bytes memory data = abi.encodeWithSelector(
                RelayerHandler(relayerHandlerImpl).initialize.selector
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(relayerHandlerImpl), data);
            relayerHandler = RelayerHandler(payable(address(proxy)));
        }

        {
            extensionHandlerImpl = new ExtensionHandler();
            bytes memory data = abi.encodeWithSelector(
                ExtensionHandler(extensionHandlerImpl).initialize.selector
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(extensionHandlerImpl), data);
            extensionHandler = ExtensionHandler(payable(address(proxy)));            
        }

        {
            accountHandlerImpl = new AccountHandler();
            bytes memory data = abi.encodeWithSelector(
                AccountHandler(accountHandlerImpl).initialize.selector,
                    address(relayerHandler),
                    dkim,
                    address(verifierImpl),
                    address(walletImpl),
                    emailValidityDuration
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(accountHandlerImpl), data);
            accountHandler = AccountHandler(payable(address(proxy)));
        }
        
        {
            unclaimsHandlerImpl = new UnclaimsHandler();
            bytes memory data = abi.encodeWithSelector(
                UnclaimsHandler(unclaimsHandlerImpl).initialize.selector,
                    address(relayerHandler),
                    address(accountHandler),
                    address(verifierImpl),
                    unclaimedFundClaimGas,
                    unclaimedStateClaimGas,
                    unclaimsExpiryDuration,
                    maxFeePerGas
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(unclaimsHandlerImpl), data);
            unclaimsHandler = UnclaimsHandler(payable(address(proxy)));
        }

        oracle = new UniswapTWAPOracle(uniswapFactory, weth);

        testToken = new TestERC20("EmailWalletV1Test", "TEST", maxAmount);
        tokenRegistry.setChainId(chainName, chainId);
        tokenRegistry.setTokenAddress(chainId, tokenName, address(testToken));

        {
            coreImpl = new EmailWalletCore();
            bytes memory data = abi.encodeWithSelector(
                EmailWalletCore(coreImpl).initialize.selector,
                    address(relayerHandler),
                    address(accountHandler),
                    address(unclaimsHandler),
                    address(extensionHandler),
                    address(verifierImpl),
                    address(tokenRegistry),
                    address(oracle),
                    address(weth),
                    2 gwei,
                    1 hours,
                    450000,
                    500000
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(coreImpl), data);
            core = EmailWalletCore(payable(address(proxy)));
            core.relayerHandler().transferOwnership(address(core));
            core.accountHandler().transferOwnership(address(core));
            core.unclaimsHandler().transferOwnership(address(core));
            core.extensionHandler().transferOwnership(address(core));
        }

        bytes[] memory defaultExtensions = new bytes[](2);

        {
            nftExtImpl = new NFTExtension();
            bytes memory data = abi.encodeWithSelector(
                NFTExtension(nftExtImpl).initialize.selector,
                    address(core)
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(nftExtImpl), data);
            nftExt = NFTExtension(payable(address(proxy)));    
        }
        nftExtTemplates[0] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"];
        nftExtTemplates[1] = ["NFT", "Approve", "{recipient}", "for", "{uint}", "of", "{string}"];
        defaultExtensions[0] = abi.encode("NFTExtension", address(nftExt), nftExtTemplates, 0.001 ether); // TODO: Check max exec gas
        nftExt.setNFTAddress(tokenName, address(testToken));

        {
            address uniswapV3Router = 0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD;
            address uniswapV3Factory = 0x1F98431c8aD98523631AE4a59f267346ea31F984;

            uniExtImpl = new UniswapExtension();
            bytes memory data = abi.encodeWithSelector(
                UniswapExtension(uniExtImpl).initialize.selector,
                    address(core),
                    tokenRegistry,
                    uniswapV3Router,
                    uniswapV3Factory
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(uniExtImpl), data);
            uniExt = UniswapExtension(payable(address(proxy))); 
        }
        uniswapExtTemplates[0] = ["Swap", "{tokenAmount}", "to", "{string}"];
        uniswapExtTemplates[1] = ["Swap", "{tokenAmount}", "to", "{string}", "with", "{amount}", "slippage"];
        uniswapExtTemplates[2] = [
            "Swap",
            "{tokenAmount}",
            "to",
            "{string}",
            "under",
            "{uint}",
            "sqrt",
            "price",
            "limit"
        ];
        uniswapExtTemplates[3] = [
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

        defaultExtensions[1] = abi.encode("UniswapExtension", address(uniExt), uniswapExtTemplates, 0.001 ether); // TODO: Check max exec gas

        core.initializeExtension(defaultExtensions);

        vm.stopBroadcast();

        console.log("TokenRegistry proxy deployed at: %s", address(tokenRegistry));
        console.log("TokenRegistry implementation deployed at: %s", address(tokenRegistryImpl));
        console.log("AllVerifiers implementation deployed at: %s", address(verifierImpl));
        console.log("ECDSAOwnedDKIMRegistry deployed at: %s", address(dkim));
        console.log("Wallet implementation deployed at: %s", address(walletImpl));
        console.log("RelayerHandler proxy deployed at: %s", address(relayerHandler));
        console.log("RelayerHandler implementation deployed at: %s", address(relayerHandlerImpl));
        console.log("ExtensionHandler proxy deployed at: %s", address(extensionHandler));
        console.log("ExtensionHandler implementation deployed at: %s", address(extensionHandlerImpl));
        console.log("AccountHandler proxy deployed at: %s", address(accountHandler));
        console.log("AccountHandler implementation deployed at: %s", address(accountHandlerImpl));
        console.log("UnclaimsHandler proxy deployed at: %s", address(unclaimsHandler));
        console.log("UnclaimsHandler implementation deployed at: %s", address(unclaimsHandlerImpl));
        console.log("UniswapTWAPOracle deployed at: %s", address(oracle));
        console.log("TestERC20 deployed at: %s", address(testToken));
        console.log("EmailWalletCore proxy deployed at: %s", address(core));
        console.log("EmailWalletCore implementation deployed at: %s", address(coreImpl));
        console.log("NFTExtension proxy deployed at: %s", address(nftExt));
        console.log("NFTExtension implementation deployed at: %s", address(nftExtImpl));
        console.log("UniswapExtension proxy deployed at: %s", address(uniExt));
        console.log("UniswapExtension implementation deployed at: %s", address(uniExtImpl));     
        console.log("---- DONE ----");
    }
}
