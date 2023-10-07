const utils = require("../../utils");
const ff = require('ffjavascript');
const stringifyBigInts = ff.utils.stringifyBigInts;
const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import * as path from "path";
const p = "21888242871839275222246405745257275088548364400416034343698204186575808495617";
const field = new ff.F1Field(p);
const emailWalletUtils = require("../../utils");
const option = {
    include: path.join(__dirname, "../../../node_modules")
};
import { genEmailSenderInput } from "../helpers/email_sender";
import { readFileSync } from "fs";

jest.setTimeout(360000);
describe("Email Sender", () => {
    it("Verify a sent email", async () => {
        const emailFilePath = path.join(__dirname, "./emails/email_sender_test1.eml");
        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        console.log(parsedEmail.canonicalizedHeader);
        const relayerRand = emailWalletUtils.genRelayerRand();
        const circuitInputs = await genEmailSenderInput(emailFilePath, relayerRand);
        const circuit = await wasm_tester(path.join(__dirname, "../src/email_sender.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        // console.log(witness);
        const maskedSubject = "Send 0.1 ETH to ";
        const paddedMaskedSubject = emailWalletUtils.padString(maskedSubject, 512);
        const maskedSubjectFields = emailWalletUtils.bytes2Fields(paddedMaskedSubject);
        for (let idx = 0; idx < maskedSubjectFields.length; ++idx) {
            expect(BigInt(maskedSubjectFields[idx])).toEqual(witness[1 + idx]);
        }
        const domainName = "gmail.com";
        const paddedDomain = emailWalletUtils.padString(domainName, 255);
        const domainFields = emailWalletUtils.bytes2Fields(paddedDomain);
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[18 + idx]);
        }
        const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
        expect(BigInt(expectedPubKeyHash)).toEqual(witness[27]);
        const expectedRelayerRandHash = emailWalletUtils.relayerRandHash(relayerRand);
        expect(BigInt(expectedRelayerRandHash)).toEqual(witness[28]);
        const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
        expect(BigInt(expectedEmailNullifier)).toEqual(witness[29]);
        const senderEmailAddr = "suegamisora@gmail.com";
        const expectedEmailAddrPointer = emailWalletUtils.emailAddrPointer(senderEmailAddr, relayerRand);
        expect(BigInt(expectedEmailAddrPointer)).toEqual(witness[30]);
        expect(1n).toEqual(witness[31]);
        const recipientEmailAddr = "alice@gmail.com"
        const expectedRecipientEmailAddrCommit = emailWalletUtils.emailAddrCommitWithSignature(recipientEmailAddr, parsedEmail.signature);
        expect(BigInt(expectedRecipientEmailAddrCommit)).toEqual(witness[32]);
        const timestamp = 1694989812n;
        expect(timestamp).toEqual(witness[33]);
        // const paddedTimestamp = emailWalletUtils.padString(timestamp, 10);
        // for (let idx = 0; idx < paddedTimestamp.length; ++idx) {
        //     expect(BigInt(paddedTimestamp[idx])).toEqual(witness[774 + idx]);
        // }
    });
});