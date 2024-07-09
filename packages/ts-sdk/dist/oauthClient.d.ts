import { Address, PrivateKeyAccount, PublicClient, Chain, Transport } from 'viem';
import RelayerApis from "./relayerApis";
export default class OauthClient<chain extends Chain> {
    publicClient: PublicClient<Transport, chain>;
    core: any;
    oauth: any;
    relayerApis: RelayerApis;
    userEmailAddr: string | null;
    userWallet: any;
    epheClient: PrivateKeyAccount;
    epheAddrNonce: string | null;
    constructor(client: PublicClient<Transport, chain>, coreAddress: Address, oauthAddress: Address, relayerHost: string, userEmailAddr?: string, userWallet?: any, epheAddrNonce?: string);
    isAccountCreated(userEmailAddr: string): Promise<boolean>;
    setup(userEmailAddr: string, username: string | null, expiryTime: number | null, tokenAllowances: [number, string][] | null): Promise<number>;
    waitEpheAddrActivated(requestId: number): Promise<boolean>;
    oauthExecuteTx(target: Address, data: `0x${string}`, ethValue: bigint | null, token_amount: bigint | null): Promise<string>;
}
