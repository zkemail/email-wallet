const emailWalletUtils = require("../../utils");

export async function genClaimInput(
  emailAddr: string,
  emailAddrRand: string,
  accountKey: string,
): Promise<{
  email_addr: number[];
  cm_rand: string;
  account_key: string;
}> {
  const paddedEmailAddr = emailWalletUtils.padEmailAddr(emailAddr);
  return {
    email_addr: paddedEmailAddr,
    cm_rand: emailAddrRand,
    account_key: accountKey,
  };
}
