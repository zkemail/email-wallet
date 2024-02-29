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

    let extensionHandlerImpl = await deployContract("ExtensionHandler");
    abi = new ethers.Interface(extensionHandlerImpl.interface.formatJson());
    data = abi.encodeFunctionData("initialize");
    proxy = await deployContract("ERC1967Proxy", [extensionHandlerImpl.target, data])
    contractArtifact = await hre.artifacts.readArtifact("ExtensionHandler");
    const extensionHandler = new ethers.Contract(
        proxy,
        contractArtifact.abi,
        getWallet()
    );

    let accountHandlerImpl = await deployContract("AccountHandler");
    abi = new ethers.Interface(accountHandlerImpl.interface.formatJson());
    let emailValidityDuration = 1209600; // as same as `14 days` in foundry script
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

    let unclaimsHandlerImpl = await deployContract("UnclaimsHandler");
    abi = new ethers.Interface(unclaimsHandlerImpl.interface.formatJson());
    data = abi.encodeFunctionData("initialize", [
        await relayerHandler.getAddress(),
        await accountHandler.getAddress(),
        await verifierImpl.getAddress(),
        450000, // unclaimedFundClaimGas
        500000, // unclaimedStateClaimGas
        2592000, // unclaimsExpiryDuration = 30 days
        2000000000, // maxFeePerGas = 2 gwei
    ]);
    proxy = await deployContract("ERC1967Proxy", [unclaimsHandlerImpl.target, data])
    contractArtifact = await hre.artifacts.readArtifact("UnclaimsHandler");
    const unclaimsHandler = new ethers.Contract(
        proxy,
        contractArtifact.abi,
        getWallet()
    );
    let tx = await unclaimsHandler.registerUnclaimedState("0x2c606e8f632736700a9fe24e7de38401ab233262f51d6d9869aa508657c19b70", unclaimsHandler.target, "0x000000000000000000000000c989c0431febb3557ccf4e59919d15305d591668000000000000000000000000000000000000000000000000000000003c62aded", 1740773888, 0, "");
    await tx.wait();
    console.log(`tx: ${JSON.stringify(tx, null, 2)}`);
}