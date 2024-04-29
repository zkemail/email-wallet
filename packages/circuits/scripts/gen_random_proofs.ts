/**
 * 
 * This script is for generating proofs for the random inputs.
 * 
 */


import { program } from "commander";
import fs from "fs";
import { promisify } from "util";
import path from "path";
import { genAccountCreationInput } from "../helpers/account_creation";
import { genClaimInput } from "../helpers/claim";
import { genEmailSenderInput } from "../helpers/email_sender";
import { genAnnouncementInput } from "../helpers/announcement";
const snarkjs = require("snarkjs");
const emailWalletUtils = require("../../utils");

program
    .requiredOption(
        "--output <string>",
        "Path to the directory storing output files"
    )
    .option("--silent", "No console logs");

program.parse();
const args = program.opts();

function log(...message: any) {
    if (!args.silent) {
        console.log(...message);
    }
}

async function prove(input: any, wasmFile: string, zkeyFileName: string, proofPath: string, pubInputsPath: string) {
    const { proof, publicSignals } = await snarkjs.groth16.fullProve(input, wasmFile, zkeyFileName, console);
    await promisify(fs.writeFile)(proofPath, JSON.stringify(proof, null, 2));
    await promisify(fs.writeFile)(pubInputsPath, JSON.stringify(publicSignals, null, 2));
}

async function exec() {
    const buildDir = args.output;
    // const phase1Path = path.join(buildDir, "powersOfTau28_hez_final_22.ptau");
    const emailAddr = "suegamisora@gmail.com";
    const relayerRand = emailWalletUtils.genRelayerRand();
    const relayerRandHash = emailWalletUtils.relayerRandHash(relayerRand);
    const newRelayerRand = emailWalletUtils.genRelayerRand();
    // const newRelayerRandHash = emailWalletUtils.relayerRandHash(newRelayerRand);
    const accountKey = emailWalletUtils.genAccountKey();
    const emailAddrRand = emailWalletUtils.emailAddrCommitRand();
    const initEmailPath = path.join(__dirname, "../tests/emails/account_init_test1.eml");
    const emailSenderEmailPath = path.join(__dirname, "../tests/emails/email_sender_test1.eml");

    const accountCreationInput = await genAccountCreationInput(emailAddr, relayerRand);
    await promisify(fs.writeFile)(path.join(buildDir, "account_creation_input.json"), JSON.stringify(accountCreationInput, null, 2));
    await prove(accountCreationInput, path.join(buildDir, "account_creation_js", "account_creation.wasm"), path.join(buildDir, "account_creation.zkey"), path.join(buildDir, "account_creation_proof.json"), path.join(buildDir, "account_creation_public.json"));
    log("✓ Proof for account creation circuit generated");

    const claimInput = await genClaimInput(emailAddr, emailAddrRand, accountKey);
    await promisify(fs.writeFile)(path.join(buildDir, "claim_input.json"), JSON.stringify(claimInput, null, 2));
    await prove(claimInput, path.join(buildDir, "claim_js", "claim.wasm"), path.join(buildDir, "claim.zkey"), path.join(buildDir, "claim_proof.json"), path.join(buildDir, "claim_public.json"));
    log("✓ Proof for claim circuit generated");

    const emailSenderInput = await genEmailSenderInput(emailSenderEmailPath, accountKey);
    await promisify(fs.writeFile)(path.join(buildDir, "email_sender_input.json"), JSON.stringify(emailSenderInput, null, 2));
    await prove(emailSenderInput, path.join(buildDir, "email_sender_js", "email_sender.wasm"), path.join(buildDir, "email_sender.zkey"), path.join(buildDir, "email_sender_proof.json"), path.join(buildDir, "email_sender_public.json"));
    log("✓ Proof for email sender circuit generated");

    const announcementInput = await genAnnouncementInput(emailAddr, emailAddrRand);
    await promisify(fs.writeFile)(path.join(buildDir, "announcement_input.json"), JSON.stringify(announcementInput, null, 2));
    await prove(announcementInput, path.join(buildDir, "announcement_js", "announcement.wasm"), path.join(buildDir, "announcement.zkey"), path.join(buildDir, "announcement_proof.json"), path.join(buildDir, "announcement_public.json"));
    log("✓ Proof for email sender circuit generated");


}


exec()
    .then(() => {
        process.exit(0);
    })
    .catch((err) => {
        console.log("Error: ", err);
        process.exit(1);
    });
