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

let { ZKEY_ENTROPY, ZKEY_BEACON } = process.env;

const phase1Url =
  "https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_22.ptau";
const buildDir = path.join(__dirname, "../build");
const phase1Path = path.join(buildDir, "powersOfTau28_hez_final_22.ptau");
const r1cPath = path.join(buildDir, "wallet.r1cs");
const solidityTemplate = path.join(
  require.resolve("snarkjs"),
  "../../templates/verifier_groth16.sol.ejs"
);

// Output paths
const zKeyPath = path.join(buildDir, "wallet.zkey");
const vKeyPath = path.join(buildDir, "vkey.json");
const solidityVerifierPath = path.join(buildDir, "verifier.sol");

function log(...message: any) {
  console.log(`\n`, ...message, "\n");
}

async function askBeacon() {
  if (!ZKEY_BEACON) {
    ZKEY_BEACON = await new Promise((resolve) => {
      const readline = require("readline").createInterface({
        input: process.stdin,
        output: process.stdout,
      });
      readline.question(
        "Enter Beacon (hex string) to apply: ",
        (entropy: string) => {
          readline.close();
          resolve(entropy);
        }
      );
    });
  }
}

async function downloadPhase1() {
  if (!fs.existsSync(phase1Path)) {
    log(`✘ Phase 1 not found at ${phase1Path}`);
    log(`䷢ Downloading Phase 1`);

    const phase1File = fs.createWriteStream(phase1Path);

    return new Promise((resolve, reject) => {
      https
        .get(phase1Url, (response) => {
          response.pipe(phase1File);
          phase1File.on("finish", () => {
            phase1File.close();
            resolve(true);
          });
        })
        .on("error", (err) => {
          fs.unlink(phase1Path, () => {});
          reject(err);
        });
    });
  }
}

async function exec() {
  await downloadPhase1();
  log("✓ Phase 1:", phase1Path);

  await zKey.newZKey(r1cPath, phase1Path, path.join(buildDir, "wallet_0.zkey"), console);
  log("✓ Partial ZKey generated");

  await zKey.contribute(path.join(buildDir, "wallet_0.zkey"), path.join(buildDir, "wallet_1.zkey"), "Contributer 1", ZKEY_ENTROPY, console);
  log("✓ First contribution completed");

  await askBeacon();
  await zKey.beacon(path.join(buildDir, "wallet_1.zkey"), zKeyPath, "Final Beacon", ZKEY_BEACON, 10, console);
  log("✓ Beacon applied");

  await zKey.verifyFromR1cs(path.join(buildDir, "wallet.r1cs"), phase1Path, zKeyPath, console);
  log(`✓ Final ZKey verified - ${zKeyPath}`);

  const vKey = await zKey.exportVerificationKey(zKeyPath, console);
  fs.writeFileSync(vKeyPath, JSON.stringify(vKey, null, 2));
  log(`✓ Verification key exported - ${vKeyPath}`);

  const templates = {
    groth16: fs.readFileSync(solidityTemplate, "utf8"),
  };
  const code = await zKey.exportSolidityVerifier(zKeyPath, templates, console);
  fs.writeFileSync(solidityVerifierPath, code);
  log(`✓ Solidity verifier exported - ${solidityVerifierPath}`);
}

exec()
  .then(() => {
    process.exit(0);
  })
  .catch((err) => {
    console.log("Error: ", err);
    process.exit(1);
  });
