const emailWalletUtils = require("../../utils");

export async function genAnnouncementInput(
  emailAddr: string,
  emailAddrRand: string,
): Promise<{
  email_addr: number[];
  cm_rand: string;
}> {
  const paddedEmailAddr = emailWalletUtils.padEmailAddr(emailAddr);
  return {
    email_addr: paddedEmailAddr,
    cm_rand: emailAddrRand,
  };
}
