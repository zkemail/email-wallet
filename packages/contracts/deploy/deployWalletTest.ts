import * as dotenv from "dotenv";
dotenv.config();
import { deployContract, getWallet } from "./utils";
import * as hre from 'hardhat';
import * as ethers from 'ethers';
import * as zk from 'zksync-ethers';
import { Deployer } from '@matterlabs/hardhat-zksync-deploy';
import { HardhatRuntimeEnvironment } from "hardhat/types";
import { utils } from 'zksync-ethers';
// An example of a basic deploy script
// It will deploy a Greeter contract to selected network
// as well as verify it on Block Explorer if possible for the network
export default async function () {

    let verifierImpl = await deployContract("AllVerifiers");

    let signer = process.env.SIGNER as string;
    let dkim = await deployContract("ECDSAOwnedDKIMRegistry", [signer]);

    let weth = process.env.WETH as string;
    let walletImpl = await deployContract("Wallet", [weth]);

    let relayerHandlerImpl = await deployContract("RelayerHandler");
    let abi = new ethers.Interface(relayerHandlerImpl.interface.formatJson());
    let data = abi.encodeFunctionData("initialize");
    let proxy = await deployContract("ERC1967Proxy", [relayerHandlerImpl.target, data])
    let contractArtifact = await hre.artifacts.readArtifact("RelayerHandler");
    const relayerHandler = new ethers.Contract(
        proxy,
        contractArtifact.abi,
        getWallet()
    );


    let accountHandlerImpl = await deployContract("AccountHandler");
    abi = new ethers.Interface(accountHandlerImpl.interface.formatJson());
    let emailValidityDuration = 3600; // as same as `1 hours` in foundry script
    const deployer = new Deployer(hre, getWallet());
    const proxyArtifact = await deployer.loadArtifact("ERC1967Proxy");
    const bytecodeHash = utils.hashBytecode(proxyArtifact.bytecode);
    data = abi.encodeFunctionData("initialize", [
        await relayerHandler.getAddress(),
        await dkim.getAddress(),
        await verifierImpl.getAddress(),
        await walletImpl.getAddress(),
        emailValidityDuration,
        bytecodeHash
    ]);
    proxy = await deployContract("ERC1967Proxy", [accountHandlerImpl.target, data])
    contractArtifact = await hre.artifacts.readArtifact("AccountHandler");
    const accountHandler = new ethers.Contract(
        proxy,
        contractArtifact.abi,
        getWallet()
    );

    let tx = await accountHandler.deployWallet("0x07ddcdf198d2a7a036f7638dd2121ab8d96a2ca67789239269b1a5a0d101b226");
    await tx.wait();
    console.log(`tx: ${JSON.stringify(tx, null, 2)}`);
}