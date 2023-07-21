import {
  Uint8ArrayToCharArray,
  bytesToBigInt,
  fromHex,
  int8toBytes,
  mergeUInt8Arrays,
  stringToBytes
} from "@zk-email/helpers/dist/binaryFormat";
import { generateCircuitInputs } from "@zk-email/helpers/dist/input-helpers";
import { verifyDKIMSignature } from "@zk-email/helpers/dist/dkim";

export const MAX_HEADER_PADDED_BYTES = 1024; // NOTE: this must be the same as the first arg in the email in main args circom
export const MAX_BODY_PADDED_BYTES = 1536; // NOTE: this must be the same as the arg to sha the remainder number of bytes in the email in main args circom

// Returns the part of str that appears after substr
function trimStrByStr(str: string, substr: string) {
  const index = str.indexOf(substr);
  if (index === -1) return str;
  return str.slice(index + substr.length, str.length);
}

// padWithZero(bodyRemaining, MAX_BODY_PADDED_BYTES)
function padWithZero(arr: Uint8Array, length: number) {
  while (arr.length < length) {
    arr = mergeUInt8Arrays(arr, int8toBytes(0));
  }
  return arr;
}

export function generateWalletCircuitInputs({
  rsaSignature,
  rsaModulus,
  body,
  bodyHash,
  message, // the message that was signed (header + bodyHash)
  nonce
}: {
  body: Buffer;
  message: Buffer;
  bodyHash: string;
  rsaSignature: BigInt;
  rsaModulus: BigInt;
  nonce: string | null;
}) {
  const emailVerifierInputs = generateCircuitInputs({
    rsaSignature,
    rsaModulus,
    body,
    bodyHash,
    message,
    maxMessageLength: MAX_HEADER_PADDED_BYTES,
    maxBodyLength: MAX_BODY_PADDED_BYTES
  });

  const messageStr = message.toString();
  const emailSubject = trimStrByStr(messageStr, "\r\nsubject:");
  const email_from_idx =
    messageStr.length -
    trimStrByStr(trimStrByStr(messageStr, "from:"), "<").length;

  // First word after "subject:" (usually send/Send)
  const command = emailSubject.split(" ")[0];
  const command_idx = messageStr.length - emailSubject.length;

  // Index of first word after command
  const amount_idx =
    messageStr.length - trimStrByStr(emailSubject, command).length;

  // Index of second word after command
  const currency_idx =
    messageStr.length -
    trimStrByStr(trimStrByStr(emailSubject, command), " ").length;

  // Index of first word after subject and "to"
  const recipient_idx =
    messageStr.length - trimStrByStr(emailSubject, " to ").length;

  // Used to get the private message-id
  const message_id_idx =
    messageStr.length - trimStrByStr(messageStr, "\r\nmessage-id:<").length;

  const message_id = messageStr.slice(message_id_idx).split(">\r\n")[0];
  const MAX_MESSAGE_ID_LEN = 128;
  const message_id_array = Uint8ArrayToCharArray(
    padWithZero(stringToBytes(message_id), MAX_MESSAGE_ID_LEN)
  );

  let custom_message_id_from = message_id_array;
  let custom_message_id_recipient = message_id_array;

  // If the message-id is in the format (from)_(_to), then we can use it to get the private message-ids
  // TODO: Use JSON to pass this in instead of the filename string parsing
  if (nonce !== null) {
    const matchResult = nonce.match(/\(([^)]+)\)_\(([^)]+)\)/);

    if (matchResult) {
      const [_, _custom_message_id_from, _custom_message_id_recipient] =
        matchResult;
      custom_message_id_from = Uint8ArrayToCharArray(
        padWithZero(stringToBytes(_custom_message_id_from), MAX_MESSAGE_ID_LEN)
      );
      custom_message_id_recipient = Uint8ArrayToCharArray(
        padWithZero(
          stringToBytes(_custom_message_id_recipient),
          MAX_MESSAGE_ID_LEN
        )
      );
    }
  }

  const address = bytesToBigInt(
    fromHex("0x0000000000000000000000000000000000000000")
  ).toString();

  const nullifier = emailVerifierInputs.signature[0];

  return {
    in_padded: emailVerifierInputs.in_padded,
    modulus: emailVerifierInputs.modulus,
    signature: emailVerifierInputs.signature,
    in_len_padded_bytes: emailVerifierInputs.in_len_padded_bytes,
    body_hash_idx: emailVerifierInputs.body_hash_idx,
    relayer: address,
    nullifier: nullifier,
    email_from_idx: email_from_idx.toString(),
    command_idx: command_idx.toString(),
    message_id_idx: message_id_idx.toString(),
    amount_idx: amount_idx.toString(),
    currency_idx: currency_idx.toString(),
    recipient_idx: recipient_idx.toString(),
    custom_message_id_from,
    custom_message_id_recipient
  };
}

export async function generateWalletCircuitInputsFromEmail(
  rawEmail: Buffer,
  nonce: string
) {
  const dkimResult = await verifyDKIMSignature(rawEmail);

  const circuitInputs = generateWalletCircuitInputs({
    rsaSignature: dkimResult.signature,
    rsaModulus: dkimResult.modulus,
    body: dkimResult.body,
    bodyHash: dkimResult.bodyHash,
    message: dkimResult.message,
    nonce
  });

  return circuitInputs;
}
