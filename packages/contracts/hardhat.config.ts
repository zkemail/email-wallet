import { HardhatUserConfig } from "hardhat/config";

import "@matterlabs/hardhat-zksync-deploy";
import "@matterlabs/hardhat-zksync-solc";
import "@matterlabs/hardhat-zksync-verify";
import "@matterlabs/hardhat-zksync-node";

const config: HardhatUserConfig = {
  defaultNetwork: "zkSyncSepoliaTestnet",
  paths: {
    // sources: "./contracts",
    sources: "./src",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  },
networks: {
    zkSyncSepoliaTestnet: {
      url: "https://sepolia.era.zksync.dev",
      ethNetwork: "sepolia",
      zksync: true,
      verifyURL: "https://explorer.sepolia.era.zksync.dev/contract_verification",
    },
    zkSyncMainnet: {
      url: "https://mainnet.era.zksync.io",
      ethNetwork: "mainnet",
      zksync: true,
      verifyURL: "https://zksync2-mainnet-explorer.zksync.io/contract_verification",
    },
    inMemoryNode: {
        url: "http://127.0.0.1:8011",
        ethNetwork: "", // in-memory node doesn't support eth node; removing this line will cause an error
        zksync: true,
    },    
    hardhat: {
      zksync: true,
    },
  },
  zksolc: {
    version: "latest",
    // compilerSource: "binary",
    settings: {
      // find all available options in the official documentation
      // https://era.zksync.io/docs/tools/hardhat/hardhat-zksync-solc.html#configuration
    //   libraries: {
    //     "src/libraries/DecimalUtils.sol": {
    //       "DecimalUtils": "0x23b13d016E973C9915c6252271fF06cCA2098885"
    //     },
    //   }
    },
  },
  solidity: {
    version: "0.8.23",
  },
};

export default config;