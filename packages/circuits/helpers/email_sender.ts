import fs from "fs";
import { promisify } from "util";
const emailWalletUtils = require("@zk-email/relayer-utils");

export async function genEmailSenderInput(
  emailFilePath: string,
  accountCode: string,
): Promise<{
  in_padded: string[];
  pubkey: string[];
  signature: string[];
  in_padded_len: string;
  sender_account_code: string;
  sender_email_idx: number;
  subject_idx: number;
  recipient_email_idx: number;
  domain_idx: number;
  timestamp_idx: number;
}> {
  const emailRaw = await promisify(fs.readFile)(emailFilePath, "utf8");
  const jsonStr = await emailWalletUtils.genEmailSenderInput(emailRaw, accountCode);
  return JSON.parse(jsonStr);
}
