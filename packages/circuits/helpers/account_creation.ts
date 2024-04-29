import fs from "fs";
import { promisify } from "util";
const emailWalletUtils = require("../../utils");

export async function genAccountCreationInput(
  emailFilePath: string,
  relayerRand: string,
): Promise<{
  in_padded: string[];
  pubkey: string[];
  signature: string[];
  in_padded_len: string;
  relayer_rand: string;
  sender_email_idx: number;
  code_idx: number;
  domain_idx: number;
  timestamp_idx: number;
}> {
  const emailRaw = await promisify(fs.readFile)(emailFilePath, "utf8");
  const jsonStr = await emailWalletUtils.genAccountCreationInput(emailRaw, relayerRand);
  return JSON.parse(jsonStr);
}
