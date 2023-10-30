// @ts-ignore
import { ethers } from "hardhat";
import { Signer, encodeBytes32String, id } from "ethers";
import { expect } from "chai";
import { EmailWalletCore } from "../typechain-types";

const mockProof = new Uint8Array([1]);

/**
 * Units tests use mock verifier - only test logic and flows; not ZK proofs
 */
describe("Email Wallet Contracts > Account", function () {
  let coreContract: EmailWalletCore;
  let owner: Signer;
  let relayer: Signer;

  before(async function () {
    [owner, relayer] = await ethers.getSigners();
    const testVerifier = await ethers.deployContract("TestVerifier", owner);

    coreContract = (await ethers.deployContract(
      "EmailWalletCore",
      [testVerifier.getAddress()],
      owner
    )) as EmailWalletCore;

    // Register main relayer used in all other tests
    await coreContract.connect(relayer).registerRelayer(id("R1"));
  });

  it("should be able to create a new account", async function () {
    const pointer = encodeBytes32String("100");
    const indicator = encodeBytes32String("200");

    await coreContract
      .connect(relayer)
      .createAccount(pointer, indicator, mockProof);

    expect(await coreContract.indicatorOfPointer(pointer)).to.equal(
      indicator
    );
  });

  it("should not allow creating two accounts for same pointer", async function () {
    const pointer = encodeBytes32String("1000");
    const indicator1 = encodeBytes32String("2001");
    const indicator2 = encodeBytes32String("2002");

    await coreContract
      .connect(relayer)
      .createAccount(pointer, indicator1, mockProof);

    expect(
      coreContract.connect(relayer).createAccount(pointer, indicator2, mockProof)
    ).to.be.revertedWith("account already exists");
  });
});
