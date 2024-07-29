const emailWalletUtils = require("@zk-email/relayer-utils");

export async function genPsiPointInput(
    emailAddr: string,
    accountCode: string,
    relayerRand: string,
): Promise<{
    email_addr: number[];
    account_code: string;
    relayer_rand: string;
}> {
    const paddedEmailAddr = emailWalletUtils.padEmailAddr(emailAddr);
    return {
        email_addr: paddedEmailAddr,
        account_code: accountCode,
        relayer_rand: relayerRand,
    };
}
