// @ts-ignore
import { groth16, zKey } from "snarkjs";
import https from "https";
import fs from "fs";
import path from "path";

let { ZKEY_ENTROPY, ZKEY_BEACON } = process.env;

const phase1Url = "https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_22.ptau";
const buildDir = path.join(__dirname, "../build");
const phase1Path = path.join(buildDir, "powersOfTau28_hez_final_22.ptau");
const r1cPath = path.join(buildDir, "wallet.r1cs");

// Output paths
const zKeyPath = path.join(buildDir, "wallet.zkey");
const vKeyPath = path.join(buildDir, "vkey.json");

function log(...message: any) {
  console.log('\n', ...message);
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

  log("✓ Phase 1:", phase1Path);
}

async function exec() {
  await downloadPhase1();  

  await zKey.newZKey(r1cPath, phase1Path, path.join(buildDir, "wallet_0.zkey"), console);
  log("✓ Partial ZKey generated", path.join(buildDir, "wallet_0.zkey"));

  await zKey.contribute(path.join(buildDir, "wallet_0.zkey"), path.join(buildDir, "wallet_1.zkey"), "Contributer 1", ZKEY_ENTROPY);
  log("✓ First contribution completed");

  await zKey.beacon(path.join(buildDir, "wallet_1.zkey"), zKeyPath, "Final Beacon", ZKEY_BEACON, 10);
  log("✓ Beacon applied");

  await zKey.verifyFromR1cs(path.join(buildDir, "wallet.r1cs"), phase1Path, zKeyPath);
  log(`✓ Final ZKey verified - ${zKeyPath}`);

  await zKey.exportVerificationKey(zKeyPath, vKeyPath);
  log(`✓ Verification key exported - ${vKeyPath}`);

  await zKey.exportSolidityVerifier(zKeyPath, path.join(buildDir, "verifier.sol"));
  log(`✓ Solidity verifier exported - ${path.join(buildDir, "verifier.sol")}`);
}

exec().catch((err) => {
  console.log("Error: ", err);
  process.exit(1);
});