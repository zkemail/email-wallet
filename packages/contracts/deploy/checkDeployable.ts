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

    let contractArtifact = await hre.artifacts.readArtifact("AccountHandler");
    const accountHandler = new ethers.Contract(
        "0xC7b4b4d9080039c9e229D2414159559Deb303F54",
        contractArtifact.abi,
        getWallet()
    );
    const res = await accountHandler.isDeployableWallet(
        "0x2f2ac13fa03df06103f8799c616d5ee287103b5d51ae0173cdd277c664623ab5"
    );
    // await tx.wait();
    console.log(`Depolyable checked`);
    console.log(res);
}
