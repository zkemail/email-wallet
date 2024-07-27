const ff = require("ffjavascript");
const emailWalletUtils = require("@zk-email/relayer-utils");
import { hash_to_curve, point_scalar_mul } from "circom-grumpkin";
import { genPsiPointsInput } from "../helpers/psi_points";
const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import * as path from "path";
const option = {
  include: path.join(__dirname, "../../../node_modules"),
};

jest.setTimeout(120000);
describe("PSI point", () => {
  let circuit: any;
  beforeAll(async () => {
    circuit = await wasm_tester(path.join(__dirname, "../src/psi_points.circom"), option);
  });

  it("psi point calculation test", async () => {
    const emailAddr = "suegamisora@gmail.com";
    const paddedEmailAddr = emailWalletUtils.padEmailAddr(emailAddr);
    const hashedPoint = hash_to_curve(paddedEmailAddr);
    const relayerRand = emailWalletUtils.genRelayerRand();
    const clientRand = emailWalletUtils.genRelayerRand();
    const requestPoint = point_scalar_mul(hashedPoint.x, hashedPoint.y, BigInt(clientRand));
    const responsePoint = point_scalar_mul(requestPoint.x, requestPoint.y, BigInt(relayerRand));
    const scalarField = new ff.F1Field("21888242871839275222246405745257275088696311157297823662689037894645226208583");
    const invClientRand = scalarField.inv(BigInt(clientRand));
    const invedPoint = point_scalar_mul(responsePoint.x, responsePoint.y, invClientRand);
    const psiPoint = point_scalar_mul(hashedPoint.x, hashedPoint.y, BigInt(relayerRand));
    expect(psiPoint.x).toEqual(invedPoint.x);
    expect(psiPoint.y).toEqual(invedPoint.y);
  });

  it("psi circuits test", async () => {
    const emailAddr = "suegamisora@gmail.com";
    const accountCode = emailWalletUtils.genAccountCode();
    const relayerRand = emailWalletUtils.genRelayerRand();
    const circuitInputs = await genPsiPointsInput(emailAddr, accountCode, relayerRand);
    const witness = await circuit.calculateWitness(circuitInputs);
    await circuit.checkConstraints(witness);
    const expectedAccountSalt = emailWalletUtils.accountSalt(emailAddr, accountCode);
    expect(BigInt(expectedAccountSalt)).toEqual(witness[1]);
    const paddedEmailAddr = emailWalletUtils.padEmailAddr(emailAddr);
    const hashedPoint = hash_to_curve(paddedEmailAddr);
    const psiPoint = point_scalar_mul(hashedPoint.x, hashedPoint.y, BigInt(relayerRand));
    expect(psiPoint.x).toEqual(witness[2]);
    expect(psiPoint.y).toEqual(witness[3]);
  });

});
