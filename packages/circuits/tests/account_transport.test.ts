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

jest.setTimeout(1080000);
describe("Account Transport", () => {
    it("transport an account", async () => {
        const emailFilePath = path.join(__dirname, "./emails/account_init_test1.eml");
        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        const oldRelayerRand = emailWalletUtils.genRelayerRand();
        const oldRelayerRandHash = emailWalletUtils.relayerRandHash(oldRelayerRand);
        const newRelayerRand = emailWalletUtils.genRelayerRand();
        const newRelayerRandHash = emailWalletUtils.relayerRandHash(newRelayerRand);
        const circuitInputs = await genAccountTransportInput(emailFilePath, oldRelayerRandHash, newRelayerRand);
        const circuit = await wasm_tester(path.join(__dirname, "../src/account_transport.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
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
        const accountKey = "0x01eb9b204cc24c3baee11accc37d253a9c53e92b1a2cc07763475c135d575b76";
        const expectedOldAkCommit = emailWalletUtils.accountKeyCommit(accountKey, emailAddr, oldRelayerRandHash);
        expect(BigInt(expectedOldAkCommit)).toEqual(witness[12]);
        const expectedNewAkCommit = emailWalletUtils.accountKeyCommit(accountKey, emailAddr, newRelayerRandHash);
        expect(BigInt(expectedNewAkCommit)).toEqual(witness[13]);
        expect(BigInt(newRelayerRandHash)).toEqual(witness[14]);
        const timestamp = 1697222111n;
        expect(timestamp).toEqual(witness[15]);
        expect(BigInt(oldRelayerRandHash)).toEqual(witness[16]);
    });

    it("transport an account, the from field has a dummy email address name", async () => {
        const emailFilePath = path.join(__dirname, "./emails/account_init_test2.eml");
        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        const oldRelayerRand = emailWalletUtils.genRelayerRand();
        const oldRelayerRandHash = emailWalletUtils.relayerRandHash(oldRelayerRand);
        const newRelayerRand = emailWalletUtils.genRelayerRand();
        const newRelayerRandHash = emailWalletUtils.relayerRandHash(newRelayerRand);
        const circuitInputs = await genAccountTransportInput(emailFilePath, oldRelayerRandHash, newRelayerRand);
        const circuit = await wasm_tester(path.join(__dirname, "../src/account_transport.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
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
        const accountKey = "0x01eb9b204cc24c3baee11accc37d253a9c53e92b1a2cc07763475c135d575b76";
        const expectedOldAkCommit = emailWalletUtils.accountKeyCommit(accountKey, emailAddr, oldRelayerRandHash);
        expect(BigInt(expectedOldAkCommit)).toEqual(witness[12]);
        const expectedNewAkCommit = emailWalletUtils.accountKeyCommit(accountKey, emailAddr, newRelayerRandHash);
        expect(BigInt(expectedNewAkCommit)).toEqual(witness[13]);
        expect(BigInt(newRelayerRandHash)).toEqual(witness[14]);
        const timestamp = 1697224006n;
        expect(timestamp).toEqual(witness[15]);
        expect(BigInt(oldRelayerRandHash)).toEqual(witness[16]);
    });

    it("transport an account, the from field has a non-English name", async () => {
        const emailFilePath = path.join(__dirname, "./emails/account_init_test3.eml");
        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        const oldRelayerRand = emailWalletUtils.genRelayerRand();
        const oldRelayerRandHash = emailWalletUtils.relayerRandHash(oldRelayerRand);
        const newRelayerRand = emailWalletUtils.genRelayerRand();
        const newRelayerRandHash = emailWalletUtils.relayerRandHash(newRelayerRand);
        const circuitInputs = await genAccountTransportInput(emailFilePath, oldRelayerRandHash, newRelayerRand);
        const circuit = await wasm_tester(path.join(__dirname, "../src/account_transport.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
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
        const accountKey = "0x01eb9b204cc24c3baee11accc37d253a9c53e92b1a2cc07763475c135d575b76";
        const expectedOldAkCommit = emailWalletUtils.accountKeyCommit(accountKey, emailAddr, oldRelayerRandHash);
        expect(BigInt(expectedOldAkCommit)).toEqual(witness[12]);
        const expectedNewAkCommit = emailWalletUtils.accountKeyCommit(accountKey, emailAddr, newRelayerRandHash);
        expect(BigInt(expectedNewAkCommit)).toEqual(witness[13]);
        expect(BigInt(newRelayerRandHash)).toEqual(witness[14]);
        const timestamp = 1697224182n;
        expect(timestamp).toEqual(witness[15]);
        expect(BigInt(oldRelayerRandHash)).toEqual(witness[16]);
    });
});