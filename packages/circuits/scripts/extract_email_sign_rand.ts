/**
 *
 * This script is for generating zKey and Verification Key for the circuit.
 * Running this will download the phase1 file (if not already present),
 * generate the zKey, and also export solidity and json verification keys.
 *
 * Running this will overwrite any existing zKey and verification key files.
 *
 */

// @ts-ignore
import { zKey } from "snarkjs";
import https from "https";
import fs from "fs";
import path from "path";
import process from "process";
import { promisify } from "util";
import { program } from "commander";
const emailWalletUtils = require("../../utils");


program
    .requiredOption(
        "--email-file <string>",
        "Path to an email file"
    )
    .requiredOption(
        "--output <string>",
        "Path of a ouput file to write the randomness"
    );

program.parse();
const args = program.opts();

async function exec() {
    const emailRaw = await promisify(fs.readFile)(args.emailFile, "utf8");
    const parsedEmail = await emailWalletUtils.parseEmail(emailRaw);
    const rand = emailWalletUtils.extractRandFromSignature(parsedEmail.signature);
    fs.writeFileSync(args.output, rand);
}


exec()
    .then(() => {
        process.exit(0);
    })
    .catch((err) => {
        console.log("Error: ", err);
        process.exit(1);
    });
