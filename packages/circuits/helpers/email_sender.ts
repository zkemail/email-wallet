import fs from "fs";
import { promisify } from "util";
const emailWalletUtils = require("@zk-email/relayer-utils");

export async function genEmailSenderInput(
  emailFilePath: string,
  accountCode: string,
): Promise<{
  padded_header: string[];
  public_key: string[];
  signature: string[];
  padded_header_len: string;
  sender_account_code: string;
  from_addr_idx: number;
  subject_idx: number;
  domain_idx: number;
  timestamp_idx: number;
  code_idx: number;
  recipient_email_idx: number;
}> {
  const emailRaw = await promisify(fs.readFile)(emailFilePath, "utf8");
  const jsonStr = await emailWalletUtils.genEmailSenderInput(emailRaw, accountCode);
  return JSON.parse(jsonStr);
}
