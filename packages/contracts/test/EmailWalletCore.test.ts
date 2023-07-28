// @ts-ignore
import { ethers } from "hardhat";
import { Signer, encodeBytes32String, id } from "ethers";
import { expect } from "chai";
import { EmailWalletCore } from "../typechain-types";

/**
 * Units tests use mock verifier - only test logic and flows; not ZK proofs
 */
describe("Email Wallet", function () {
  let coreContract: EmailWalletCore;
  let owner: Signer;
  let relayer: Signer;

  const mockProof = new Uint8Array([1]);
  const relayerHash = id("R1");

  before(async function () {
    [owner, relayer] = await ethers.getSigners();
    const testVerifier = await ethers.deployContract("TestVerifier", owner);

    coreContract = (await ethers.deployContract(
      "EmailWalletCore",
      [testVerifier.getAddress()],
      owner
    )) as EmailWalletCore;

    // Register main relayer used in all other tests
    await coreContract.connect(relayer).registerRelayer(relayerHash);
  });

  describe("Relayer", function () {
    it("should register relayer", async function () {
      // Use a random signer as test relayer
      const testRelayer2 = (await ethers.getSigners())[5];
      const hash = id("R2");

      await coreContract.connect(testRelayer2).registerRelayer(hash);
      expect(await coreContract.relayers(testRelayer2.getAddress())).to.equal(
        hash
      );
    });

    it("should not allow relayer to register twice", async function () {
      const testRelayer2 = (await ethers.getSigners())[5];

      expect(
        coreContract.connect(testRelayer2).registerRelayer(id("R3"))
      ).to.be.revertedWith("relayer already registered");
    });
  });

  describe("Account", function () {
    it("should be able to create a new account", async function () {
      const viewingKey = encodeBytes32String("123456");
      const pointer = encodeBytes32String("100");
      const indicator = encodeBytes32String("200");

      await coreContract
        .connect(relayer)
        .createAccount(viewingKey, pointer, indicator, mockProof);

      expect(await coreContract.indicatorOfPointer(pointer)).to.equal(
        indicator
      );

      expect(await coreContract.relayerOfViewingKey(viewingKey)).to.equal(
        relayerHash
      );
    });
  });
});
