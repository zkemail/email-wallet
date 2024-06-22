const emailWalletUtils = require("@zk-email/relayer-utils");

export async function genClaimInput(
  emailAddr: string,
  emailAddrRand: string,
  accountCode: string,
): Promise<{
  email_addr: number[];
  cm_rand: string;
  account_code: string;
}> {
  const paddedEmailAddr = emailWalletUtils.padEmailAddr(emailAddr);
  return {
    email_addr: paddedEmailAddr,
    cm_rand: emailAddrRand,
    account_code: accountCode,
  };
}
