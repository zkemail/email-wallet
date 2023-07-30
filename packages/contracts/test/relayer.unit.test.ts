// @ts-ignore
import { ethers } from "hardhat";
import { Signer, id } from "ethers";
import { expect } from "chai";
import { EmailWalletCore } from "../typechain-types";

/**
 * Units tests use mock verifier - only test logic and flows; not ZK proofs
 */
describe("Email Wallet Contracts > Relayer", function () {
  let coreContract: EmailWalletCore;
  let owner: Signer;

  before(async function () {
    [owner] = await ethers.getSigners();
    const testVerifier = await ethers.deployContract("TestVerifier", owner);

    coreContract = (await ethers.deployContract(
      "EmailWalletCore",
      [testVerifier.getAddress()],
      owner
    )) as EmailWalletCore;
  });

  it("should register relayer", async function () {
    // Use a random signer as test relayer
    const testRelayer = (await ethers.getSigners())[5];
    const hash = id("R2");

    await coreContract.connect(testRelayer).registerRelayer(hash);
    expect(await coreContract.relayers(testRelayer.getAddress())).to.equal(
      hash
    );
  });

  it("should not allow relayer to register twice", async function () {
    const testRelayer2 = (await ethers.getSigners())[6];

    coreContract.connect(testRelayer2).registerRelayer(id("R9"));

    expect(
      coreContract.connect(testRelayer2).registerRelayer(id("R9"))
    ).to.be.revertedWith("relayer already registered");
  });
});
