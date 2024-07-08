import { Address, GetContractReturnType, PrivateKeyAccount, PublicClient, WalletClient, getContract, encodePacked } from 'viem'
import { emailWalletCoreAbi, walletAbi, iOauthAbi } from './generated'
import { privateKeyToAccount, generatePrivateKey, sign } from 'viem/accounts'
import RelayerApis from "./relayerApis";

export default class OauthCore {
    publicClient: PublicClient;
    core: GetContractReturnType<typeof emailWalletCoreAbi, PublicClient>;
    oauth: GetContractReturnType<typeof iOauthAbi, PublicClient>;
    relayerApis: RelayerApis;
    userEmailAddr: string | null = null;
    // accountCode: string | null = null;
    userWallet: GetContractReturnType<typeof walletAbi, PublicClient> | null = null;
    epheClient: PrivateKeyAccount;
    // epheAddrNonce: bigint | null = null;


    constructor(
        client: PublicClient,
        coreAddress: Address,
        ioauthAddress: Address,
        relayerHost: string,
    ) {
        this.publicClient = client;
        this.core = getContract({
            address: coreAddress,
            abi: emailWalletCoreAbi,
            client,
        });
        this.oauth = getContract({
            address: ioauthAddress,
            abi: iOauthAbi,
            client,
        })
        this.relayerApis = new RelayerApis(relayerHost);
        this.epheClient = privateKeyToAccount(generatePrivateKey());
    }

    public async isAccountCreated(
        userEmailAddr: string,
    ): Promise<boolean> {
        return await this.relayerApis.isAccountCreated(userEmailAddr);
    }


    public async setup(
        userEmailAddr: string,
        username: string | null,
        expiryTime: number | null,
        tokenAllowances: [number, string][] | null
    ) {
        this.userEmailAddr = userEmailAddr;
        await this.relayerApis.signupOrIn(userEmailAddr, this.epheClient.address, username, expiryTime, tokenAllowances);
    }

    // public async loadAccountCode(
    //     accountCode: string
    // ) {
    //     if (this.userEmailAddr === null) {
    //         throw new Error("Not setup yet")
    //     }
    //     const walletAddr = await this.relayerApis.getWalletAddress(this.userEmailAddr, accountCode);
    //     this.userWallet = getContract({
    //         address: walletAddr,
    //         abi: walletAbi,
    //         client: this.publicClient
    //     });
    // }


    // public async requestEmailAuthentication(
    //     userEmailAddr: string,
    // ) {
    //     if (this.userEmailAddr !== null) {
    //         throw new Error("Already requested")
    //     }
    //     this.userEmailAddr = userEmailAddr;
    //     await this.relayerApis.recoverAccountCode(userEmailAddr);
    // }

    // public async completeEmailAuthentication(
    //     accountCode: string,
    // ) {
    //     if (this.userEmailAddr === null) {
    //         throw new Error("Not requested yet")
    //     }
    //     if (this.accountCode !== null) {
    //         throw new Error("Already completed")
    //     }
    //     this.accountCode = accountCode;
    //     const walletAddr = await this.relayerApis.getWalletAddress(this.userEmailAddr, accountCode);
    //     this.userWallet = getContract({
    //         address: walletAddr,
    //         abi: walletAbi,
    //         client: this.publicClient
    //     });
    // }

    public async getOauthUsername(): Promise<string> {
        if (this.userWallet === null) {
            throw new Error("An account code is not loaded")
        }
        const username = await this.oauth.read.getUsernameOfWallet([this.userWallet.address]);
        return username;
    }

    // public async oauthSignup(
    //     username: string,
    //     nonce: string | null,
    //     expiry_time: number | null,
    //     token_allowances: [number, string][] | null
    // ) {
    //     if (this.userEmailAddr === null || this.userWallet === null) {
    //         throw new Error("Not authenticated yet")
    //     }
    //     const requestId = await this.relayerApis.signup(this.userEmailAddr, username, nonce, expiry_time, token_allowances);
    //     console.log(`Request ID: ${requestId}`);
    // }

    // public async oauthSignin(
    //     expiry_time: number | null,
    //     token_allowances: [number, string][] | null
    // ) {
    //     if (this.userEmailAddr === null || this.userWallet === null) {
    //         throw new Error("Not authenticated yet")
    //     }
    //     const username = await this.getOauthUsername();
    //     if (username === "") {
    //         throw new Error("Not signed up yet")
    //     }
    //     const epheAddr = this.epheClient.address;
    //     const chainId = await this.publicClient.getChainId();
    //     const signedMessageHash = encodePacked(["address", "uint256", "address", "string"], [this.oauth.address, BigInt(chainId), epheAddr, username]);
    //     const signature = await this.epheClient.signMessage({
    //         message: { raw: signedMessageHash },
    //     });
    //     const epheAddrNonce = await this.relayerApis.registerEpheAddr(this.userWallet.address, epheAddr, signature);
    //     this.epheAddrNonce = BigInt(epheAddrNonce);
    //     const requestId = await this.relayerApis.signin(this.userEmailAddr, username, epheAddrNonce, expiry_time, token_allowances);
    //     console.log(`Request ID: ${requestId}`);
    // }

    public async oauthExecuteTx(
        target: Address,
        data: `0x{string}`,
        ethValue: bigint | null,
        token_amount: bigint | null
    ): Promise<string> {
        if (this.userEmailAddr === null || this.userWallet === null) {
            throw new Error("Not authenticated yet")
        }
        if (this.epheAddrNonce === null) {
            throw new Error("Not signed in yet")
        }
        const txNonce = await this.userWallet.read.epheTxNonce();
        const tx = {
            walletAddr: this.userWallet.address,
            txNonce,
            epheAddr: this.epheClient.address,
            epheAddrNonce: this.epheAddrNonce,
            target,
            ethValue: ethValue === null ? 0n : ethValue,
            data,
            tokenAmount: token_amount === null ? 0n : token_amount,
            signature: "0x" as Address,
        };
        const signedTxHash = await this.userWallet.read.hashEphemeralTx([tx]);
        const signature = await this.epheClient.signMessage({
            message: { raw: signedTxHash },
        });
        const txHash = await this.relayerApis.executeEphemeralTx(
            tx.walletAddr,
            tx.txNonce,
            tx.epheAddr,
            tx.epheAddrNonce,
            tx.target,
            tx.ethValue,
            tx.data,
            tx.tokenAmount,
            signature
        );
        return txHash;
    }


}