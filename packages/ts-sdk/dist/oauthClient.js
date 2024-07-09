"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const viem_1 = require("viem");
const generated_1 = require("./generated");
const accounts_1 = require("viem/accounts");
const relayerApis_1 = __importDefault(require("./relayerApis"));
class OauthClient {
    constructor(client, coreAddress, oauthAddress, relayerHost, userEmailAddr, userWallet, epheAddrNonce) {
        this.userEmailAddr = null;
        // accountCode: string | null = null;
        this.userWallet = null; //GetContractReturnType<typeof walletAbi, PublicClient<Transport, chain>> | null = null;
        this.epheAddrNonce = null;
        this.publicClient = client;
        this.core = (0, viem_1.getContract)({
            address: coreAddress,
            abi: generated_1.emailWalletCoreAbi,
            client: {
                "public": client
            },
        });
        this.oauth = (0, viem_1.getContract)({
            address: oauthAddress,
            abi: generated_1.iOauthAbi,
            client,
        });
        this.relayerApis = new relayerApis_1.default(relayerHost);
        this.epheClient = (0, accounts_1.privateKeyToAccount)((0, accounts_1.generatePrivateKey)());
        if (userEmailAddr !== undefined) {
            this.userEmailAddr = userEmailAddr;
        }
        if (userWallet !== undefined) {
            this.userWallet = userWallet;
        }
        if (epheAddrNonce !== undefined) {
            this.epheAddrNonce = epheAddrNonce;
        }
    }
    async isAccountCreated(userEmailAddr) {
        return await this.relayerApis.isAccountCreated(userEmailAddr);
    }
    async setup(userEmailAddr, username, expiryTime, tokenAllowances) {
        if (this.userEmailAddr !== null) {
            return 0;
        }
        this.userEmailAddr = userEmailAddr;
        return await this.relayerApis.signupOrIn(userEmailAddr, this.epheClient.address, username, expiryTime, tokenAllowances);
    }
    async waitEpheAddrActivated(requestId) {
        if (this.userWallet !== null || this.epheAddrNonce !== null) {
            return true;
        }
        if (this.userEmailAddr === null) {
            throw new Error("Not setup yet");
        }
        const signedMsg = `${this.relayerApis.relayerHost.replace(/^https?:\/\//, '')}/api/epheAddrStatus/${requestId}`;
        const signature = await this.epheClient.signMessage({
            message: signedMsg,
        });
        let res = await this.relayerApis.epheAddrStatus(requestId, signature);
        while (!res.is_activated) {
            await new Promise(resolve => setTimeout(resolve, 1000));
            res = await this.relayerApis.epheAddrStatus(requestId, signature);
            console.log(res);
        }
        this.userWallet = (0, viem_1.getContract)({
            address: res.wallet_addr,
            abi: generated_1.walletAbi,
            client: this.publicClient
        });
        this.epheAddrNonce = res.nonce;
        return true;
    }
    async oauthExecuteTx(target, data, ethValue, token_amount) {
        if (this.userEmailAddr === null || this.userWallet === null) {
            throw new Error("Not authenticated yet");
        }
        if (this.epheAddrNonce === null) {
            throw new Error("Not signed in yet");
        }
        const txNonce = await this.userWallet.read.epheTxNonce();
        const tx = {
            walletAddr: this.userWallet.address,
            txNonce,
            epheAddr: this.epheClient.address,
            epheAddrNonce: BigInt(this.epheAddrNonce),
            target,
            ethValue: ethValue === null ? 0n : ethValue,
            data,
            tokenAmount: token_amount === null ? 0n : token_amount,
            signature: "0x",
        };
        const signedTxHash = await this.userWallet.read.hashEphemeralTx([tx]);
        const signature = await this.epheClient.signMessage({
            message: { raw: signedTxHash },
        });
        const txHash = await this.relayerApis.executeEphemeralTx(tx.walletAddr, tx.txNonce.toString(), tx.epheAddr, this.epheAddrNonce, tx.target, tx.ethValue.toString(), tx.data, tx.tokenAmount.toString(), signature);
        return txHash;
    }
}
exports.default = OauthClient;
//# sourceMappingURL=oauthClient.js.map