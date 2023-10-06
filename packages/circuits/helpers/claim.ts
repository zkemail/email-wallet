const emailWalletUtils = require("../../utils");

export async function genClaimInput(emailAddr: string, relayerRand: string, emailAddrRand: string):
  Promise<{
    recipient_email_addr: number[],
    recipient_relayer_rand: string,
    cm_rand: string,
  }> {
  const paddedEmailAddr = emailWalletUtils.padEmailAddr(emailAddr);
  return {
    recipient_email_addr: paddedEmailAddr,
    recipient_relayer_rand: relayerRand,
    cm_rand: emailAddrRand,
  };
}

