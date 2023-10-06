const emailWalletUtils = require("../../utils");

export async function genAccountCreationInput(emailAddr: string, relayerRand: string, accountKey: string):
  Promise<{
    email_addr: number[],
    relayer_rand: string,
    account_key: string,
  }> {
  const paddedEmailAddr = emailWalletUtils.padEmailAddr(emailAddr);
  return {
    email_addr: paddedEmailAddr,
    relayer_rand: relayerRand,
    account_key: accountKey,
  };
}

