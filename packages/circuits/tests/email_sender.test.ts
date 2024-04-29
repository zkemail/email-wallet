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

jest.setTimeout(2440000);
describe("Email Sender", () => {
    it("Verify a sent email whose subject has an email address", async () => {
        const emailFilePath = path.join(__dirname, "./emails/email_sender_test1.eml");
        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        console.log(parsedEmail.canonicalizedHeader);
        const accountKey = await emailWalletUtils.genAccountKey();
        const circuitInputs = await genEmailSenderInput(emailFilePath, accountKey);
        const circuit = await wasm_tester(path.join(__dirname, "../src/email_sender.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        const domainName = "gmail.com";
        const paddedDomain = emailWalletUtils.padString(domainName, 255);
        const domainFields = emailWalletUtils.bytes2Fields(paddedDomain);
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
        }
        const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
        expect(BigInt(expectedPubKeyHash)).toEqual(witness[1 + domainFields.length]);
        const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
        expect(BigInt(expectedEmailNullifier)).toEqual(witness[1 + domainFields.length + 1]);
        const timestamp = 1694989812n;
        expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);
        const maskedSubject = "Send 0.1 ETH to ";
        const paddedMaskedSubject = emailWalletUtils.padString(maskedSubject, 605);
        const maskedSubjectFields = emailWalletUtils.bytes2Fields(paddedMaskedSubject);
        for (let idx = 0; idx < maskedSubjectFields.length; ++idx) {
            expect(BigInt(maskedSubjectFields[idx])).toEqual(witness[1 + domainFields.length + 3 + idx]);
        }
        const senderEmailAddr = "suegamisora@gmail.com";
        const walletSalt = emailWalletUtils.walletSalt(senderEmailAddr, accountKey);
        expect(BigInt(walletSalt)).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length]);
        expect(1n).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 1]);
        const recipientEmailAddr = "alice@gmail.com"
        const expectedRecipientEmailAddrCommit = emailWalletUtils.emailAddrCommitWithSignature(recipientEmailAddr, parsedEmail.signature);
        expect(BigInt(expectedRecipientEmailAddrCommit)).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 2]);
    });

    it("Verify a sent email whose subject does not have an email address", async () => {
        const emailFilePath = path.join(__dirname, "./emails/email_sender_test2.eml");
        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        console.log(parsedEmail.canonicalizedHeader);
        const accountKey = await emailWalletUtils.genAccountKey();
        const circuitInputs = await genEmailSenderInput(emailFilePath, accountKey);
        const circuit = await wasm_tester(path.join(__dirname, "../src/email_sender.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        const domainName = "gmail.com";
        const paddedDomain = emailWalletUtils.padString(domainName, 255);
        const domainFields = emailWalletUtils.bytes2Fields(paddedDomain);
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
        }
        const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
        expect(BigInt(expectedPubKeyHash)).toEqual(witness[1 + domainFields.length]);
        const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
        expect(BigInt(expectedEmailNullifier)).toEqual(witness[1 + domainFields.length + 1]);
        const timestamp = 1696964295n;
        expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);
        const maskedSubject = "Swap 1 ETH to DAI";
        const paddedMaskedSubject = emailWalletUtils.padString(maskedSubject, 605);
        const maskedSubjectFields = emailWalletUtils.bytes2Fields(paddedMaskedSubject);
        for (let idx = 0; idx < maskedSubjectFields.length; ++idx) {
            expect(BigInt(maskedSubjectFields[idx])).toEqual(witness[1 + domainFields.length + 3 + idx]);
        }
        const senderEmailAddr = "suegamisora@gmail.com";
        const walletSalt = emailWalletUtils.walletSalt(senderEmailAddr, accountKey);
        expect(BigInt(walletSalt)).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length]);
        expect(0n).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 1]);
        expect(0n).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 2]);
    });

    it("Verify a sent email whose from field has a dummy email address name", async () => {
        const emailFilePath = path.join(__dirname, "./emails/email_sender_test3.eml");
        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        console.log(parsedEmail.canonicalizedHeader);
        const accountKey = await emailWalletUtils.genAccountKey();
        const circuitInputs = await genEmailSenderInput(emailFilePath, accountKey);
        const circuit = await wasm_tester(path.join(__dirname, "../src/email_sender.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        const domainName = "gmail.com";
        const paddedDomain = emailWalletUtils.padString(domainName, 255);
        const domainFields = emailWalletUtils.bytes2Fields(paddedDomain);
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
        }
        const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
        expect(BigInt(expectedPubKeyHash)).toEqual(witness[1 + domainFields.length]);
        const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
        expect(BigInt(expectedEmailNullifier)).toEqual(witness[1 + domainFields.length + 1]);
        const timestamp = 1696965932n;
        expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);
        const maskedSubject = "Send 1 ETH to ";
        const paddedMaskedSubject = emailWalletUtils.padString(maskedSubject, 605);
        const maskedSubjectFields = emailWalletUtils.bytes2Fields(paddedMaskedSubject);
        for (let idx = 0; idx < maskedSubjectFields.length; ++idx) {
            expect(BigInt(maskedSubjectFields[idx])).toEqual(witness[1 + domainFields.length + 3 + idx]);
        }
        const senderEmailAddr = "suegamisora@gmail.com";
        const walletSalt = emailWalletUtils.walletSalt(senderEmailAddr, accountKey);
        expect(BigInt(walletSalt)).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length]);
        expect(1n).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 1]);
        const recipientEmailAddr = "bob@example.com"
        const expectedRecipientEmailAddrCommit = emailWalletUtils.emailAddrCommitWithSignature(recipientEmailAddr, parsedEmail.signature);
        expect(BigInt(expectedRecipientEmailAddrCommit)).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 2]);
    });

    it("Verify a sent email whose from field has a non-English name", async () => {
        const emailFilePath = path.join(__dirname, "./emails/email_sender_test4.eml");
        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        console.log(parsedEmail.canonicalizedHeader);
        const accountKey = await emailWalletUtils.genAccountKey();
        const circuitInputs = await genEmailSenderInput(emailFilePath, accountKey);
        const circuit = await wasm_tester(path.join(__dirname, "../src/email_sender.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        const domainName = "gmail.com";
        const paddedDomain = emailWalletUtils.padString(domainName, 255);
        const domainFields = emailWalletUtils.bytes2Fields(paddedDomain);
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
        }
        const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
        expect(BigInt(expectedPubKeyHash)).toEqual(witness[1 + domainFields.length]);
        const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
        expect(BigInt(expectedEmailNullifier)).toEqual(witness[1 + domainFields.length + 1]);
        const timestamp = 1696967028n;
        expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);
        const maskedSubject = "Send 1 ETH to ";
        const paddedMaskedSubject = emailWalletUtils.padString(maskedSubject, 605);
        const maskedSubjectFields = emailWalletUtils.bytes2Fields(paddedMaskedSubject);
        for (let idx = 0; idx < maskedSubjectFields.length; ++idx) {
            expect(BigInt(maskedSubjectFields[idx])).toEqual(witness[1 + domainFields.length + 3 + idx]);
        }
        const senderEmailAddr = "suegamisora@gmail.com";
        const walletSalt = emailWalletUtils.walletSalt(senderEmailAddr, accountKey);
        expect(BigInt(walletSalt)).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length]);
        expect(1n).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 1]);
        const recipientEmailAddr = "bob@example.com"
        const expectedRecipientEmailAddrCommit = emailWalletUtils.emailAddrCommitWithSignature(recipientEmailAddr, parsedEmail.signature);
        expect(BigInt(expectedRecipientEmailAddrCommit)).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 2]);
    });

    it("Verify a sent email whose subject has an email address and an invitation code", async () => {
        const emailFilePath = path.join(__dirname, "./emails/email_sender_test5.eml");
        const emailRaw = readFileSync(emailFilePath, "utf8");
        const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
        console.log(parsedEmail.canonicalizedHeader);
        const accountKey = await emailWalletUtils.genAccountKey();
        const circuitInputs = await genEmailSenderInput(emailFilePath, accountKey);
        const circuit = await wasm_tester(path.join(__dirname, "../src/email_sender.circom"), option);
        const witness = await circuit.calculateWitness(circuitInputs);
        await circuit.checkConstraints(witness);
        const domainName = "gmail.com";
        const paddedDomain = emailWalletUtils.padString(domainName, 255);
        const domainFields = emailWalletUtils.bytes2Fields(paddedDomain);
        for (let idx = 0; idx < domainFields.length; ++idx) {
            expect(BigInt(domainFields[idx])).toEqual(witness[1 + idx]);
        }
        const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
        expect(BigInt(expectedPubKeyHash)).toEqual(witness[1 + domainFields.length]);
        const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
        expect(BigInt(expectedEmailNullifier)).toEqual(witness[1 + domainFields.length + 1]);
        const timestamp = 1707866856n;
        expect(timestamp).toEqual(witness[1 + domainFields.length + 2]);
        const maskedSubject = "Send 1.23 ETH to ";
        const paddedMaskedSubject = emailWalletUtils.padString(maskedSubject, 605);
        const maskedSubjectFields = emailWalletUtils.bytes2Fields(paddedMaskedSubject);
        for (let idx = 0; idx < maskedSubjectFields.length; ++idx) {
            expect(BigInt(maskedSubjectFields[idx])).toEqual(witness[1 + domainFields.length + 3 + idx]);
        }
        const senderEmailAddr = "suegamisora@gmail.com";
        const walletSalt = emailWalletUtils.walletSalt(senderEmailAddr, accountKey);
        expect(BigInt(walletSalt)).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length]);
        expect(1n).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 1]);
        const recipientEmailAddr = "alice@gmail.com"
        const expectedRecipientEmailAddrCommit = emailWalletUtils.emailAddrCommitWithSignature(recipientEmailAddr, parsedEmail.signature);
        expect(BigInt(expectedRecipientEmailAddrCommit)).toEqual(witness[1 + domainFields.length + 3 + maskedSubjectFields.length + 2]);
    });

    // it("Verify an email 1 failed in v0", async () => {
    //     const emailFilePath = path.join(__dirname, "./emails/v0_failed_test1.eml");
    //     const emailRaw = readFileSync(emailFilePath, "utf8");
    //     const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
    //     console.log(parsedEmail.canonicalizedHeader);
    //     const accountKey = await emailWalletUtils.genAccountKey();
    //     const circuitInputs = await genEmailSenderInput(emailFilePath, accountKey);
    //     const circuit = await wasm_tester(path.join(__dirname, "../src/email_sender.circom"), option);
    //     const witness = await circuit.calculateWitness(circuitInputs);
    //     await circuit.checkConstraints(witness);
    //     const maskedSubject = "Send 100 USDC to ";
    //     const paddedMaskedSubject = emailWalletUtils.padString(maskedSubject, 512);
    //     const maskedSubjectFields = emailWalletUtils.bytes2Fields(paddedMaskedSubject);
    //     for (let idx = 0; idx < maskedSubjectFields.length; ++idx) {
    //         expect(BigInt(maskedSubjectFields[idx])).toEqual(witness[1 + idx]);
    //     }
    //     const domainName = "gmail.com";
    //     const paddedDomain = emailWalletUtils.padString(domainName, 255);
    //     const domainFields = emailWalletUtils.bytes2Fields(paddedDomain);
    //     for (let idx = 0; idx < domainFields.length; ++idx) {
    //         expect(BigInt(domainFields[idx])).toEqual(witness[18 + idx]);
    //     }
    //     const expectedPubKeyHash = emailWalletUtils.publicKeyHash(parsedEmail.publicKey);
    //     expect(BigInt(expectedPubKeyHash)).toEqual(witness[27]);
    //     const expectedEmailNullifier = emailWalletUtils.emailNullifier(parsedEmail.signature);
    //     expect(BigInt(expectedEmailNullifier)).toEqual(witness[28]);
    //     const timestamp = 1693290766n;
    //     expect(timestamp).toEqual(witness[29]);
    //     const senderEmailAddr = "suegamisora@gmail.com";
    //     const walletSalt = emailWalletUtils.walletSalt(senderEmailAddr, accountKey);
    //     expect(BigInt(walletSalt)).toEqual(witness[30]);
    //     expect(1n).toEqual(witness[31]);
    //     const recipientEmailAddr = "vitalik@ethereum.org"
    //     const expectedRecipientEmailAddrCommit = emailWalletUtils.emailAddrCommitWithSignature(recipientEmailAddr, parsedEmail.signature);
    //     expect(BigInt(expectedRecipientEmailAddrCommit)).toEqual(witness[32]);
    // });

});