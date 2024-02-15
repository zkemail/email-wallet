const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import * as path from "path";
const emailWalletUtils = require("../../utils");
const option = {
    include: path.join(__dirname, "../../../node_modules")
};
import { readFileSync } from "fs";

// const grumpkin = require("circom-grumpkin");
jest.setTimeout(120000);
describe("Invitation Code Regex", () => {
    it("invitation code", async () => {
        const codeStr = "Code 123abc";
        // const prefixLen = "ACCOUNTKEY.0x".length;
        // const revealed = "123abc";
        const paddedStr = emailWalletUtils.padString(codeStr, 256);
        const circuitInputs = {
            msg: paddedStr,
        };
        const circuit = await wasm_tester(path.join(__dirname, "./circuits/test_invitation_code_regex.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        // console.log(witness);
        expect(1n).toEqual(witness[1]);
        const prefixIdxes = emailWalletUtils.extractInvitationCodeIdxes(codeStr)[0];
        for (let idx = 0; idx < 256; ++idx) {
            if (idx >= prefixIdxes[0] && idx < prefixIdxes[1]) {
                expect(BigInt(paddedStr[idx])).toEqual(witness[2 + idx]);
            } else {
                expect(0n).toEqual(witness[2 + idx]);
            }
        }
    });

    it("invitation code in the subject", async () => {
        const codeStr = "Send 0.1 ETH to alice@gmail.com code 123abc";
        // const prefixLen = "sepolia+ACCOUNTKEY.0x".length;
        // const revealed = "123abc";
        const paddedStr = emailWalletUtils.padString(codeStr, 256);
        const circuitInputs = {
            msg: paddedStr,
        };
        const circuit = await wasm_tester(path.join(__dirname, "./circuits/test_invitation_code_regex.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        // console.log(witness);
        expect(1n).toEqual(witness[1]);
        const prefixIdxes = emailWalletUtils.extractInvitationCodeIdxes(codeStr)[0];
        // const revealedStartIdx = emailWalletUtils.extractSubstrIdxes(codeStr, readFileSync(path.join(__dirname, "../src/regexes/invitation_code.json"), "utf8"))[0][0];
        // console.log(emailWalletUtils.extractSubstrIdxes(codeStr, readFileSync(path.join(__dirname, "../src/regexes/invitation_code.json"), "utf8")));
        for (let idx = 0; idx < 256; ++idx) {
            if (idx >= prefixIdxes[0] && idx < prefixIdxes[1]) {
                expect(BigInt(paddedStr[idx])).toEqual(witness[2 + idx]);
            } else {
                expect(0n).toEqual(witness[2 + idx]);
            }
        }
    });

    it("invitation code in the email address", async () => {
        const codeStr = "sepolia+code123456@sendeth.org";
        // const prefixLen = "sepolia+ACCOUNTKEY.0x".length;
        // const revealed = "123abc";
        const paddedStr = emailWalletUtils.padString(codeStr, 256);
        const circuitInputs = {
            msg: paddedStr,
        };
        const circuit = await wasm_tester(path.join(__dirname, "./circuits/test_invitation_code_regex.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        // console.log(witness);
        expect(1n).toEqual(witness[1]);
        const prefixIdxes = emailWalletUtils.extractInvitationCodeIdxes(codeStr)[0];
        // const revealedStartIdx = emailWalletUtils.extractSubstrIdxes(codeStr, readFileSync(path.join(__dirname, "../src/regexes/invitation_code.json"), "utf8"))[0][0];
        // console.log(emailWalletUtils.extractSubstrIdxes(codeStr, readFileSync(path.join(__dirname, "../src/regexes/invitation_code.json"), "utf8")));
        for (let idx = 0; idx < 256; ++idx) {
            if (idx >= prefixIdxes[0] && idx < prefixIdxes[1]) {
                expect(BigInt(paddedStr[idx])).toEqual(witness[2 + idx]);
            } else {
                expect(0n).toEqual(witness[2 + idx]);
            }
        }
    });
});