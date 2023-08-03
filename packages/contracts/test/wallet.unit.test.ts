// @ts-ignore
import { ethers } from "hardhat";
import {
  AbiCoder,
  Signer,
  encodeBytes32String,
  formatEther,
  getAddress,
  id,
  keccak256,
  parseEther,
  stripZerosLeft,
} from "ethers";
import { expect } from "chai";
import { EmailWalletCore } from "../typechain-types";

const mockProof = new Uint8Array([1]);

/**
 * Units tests use mock verifier - only test logic and flows; not ZK proofs
 */
describe("Email Wallet Contracts > Wallet", function () {
  let coreContract: EmailWalletCore;
  let owner: Signer;
  let relayer: Signer;
  const domainPublicKeyHash = keccak256(new Uint8Array([4, 3, 6]));

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

    await coreContract.setDKIMPublicKeyHash(
      "ethereum.org",
      domainPublicKeyHash
    );
  });

  it("should be able to send ETH to another email", async function () {
    // Create sender and initialize it
    const senderPointer = encodeBytes32String("SP");
    const senderIndicator = encodeBytes32String("SI");
    const recipientPointer = encodeBytes32String("RP");
    const recipientIndicator = encodeBytes32String("RI");

    await coreContract
      .connect(relayer)
      .createAccount(senderPointer, senderIndicator, mockProof);
    await coreContract
      .connect(relayer)
      .initializeAccount(senderPointer, senderIndicator, mockProof);

    // Register recipient
    await coreContract
      .connect(relayer)
      .createAccount(recipientPointer, recipientIndicator, mockProof);

    await coreContract.connect(relayer).executeEmailOp({
      senderPointer,
      senderIndicator,
      hasRecipient: true,
      isRecipientExternal: false,
      isRecipientInitialized: false,
      recipientRelayer: await relayer.getAddress(),
      recipientPointer,
      recipientIndicator,
      recipientExternalAddress: getAddress(
        "0x0000000000000000000000000000000000000000"
      ),
      command: "Send",
      emailNullifier: encodeBytes32String("NULLIFIER"),
      dkimPublicKeyHash: domainPublicKeyHash,
      domainName: "ethereum.org",
      maskedSubjectStr: "Send 1 ETH to ",
      emailProof: mockProof,
      amount: parseEther("1"),
      tokenName: "ETH",
    });
  });
});
