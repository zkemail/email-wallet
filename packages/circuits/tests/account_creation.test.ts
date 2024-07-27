const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import * as path from "path";
const emailWalletUtils = require("@zk-email/relayer-utils");
const option = {
  include: path.join(__dirname, "../../../node_modules"),
};
import { genEmailSenderInput } from "../helpers/email_sender";
import { readFileSync } from "fs";
import { hash_to_curve, point_scalar_mul } from "circom-grumpkin";

jest.setTimeout(2080000);
describe("Account Initialization", () => {
  let circuit: any;
  beforeAll(async () => {
    circuit = await wasm_tester(path.join(__dirname, "../src/email_sender.circom"), option);

  });

  it("init an account", async () => {
    const emailFilePath = path.join(__dirname, "./emails/account_creation_test1.eml");
    const emailRaw = readFileSync(emailFilePath, "utf8");
    const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
    const relayerRand = emailWalletUtils.genRelayerRand();
    const accountCode = "0x01eb9b204cc24c3baee11accc37d253a9c53e92b1a2cc07763475c135d575b76";
    const circuitInputs = await genEmailSenderInput(emailFilePath, accountCode);
    const witness = await circuit.calculateWitness(circuitInputs);
    await circuit.checkConstraints(witness);
    const domainName = "gmail.com";
    const paddedDomain = emailWalletUtils.padString(domainName, 255);
    const domainFields = emailWalletUtils.bytes2Fields(paddedDomain);
    for (let idx = 0; idx < domainFields.length; ++idx) {
      expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
    }
    const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
    expect(BigInt(expectedPubKeyHash)).toEqual(witness[1 + domainFields.length]);
    const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
    expect(BigInt(expectedEmailNullifier)).toEqual(witness[1 + domainFields.length + 1]);
    const timestamp = 1707866192n;
    expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);
    const maskedSubject = "Send 0.12 ETH to ";
    const paddedMaskedSubject = emailWalletUtils.padString(maskedSubject, 605);
    const maskedSubjectFields = emailWalletUtils.bytes2Fields(paddedMaskedSubject);
    for (let idx = 0; idx < maskedSubjectFields.length; ++idx) {
      expect(BigInt(maskedSubjectFields[idx])).toEqual(witness[1 + domainFields.length + 3 + idx]);
    }
    const senderEmailAddr = "suegamisora@gmail.com";
    const accountSalt = emailWalletUtils.accountSalt(senderEmailAddr, accountCode);
    expect(BigInt(accountSalt)).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length]);
    expect(1n).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 1]);
    expect(1n).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 2]);
    const recipientEmailAddr = "alice@gmail.com";
    const expectedRecipientEmailAddrCommit = emailWalletUtils.emailAddrCommitWithSignature(
      recipientEmailAddr,
      parsedEmail.signature,
    );
    expect(BigInt(expectedRecipientEmailAddrCommit)).toEqual(
      witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 3],
    );
  });
});
