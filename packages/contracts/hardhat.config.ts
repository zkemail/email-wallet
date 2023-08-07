import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "hardhat-tracer";

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.19",
    settings: {
      viaIR: true,
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
  paths: {
    sources: "./",
    tests: "./test",
    cache: "./cache", 
    artifacts: "./artifacts"
  },
};

export default config;
