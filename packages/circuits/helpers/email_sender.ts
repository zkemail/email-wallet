import fs from "fs";
import { promisify } from "util";
const relayerUtils = require("@zk-email/relayer-utils");

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
  const options = {
    maxHeaderLength: 1024,
    ignoreBodyHashCheck: true,
  }
  const jsonStr = await relayerUtils.genEmailCircuitInput(emailRaw, accountCode, options);
  const {
    body_hash_idx,
    precomputed_sha,
    padded_body,
    padded_body_len,
    command_idx,
    padded_cleaned_body,
    account_code,
    ...circuitInputsRelevant
  } = JSON.parse(jsonStr);
  const parsedEmail = await relayerUtils.parseEmail(emailRaw);
  const subjectEmailIdxes = relayerUtils.extractSubjectAllIdxes(
    parsedEmail.canonicalizedHeader
  )[0];
  const subject = parsedEmail.canonicalizedHeader.slice(
    subjectEmailIdxes[0],
    subjectEmailIdxes[1]
  );
  let recipientEmailIdx = 0;
  try {
    recipientEmailIdx = relayerUtils.extractEmailAddrIdxes(subject)[0][0];
  } catch (e) {
    console.log("No email address in subject");
    recipientEmailIdx = 0;
  }
  console.log({
    sender_account_code: account_code,
    recipient_email_idx: recipientEmailIdx,
    ...circuitInputsRelevant,
  });
  return {
    sender_account_code: account_code,
    recipient_email_idx: recipientEmailIdx,
    ...circuitInputsRelevant,
  };
}
