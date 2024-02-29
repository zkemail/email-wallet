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

  let contractArtifact = await hre.artifacts.readArtifact("AllVerifiers");
  const allVerifiers = new ethers.Contract(
    "0x394d424b8ae74A3A0062803A2d552197AA8561F7",
    contractArtifact.abi,
    getWallet()
  );
  const res = await allVerifiers.verifyAccountCreationProof(
    "gmail.com",
    "0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
    "0x2c02f4fc7cd2845c46afe87627bc8f02c3ec838978ea983484e44040c462916b",
    1709002239,
    "0x1ecb36b7695a7ce893b24bc0b39727c8866a7f874bdf016bea54d7d0da3d610d",
    "0x1103a476abb83ff3885e90486157dbbc5db4df3236633b20335fb0da8b0135a40f10197b467bb342d01016490456850e5eb5c31d69a18da7ace8a2ac77ae70ab",
    "0x2bf5b43ab3448e56262a124870cc8a725683f96253d10e58367e40d52291b7f302fcb5d8e33d9de0f523e9f9317adda86a3e0db883e4b3feaee4efb8111d5156240bd466fcdec5b1cb05a1b2b94f2f007145eb424f9dbdd7af5168497ae8c5f9006f9c8924a07926fe1127880d0d710aa9c1e34f86da2530a5b641740a28bbf11f1ad35499948d70592a7bd0a3bc269ee3fe1d76872ec691b26ebcca5058b85815bc5a909d34ba5d4b97639c4e124b5ab64f0d9bf49dea6de20eea3b498af445066dba5879ac42fcdfebc068e6a6e39a601bfeff13cf5538380948f52a37b113232d39039aabe954fd645feef716a038db883f18db64a7a87ef9dee2ac791f32"
  );
  // await tx.wait();
  console.log(`Verified account creation proof`);
  console.log(res);


  /**
   *   function accountCreation
    emailAddr
    emaiwallet.bob@gmail.com
    walletSalt
    0x1ecb36b7695a7ce893b24bc0b39727c8866a7f874bdf016bea54d7d0da3d610d
    createAccount:
    
    emailProof.domain
    gmail.com
    emailProof.dkimPublicKeyHash
    0x0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788
    emailProof.nullifier
    0x2c02f4fc7cd2845c46afe87627bc8f02c3ec838978ea983484e44040c462916b
    emailProof.timestamp
    1709002239
    walletSalt
    0x1ecb36b7695a7ce893b24bc0b39727c8866a7f874bdf016bea54d7d0da3d610d
    psiPoint
    0x1103a476abb83ff3885e90486157dbbc5db4df3236633b20335fb0da8b0135a40f10197b467bb342d01016490456850e5eb5c31d69a18da7ace8a2ac77ae70ab
    emailProof.proof
    0x2bf5b43ab3448e56262a124870cc8a725683f96253d10e58367e40d52291b7f302fcb5d8e33d9de0f523e9f9317adda86a3e0db883e4b3feaee4efb8111d5156240bd466fcdec5b1cb05a1b2b94f2f007145eb424f9dbdd7af5168497ae8c5f9006f9c8924a07926fe1127880d0d710aa9c1e34f86da2530a5b641740a28bbf11f1ad35499948d70592a7bd0a3bc269ee3fe1d76872ec691b26ebcca5058b85815bc5a909d34ba5d4b97639c4e124b5ab64f0d9bf49dea6de20eea3b498af445066dba5879ac42fcdfebc068e6a6e39a601bfeff13cf5538380948f52a37b113232d39039aabe954fd645feef716a038db883f18db64a7a87ef9dee2ac791f32
   * 
   */

  // // TestToken
  // let testToken = await deployContract("TestERC20", ["TestToken", "TT"]);

  // // TokenRegistry
  // let tokenRegistryImpl = await deployContract("TokenRegistry");
  // let abi = new ethers.Interface(tokenRegistryImpl.interface.formatJson());
  // let data = abi.encodeFunctionData("initialize");
  // let proxy = await deployContract("ERC1967Proxy", [tokenRegistryImpl.target, data] )
  // let contractArtifact = await hre.artifacts.readArtifact("TokenRegistry");
  // const tokenRegistry = new ethers.Contract(
  //   proxy,
  //   contractArtifact.abi,
  //   getWallet() // Interact with the contract on behalf of this wallet
  // );

  // // TODO: The below code is not working in inMemoryNode
  // // See https://github.com/matter-labs/hardhat-zksync/issues/714
  // let chainName = process.env.CHAIN_NAME as string;
  // let chainId = parseInt(process.env.CHAIN_ID as string);
  // console.log(`Setting chainId ${chainId} for chain ${chainName}`);
  // let tx = await tokenRegistry.setChainId(chainName, chainId);
  // await tx.wait();

  // let tokenName = process.env.TOKEN_NAME as string;
  // console.log(`Setting token name ${tokenName} for chain ${chainName}`);
  // tx = await tokenRegistry["setTokenAddress(uint256,string,address)"](chainId, tokenName, testToken.target);
  // await tx.wait();

  // let verifierImpl = await deployContract("AllVerifiers");

  // let signer = process.env.SIGNER as string;
  // let dkim = await deployContract("ECDSAOwnedDKIMRegistry", [signer]);

  // let weth = process.env.WETH as string;
  // let walletImpl = await deployContract("Wallet", [weth]);

  // let relayerHandlerImpl = await deployContract("RelayerHandler");
  // abi = new ethers.Interface(relayerHandlerImpl.interface.formatJson());
  // data = abi.encodeFunctionData("initialize");
  // proxy = await deployContract("ERC1967Proxy", [relayerHandlerImpl.target, data] )
  // contractArtifact = await hre.artifacts.readArtifact("RelayerHandler");
  // const relayerHandler = new ethers.Contract(
  //   proxy,
  //   contractArtifact.abi,
  //   getWallet()
  // );

  // let extensionHandlerImpl = await deployContract("ExtensionHandler");
  // abi = new ethers.Interface(extensionHandlerImpl.interface.formatJson());
  // data = abi.encodeFunctionData("initialize");
  // proxy = await deployContract("ERC1967Proxy", [extensionHandlerImpl.target, data] )
  // contractArtifact = await hre.artifacts.readArtifact("ExtensionHandler");
  // const extensionHandler = new ethers.Contract(
  //   proxy,
  //   contractArtifact.abi,
  //   getWallet()
  // );

  // let accountHandlerImpl = await deployContract("AccountHandler");
  // abi = new ethers.Interface(accountHandlerImpl.interface.formatJson());
  // let emailValidityDuration = 3600; // as same as `1 hours` in foundry script
  // data = abi.encodeFunctionData("initialize", [
  //   await relayerHandler.getAddress(), 
  //   await dkim.getAddress(), 
  //   await verifierImpl.getAddress(),
  //   await walletImpl.getAddress(),
  //   emailValidityDuration
  // ]);
  // proxy = await deployContract("ERC1967Proxy", [accountHandlerImpl.target, data] )
  // contractArtifact = await hre.artifacts.readArtifact("AccountHandler");
  // const accountHandler = new ethers.Contract(
  //   proxy,
  //   contractArtifact.abi,
  //   getWallet()
  // );

  // let unclaimsHandlerImpl = await deployContract("UnclaimsHandler");
  // abi = new ethers.Interface(unclaimsHandlerImpl.interface.formatJson());
  // data = abi.encodeFunctionData("initialize", [
  //   await relayerHandler.getAddress(), 
  //   await accountHandler.getAddress(),
  //   await verifierImpl.getAddress(),
  //   450000, // unclaimedFundClaimGas
  //   500000, // unclaimedStateClaimGas
  //   2592000, // unclaimsExpiryDuration = 30 days
  //   2000000000, // maxFeePerGas = 2 gwei
  // ]);
  // proxy = await deployContract("ERC1967Proxy", [unclaimsHandlerImpl.target, data] )
  // contractArtifact = await hre.artifacts.readArtifact("UnclaimsHandler");
  // const unclaimsHandler = new ethers.Contract(
  //   proxy,
  //   contractArtifact.abi,
  //   getWallet()
  // );

  // let uniswapFactory = process.env.UNISWAP_FACTORY as string;
  // let oracle = await deployContract("UniswapTWAPOracle", [uniswapFactory, weth]);

  // let coreImpl = await deployContract("EmailWalletCore");
  // abi = new ethers.Interface(coreImpl.interface.formatJson());
  // data = abi.encodeFunctionData("initialize", [
  //   await relayerHandler.getAddress(),
  //   await accountHandler.getAddress(),
  //   await unclaimsHandler.getAddress(),
  //   await extensionHandler.getAddress(),
  //   await verifierImpl.getAddress(),
  //   await tokenRegistry.getAddress(),
  //   await oracle.getAddress(),
  //   await weth,
  //   2000000000, // maxFeePerGas = 2 gwei
  //   3600, // emailValidityDuration = 1 hour
  //   450000, // unclaimedFundClaimGas
  //   500000, // unclaimedStateClaimGas
  // ]);
  // proxy = await deployContract("ERC1967Proxy", [coreImpl.target, data] )
  // contractArtifact = await hre.artifacts.readArtifact("EmailWalletCore");
  // const core = new ethers.Contract(
  //   proxy,
  //   contractArtifact.abi,
  //   getWallet()
  // );

  // // TODO: The below code is not working in inMemoryNode
  // // See https://github.com/matter-labs/hardhat-zksync/issues/714
  // tx = await relayerHandler.transferOwnership(await core.getAddress());
  // await tx.wait();
  // tx = await accountHandler.transferOwnership(await core.getAddress());
  // await tx.wait();
  // tx = await unclaimsHandler.transferOwnership(await core.getAddress());
  // await tx.wait();
  // tx = await extensionHandler.transferOwnership(await core.getAddress());
  // await tx.wait();

  //   let coreAddress = "0x46d89D4C8ab7D1E6C6553C917E60e363180bAF65";
  //   let contractArtifact = await hre.artifacts.readArtifact("EmailWalletCore");
  //   const core = new ethers.Contract(
  //     coreAddress,
  //     contractArtifact.abi,
  //     getWallet()
  //   );
  //   console.log(`Setting core address = ${coreAddress}`);

  // let relayerHandlerAddress = "0x4aA60b01c1a22787Fdd88aDEF24F356B82c3D3B0";
  // // NFTExtension
  // let nftExtensionImpl = await deployContract("NFTExtension");
  // let abi = new ethers.Interface(nftExtensionImpl.interface.formatJson());
  // let data = abi.encodeFunctionData("initialize", [
  //   relayerHandlerAddress
  // ]);  
  // let proxy = await deployContract("ERC1967Proxy", [nftExtensionImpl.target, data] )
  // contractArtifact = await hre.artifacts.readArtifact("NFTExtension");
  // const nftExtension = new ethers.Contract(
  //   proxy,
  //   contractArtifact.abi,
  //   getWallet() // Interact with the contract on behalf of this wallet
  // );
  // console.log(`Setting NFTExtension address = ${await nftExtension.getAddress()}`, );
  //   contractArtifact = await hre.artifacts.readArtifact("NFTExtension");
  //   const nftExtension = new ethers.Contract(
  //     "0x5Dd1F8C1761190a70385d3b83F308dbF68E5C074",
  //     contractArtifact.abi,
  //     getWallet()
  //   );
  //   console.log(`Setting nftExtension address = ${await nftExtension.getAddress()}`);


  // let tx = await nftExtension.setNFTAddress("ETHDenver", "0xc989c0431feBb3557CCf4e59919D15305D591668");
  // await tx.wait();
  // console.log(`Setting NFT address for ETHDenver`);

  //   let nftExtTemplates: string[][] = [
  //     ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"],
  //     ["NFT", "Approve", "{recipient}", "for", "{uint}", "of", "{string}"]
  //   ];
  //   console.log("Initializing NFTExtension with templates", nftExtTemplates);
  //   // nftExtTemplates[0] = ["NFT", "Send", "{uint}", "of", "{string}", "to", "{recipient}"];
  //   // nftExtTemplates[1] = ["NFT", "Approve", "{recipient}", "for", "{uint}", "of", "{string}"];
  //   let defaultExtensions: any[] = new Array(1);
  //   let abiCoder = ethers.AbiCoder.defaultAbiCoder();
  //   defaultExtensions[0] = abiCoder.encode(
  //     ["string", "address", "string[][]", "uint256"], 
  //     ["NFTExtension", (await nftExtension.getAddress()), nftExtTemplates, ethers.parseEther("0.001")] ); // TODO: Check max exec gas
  //   console.log("Initializing core with default extensions", defaultExtensions);
  //   await core.initializeExtension(defaultExtensions);

}