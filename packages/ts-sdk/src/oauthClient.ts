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
    epheAddrNonce: string | null = null;


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
    ): Promise<string> {
        this.userEmailAddr = userEmailAddr;
        return await this.relayerApis.signupOrIn(userEmailAddr, this.epheClient.address, username, expiryTime, tokenAllowances);
    }

    public async waitEpheAddrActivated(
        requestId: string
    ): Promise<boolean> {
        if (this.userEmailAddr === null) {
            throw new Error("Not setup yet")
        }
        const signedMsg = `${this.relayerApis.relayerHost}/api/epheAddrStatus/${requestId}`;
        const signature = await this.epheClient.signMessage({
            message: signedMsg,
        });

        let res = await this.relayerApis.epheAddrStatus(requestId, signature);
        while (!res.is_activated) {
            await new Promise(resolve => setTimeout(resolve, 1000));
            res = await this.relayerApis.epheAddrStatus(requestId, signature);
        }
        this.userWallet = getContract({
            address: res.wallet_addr as `0x${string}`,
            abi: walletAbi,
            client: this.publicClient
        });
        this.epheAddrNonce = res.nonce as string;
        return true;
    }

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


}