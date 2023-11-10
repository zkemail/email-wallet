import { SwapWidget } from "@uniswap/widgets";
import "@uniswap/widgets/fonts.css";
import { useAccount, useConnect, useDisconnect } from "wagmi";

// 1. Import modules.
import { createWalletClient, custom, parseEther, createPublicClient } from "viem";
import { mainnet, sepolia } from "viem/chains";

import { InjectedConnector } from "wagmi/connectors/injected";

import { WagmiConfig, createConfig } from "wagmi";

const config = createConfig({
  autoConnect: true,
  publicClient: createPublicClient({
    chain: sepolia,
    transport: custom(window.ethereum),
  }),
});

function Profile() {
  const { address, isConnected } = useAccount();
  const { connect } = useConnect({
    connector: new InjectedConnector(),
  });
  const { disconnect } = useDisconnect();

  if (isConnected)
    return (
      <div>
        Connected to {address}
        <button onClick={() => disconnect()}>Disconnect</button>
      </div>
    );
  return <button onClick={() => connect()}>Connect Wallet</button>;
}

export default function App() {
  function updateStatus() {
    // const connected = window.ethereum.isConnected();
    // const status = connected ? "Connected" : "Not connected";
    // document.getElementById("status").innerText = status;
    // document.getElementById("btn-connect").disabled = connected;
  }

  async function onConnectClick() {
    // window.ethereum.connect();
    // updateStatus();

    const client = createWalletClient({
      chain: mainnet,
      transport: custom(window.ethereum),
    });

    console.log(await client.getAddresses());

    const request = await client.prepareTransactionRequest({
      account: "0x7Bcd6F009471e9974a77086a69289D16EaDbA286",
      to: "0x7b79995e5f793a07bc00c21412e50ecae098e7f9",
      value: parseEther(" 0.001"),
      abi: [
        {
          anonymous: false,
          inputs: [
            { indexed: true, internalType: "address", name: "src", type: "address" },
            { indexed: true, internalType: "address", name: "guy", type: "address" },
            { indexed: false, internalType: "uint256", name: "wad", type: "uint256" },
          ],
          name: "Approval",
          type: "event",
        },
        {
          anonymous: false,
          inputs: [
            { indexed: true, internalType: "address", name: "dst", type: "address" },
            { indexed: false, internalType: "uint256", name: "wad", type: "uint256" },
          ],
          name: "Deposit",
          type: "event",
        },
        {
          anonymous: false,
          inputs: [
            { indexed: true, internalType: "address", name: "src", type: "address" },
            { indexed: true, internalType: "address", name: "dst", type: "address" },
            { indexed: false, internalType: "uint256", name: "wad", type: "uint256" },
          ],
          name: "Transfer",
          type: "event",
        },
        {
          anonymous: false,
          inputs: [
            { indexed: true, internalType: "address", name: "src", type: "address" },
            { indexed: false, internalType: "uint256", name: "wad", type: "uint256" },
          ],
          name: "Withdrawal",
          type: "event",
        },
        {
          inputs: [
            { internalType: "address", name: "", type: "address" },
            { internalType: "address", name: "", type: "address" },
          ],
          name: "allowance",
          outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [
            { internalType: "address", name: "guy", type: "address" },
            { internalType: "uint256", name: "wad", type: "uint256" },
          ],
          name: "approve",
          outputs: [{ internalType: "bool", name: "", type: "bool" }],
          stateMutability: "nonpayable",
          type: "function",
        },
        {
          inputs: [{ internalType: "address", name: "", type: "address" }],
          name: "balanceOf",
          outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [],
          name: "decimals",
          outputs: [{ internalType: "uint8", name: "", type: "uint8" }],
          stateMutability: "view",
          type: "function",
        },
        { inputs: [], name: "deposit", outputs: [], stateMutability: "payable", type: "function" },
        {
          inputs: [],
          name: "name",
          outputs: [{ internalType: "string", name: "", type: "string" }],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [],
          name: "symbol",
          outputs: [{ internalType: "string", name: "", type: "string" }],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [],
          name: "totalSupply",
          outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [
            { internalType: "address", name: "dst", type: "address" },
            { internalType: "uint256", name: "wad", type: "uint256" },
          ],
          name: "transfer",
          outputs: [{ internalType: "bool", name: "", type: "bool" }],
          stateMutability: "nonpayable",
          type: "function",
        },
        {
          inputs: [
            { internalType: "address", name: "src", type: "address" },
            { internalType: "address", name: "dst", type: "address" },
            { internalType: "uint256", name: "wad", type: "uint256" },
          ],
          name: "transferFrom",
          outputs: [{ internalType: "bool", name: "", type: "bool" }],
          stateMutability: "nonpayable",
          type: "function",
        },
        {
          inputs: [{ internalType: "uint256", name: "wad", type: "uint256" }],
          name: "withdraw",
          outputs: [],
          stateMutability: "nonpayable",
          type: "function",
        },
        { stateMutability: "payable", type: "receive" },
      ],
      data: "0xd0e30db0",
      chain: sepolia,
    });

    console.log("request", request);

    // const signature = await client.signTransaction(request);

    // console.log("signature", signature);

    // const hash = await client.sendRawTransaction(signature)

    // console.log("hash", hash);

    const tx = await client.sendTransaction(request);

    console.log("tx", tx);
  }

  return (
    <>
      <section className="hero">
        <div className="hero-body">
          <div className="title">Connect Email Wallet</div>
          <hr />
          <div className="subtitle">
            <div>
              Status: <span id="status">Not connected</span>
            </div>
            <button id="btn-connect" className="button is-secondary mt-5" onClick={onConnectClick}>
              Connect
            </button>
          </div>
        </div>
      </section>

      <section>
        <div className="Uniswap">
          <SwapWidget 
          hideConnectionUI={"true"}
          onConnectWalletClick={() => false}
          provider={window.ethereum}
            />
        </div>
      </section>

      <WagmiConfig config={config}>
        <Profile />
      </WagmiConfig>
    </>
  );
}
