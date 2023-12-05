// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "forge-std/Script.sol";
import "../src/Wallet.sol";
import "../src/EmailWalletCore.sol";
import "../src/verifier/Verifier.sol";
import "../src/extensions/UniswapExtension.sol";
import "../src/extensions/NFTExtension.sol";

contract Deploy is Script {
    uint256 constant maxFeePerGas = 2 gwei;
    uint256 constant emailValidityDuration = 1 hours;
    uint256 constant unclaimedFundClaimGas = 450000;
    uint256 constant unclaimedStateClaimGas = 500000;
    uint256 constant unclaimsExpiryDuration = 30 days;

    string[][] nftExtTemplates = new string[][](3);
    string[][] uniswapExtTemplates = new string[][](4);

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        address tokenRegistry = vm.envAddress("TOKEN_REGISTRY");
        if (tokenRegistry == address(0)) {
            console.log("TOKEN_REGISTRY env var not set. Deploy TokenRegistry and set env var");
            return;
        }

        address dkimRegistry = vm.envAddress("DKIM_REGISTRY");
        if (dkimRegistry == address(0)) {
            console.log("DKIM_REGISTRY env var not set. Deploy DKIMRegistry and set env var");
            return;
        }

        address priceOracle = vm.envAddress("PRICE_ORACLE");
        if (priceOracle == address(0)) {
            console.log("PRICE_ORACLE env var not set. Deploy UniswapTWAPOracle and set env var");
            return;
        }

        address weth = vm.envAddress("WETH");
        if (weth == address(0)) {
            console.log("WETH env var not set.");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);

        // Deploy Verifier
        AllVerifiers verifier = new AllVerifiers();

        // Deploy wallet implementation
        Wallet walletImp = new Wallet(address(weth));

        // Deploy core contract as proxy
        RelayerHandler relayerHandler = new RelayerHandler();
        ExtensionHandler extensionHandler = new ExtensionHandler();

        AccountHandler accountHandler;
        {
            AccountHandler accountHandlerImpl = new AccountHandler();
            bytes memory data = abi.encodeWithSelector(
                AccountHandler(accountHandlerImpl).initialize.selector,
                    address(relayerHandler),
                    dkimRegistry,
                    address(verifier),
                    address(walletImp),
                    emailValidityDuration
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(accountHandlerImpl), data);
            accountHandler = AccountHandler(payable(address(proxy)));
        }

        UnclaimsHandler unclaimsHandler = new UnclaimsHandler(
            address(relayerHandler),
            address(accountHandler),
            address(verifier),
            unclaimedFundClaimGas,
            unclaimedStateClaimGas,
            unclaimsExpiryDuration,
            maxFeePerGas
        );

        // Deploy core contract as proxy
        EmailWalletCore core;
        {
            EmailWalletCore coreImpl = new EmailWalletCore();
            bytes memory data = abi.encodeWithSelector(
                EmailWalletCore(coreImpl).initialize.selector,
                    address(relayerHandler),
                    address(accountHandler),
                    address(unclaimsHandler),
                    address(extensionHandler),
                    address(verifier),
                    tokenRegistry,
                    address(priceOracle),
                    address(weth),
                    maxFeePerGas,
                    emailValidityDuration,
                    unclaimedFundClaimGas,
                    unclaimedStateClaimGas
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(coreImpl), data);
            core = EmailWalletCore(payable(address(proxy)));
        }
        relayerHandler.transferOwnership(address(core));
        accountHandler.transferOwnership(address(core));
        unclaimsHandler.transferOwnership(address(core));
        extensionHandler.transferOwnership(address(core));
    
        // Set default extentions
        bytes[] memory defaultExtensions = new bytes[](2);

        NFTExtension nftExt = new NFTExtension(address(core));
        nftExtTemplates[0] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"];
        nftExtTemplates[1] = ["NFT", "Approve", "{recipient}", "for", "{uint}", "of", "{string}"];
        defaultExtensions[0] = abi.encode("NFTExtension", address(nftExt), nftExtTemplates, 0.001 ether); // TODO: Check max exec gas

        address uniswapV3Router = 0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD;
        address uniswapV3Factory = 0x1F98431c8aD98523631AE4a59f267346ea31F984;

        UniswapExtension uniExt = new UniswapExtension(
            address(core),
            tokenRegistry,
            uniswapV3Router,
            uniswapV3Factory
        );
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

        console.log("Verifier deployed at: %s", address(verifier));
        console.log("Wallet implementation deployed at: %s", address(walletImp));

        console.log("RelayerHandler deployed at: %s", address(relayerHandler));
        console.log("ExtensionHandler deployed at: %s", address(extensionHandler));
        console.log("AccountHandler deployed at: %s", address(accountHandler));
        console.log("UnclaimsHandler deployed at: %s", address(unclaimsHandler));

        console.log("EmailWalletCore deployed at: %s", address(core));
        console.log("NFT Extension deployed at: %s", address(nftExt));
        console.log("Uniswap Extension deployed at: %s", address(uniExt));
    }
}
