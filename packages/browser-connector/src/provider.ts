import {
  AbstractProvider,
  JsonRpcPayload,
  JsonRpcResult,
  JsonRpcError,
  JsonRpcSigner,
  getDefaultProvider,
  JsonRpcProvider,
  AbiCoder
} from "ethers";
import { EmailWalletSigner } from "./signer";

interface RequestArguments {
  method: string;
  params?: unknown[] | object;
}

export class EmailWalletProvider {
  _isConnected: boolean;
  relayerEmail: string;

  jsonProvider: JsonRpcProvider;

  constructor() {
    this._isConnected = true;
    this.relayerEmail = "";

    this.jsonProvider = new JsonRpcProvider("https://ethereum-sepolia.blockpi.network/v1/rpc/public", { name: "sepolia", chainId: 11155111 });
  }

  connect(relayerEmail: string) {
    this.relayerEmail = relayerEmail;
    this._isConnected = true;
  }

  isConnected() {
    return this._isConnected;
  }

  async request(args: RequestArguments): Promise<unknown> {
    // console.log(args.method, args.params);

    if (args.method === "eth_accounts") {
      return ["0x70997970c51812dc3a010c7d01b50e0d17dc7123"];
    }

    if (args.method === "eth_requestAccounts") {
      return ["0x70997970c51812dc3a010c7d01b50e0d17dc7123"];
    }

    if (args.method === "eth_signTransaction") {
      return "0xa3f20717a250c2b0b729b7e5becbff67fdaef7e0699da4de7ca5895b02a170a12d887fd3b17bfdce3481f10bea41f45ba9f709d39ce8325427b57afcfc994cee1b";
    }

    if (args.method === "eth_sendTransaction") {
      const [tx] = args.params as any[];

      const { to, value, data } = tx;

      const encoded = AbiCoder.defaultAbiCoder().encode(["address", "uint256", "bytes"], [to, value, data]);

      const subject = `Execute ${encoded}`;

      const mailToLink = `mailto:relayer@sendeth.org?subject=${subject}`

      console.log({ to, data, subject, mailToLink });

      window.open(mailToLink, "_blank");

      return {};
    }

    const res = await this.jsonProvider.send(args.method, args.params as any[]);

    // console.log(res);

    return res;
    // return {};
  }

  on(eventName: string, listener: (...args: any[]) => void): void {
    console.log("listener", eventName);
  }

  off(eventName: string, listener: (...args: any[]) => void): void {
    console.log("listener", eventName);
  }

  getBlockNumber(): Promise<number> {
    return this.jsonProvider.getBlockNumber();
  }

  lookupAddress(address: string): Promise<string | null> {
    return this.jsonProvider.lookupAddress(address);
  }

 
}
