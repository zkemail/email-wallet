# ZK Email Wallet

A smart contract wallet controlled using emails.

ZK Email Wallet (ZKEW) is an Ethereum wallet that can be controlled using your email account. It is a smart contract wallet that uses zk-SNARKs to verify the validity of the email messages. The wallet is non-custodial and does not require any trusted third parties.

In the current version, you can:
- ✓ Send ETH to email address.
- ✓ Send ERC20 to email address.
- ✓ Email address to wallet link not exposed publicly.

In the future, you will be able to:
- ✓ Swap tokens using Uniswap.
- ✓ Sender don't learn about all your transactions history (UTXO like model).

<br />

## ☞ How it works

Emails you send are (usually) signed using a private key controlled by your email provider. This signature, also known as DKIM is included in the headers section of the email.

DKIM signature is used to verify the email was sent by you and lets you securely control your wallet. Instead of verifying the signature directly on-chain, a zk proof of signature is created (by a permissionless entity called Relayer) and verified on-chain.

Here is how a typical interaction with the wallet looks like:

- You send an email to your Relayer's email address with a subject like "Send 1 ETH to recipient@gmail.com".
- The Relayer verifies the DKIM signature and creates a zk proof of email.
- The ZK circuit also validate the subject regex, recipients email address and the amount to be sent. 
- Private data like the sender email, recipient's email in the subject line, is  not exposed on-chain.
- The Relayer create an Ethereum transaction that reflect the intended action in the subject.
- The smart contract verify the ZK Proof and ensure the subject line matches the transaction.
- The smart contract executes the transaction and sends 1 ETH to recipient's Email Wallet address.
- The Relayer wait for the transaction confirmation, and send you and the recipient and email with the transaction details.

<br />

## ☞ How to use

During the test period, we (a Relayer we are running) will create an account and mint some `TEST` tokens for you. 

You can get started by sending an email to `relayer@sendeth.org` with a subject like `Send 1 TEST to someone@gmail.com`.

<br />

## ☞ Building and Running locally

This is a mono-repo that contains the circom circuits, smart contracts, the relayer.

### Directory structure

```

/packages
├── /circuits  
├──── /src              # Circom circuits.
├──── /build            # Compiled circuits.
├──── /hepers           # Helper functions to generate circuit input.
├──── /scripts          # CLI scripts to generate input, zkeys.
├──── /test             # Circom tests for circuit
├
├── /contracts
├──── /src              # Solidity contracts.
├──── /test             # Solidity tests for contracts.
├──── /script           # Scripts to deploy wallet.
├
├── /relayer
├──── /src              # Relayer code.
├──── /db               # Embedded database to store data.
├──── /received_emls    # Received eml files saved here.
├──── /proofs           # Generated inputs, witness and proof saved here.
├──── /coordinator.py   # Coordinator script that watch for emls and trigger proofgen.sh
├──── /proofgen.sh      # Script to generate proof and call relayer.

```

### Building 

