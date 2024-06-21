const circom_tester = require("circom_tester");
const wasm_tester = circom_tester.wasm;
import * as path from "path";
const emailWalletUtils = require("@zk-email/relayer-utils");
const option = {
  include: path.join(__dirname, "../../../node_modules"),
};
import { genAnnouncementInput } from "../helpers/announcement";

// const grumpkin = require("circom-grumpkin");
jest.setTimeout(120000);
describe("Announcement", () => {
  it("announce a randomness and an email address for the email address commitment", async () => {
    const emailAddr = "suegamisora@gmail.com";
    const emailAddrRand = emailWalletUtils.emailAddrCommitRand();
    console.log(emailAddrRand);
    const circuitInputs = await genAnnouncementInput(emailAddr, emailAddrRand);
    const circuit = await wasm_tester(path.join(__dirname, "../src/announcement.circom"), option);
    const witness = await circuit.calculateWitness(circuitInputs);
    await circuit.checkConstraints(witness);
    // expect(expectedRelayerRandHash).toEqual("0x" + witness[1].toString(16));
    const paddedEmailAddr = emailWalletUtils.padString(emailAddr, 256);
    const emailAddrFields = emailWalletUtils.bytes2Fields(paddedEmailAddr);
    for (let idx = 0; idx < emailAddrFields.length; ++idx) {
      expect(BigInt(emailAddrFields[idx])).toEqual(witness[1 + idx]);
    }
    const expectedEmailAddrCommit = emailWalletUtils.emailAddrCommit(emailAddr, emailAddrRand);
    console.log(expectedEmailAddrCommit);
    expect(BigInt(expectedEmailAddrCommit)).toEqual(witness[1 + emailAddrFields.length]);
    expect(BigInt(emailAddrRand)).toEqual(witness[2 + emailAddrFields.length]);
  });
});
