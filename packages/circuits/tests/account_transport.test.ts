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
        const senderEmailIdxes = emailWalletUtils.extractSubstrIdxes(parsedEmail.canonicalizedHeader, readFileSync(path.join(__dirname, "../src/regexes/from_addr.json"), "utf8"))[0];
        const codeIdx = emailWalletUtils.extractSubstrIdxes(parsedEmail.canonicalizedHeader, readFileSync(path.join(__dirname, "../src/regexes/invitation_code.json"), "utf8"))[0][0];
        const fromEmailAddrPart = parsedEmail.canonicalizedHeader.slice(senderEmailIdxes[0], senderEmailIdxes[1]);
        const domainIdx = emailWalletUtils.extractSubstrIdxes(fromEmailAddrPart, readFileSync(path.join(__dirname, "../src/regexes/email_domain.json"), "utf8"))[0][0];
        const circuitInputs = {
            in_padded: emailCircuitInputs.in_padded,
            pubkey: emailCircuitInputs.pubkey,
            signature: emailCircuitInputs.signature,
            in_padded_len: emailCircuitInputs.in_len_padded_bytes,
            sender_relayer_rand_hash: relayerRandHash,
            sender_email_idx: senderEmailIdxes[0],
            code_idx: codeIdx,
            domain_idx: domainIdx
        };
        const circuit = await wasm_tester(path.join(__dirname, "../src/account_transport.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        // console.log(witness);
        const domainName = "gmail.com";
        const paddedDomain = emailWalletUtils.padString(domainName, 255);
        for (let idx = 0; idx < domainName.length; ++idx) {
            expect(BigInt(paddedDomain[idx])).toEqual(witness[1 + idx]);
        }
        const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
        expect(BigInt(expectedPubKeyHash)).toEqual(witness[256]);
        const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
        expect(BigInt(expectedEmailNullifier)).toEqual(witness[257]);
        const emailAddr = "suegamisora@gmail.com";
        const viewingKey = "0x000123456789abcdef0123456789abcdef0123456789abcdef0123456789abcd";
        // const viewingKey = "0xcdab8967452301efcdab8967452301efcdab8967452301efcdab896745230100";
        const expectedVkCommit = emailWalletUtils.viewingKeyCommit(viewingKey, emailAddr, relayerRandHash);
        expect(BigInt(expectedVkCommit)).toEqual(witness[258]);

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