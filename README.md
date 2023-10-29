# Email Wallet

A smart contract wallet controlled using emails. Try the demo on emailwallet.org.

Email Wallet is an Ethereum wallet that can be controlled by sending an email. It is a smart contract wallet that uses zk-SNARKs to verify the validity of the email messages. Assets in your email wallet remain secure as long as your email domain server of your email account (such as Gmail) does not forge your emails.

This was first proposed at [ICBC2023](https://speakerdeck.com/sorasuegami/icbc2023-contract-wallet-using-emails)[1].

In the current version, you can:
- ✓ Send ETH to email address.
- ✓ Send ERC20 to email address.
- ✓ Hide a link from an email address to a wallet address on-chain.

In the future, you will be able to:
- ✓ Swap tokens using Uniswap, and use other DeFi using extensions.
- ✓ Sender don't learn about all your transactions history (UTXO like model).

<br />

## ☞ How it works

Emails you send are (usually) signed using a private key controlled by your email domain server according to a DKIM protocol. This signature is included in the headers section of the email.

DKIM signature is used to verify the email was sent by you and lets you securely control your wallet. Instead of verifying the signature directly on-chain, a zk proof of signature is created (by a permissionless entity called Relayer) and verified on-chain.

Here is how a typical interaction with the wallet looks like:

- You send an email to your Relayer's email address with a subject like "Send 1 ETH to recipient@gmail.com".
- The Relayer verifies the DKIM signature and creates a zk proof of email.
- The ZK circuit also validate the subject regex, recipients email address and the amount to be sent.
- Private data like the sender email, recipient's email in the subject line, is  not exposed on-chain.
- The Relayer create an Ethereum transaction that reflect the intended action in the subject.
- The smart contract verify the ZK Proof and ensure the subject line matches the transaction.
- The smart contract executes the transaction and sends 1 ETH to recipient's Email Wallet address.
- The Relayer wait for the transaction confirmation, and send you and the recipient an email with the transaction details.

<br />

## Security Assumptions 
1. Safety guarantee: if your email domain server does not forge your emails, no one can send assets in your wallet.
2. Liveness guarantee: if your email domain server does not block your emails and you stores an invitation email that you have received before, you can execute a new transaction on-chain.
3. Privacy protection against DoS scanning attacks: no adversary can learn an address or existence of the wallet for your email address for free.

If you self-host a relayer, you can ensure complete privacy.

<br />

## ☞ How to use

During the test period, we (a Relayer we are running) will create an account and mint some `TEST` tokens for you. 

You can get started by sending an email to `relayer@sendeth.org` with a subject like `Send 1 TEST to someone@gmail.com`.

<br />

## ☞ Directory structure
This is a mono-repo that contains the circom circuits, smart contracts, the relayer.

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

<br />

## ☞ Building

#### Requirements
- NodeJS + Yarn (for monorepo, circuits)
- Rust + Cargo  (for relayer)
- Foundry       (for contracts)
- Python 3      (relayer orchestration script)
- Docker        (for Rapidsnark if local setup is not working)


#### To build the circuits:
```bash
cd packages/circuits
yarn build
```

Make sure to set the zkey entropy in a .env based on the .env.example. Then, you can generate zkeys and vkeys for the circuit using:
```bash
yarn generate-keys    # in packages/circuits
```

This will generate `wallet.zkey`, `vkey.json` and the solidity verifier `verifier.sol` files in `packages/circuits/build` directory.

Of course, this is only for local testing. When running on production, you should use the `zkey` generated by the trusted setup ceremony.


#### To build the contracts:
```bash
cd packages/contracts
yarn build
```

Note that the contracts already have hardcoded verifications keys. You should replace the keys with the ones from `vkey.json` you generated if you intend to test locally.


#### To build the relayer:
```bash
cd packages/relayer
cargo build   # output binary is in target/debug/relayer
```

<br />

#### To build the docs

To run you should:
```
cargo install mdbook
```

And to serve the website, go to docs/ then serve:

```
cd docs
mdbook serve
```

## ☞ Running Locally

1. Firstly, start a local ETH testnet. For example `anvil`
```bash
anvil --chain-id 5
```

2. Set private key of an account in the testnet (with sufficient balance) in `packages/contracts/scripts/DeployWallet.s.sol` file.
The value already in there is the default private key of the first account in `anvil` and `hardhat` network.

3. Deploy the wallet contract to the testnet.
```bash
forge script script/DeployWallet.s.sol:Deploy -vvvv --rpc-url http://localhost:8545 --broadcast

# You will see the address of the deployed wallet contract in the output. It will be the first out of the 5 addresses printed. Example: 
# == Return ==
# 0: address 0xa513E6E4b8f2a923D98304ec87F64353C4D5C853
```

4. Create a `.env` file in `packages/relayer` by taking a copy from `.env.example`.
```bash
cp packages/relayer/.env.example packages/relayer/.env
```

5. Update the `.env` file:
```bash
CONTRACT_ADDRESS=0x           # Address of the deployed wallet contract.
PRIVATE_KEY=                  # Private key for Relayer's account.
RPC_URL=http://localhost:8545
CHAIN_ID=5                    # Chain ID of the testnet.
INCOMING_EML_PATH=            # Full path to the relayer/received_eml directory.

# IMAP + SMTP (Settings will be provided by your email provider)
IMAP_DOMAIN_NAME=imap.gmail.com
IMAP_PORT=993
AUTH_TYPE=password
SMTP_DOMAIN_NAME=smtp.gmail.com
LOGIN_ID=                     # IMAP login id - usually your email address.
LOGIN_PASSWORD=               # IMAP password - usually your email password.

PROVER_LOCATION=local         # Keep this local for running the prover locally.
```

6. Ensure the contract ABIs are up to date in `packages/relayer/abi/` directory.

7. Start the relayer:
```bash
cd packages/relayer
cargo run relayer # You can also run the built binary in target/debug/relayer
```

8. Relayer will watch for emails in the configured account and save the `eml` files in `packages/relayer/received_eml` directory after validations. Proofs need to be generated for these emails, and then relayer need to be called with `chain` command to submit the proofs to the contract and make the transaction. 
`proofgen.sh` will do this for you.
```bash
# Example
./proofgen.sh ${NONCE_OF_THE_EMAIL}
```

9. Proofs are generated using Rapidsnark. You can setup Rapidsnark locally or use docker.

- To setup Rapidsnark locally, follow the instructions in the [official repo](https://github.com/iden3/rapidsnark#compile-prover-in-standalone-mode)
- To use docker, build rapidsnark using the Dockerfile in the repo.
```bash
docker build -f libs/rapidsnark.Dockerfile -t rapidsnark .
``````
- You can set (comment/uncomment) the `RAPIDSNARK_PATH` in `proofgen.sh` to use docker or local version.

9. To watch for additions to `received_eml` dir and trigger `proofgen.sh` automatically, run the below scripts:
```bash
# Install dependencies 
pip install -r requirements.txt
```
```bash
# Run coordinator script
python3 coordinator.py
```
- The coordinator script can also run the prover in cloud (using modal.com)
 
10. You can test by sending an email to your relayer account with a subject like `Send 1 ETH to another@email.com`. Relayer will deploy wallet for you for the first time and you will need to fund it externally as the wallet have no balance.

<br />

## ☞ Running Relayer in production

Email Wallet is currently in test phase. Before releasing to production, we will be running a trusted setup ceremony for the circuits and deploy the contracts to mainnet (probably L2).

Anyone would be able to run a relayer node. The steps to run a relayer node will be mostly same, except you would be using the production contracts and circuit `zkey`.

We will be publishing more guides on how to deploy and run on cloud as well.


<br />
<hr />

### History

This monorepo was originally created from below individual repo. You can check them for old commit history if needed.

- Circuit from [zk-email-verify / anon_wallet branch](https://github.com/zkemail/zk-email-verify/tree/anon_wallet)
- Contracts from [zk-email-verify / anon_wallet branch](https://github.com/zkemail/zk-email-verify/tree/anon_wallet)
- Relayer from [relayer / modal_anon](https://github.com/zkemail/relayer)

## References
1. S. Suegami and K. Shibano, "Contract Wallet Using Emails," 2023 IEEE International Conference on Blockchain and Cryptocurrency (ICBC), Dubai, United Arab Emirates, 2023, pp. 1-2, doi: 10.1109/ICBC56567.2023.10174932.
