import { verifyDKIMSignature } from "@zk-email/helpers/src/dkim";
import { generateWalletCircuitInputs } from "../helpers/input";
// import snarkjs from "snarkjs";

const path = require("path");
const fs = require("fs");
const wasm_tester = require("circom_tester").wasm;

const F1Field = require("ffjavascript").F1Field;
const Scalar = require("ffjavascript").Scalar;

exports.p = Scalar.fromString(
  "21888242871839275222246405745257275088548364400416034343698204186575808495617"
);

describe("Wallet test", function () {
  jest.setTimeout(10 * 60 * 1000); // 10 minutes

  let dkimResult: import("@zk-email/helpers/src/dkim").DKIMVerificationResult;
  let circuit: any;

  beforeAll(async () => {
    const rawEmail = fs.readFileSync(
      path.join(__dirname, "../../../sample_data/sample_send.eml"),
      "utf8"
    );
    dkimResult = await verifyDKIMSignature(rawEmail);

    circuit = await wasm_tester(path.join(__dirname, "../wallet.circom"), {
      // NOTE: We are running tests against pre-compiled circuit in the below path
      // You need to manually compile when changes are made to circuit if `recompile` is set to `false`.
      recompile: false,
      output: path.join(__dirname, "../build/wallet"),
    });
  });

  it("should verify wallet email", async function () {
    const walletVerifierInputs = generateWalletCircuitInputs({
      rsaSignature: dkimResult.signature,
      rsaModulus: dkimResult.modulus,
      body: dkimResult.body,
      bodyHash: dkimResult.bodyHash,
      message: dkimResult.message,
      nonce: "0x00000000000000000000",
    });

    const witness = await circuit.calculateWitness(walletVerifierInputs);
    await circuit.checkConstraints(witness);
  });
});
