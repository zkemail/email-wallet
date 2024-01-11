#### To build the relayer:

```bash
cd packages/relayer
cargo build   # output binary is in target/debug/relayer
```

<br />

## ☞ Running Locally

1. Firstly, start a local ETH testnet. For example `anvil`

```bash
anvil --chain-id 5
```

2. Set private key of an account in the testnet (with sufficient balance) in `packages/contracts/scripts/DeployEmailWallet.s.sol` file.
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
FEE_PER_GAS=1000000000        # Fee per gas in wei.
DATABASE_PATH=                # Full path to the relayer/db directory.
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
```

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

## ☞ Running Relayer as Docker container

### Building Docker Image

#### For local development
```bash
cd ../../
docker buildx build -t email_wallet_v1_relayer:latest-arm -f Relayer.Dockerfile . \
  --build-arg modal_token_id=${MODAL_TOKEN_ID} \
  --build-arg modal_token_secret=${MODAL_TOKEN_SECRET}
```

#### For x86 compatibility(e.g. For production)
```bash
cd ../../
docker buildx build -t email_wallet_v1_relayer:latest -f Relayer.Dockerfile . \
  --build-arg modal_token_id=${MODAL_TOKEN_ID} \
  --build-arg modal_token_secret=${MODAL_TOKEN_SECRET} \
  --platform=linux/arm64
```

### Create .env

Create `.env` in the execution directory with reference to `env_example`.


### How to Run

```bash
docker run \
-p 80:80 \
-v $(pwd)/.env:/email-wallet/packages/relayer/.env \
-e SETUP=false
email_wallet_v1_relayer:latest
```