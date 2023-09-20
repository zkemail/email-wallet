const utils = require("../../utils");
const ff = require('ffjavascript');
const stringifyBigInts = ff.utils.stringifyBigInts;
const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import * as path from "path";
import { readFileSync } from "fs";
const p = "21888242871839275222246405745257275088548364400416034343698204186575808495617";
const field = new ff.F1Field(p);
const emailWalletUtils = require("../../utils");
const option = {
    include: path.join(__dirname, "../../../node_modules")
};
// const grumpkin = require("circom-grumpkin");
jest.setTimeout(120000);
describe("Timestamp Regex", () => {
    it("timestamp in the header", async () => {
        const emailRaw = readFileSync(path.join(__dirname, "./emails/email_sender_test1.eml"), "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        const revealed = "1694989812";
        const paddedStr = emailWalletUtils.padString(parsedEmail.canonicalizedHeader, 1024);
        const circuitInputs = {
            msg: paddedStr,
        };
        const circuit = await wasm_tester(path.join(__dirname, "./circuits/test_timestamp_regex.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        expect(1n).toEqual(witness[1]);
        const timestampIdx = emailWalletUtils.extractSubstrIdxes(parsedEmail.canonicalizedHeader, readFileSync(path.join(__dirname, "../src/regexes/timestamp.json"), "utf8"))[0];
        for (let idx = 0; idx < revealed.length; ++idx) {
            expect(BigInt(paddedStr[timestampIdx + idx])).toEqual(witness[2 + timestampIdx + idx]);
        }
    });
});