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
    string[][] uniswapExtTemplates = new string[][](1);

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
        EmailWalletCore core = new EmailWalletCore(
            address(verifier),
            address(walletImp),
            tokenRegistry,
            dkimRegistry,
            priceOracle,
            weth,
            maxFeePerGas,
            emailValidityDuration,
            unclaimedFundClaimGas,
            unclaimedStateClaimGas,
            unclaimsExpiryDuration
        );

        bytes[] memory defaultExtensions = new bytes[](2);

        NFTExtension nftExt = new NFTExtension(address(core));
        nftExtTemplates[0] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"];
        nftExtTemplates[1] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}", "safely"];
        nftExtTemplates[2] = ["NFT", "Approve", "{recipient}", "for", "{uint}", "of", "{string}"];
        defaultExtensions[0] = abi.encode("NFTExtension", address(nftExt), nftExtTemplates, 0.001 ether); // TODO: Check max exec gas

        address uniswapV2Router = 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;
        UniswapExtension uniExt = new UniswapExtension(address(core), address(tokenRegistry), uniswapV2Router);
        uniswapExtTemplates[0] = ["Swap", "{tokenAmount}", "to", "{string}"];
        defaultExtensions[1] = abi.encode("UniswapExtension", address(uniExt), uniswapExtTemplates, 0.001 ether); // TODO: Check max exec gas

        core.initialize(defaultExtensions);

        vm.stopBroadcast();

        console.log("Verifier deployed at: %s", address(verifier));
        console.log("Wallet implementation deployed at: %s", address(walletImp));
        console.log("EmailWalletCore deployed at: %s", address(core));
        console.log("NFT Extension deployed at: %s", address(nftExt));
        console.log("Uniswap Extension deployed at: %s", address(uniExt));
    }
}
