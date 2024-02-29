// AllVerifiers
import * as dotenv from "dotenv";
import * as ethers from 'ethers';
import * as hre from 'hardhat';
import { getWallet } from "./utils";
dotenv.config();
// An example of a basic deploy script
// It will deploy a Greeter contract to selected network
// as well as verify it on Block Explorer if possible for the network
export default async function () {

    let contractArtifact = await hre.artifacts.readArtifact("TestERC20");
    const testERC20 = new ethers.Contract(
        "0xA9B5904976f4A339667638506963BAEcdDEc6cEF",
        contractArtifact.abi,
        getWallet()
    );
    const res = await testERC20.balanceOf(
        "0x66f3956e0b5fee0af1cce20b17e9e9f1ed712bbd"
    );
    // await tx.wait();
    console.log(`Balance checked`);
    console.log(res);
}
