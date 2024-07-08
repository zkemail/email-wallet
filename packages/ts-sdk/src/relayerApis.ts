import axios from 'axios';
import { Address } from 'viem';

export default class RelayerApis {
    relayerHost: string;

    constructor(relayerHost: string) {
        this.relayerHost = relayerHost;
    }


    public async getWalletAddress(
        emailAddr: string,
        accountCode: string
    ): Promise<Address> {
        const url = `${this.relayerHost}/api/getWalletAddress`;
        const res = await axios.post(url, { email_addr: emailAddr, account_code: accountCode });
        return res.data;
    }

    public async isAccountCreated(
        emailAddr: string
    ): Promise<boolean> {
        const url = `${this.relayerHost}/api/isAccountCreated`;
        const res = await axios.post(url, { email_addr: emailAddr });
        return res.data === "true";
    }

    // public async recoverAccountCode(
    //     emailAddr: string,
    // ): Promise<void> {
    //     const url = `${this.relayerHost}/api/recoverAccountCode`;
    //     await axios.post(url, { email_addr: emailAddr });
    // }

    public async signupOrIn(
        emailAddr: string,
        epheAddr: string | null,
        username: string | null,
        expiryTime: number | null,
        tokenAllowances: [number, string][] | null
    ): Promise<string> {
        const url = `${this.relayerHost}/api/signupOrIn`;
        const requestData: {
            email_addr: string;
            ephe_addr?: string;
            username?: string;
            expiry_time?: number;
            token_allowances?: [number, string][];
        } = {
            email_addr: emailAddr,
        };
        if (epheAddr !== null) {
            requestData.ephe_addr = epheAddr;
        }
        if (username !== null) {
            requestData.username = username;
        }
        if (expiryTime !== null) {
            requestData.expiry_time = expiryTime;
        }
        if (tokenAllowances !== null) {
            requestData.token_allowances = tokenAllowances;
        }
        const res = await axios.post(url, requestData);
        return res.data;
    }

    public async epheAddrStatus(
        requestId: string,
        signature: string
    ): Promise<{
        is_activated: boolean;
        wallet_addr?: string;
        nonce?: string;
    }> {
        const url = `${this.relayerHost}/api/epheAddrStatus`;
        const res = await axios.post(url, { request_id: requestId, signature });
        return res.data;
    }

    // public async signin(
    //     emailAddr: string,
    //     username: string,
    //     nonce: string,
    //     expiry_time: number | null,
    //     token_allowances: [number, string][] | null
    // ): Promise<string> {
    //     const url = `${this.relayerHost}/api/login`;
    //     const requestData: {
    //         email_addr: string;
    //         username: string;
    //         nonce: string;
    //         expiry_time?: number;
    //         token_allowances?: [number, string][];
    //     } = {
    //         email_addr: emailAddr,
    //         username,
    //         nonce,
    //     };
    //     if (expiry_time !== null) {
    //         requestData.expiry_time = expiry_time;
    //     }
    //     if (token_allowances !== null) {
    //         requestData.token_allowances = token_allowances;
    //     }
    //     const res = await axios.post(url, requestData);
    //     return res.data;
    // }

    // public async registerEpheAddr(
    //     walletAddr: Address,
    //     epheAddr: Address,
    //     signature: string
    // ): Promise<string> {
    //     const url = `${this.relayerHost}/api/registerEpheAddr`;
    //     const res = await axios.post(url, { wallet_addr: walletAddr, ephe_addr: epheAddr, signature });
    //     return res.data;
    // }

    public async executeEphemeralTx(
        walletAddr: Address,
        txNonce: string,
        epheAddr: Address,
        epheAddrNonce: string,
        target: Address,
        ethValue: string,
        data: string,
        tokenAmount: string,
        signature: string
    ): Promise<string> {
        const url = `${this.relayerHost}/api/executeEphemeralTx`;
        const res = await axios.post(url, {
            wallet_addr: walletAddr,
            tx_nonce: txNonce,
            ephe_addr: epheAddr,
            ephe_addr_nonce: epheAddrNonce,
            target,
            eth_value: ethValue,
            data,
            token_amount: tokenAmount,
            signature,
        });
        return res.data;
    }

}