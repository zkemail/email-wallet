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
      url: "https://zksync-era-sepolia.blockpi.network/v1/rpc/public",
      // If you want to deploy missing libraries, use the below url
      //   url: "https://sepolia.era.zksync.dev", 
      ethNetwork: "sepolia",
      zksync: true,
      //   verifyURL: "https://explorer.sepolia.era.zksync.dev/contract_verification",
      forceDeploy: true
    },
    zkSyncMainnet: {
      url: "https://mainnet.era.zksync.io",
      ethNetwork: "mainnet",
      zksync: true,
      // verifyURL: "https://zksync2-mainnet-explorer.zksync.io/contract_verification",
      forceDeploy: true
    },
    inMemoryNode: {
      url: "http://127.0.0.1:8011",
      ethNetwork: "", // in-memory node doesn't support eth node; removing this line will cause an error
      zksync: true,
      forceDeploy: true
    },
    hardhat: {
      zksync: true,
    },
  },
  zksolc: {
    version: "latest",
    settings: {
      isSystem: true,
      libraries: {
        "src/libraries/DecimalUtils.sol": {
          "DecimalUtils": "0xc0e82deCFAB7B7414fCD3417E6D07Ce98c0A5230"
        },
        "src/libraries/SubjectUtils.sol": {
          "SubjectUtils": "0x63d8162B05206e942820c1A6c14c9003F81faf89"
        }
      },
      optimizer: {
        enabled: true,
        runs: 1
      }
    }
  },
  solidity: {
    version: "0.8.23",
    settings: {
      optimizer: {
        enabled: true,
        runs: 1
      }
    }
  },
};

export default config;