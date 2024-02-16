import { deployContract } from "./utils";

import ethers from "ethers";
// An example of a basic deploy script
// It will deploy a Greeter contract to selected network
// as well as verify it on Block Explorer if possible for the network
export default async function () {

  // TokenRegistry
  const contractArtifactName = "TokenRegistry";
  let tokenRegistry = await deployContract(contractArtifactName);
  // let selector = ethers.id("balanceOf(address)").substring(0, 10);
  // let tokenRegistryProxy = await deployContract("ERC1967Proxy", [tokenRegistry.getAddress(), selector] )

}
