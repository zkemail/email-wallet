import { EmailWalletProvider } from "./provider";
import { InfuraProvider } from "ethers";

const infura = new InfuraProvider("sepolia", "d87bf713dd3a44e09e773c13a2e84cc6");

// @ts-ignore
window.ethereum = new EmailWalletProvider(infura);

console.log("EmailWalletProvider loaded");

export { EmailWalletProvider };
