import { Address } from 'viem';
export default class RelayerApis {
    relayerHost: string;
    constructor(relayerHost: string);
    getWalletAddress(emailAddr: string, accountCode: string): Promise<Address>;
    isAccountCreated(emailAddr: string): Promise<boolean>;
    signupOrIn(emailAddr: string, epheAddr: string | null, username: string | null, expiryTime: number | null, tokenAllowances: [number, string][] | null): Promise<number>;
    epheAddrStatus(requestId: number, signature: string): Promise<{
        is_activated: boolean;
        wallet_addr?: string;
        nonce?: string;
    }>;
    executeEphemeralTx(walletAddr: Address, txNonce: string, epheAddr: Address, epheAddrNonce: string, target: Address, ethValue: string, data: string, tokenAmount: string, signature: string): Promise<string>;
}
