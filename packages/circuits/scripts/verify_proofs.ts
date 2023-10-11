/**
 * 
 * This script is for verifying proofs
 * 
 */


import { program } from "commander";
import fs from "fs";
import { promisify } from "util";
import path from "path";
const snarkjs = require("snarkjs");

program
    .requiredOption(
        "--input <string>",
        "Path to the directory storing input files"
    )
    .option("--silent", "No console logs");

program.parse();
const args = program.opts();

function log(...message: any) {
    if (!args.silent) {
        console.log(...message);
    }
}

async function verify(vkPath: string, proofPath: string, pubInputsPath: string) {
    const vk = await promisify(fs.readFile)(vkPath);
    const proof = await promisify(fs.readFile)(proofPath);
    const pubInputs = await promisify(fs.readFile)(pubInputsPath);
    console.log(await snarkjs.groth16.verify(JSON.parse(vk.toString()), JSON.parse(pubInputs.toString()), JSON.parse(proof.toString()), console));
    // const { proof, publicSignals } = await snarkjs.groth16.fullProve(input, wasmFile, zkeyFileName, console);
    // await promisify(fs.writeFile)(proofPath, JSON.stringify(proof, null, 2));
    // await promisify(fs.writeFile)(pubInputsPath, JSON.stringify(publicSignals, null, 2));
}

async function exec() {
    const buildDir = args.input;
    // const phase1Path = path.join(buildDir, "powersOfTau28_hez_final_22.ptau");
    await verify(path.join(buildDir, "account_creation.vkey"), path.join(buildDir, "account_creation_proof.json"), path.join(buildDir, "account_creation_public.json"));
    log("✓ Proof for account creation circuit verified");

    await verify(path.join(buildDir, "account_init.vkey"), path.join(buildDir, "account_init_proof.json"), path.join(buildDir, "account_init_public.json"));
    log("✓ Proof for account initialization circuit verified");

    await verify(path.join(buildDir, "account_transport.vkey"), path.join(buildDir, "account_transport_proof.json"), path.join(buildDir, "account_transport_public.json"));
    log("✓ Proof for account transport circuit verified");

    await verify(path.join(buildDir, "claim.vkey"), path.join(buildDir, "claim_proof.json"), path.join(buildDir, "claim_public.json"));
    log("✓ Proof for claim circuit verified");

    await verify(path.join(buildDir, "email_sender.vkey"), path.join(buildDir, "email_sender_proof.json"), path.join(buildDir, "email_sender_public.json"));
    log("✓ Proof for email sender circuit verified");


}


exec()
    .then(() => {
        process.exit(0);
    })
    .catch((err) => {
        console.log("Error: ", err);
        process.exit(1);
    });
