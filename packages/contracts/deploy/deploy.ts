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

  // TestToken
  let testToken = await deployContract("TestERC20", ["TestToken", "TT"]);

  // TokenRegistry
  let tokenRegistryImpl = await deployContract("TokenRegistry");
  let abi = new ethers.Interface(tokenRegistryImpl.interface.formatJson());
  let data = abi.encodeFunctionData("initialize");
  let proxy = await deployContract("ERC1967Proxy", [tokenRegistryImpl.target, data] )
  let contractArtifact = await hre.artifacts.readArtifact("TokenRegistry");
  const tokenRegistry = new ethers.Contract(
    proxy,
    contractArtifact.abi,
    getWallet() // Interact with the contract on behalf of this wallet
  );

  // TODO: The below code is not working in inMemoryNode
  // See https://github.com/matter-labs/hardhat-zksync/issues/714
  let chainName = process.env.CHAIN_NAME as string;
  let chainId = parseInt(process.env.CHAIN_ID as string);
  console.log(`Setting chainId ${chainId} for chain ${chainName}`);
  let tx = await tokenRegistry.setChainId(chainName, chainId);
  await tx.wait();

  let tokenName = process.env.TOKEN_NAME as string;
  console.log(`Setting token name ${tokenName} for chain ${chainName}`);
  tx = await tokenRegistry["setTokenAddress(uint256,string,address)"](chainId, tokenName, testToken.target);
  await tx.wait();

  let verifierImpl = await deployContract("AllVerifiers");

  let signer = process.env.SIGNER as string;
  let dkim = await deployContract("ECDSAOwnedDKIMRegistry", [signer]);

  let weth = process.env.WETH as string;
  let walletImpl = await deployContract("Wallet", [weth]);

  let relayerHandlerImpl = await deployContract("RelayerHandler");
  abi = new ethers.Interface(relayerHandlerImpl.interface.formatJson());
  data = abi.encodeFunctionData("initialize");
  proxy = await deployContract("ERC1967Proxy", [relayerHandlerImpl.target, data] )
  contractArtifact = await hre.artifacts.readArtifact("RelayerHandler");
  const relayerHandler = new ethers.Contract(
    proxy,
    contractArtifact.abi,
    getWallet()
  );

  let extensionHandlerImpl = await deployContract("ExtensionHandler");
  abi = new ethers.Interface(extensionHandlerImpl.interface.formatJson());
  data = abi.encodeFunctionData("initialize");
  proxy = await deployContract("ERC1967Proxy", [extensionHandlerImpl.target, data] )
  contractArtifact = await hre.artifacts.readArtifact("ExtensionHandler");
  const extensionHandler = new ethers.Contract(
    proxy,
    contractArtifact.abi,
    getWallet()
  );

  let accountHandlerImpl = await deployContract("AccountHandler");
  abi = new ethers.Interface(accountHandlerImpl.interface.formatJson());
  let emailValidityDuration = 3600; // as same as `1 hours` in foundry script
  data = abi.encodeFunctionData("initialize", [
    await relayerHandler.getAddress(), 
    await dkim.getAddress(), 
    await verifierImpl.getAddress(),
    await walletImpl.getAddress(),
    emailValidityDuration
  ]);
  proxy = await deployContract("ERC1967Proxy", [accountHandlerImpl.target, data] )
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
  proxy = await deployContract("ERC1967Proxy", [unclaimsHandlerImpl.target, data] )
  contractArtifact = await hre.artifacts.readArtifact("UnclaimsHandler");
  const unclaimsHandler = new ethers.Contract(
    proxy,
    contractArtifact.abi,
    getWallet()
  );

  let uniswapFactory = process.env.UNISWAP_FACTORY as string;
  let oracle = await deployContract("UniswapTWAPOracle", [uniswapFactory, weth]);

  let coreImpl = await deployContract("EmailWalletCore");
  abi = new ethers.Interface(coreImpl.interface.formatJson());
  data = abi.encodeFunctionData("initialize", [
    await relayerHandler.getAddress(),
    await accountHandler.getAddress(),
    await unclaimsHandler.getAddress(),
    await extensionHandler.getAddress(),
    await verifierImpl.getAddress(),
    await tokenRegistry.getAddress(),
    await oracle.getAddress(),
    await weth,
    2000000000, // maxFeePerGas = 2 gwei
    3600, // emailValidityDuration = 1 hour
    450000, // unclaimedFundClaimGas
    500000, // unclaimedStateClaimGas
  ]);
  proxy = await deployContract("ERC1967Proxy", [coreImpl.target, data] )
  contractArtifact = await hre.artifacts.readArtifact("EmailWalletCore");
  const core = new ethers.Contract(
    proxy,
    contractArtifact.abi,
    getWallet()
  );

  // TODO: The below code is not working in inMemoryNode
  // See https://github.com/matter-labs/hardhat-zksync/issues/714
  tx = await relayerHandler.transferOwnership(await core.getAddress());
  await tx.wait();
  tx = await accountHandler.transferOwnership(await core.getAddress());
  await tx.wait();
  tx = await unclaimsHandler.transferOwnership(await core.getAddress());
  await tx.wait();
  tx = await extensionHandler.transferOwnership(await core.getAddress());
  await tx.wait();
}