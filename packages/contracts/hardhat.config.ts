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
        libraries: {
              "src/libraries/DecimalUtils.sol": {
                "DecimalUtils": "0x51eF809FFd89cf8056D4C17F0aFF1b6F8257EB60"
              },
              "src/libraries/SubjectUtils.sol": {
                "SubjectUtils": "0x775f1fbfc46720025ACC2FFE652e578de642e6a2"
              }
            }
    },
  },
  solidity: {
    version: "0.8.23",
  },
};

export default config;