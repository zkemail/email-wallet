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
import { genAccountTransportInput } from "../helpers/account_transport";
import { readFileSync } from "fs";

// const zkemailHelper = require("@zk-email/helpers");
// const grumpkin = require("circom-grumpkin");
jest.setTimeout(360000);
describe("Account Transport", () => {
    it("transport an account", async () => {
        const emailFilePath = path.join(__dirname, "./emails/account_init_test1.eml");
        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        const relayerRand = emailWalletUtils.genRelayerRand();
        const relayerRandHash = emailWalletUtils.relayerRandHash(relayerRand);
        const circuitInputs = await genAccountTransportInput(emailFilePath, relayerRandHash);
        const circuit = await wasm_tester(path.join(__dirname, "../src/account_transport.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        // console.log(witness);
        const domainName = "gmail.com";
        const paddedDomain = emailWalletUtils.padString(domainName, 255);
        const domainFields = emailWalletUtils.bytes2Fields(paddedDomain);
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
        }
        const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
        expect(BigInt(expectedPubKeyHash)).toEqual(witness[10]);
        const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
        expect(BigInt(expectedEmailNullifier)).toEqual(witness[11]);
        const emailAddr = "suegamisora@gmail.com";
        const accountKey = "0x000123456789abcdef0123456789abcdef0123456789abcdef0123456789abcd";
        const expectedAkCommit = emailWalletUtils.accountKeyCommit(accountKey, emailAddr, relayerRandHash);
        expect(BigInt(expectedAkCommit)).toEqual(witness[12]);
        const timestamp = 1694979179n;
        expect(timestamp).toEqual(witness[13]);
        // const paddedTimestamp = emailWalletUtils.padString(timestamp, 10);
        // for (let idx = 0; idx < paddedTimestamp.length; ++idx) {
        //     expect(BigInt(paddedTimestamp[idx])).toEqual(witness[259 + idx]);
        // }
    });
});