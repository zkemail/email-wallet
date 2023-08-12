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
import { EmailWalletCore, TestERC20 } from "../typechain-types";

const mockProof = new Uint8Array([1]);

/**
 * Units tests use mock verifier - only test logic and flows; not ZK proofs
 */
describe("Email Wallet Contracts > Wallet", function () {
  let coreContract: EmailWalletCore;
  let erc20Contract: TestERC20;
  let owner: Signer;
  let relayer: Signer;
  const domainPublicKeyHash = keccak256(new Uint8Array([4, 3, 6]));

  const senderEmailAddressPointer = encodeBytes32String("SP");
  const senderVKCommitment = encodeBytes32String("SI");
  const recipientEmailAddressPointer = encodeBytes32String("RP");
  const recipientVKCommitment = encodeBytes32String("RI");

  const senderWalletSalt = encodeBytes32String("SND");
  const recipientWalletSalt = encodeBytes32String("RCP");

  before(async function () {
    [owner, relayer] = await ethers.getSigners();
    const testVerifier = await ethers.deployContract("TestVerifier", owner);

    coreContract = (await ethers.deployContract(
      "EmailWalletCore",
      [testVerifier.getAddress()],
      owner,
    )) as EmailWalletCore;

    erc20Contract = (await ethers.deployContract(
      "TestERC20",
      [parseEther("10")],
      owner,
    )) as TestERC20;

    await coreContract.setTokenAddress("TST", erc20Contract.getAddress());

    // Register main relayer used in all other tests
    await coreContract.connect(relayer).registerRelayer(id("R1"));

    await coreContract.setDKIMPublicKeyHash("ethereum.org", domainPublicKeyHash);

    // Create Sender
    await coreContract
      .connect(relayer)
      .createAccount(senderEmailAddressPointer, senderVKCommitment, senderWalletSalt, mockProof);

    await coreContract
      .connect(relayer)
      .initializeAccount(senderEmailAddressPointer, senderVKCommitment, "gmail.com", mockProof);

    // Register recipient
    await coreContract
      .connect(relayer)
      .createAccount(
        recipientEmailAddressPointer,
        recipientVKCommitment,
        recipientWalletSalt,
        mockProof,
      );
  });

  it("should be able to send ETH to another email", async function () {
    const senderAddress = await coreContract.getAddressOfSalt(senderWalletSalt);

    // Transfer some ETH to the sender
    await owner.sendTransaction({
      to: senderAddress,
      value: parseEther("5"),
    });

    expect(await ethers.provider.getBalance(senderAddress)).to.equal(parseEther("5"));

    await coreContract.connect(relayer).executeEmailOp({
      senderEmailAddressPointer: senderEmailAddressPointer,
      senderAccountProof: mockProof,

      hasRecipient: true,
      isRecipientExternal: false,
      recipientRelayer: await relayer.getAddress(),
      recipientEmailAddressPointer: recipientEmailAddressPointer,
      recipientEmailAddressWitness: encodeBytes32String("wtns"),
      recipientExternalAddress: getAddress("0x0000000000000000000000000000000000000000"),
      recipientAccountProof: mockProof,

      command: "Send",
      emailNullifier: encodeBytes32String("NULLIFIER"),
      emailDomain: "ethereum.org",
      maskedSubjectStr: "Send 2 ETH to ",
      amount: parseEther("2"),
      tokenName: "ETH",
      emailProof: mockProof,
    });

    expect(await ethers.provider.getBalance(senderAddress)).to.equal(
      parseEther("3"), // 5 - 2
    );
  });

  it("should be able to send ERC20 to another email", async function () {
    const senderAddress = await coreContract.getAddressOfSalt(senderWalletSalt);

    await erc20Contract.transfer(senderAddress, parseEther("5"));

    expect(await erc20Contract.balanceOf(senderAddress)).to.equal(parseEther("5"));

    await coreContract.connect(relayer).executeEmailOp({
      senderEmailAddressPointer: senderEmailAddressPointer,
      senderAccountProof: mockProof,

      hasRecipient: true,
      isRecipientExternal: false,
      recipientRelayer: await relayer.getAddress(),
      recipientEmailAddressPointer: recipientEmailAddressPointer,
      recipientEmailAddressWitness: encodeBytes32String("wtns"),
      recipientExternalAddress: getAddress("0x0000000000000000000000000000000000000000"),
      recipientAccountProof: mockProof,

      command: "Send",
      emailNullifier: encodeBytes32String("NULLIFIER2"),
      emailDomain: "ethereum.org",
      maskedSubjectStr: "Send 2 TST to ",
      amount: parseEther("2"),
      tokenName: "TST",
      emailProof: mockProof,
    });

    expect(await erc20Contract.balanceOf(senderAddress)).to.equal(
      parseEther("3"), // 5 - 2
    );
  });
});
