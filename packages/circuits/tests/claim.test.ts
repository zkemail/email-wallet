const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import * as path from "path";
const emailWalletUtils = require("@zk-email/relayer-utils");
const option = {
  include: path.join(__dirname, "../../../node_modules"),
};
import { genClaimInput } from "../helpers/claim";

// const grumpkin = require("circom-grumpkin");
jest.setTimeout(120000);
describe("Claim", () => {
  it("claim an unclaimed funds/states", async () => {
    const emailAddr = "suegamisora@gmail.com";
    const emailAddrRand = emailWalletUtils.emailAddrCommitRand();
    const accountCode = emailWalletUtils.genAccountCode();
    const circuitInputs = await genClaimInput(emailAddr, emailAddrRand, accountCode);
    const circuit = await wasm_tester(path.join(__dirname, "../src/claim.circom"), option);
    const witness = await circuit.calculateWitness(circuitInputs);
    await circuit.checkConstraints(witness);
    const expectedEmailAddrCommit = emailWalletUtils.emailAddrCommit(emailAddr, emailAddrRand);
    expect(BigInt(expectedEmailAddrCommit)).toEqual(witness[1]);
    const accountSalt = emailWalletUtils.accountSalt(emailAddr, accountCode);
    expect(BigInt(accountSalt)).toEqual(witness[2]);
  });
});
