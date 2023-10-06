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
jest.setTimeout(240000);
describe("Account Transport", () => {
    it("transport an account", async () => {
        const emailRaw = readFileSync(path.join(__dirname, "./emails/account_init_test1.eml"), "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
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
        const relayerRandHash = emailWalletUtils.relayerRandHash(relayerRand);
        const senderEmailIdxes = emailWalletUtils.extractFromAddrIdxes(parsedEmail.canonicalizedHeader)[0];
        const codeIdx = emailWalletUtils.extractInvitationCodeIdxes(parsedEmail.canonicalizedHeader)[0][0];
        const fromEmailAddrPart = parsedEmail.canonicalizedHeader.slice(senderEmailIdxes[0], senderEmailIdxes[1]);
        const domainIdx = emailWalletUtils.extractEmailDomainIdxes(fromEmailAddrPart)[0][0];
        const timestampIdx = emailWalletUtils.extractTimestampIdxes(parsedEmail.canonicalizedHeader)[0][0];
        const circuitInputs = {
            in_padded: emailCircuitInputs.in_padded,
            pubkey: emailCircuitInputs.pubkey,
            signature: emailCircuitInputs.signature,
            in_padded_len: emailCircuitInputs.in_len_padded_bytes,
            sender_relayer_rand_hash: relayerRandHash,
            sender_email_idx: senderEmailIdxes[0],
            code_idx: codeIdx,
            domain_idx: domainIdx,
            timestamp_idx: timestampIdx
        };
        const circuit = await wasm_tester(path.join(__dirname, "../src/account_transport.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        // console.log(witness);
        const domainName = "gmail.com";
        const paddedDomain = emailWalletUtils.padString(domainName, 255);
        for (let idx = 0; idx < paddedDomain.length; ++idx) {
            expect(BigInt(paddedDomain[idx])).toEqual(witness[1 + idx]);
        }
        const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
        expect(BigInt(expectedPubKeyHash)).toEqual(witness[256]);
        const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
        expect(BigInt(expectedEmailNullifier)).toEqual(witness[257]);
        const emailAddr = "suegamisora@gmail.com";
        const accountKey = "0x000123456789abcdef0123456789abcdef0123456789abcdef0123456789abcd";
        const expectedAkCommit = emailWalletUtils.accountKeyCommit(accountKey, emailAddr, relayerRandHash);
        expect(BigInt(expectedAkCommit)).toEqual(witness[258]);
        const timestamp = "1694979179";
        const paddedTimestamp = emailWalletUtils.padString(timestamp, 10);
        for (let idx = 0; idx < paddedTimestamp.length; ++idx) {
            expect(BigInt(paddedTimestamp[idx])).toEqual(witness[259 + idx]);
        }
    });
});