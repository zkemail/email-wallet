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
import { genClaimInput } from "../helpers/claim";

// const grumpkin = require("circom-grumpkin");
jest.setTimeout(120000);
describe("Claim", () => {
    it("claim an unclaimed funds/states", async () => {
        const emailAddr = "suegamisora@gmail.com";
        const emailAddrRand = emailWalletUtils.emailAddrCommitRand();
        const accountKey = emailWalletUtils.genAccountKey();
        const circuitInputs = await genClaimInput(emailAddr, emailAddrRand, accountKey);
        const circuit = await wasm_tester(path.join(__dirname, "../src/claim.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        const expectedEmailAddrCommit = emailWalletUtils.emailAddrCommit(emailAddr, emailAddrRand);
        expect(BigInt(expectedEmailAddrCommit)).toEqual(witness[1]);
        const walletSalt = emailWalletUtils.walletSalt(emailAddr, accountKey);
        expect(BigInt(walletSalt)).toEqual(witness[2]);
    });
});