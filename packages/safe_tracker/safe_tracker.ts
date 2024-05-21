import { ethers } from "ethers";
// Imports the Alchemy SDK
const { Alchemy, Network } = require("alchemy-sdk");
const { keccak256 } = require("js-sha3");
import { config as dotenvConfig } from 'dotenv';
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
  Remove = "remove"
}

// Function to send a POST request to the bore.pub API
const sendSafeRequest = async (walletAddress: string, safeAddress: string, operation: Operation) => {
  const safeRequest: SafeRequest = {
    wallet_addr: walletAddress,
    safe_addr: safeAddress,
  };

  try {
    const response = await fetch(`http://bore.pub:40078/api/${operation}SafeOwner`, {
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
const parseLogData = (log) => {
  console.log("Data:", log.data);
  let affectedAddress;
  try {
    const abiCoder = new ethers.AbiCoder();
    const decodedData = abiCoder.decode(["address"], log.data);
    affectedAddress = decodedData[0];
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

  // await sendSafeRequest("0x0000000000000000000000000000000000000000", "0x0000000000000000000000000000000000000000", "add" as Operation);
  // Subscribe to logs using Alchemy
  // Note that you can only subscribe to one topic in a single element array
  const subscriptionAdd = alchemy.ws.on([addedOwnerEvent], async (log, event) => {
    // Parse the logs for the specific transaction
    console.log("Owner added!");
    const { affectedAddress, eventSenderAddress } = parseLogData(log);
    if (affectedAddress && eventSenderAddress) {
      await sendSafeRequest(affectedAddress, eventSenderAddress, "add" as Operation);
    }
  });

  // Subscribe to logs using Alchemy
  const subscriptionRemove = alchemy.ws.on([removedOwnerEvent], async (log, event) => {
    // Parse the logs for the specific transaction
    console.log("Owner removed!");
    const { affectedAddress, eventSenderAddress } = parseLogData(log);
    if (affectedAddress && eventSenderAddress) {
      await sendSafeRequest(affectedAddress, eventSenderAddress, "remove" as Operation);
    }
  });

  console.log("Subscribed to Safe owner logs...");
};

main();
