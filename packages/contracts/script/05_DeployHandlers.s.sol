// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {RelayerHandler} from "../src/handlers/RelayerHandler.sol";
import {AccountHandler} from "../src/handlers/AccountHandler.sol";
import {UnclaimsHandler} from "../src/handlers/UnclaimsHandler.sol";
import {ExtensionHandler} from "../src/handlers/ExtensionHandler.sol";
import "forge-std/Script.sol";
import "forge-std/console.sol";

contract Deploy is Script {
    uint256 constant emailValidityDuration = 14 days;
    uint256 constant unclaimedFundClaimGas = 450000;
    uint256 constant unclaimedStateClaimGas = 500000;
    uint256 constant unclaimsExpiryDuration = 30 days;
    uint256 constant maxFeePerGas = 2 gwei;

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        if (deployerPrivateKey == 0) {
            console.log("PRIVATE_KEY env var not set");
            return;
        }

        address dkimRegistry = vm.envAddress("DKIM_REGISTRY");
        if (dkimRegistry == address(0)) {
            console.log("DKIM_REGISTRY env var not set. Deploy DKIMRegistry and set env var");
            return;
        }

        address wallet = vm.envAddress("WALLET");
        if (wallet == address(0)) {
            console.log("WALLET env var not set.");
            return;
        }

        address verifier = vm.envAddress("VERIFIER");
        if (verifier == address(0)) {
            console.log("VERIFIER env var not set.");
            return;
        }

        vm.startBroadcast(deployerPrivateKey);

        RelayerHandler relayerHandler;
        {
            RelayerHandler relayerHandlerImpl = new RelayerHandler();
            bytes memory data = abi.encodeWithSelector(
                RelayerHandler(relayerHandlerImpl).initialize.selector
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(relayerHandlerImpl), data);
            relayerHandler = RelayerHandler(payable(address(proxy)));

            console.log("RelayerHandler proxy deployed at: %s", address(relayerHandler));
            console.log("RelayerHandler implementation deployed at: %s", address(relayerHandlerImpl));
        }


        ExtensionHandler extensionHandler;
        {
            ExtensionHandler extensionHandlerImpl = new ExtensionHandler();
            bytes memory data = abi.encodeWithSelector(
                ExtensionHandler(extensionHandlerImpl).initialize.selector
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(extensionHandlerImpl), data);
            extensionHandler = ExtensionHandler(payable(address(proxy)));            

            console.log("ExtensionHandler proxy deployed at: %s", address(extensionHandler));
            console.log("ExtensionHandler implementation deployed at: %s", address(extensionHandlerImpl));
        }


        AccountHandler accountHandler;
        {
            AccountHandler accountHandlerImpl = new AccountHandler();
            bytes memory data = abi.encodeWithSelector(
                AccountHandler(accountHandlerImpl).initialize.selector,
                    address(relayerHandler),
                    dkimRegistry,
                    address(verifier),
                    address(wallet),
                    emailValidityDuration
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(accountHandlerImpl), data);
            accountHandler = AccountHandler(payable(address(proxy)));

            console.log("AccountHandler proxy deployed at: %s", address(accountHandler));
            console.log("AccountHandler implementation deployed at: %s", address(accountHandlerImpl));
        }


        UnclaimsHandler unclaimsHandler;
        {
            UnclaimsHandler unclaimsHandlerImpl = new UnclaimsHandler();
            bytes memory data = abi.encodeWithSelector(
                UnclaimsHandler(unclaimsHandlerImpl).initialize.selector,
                    address(relayerHandler),
                    address(accountHandler),
                    address(verifier),
                    unclaimedFundClaimGas,
                    unclaimedStateClaimGas,
                    unclaimsExpiryDuration,
                    maxFeePerGas
            );            
            ERC1967Proxy proxy = new ERC1967Proxy(address(unclaimsHandlerImpl), data);
            unclaimsHandler = UnclaimsHandler(payable(address(proxy)));

            console.log("UnclaimsHandler proxy deployed at: %s", address(unclaimsHandler));
            console.log("UnclaimsHandler implementation deployed at: %s", address(unclaimsHandlerImpl));
        }

        console.log("Please add these addresses into .env");
        console.log("---- DONE ----");
        
        vm.stopBroadcast();
    }
}
