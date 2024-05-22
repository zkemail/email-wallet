import { ethers } from "ethers";
import express from "express";
// Imports the Alchemy SDK
const { Alchemy, Network } = require("alchemy-sdk");
import { config as dotenvConfig } from "dotenv";
import fetch from "node-fetch";
dotenvConfig();

// Configures the Alchemy SDK
const config = {
  apiKey: process.env.ALCHEMY_API_KEY, // API key is read from environment variables
  network: Network.BASE_SEPOLIA, // Replace with your network
};

// Creates an Alchemy object instance with the config to use for making requests
const alchemy = new Alchemy(config);

// Define the structure for the SafeRequest
interface SafeRequest {
  wallet_addr: string;
  safe_addr: string;
}

enum Operation {
  Add = "add",
  Remove = "remove",
}

// Define the structure for the LogType
interface LogType {
  address: string;
  data: string;
  topics: string[];
  blockNumber: number;
  transactionHash: string;
  transactionIndex: number;
  blockHash: string;
  logIndex: number;
  removed: boolean;
}

// Function to send a POST request to the bore.pub API
const sendSafeRequest = async (walletAddress: string, safeAddress: string, operation: Operation) => {
  const safeRequest: SafeRequest = {
    wallet_addr: walletAddress,
    safe_addr: safeAddress,
  };

  try {
    const response = await fetch(`${process.env.RELAYER_URL}/api/${operation}SafeOwner`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(safeRequest),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.text();
    console.log("Response from API:", data);
  } catch (error) {
    console.error("Error sending request to API:", error);
  }
};

// Function to parse log data and extract necessary information
const parseLogData = async (log: LogType) => {
  console.log("Data:", log.data);
  let affectedAddress;
  try {
    if (log.data == "0x") {
      throw new Error("No data on log found");
    }
    const abiCoder = new ethers.AbiCoder();
    const decodedData = abiCoder.decode(["address"], log.data);
    affectedAddress = decodedData[0];
    const codeAtAddress = await alchemy.core.getCode(affectedAddress);
    if (codeAtAddress === "0x") {
      throw new Error(`No contract deployed at address: ${affectedAddress}, skipping...`);
    }
  } catch (error) {
    console.log("Failed to decode log data:", error);
    return { affectedAddress: "", eventSenderAddress: "" };
  }
  console.log(`Affected Address: ${affectedAddress}`);
  const eventSenderAddress = log.address;
  console.log(`Event Sender Address: ${eventSenderAddress}`);
  return { affectedAddress, eventSenderAddress };
};

const main = async () => {
  // Event selectors for AddedOwner and RemovedOwner
  const addedOwnerEvent = ethers.id("AddedOwner(address)");
  const removedOwnerEvent = ethers.id("RemovedOwner(address)");
  const safeSetupEvent = ethers.id("SafeSetup(address,address[],uint256,address,address)");

  alchemy.ws.on([safeSetupEvent], async (log: LogType, event: { event: string }) => {
    console.log("Safe setup detected!");
    const abiCoder = new ethers.AbiCoder();
    const decodedData = abiCoder.decode(["address[]", "uint256", "address", "address"], log.data);
    const owners = decodedData[0];
    console.log(`Owners: ${owners}`);
    const eventSenderAddress = log.address;
    console.log(`Event Sender Address: ${eventSenderAddress}`);
    for (const owner of owners) {
      const codeAtAddress = await alchemy.core.getCode(owner);
      if (codeAtAddress === "0x") {
        console.log(`No contract deployed at address: ${owner}, skipping...`);
        continue;
      }
      console.log(`Adding owner: ${owner}`);
      await sendSafeRequest(owner, eventSenderAddress, "add" as Operation);
    }
  });
  // await sendSafeRequest("0x0000000000000000000000000000000000000000", "0x0000000000000000000000000000000000000000", "add" as Operation);
  // Subscribe to logs using Alchemy
  // Note that you can only subscribe to one topic in a single element array
  alchemy.ws.on([addedOwnerEvent], async (log: LogType, event: { event: string }) => {
    // Parse the logs for the specific transaction
    console.log("Owner added!");
    const { affectedAddress, eventSenderAddress } = await parseLogData(log);
    if (affectedAddress && eventSenderAddress) {
      await sendSafeRequest(affectedAddress, eventSenderAddress, "add" as Operation);
    }
  });

  // Subscribe to logs using Alchemy
  alchemy.ws.on([removedOwnerEvent], async (log: LogType, event: { event: string }) => {
    // Parse the logs for the specific transaction
    console.log("Owner removed!");
    const { affectedAddress, eventSenderAddress } = await parseLogData(log);
    if (affectedAddress && eventSenderAddress) {
      await sendSafeRequest(affectedAddress, eventSenderAddress, "remove" as Operation);
    }
  });

  console.log("Subscribed to Safe owner logs...");

  // Set up Express server
  const app = express();
  const port = 3000; // Define the port to use

  app.get("/", (_: any, res: any) => {
    res.send("Safe Tracker is running!");
  });

  app.listen(port, () => {
    console.log(`Server listening at http://0.0.0.0:${port}`);
  });
};

main();
