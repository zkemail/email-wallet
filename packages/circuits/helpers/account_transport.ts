import fs from "fs";
import { promisify } from "util";
import { generateCircuitInputs } from "@zk-email/helpers/dist/input-helpers";
const emailWalletUtils = require("../../utils");


export async function genAccountTransportInput(emailFilePath: string, oldRelayerRandHash: string, newRelayerRand: string):
  Promise<{
    in_padded: string[],
    pubkey: string[],
    signature: string[],
    in_padded_len: string,
    old_relayer_rand_hash: string,
    new_relayer_rand: string,
    sender_email_idx: number,
    code_idx: number,
    domain_idx: number,
    timestamp_idx: number
  }> {
  const emailRaw = await promisify(fs.readFile)(emailFilePath, "utf8");
  const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
  const emailCircuitInputs = generateCircuitInputs({
    body: Buffer.from(""),
    message: Buffer.from(parsedEmail.canonicalizedHeader),
    bodyHash: "",
    rsaSignature: BigInt(parsedEmail.signature),
    rsaPublicKey: BigInt(parsedEmail.publicKey),
    maxMessageLength: 1024,
    maxBodyLength: 64,
    ignoreBodyHashCheck: true
  });
  const senderEmailIdxes = emailWalletUtils.extractFromAddrIdxes(parsedEmail.canonicalizedHeader)[0];
  const codeIdx = emailWalletUtils.extractInvitationCodeIdxes(parsedEmail.canonicalizedHeader)[0][0];
  const fromEmailAddrPart = parsedEmail.canonicalizedHeader.slice(senderEmailIdxes[0], senderEmailIdxes[1]);
  const domainIdx = emailWalletUtils.extractEmailDomainIdxes(fromEmailAddrPart)[0][0];
  const timestampIdx = emailWalletUtils.extractTimestampIdxes(parsedEmail.canonicalizedHeader)[0][0];
  return {
    in_padded: emailCircuitInputs.in_padded,
    pubkey: emailCircuitInputs.pubkey,
    signature: emailCircuitInputs.signature,
    in_padded_len: emailCircuitInputs.in_len_padded_bytes,
    old_relayer_rand_hash: oldRelayerRandHash,
    new_relayer_rand: newRelayerRand,
    sender_email_idx: senderEmailIdxes[0],
    code_idx: codeIdx,
    domain_idx: domainIdx,
    timestamp_idx: timestampIdx
  };
}
