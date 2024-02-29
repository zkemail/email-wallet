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
    let testToken = await deployContract("TestERC20", ["TestToken2", "TT2"]);
    let tx = await testToken.freeMint(1000);
    await tx.wait();
    console.log(`TestToken2 deployed at ${testToken.address}`);
    console.log(`tx hash ${tx.hash}`);
}