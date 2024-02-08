// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "../src/EmailWalletCore.sol";
import "../src/extensions/UniswapExtension.sol";
import "../src/extensions/NFTExtension.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";

contract Deploy is Script {
    uint256 constant emailValidityDuration = 1 hours;
    uint256 constant unclaimedFundClaimGas = 450000;
    uint256 constant unclaimedStateClaimGas = 500000;
    uint256 constant maxFeePerGas = 2 gwei;

    string[][] nftExtTemplates = new string[][](3);
    string[][] uniswapExtTemplates = new string[][](4);

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        address coreAddress = vm.envAddress("EMAIL_WALLET_CORE");
        if (coreAddress == address(0)) {
            console.log("EMAIL_WALLET_CORE env var not set");
            return;
        }
        EmailWalletCore core = EmailWalletCore(payable(address(coreAddress)));

        address tokenRegistry = vm.envAddress("TOKEN_REGISTRY");
        if (tokenRegistry == address(0)) {
            console.log("TOKEN_REGISTRY env var not set. Deploy TokenRegistry and set env var");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);

        bytes[] memory defaultExtensions = new bytes[](2);

        NFTExtension nftExt;
        {
            NFTExtension nftExtImpl = new NFTExtension();
            bytes memory data = abi.encodeWithSelector(
                NFTExtension(nftExtImpl).initialize.selector,
                    address(core)
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(nftExtImpl), data);
            nftExt = NFTExtension(payable(address(proxy)));

            console.log("NFTExtension proxy deployed at: %s", address(nftExt));
            console.log("NFTExtension implementation deployed at: %s", address(nftExtImpl));    
        }
        nftExtTemplates[0] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"];
        nftExtTemplates[1] = ["NFT", "Approve", "{recipient}", "for", "{uint}", "of", "{string}"];
        defaultExtensions[0] = abi.encode("NFTExtension", address(nftExt), nftExtTemplates, 0.001 ether); // TODO: Check max exec gas

        UniswapExtension uniExt;
        {
            address uniswapV3Router = 0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD;
            address uniswapV3Factory = 0x1F98431c8aD98523631AE4a59f267346ea31F984;

            UniswapExtension uniExtImpl = new UniswapExtension();
            bytes memory data = abi.encodeWithSelector(
                UniswapExtension(uniExtImpl).initialize.selector,
                    address(core),
                    tokenRegistry,
                    uniswapV3Router,
                    uniswapV3Factory
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(uniExtImpl), data);
            uniExt = UniswapExtension(payable(address(proxy)));

            console.log("UniswapExtension proxy deployed at: %s", address(uniExt));
            console.log("UniswapExtension implementation deployed at: %s", address(uniExtImpl));    
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

        console.log("---- DONE ----");

        vm.stopBroadcast();
    }
}
