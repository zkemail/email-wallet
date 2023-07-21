import { program } from "commander";
import fs from "fs";
import path from "path";
import { generateWalletCircuitInputsFromEmail } from "../helpers/input";

program
  .requiredOption(
    "--email-file <string>",
    "Full path to the .eml file for generating inputs"
  )
  .requiredOption(
    "--output <string>",
    "Path of input.json to write the generated inputs"
  )
  .requiredOption("--nonce <string>", "Nonce")
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

  const emailBuffer = fs.readFileSync(args.emailFile);

  const circuitInputs = await generateWalletCircuitInputsFromEmail(
    emailBuffer,
    args.nonce
  );

  log("\n\nGenerated Inputs:", circuitInputs, "\n\n");

  fs.writeFileSync(args.output, JSON.stringify(circuitInputs, null, 2));

  log("Inputs written to", args.output);
}

generate().catch((err) => {
  console.error("Error generating inputs", err);
  process.exit(1);
});
