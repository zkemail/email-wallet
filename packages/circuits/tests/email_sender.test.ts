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
import { hash_to_curve, point_scalar_mul } from "circom-grumpkin";
import { readFileSync } from "fs";
import { toCircomBigIntBytes } from "@zk-email/helpers/dist/binaryFormat";
import { generateCircuitInputs } from "@zk-email/helpers/dist/input-helpers";

// const zkemailHelper = require("@zk-email/helpers");
// const grumpkin = require("circom-grumpkin");
jest.setTimeout(360000);
describe("Email Sender", () => {
    it("Verify a sent email", async () => {
        const emailRaw = readFileSync(path.join(__dirname, "./emails/email_sender_test1.eml"), "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        console.log(parsedEmail.canonicalizedHeader);
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
        const relayerRand = emailWalletUtils.genRelayerRand();
        const senderEmailIdxes = emailWalletUtils.extractFromAddrIdxes(parsedEmail.canonicalizedHeader)[0];
        const fromEmailAddrPart = parsedEmail.canonicalizedHeader.slice(senderEmailIdxes[0], senderEmailIdxes[1]);
        const domainIdx = emailWalletUtils.extractEmailDomainIdxes(fromEmailAddrPart)[0][0];
        const subjectEmailIdxes = emailWalletUtils.extractSubjectAllIdxes(parsedEmail.canonicalizedHeader)[0];
        const subject = parsedEmail.canonicalizedHeader.slice(subjectEmailIdxes[0], subjectEmailIdxes[1]);
        console.log(subject);
        const recipientEmailIdx = emailWalletUtils.extractEmailAddrIdxes(subject)[0][0];
        console.log(recipientEmailIdx);
        const timestampIdx = emailWalletUtils.extractTimestampIdxes(parsedEmail.canonicalizedHeader)[0][0];
        // const recipientEmailAddr = subject.slice(recipientEmailIdx, recipientEmailIdx + 256);
        // console.log(recipientEmailAddr);
        // const senderEmailAddr = parsedEmail.canonicalizedHeader.slice(senderEmailAddrIdx, senderEmailAddrIdx + 256);
        const circuitInputs = {
            in_padded: emailCircuitInputs.in_padded,
            pubkey: emailCircuitInputs.pubkey,
            signature: emailCircuitInputs.signature,
            in_padded_len: emailCircuitInputs.in_len_padded_bytes,
            sender_relayer_rand: relayerRand,
            sender_email_idx: senderEmailIdxes[0],
            subject_idx: subjectEmailIdxes[0],
            recipient_email_idx: recipientEmailIdx,
            domain_idx: domainIdx,
            timestamp_idx: timestampIdx
        };
        const circuit = await wasm_tester(path.join(__dirname, "../src/email_sender.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        // console.log(witness);
        // console.log(JSON.stringify(witness.slice(0, 1 + 512 + 255)));
        const maskedSubject = "Send 0.1 ETH to ";
        const paddedMaskedSubject = emailWalletUtils.padString(maskedSubject, 512);
        for (let idx = 0; idx < paddedMaskedSubject.length; ++idx) {
            expect(BigInt(paddedMaskedSubject[idx])).toEqual(witness[1 + idx]);
        }
        const domainName = "gmail.com";
        const paddedDomain = emailWalletUtils.padString(domainName, 255);
        for (let idx = 0; idx < domainName.length; ++idx) {
            expect(BigInt(paddedDomain[idx])).toEqual(witness[513 + idx]);
        }
        const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
        expect(BigInt(expectedPubKeyHash)).toEqual(witness[768]);
        const expectedRelayerRandHash = emailWalletUtils.relayerRandHash(relayerRand);
        expect(BigInt(expectedRelayerRandHash)).toEqual(witness[769]);
        const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
        expect(BigInt(expectedEmailNullifier)).toEqual(witness[770]);
        const senderEmailAddr = "suegamisora@gmail.com";
        const expectedEmailAddrPointer = emailWalletUtils.emailAddrPointer(senderEmailAddr, relayerRand);
        expect(BigInt(expectedEmailAddrPointer)).toEqual(witness[771]);
        expect(1n).toEqual(witness[772]);
        const recipientEmailAddr = "alice@gmail.com"
        const expectedRecipientEmailAddrCommit = emailWalletUtils.emailAddrCommitWithSignature(recipientEmailAddr, parsedEmail.signature);
        expect(BigInt(expectedRecipientEmailAddrCommit)).toEqual(witness[773]);
        const timestamp = "1694989812";
        const paddedTimestamp = emailWalletUtils.padString(timestamp, 10);
        for (let idx = 0; idx < paddedTimestamp.length; ++idx) {
            expect(BigInt(paddedTimestamp[idx])).toEqual(witness[774 + idx]);
        }
    });
});