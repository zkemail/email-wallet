import * as dotenv from "dotenv";
dotenv.config();
import { deployContract, getWallet } from "./utils";
import * as hre from 'hardhat';
import * as ethers from 'ethers';
import * as zk from 'zksync-ethers';
import { Deployer } from '@matterlabs/hardhat-zksync-deploy';
import { HardhatRuntimeEnvironment } from "hardhat/types";
// An example of a basic deploy script
// It will deploy a Greeter contract to selected network
// as well as verify it on Block Explorer if possible for the network
export default async function () {

    // // TestToken
    // let testToken = await deployContract("TestERC20", ["TestToken", "TT"]);

    // // TokenRegistry
    //   let tokenRegistryImpl = await deployContract("TokenRegistry");
    //   let abi = new ethers.Interface(tokenRegistryImpl.interface.formatJson());
    //   let data = abi.encodeFunctionData("initialize");
    //   let proxy = await deployContract("ERC1967Proxy", [tokenRegistryImpl.target, data])
    let contractArtifact = await hre.artifacts.readArtifact("TokenRegistry");
    const tokenRegistry = new ethers.Contract(
        "0x88026dCC1FA009B88E44CB8E9F23728CaDeb8579",
        contractArtifact.abi,
        getWallet() // Interact with the contract on behalf of this wallet
    );

    // // TODO: The below code is not working in inMemoryNode
    // // See https://github.com/matter-labs/hardhat-zksync/issues/714
    let chainName = process.env.CHAIN_NAME as string;
    let chainId = parseInt(process.env.CHAIN_ID as string);
    console.log(`Setting chainId ${chainId} for chain ${chainName}`);
    let tx = await tokenRegistry.setChainId(chainName, chainId);
    await tx.wait();

    let tokenName = process.env.TOKEN_NAME as string;
    console.log(`Setting token name ${tokenName} for chain ${chainName}`);
    tx = await tokenRegistry["setTokenAddress(uint256,string,address)"](chainId, tokenName, "0xA9B5904976f4A339667638506963BAEcdDEc6cEF");
    await tx.wait();

}