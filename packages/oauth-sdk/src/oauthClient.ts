import { Address, GetContractReturnType, PrivateKeyAccount, PublicClient, WalletClient, Chain, getContract, encodePacked, Transport } from 'viem'
import { emailWalletCoreAbi, walletAbi, iOauthAbi } from './generated'
import { privateKeyToAccount, generatePrivateKey, sign } from 'viem/accounts'
import RelayerApis from "./relayerApis";
import { getAddress } from 'viem'

export default class OauthClient<chain extends Chain> {
    publicClient: PublicClient<Transport, chain>;
    core: any;// GetContractReturnType<typeof emailWalletCoreAbi, PublicClient<Transport, chain>>;
    oauth: any; // GetContractReturnType<typeof iOauthAbi, PublicClient<Transport, chain>>;
    relayerApis: RelayerApis;
    userEmailAddr: string | null = null;
    // accountCode: string | null = null;
    userWallet: any = null; //GetContractReturnType<typeof walletAbi, PublicClient<Transport, chain>> | null = null;
    ephePrivateKey: `0x${string}`;
    epheClient: PrivateKeyAccount;
    epheAddrNonce: string | null = null;


    constructor(
        client: PublicClient<Transport, chain>,
        coreAddress: Address,
        oauthAddress: Address,
        relayerHost: string,
        userEmailAddr?: string,
        userWalletAddr?: Address,
        ephePrivateKey?: `0x${string}`,
        epheAddrNonce?: string
    ) {
        this.publicClient = client;
        this.core = getContract({
            address: coreAddress,
            abi: emailWalletCoreAbi,
            client: {
                "public": client
            },
        });
        this.oauth = getContract({
            address: oauthAddress,
            abi: iOauthAbi,
            client,
        })
        this.relayerApis = new RelayerApis(relayerHost);
        if (ephePrivateKey === undefined) {
            ephePrivateKey = generatePrivateKey();
        }
        this.ephePrivateKey = ephePrivateKey;
        this.epheClient = privateKeyToAccount(ephePrivateKey);
        if (userEmailAddr !== undefined) {
            this.userEmailAddr = userEmailAddr;
        }
        if (userWalletAddr !== undefined) {
            this.userWallet = getContract({
                address: userWalletAddr,
                abi: walletAbi,
                client: this.publicClient
            });
        }
        if (epheAddrNonce !== undefined) {
            this.epheAddrNonce = epheAddrNonce;
        }
    }

    public getPublicClient(): PublicClient<Transport, chain> {
        return this.publicClient;
    }

    public getWalletAddress(): Address | null {
        if (this.userWallet === null) {
            return null;
        }
        return this.userWallet.address;
    }

    public getEphePrivateKey(): `0x${string}` {
        return this.ephePrivateKey;
    }

    public getEpheAddrNonce(): string | null {
        return this.epheAddrNonce;
    }


    public async setup(
        userEmailAddr: string,
        username: string | null,
        expiryTime: number | null,
        tokenAllowances: [number, string][] | null
    ): Promise<number> {
        if (this.userEmailAddr !== null) {
            return 0;
        }
        this.userEmailAddr = userEmailAddr;
        return await this.relayerApis.signupOrIn(userEmailAddr, this.epheClient.address, username, expiryTime, tokenAllowances);
    }

    public async waitEpheAddrActivated(
        requestId: number
    ): Promise<boolean> {
        if (this.userWallet !== null || this.epheAddrNonce !== null) {
            return true;
        }
        if (this.userEmailAddr === null) {
            throw new Error("Not setup yet")
        }
        const relayerEmailAddr = await this.relayerApis.getRelayerEmailAddr();
        const signedMsg = `${relayerEmailAddr}:/api/epheAddrStatus/${requestId}`;
        console.log(`signedMsg: ${signedMsg}`);
        const signature = await this.epheClient.signMessage({
            message: signedMsg,
        });

        let res = await this.relayerApis.epheAddrStatus(requestId, signature);
        while (!res.is_activated) {
            await new Promise(resolve => setTimeout(resolve, 1000));
            res = await this.relayerApis.epheAddrStatus(requestId, signature);
            console.log(res);
        }
        this.userWallet = getContract({
            address: res.wallet_addr as `0x${string} `,
            abi: walletAbi,
            client: this.publicClient
        });
        this.epheAddrNonce = res.nonce as string;
        return true;
    }

    public async oauthExecuteTx(
        target: Address,
        data: `0x${string}`,
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
            epheAddrNonce: BigInt(this.epheAddrNonce),
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
            tx.txNonce.toString(),
            tx.epheAddr,
            this.epheAddrNonce,
            tx.target,
            tx.ethValue.toString(),
            tx.data,
            tx.tokenAmount.toString(),
            signature
        );
        return txHash;
    }

    public async getTokenAllowance(
        tokenAddr: Address
    ): Promise<bigint> {
        if (this.userWallet === null || this.epheAddrNonce === null) {
            throw new Error("Not signed in yet")
        }
        const res = await this.oauth.read.getTokenAllowancesOfWalletAndNonce([this.userWallet.address, this.epheAddrNonce, tokenAddr]);
        return BigInt(res);
    }
}