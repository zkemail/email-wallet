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

    public async recoverAccountCode(
        emailAddr: string,
    ): Promise<void> {
        const url = `${this.relayerHost}/api/recoverAccountCode`;
        await axios.post(url, { email_addr: emailAddr });
    }

    public async signup(emailAddr: string, username: string): Promise<string> {
        const url = `${this.relayerHost}/api/signup`;
        const res = await axios.post(url, { email_addr: emailAddr, username });
        return res.data;
    }

    public async signin(
        emailAddr: string,
        username: string,
        nonce: string,
        expiry_time: number | null,
        is_sudo: boolean | null,
        token_allowances: [number, string][] | null
    ): Promise<string> {
        const url = `${this.relayerHost}/api/login`;
        const requestData: {
            email_addr: string;
            username: string;
            nonce: string;
            expiry_time?: number;
            is_sudo?: boolean;
            token_allowances?: [number, string][];
        } = {
            email_addr: emailAddr,
            username,
            nonce,
        };
        if (expiry_time !== null) {
            requestData.expiry_time = expiry_time;
        }
        if (is_sudo !== null) {
            requestData.is_sudo = is_sudo;
        }
        if (token_allowances !== null) {
            requestData.token_allowances = token_allowances;
        }
        const res = await axios.post(url, requestData);
        return res.data;
    }

    public async registerEpheAddr(
        walletAddr: Address,
        username: string,
        epheAddr: Address,
        signature: string
    ): Promise<string> {
        const url = `${this.relayerHost}/api/registerEpheAddr`;
        const res = await axios.post(url, { wallet_addr: walletAddr, username, ephe_addr: epheAddr, signature });
        return res.data;
    }

    public async executeEphemeralTx(
        walletAddr: Address,
        txNonce: bigint,
        epheAddr: Address,
        epheAddrNonce: bigint,
        target: Address,
        ethValue: bigint,
        data: string,
        tokenAmount: bigint,
        signature: string
    ): Promise<string> {
        const url = `${this.relayerHost}/api/executeEphemeralTx`;
        const res = await axios.post(url, {
            wallet_addr: walletAddr,
            tx_nonce: txNonce.toString(),
            ephe_addr: epheAddr,
            ephe_addr_nonce: epheAddrNonce.toString(),
            target,
            eth_value: ethValue.toString(),
            data,
            token_amount: tokenAmount.toString(),
            signature,
        });
        return res.data;
    }

}