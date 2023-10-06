/**
 * 
 * This script is for generating input for the account creation circuit.
 * 
 */


import { program } from "commander";
import fs from "fs";
import { promisify } from "util";
import { genClaimInput } from "../helpers/claim";

program
  .requiredOption(
    "--email-addr <string>",
    "User's email address"
  )
  .requiredOption(
    "--relayer-rand <string>",
    "Relayer's randomness"
  )
  .requiredOption(
    "--email-addr-rand <string>",
    "Randomness for the email address commitment"
  )
  .requiredOption(
    "--output <string>",
    "Path of a json file to write the generated inputs"
  )
  .option("--silent", "No console logs");

program.parse();
const args = program.opts();

function log(...message: any) {
  if (!args.silent) {
    console.log(...message);
  }
}

async function generate() {
  if (!args.output.endsWith(".json")) {
    throw new Error("--output path arg must end with .json");
  }

  log("Generating Inputs for:", args);

  const circuitInputs = await genClaimInput(args.emailAddr, args.relayerRand, args.emailAddrRand);

  log("\n\nGenerated Inputs:", circuitInputs, "\n\n");

  await promisify(fs.writeFile)(args.output, JSON.stringify(circuitInputs, null, 2));

  log("Inputs written to", args.output);
}

generate().catch((err) => {
  console.error("Error generating inputs", err);
  process.exit(1);
});
