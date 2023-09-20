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
jest.setTimeout(120000);
describe("Email Sender", () => {
    it("Verify a sent email", async () => {
        const emailRaw = readFileSync(path.join(__dirname, "./emails/email_sender_test1.eml"), "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        console.log(parsedEmail.canonicalizedHeader);
        // const paddedHeader = emailWalletUtils.padString(parsedEmail.canonicalizedHeader, 1024);
        // console.log(paddedHeader);
        // const pubKey = toCircomBigIntBytes(BigInt(parsedEmail.publicKey));
        // const signature = toCircomBigIntBytes(BigInt(parsedEmail.signature));
        // console.log(pubKey);
        // console.log(signature);
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
        const senderEmailIdx = emailWalletUtils.extractSubstrIdxes(parsedEmail.canonicalizedHeader, readFileSync(path.join(__dirname, "../src/regexes/from_addr.json"), "utf8"))[0];
        const subjectEmailIdx = emailWalletUtils.extractSubstrIdxes(parsedEmail.canonicalizedHeader, readFileSync(path.join(__dirname, "../src/regexes/subject_all.json"), "utf8"))[0];
        const subject = parsedEmail.canonicalizedHeader.slice(subjectEmailIdx, subjectEmailIdx + 512);
        console.log(subject);
        const recipientEmailIdx = emailWalletUtils.extractSubstrIdxes(subject, readFileSync(path.join(__dirname, "../src/regexes/email_addr.json"), "utf8"))[0];
        console.log(recipientEmailIdx);
        // const recipientEmailAddr = subject.slice(recipientEmailIdx, recipientEmailIdx + 256);
        // console.log(recipientEmailAddr);
        const senderEmailAddrIdx = emailWalletUtils.extractSubstrIdxes(parsedEmail.canonicalizedHeader, readFileSync(path.join(__dirname, "../src/regexes/from_addr.json"), "utf8"))[0];
        // const senderEmailAddr = parsedEmail.canonicalizedHeader.slice(senderEmailAddrIdx, senderEmailAddrIdx + 256);
        // console.log(senderEmailAddr);
        const domainIdx = emailWalletUtils.extractSubstrIdxes(parsedEmail.canonicalizedHeader.slice(senderEmailAddrIdx, senderEmailAddrIdx + 256), readFileSync(path.join(__dirname, "../src/regexes/email_domain.json"), "utf8"))[0];
        console.log(domainIdx);
        console.log(parsedEmail.canonicalizedHeader.slice(senderEmailAddrIdx, senderEmailAddrIdx + 256).slice(domainIdx));
        const circuitInputs = {
            in_padded: emailCircuitInputs.in_padded,
            pubkey: emailCircuitInputs.pubkey,
            signature: emailCircuitInputs.signature,
            in_padded_len: emailCircuitInputs.in_len_padded_bytes,
            sender_relayer_rand: relayerRand,
            sender_email_idx: senderEmailIdx,
            subject_idx: subjectEmailIdx,
            recipient_email_idx: recipientEmailIdx,
            domain_idx: domainIdx
        };
        const circuit = await wasm_tester(path.join(__dirname, "../src/email_sender.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        console.log(witness);
        // console.log(JSON.stringify(witness.slice(0, 1 + 512 + 255)));
        const maskedSubject = "Send 0.1 ETH to ";
        const paddedMaskedSubject = emailWalletUtils.padString(maskedSubject, 512);
        for (let idx = 0; idx < paddedMaskedSubject.length; ++idx) {
            expect(BigInt(paddedMaskedSubject[idx])).toEqual(witness[1 + idx]);
        }
        const domainName = "gmail.com";
        const paddedDomain = emailWalletUtils.padString(domainName, 255);
        console.log(witness.slice(512));
        for (let idx = 0; idx < domainName.length; ++idx) {
            console.log(idx);
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
        // const viewingKey = "0x000123456789abcdef0123456789abcdef0123456789abcdef0123456789abcd";
        // // const viewingKey = "0xcdab8967452301efcdab8967452301efcdab8967452301efcdab896745230100";
        // const expectedVkCommit = emailWalletUtils.viewingKeyCommit(viewingKey, emailAddr, expectedRelayerRandHash);
        // expect(BigInt(expectedVkCommit)).toEqual(witness[260]);

        // const expectedRelayerRandHash = emailWalletUtils.relayerRandHash(relayerRand);
        // // expect(expectedRelayerRandHash).toEqual("0x" + witness[1].toString(16));
        // expect(BigInt(expectedRelayerRandHash)).toEqual(witness[1]);
        // const expectedEmailAddrPointer = emailWalletUtils.emailAddrPointer(emailAddr, relayerRand);
        // expect(BigInt(expectedEmailAddrPointer)).toEqual(witness[2]);
        // const expectedVkCommit = emailWalletUtils.viewingKeyCommit(viewingKey, emailAddr, expectedRelayerRandHash);
        // // expect(expectedVkCommit).toEqual("0x" + witness[2].toString(16));
        // expect(BigInt(expectedVkCommit)).toEqual(witness[3]);
        // const expectedWalletSalt = emailWalletUtils.walletSalt(viewingKey);
        // expect(BigInt(expectedWalletSalt)).toEqual(witness[4]);
        // const expectedExtAccountSalt = emailWalletUtils.extAccountSalt(viewingKey);
        // expect(BigInt(expectedExtAccountSalt)).toEqual(witness[5]);
        // const hashedPoint = hash_to_curve(paddedEmailAddr);
        // const expectedPsiPoint = point_scalar_mul(hashedPoint.x, hashedPoint.y, BigInt(relayerRand));
        // expect(expectedPsiPoint.x).toEqual(witness[6]);
        // expect(expectedPsiPoint.y).toEqual(witness[7]);
    });
});