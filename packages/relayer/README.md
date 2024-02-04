## Overview

The Relayer, a crucial system component, bridges email communication and blockchain transactions, ensuring secure information processing.

Below is a diagram of the Relayer infrastructure and its component interactions.

![Relayer Infrastructure](/docs/images/RelayerInfra.png)


To build the Relayer, you need to understand its three main components:

- **SMTP Server**: The entry point for emails, which the Relayer monitors for incoming messages that trigger blockchain transactions.
- **Relayer**: The core service that listens for emails from the SMTP server, communicates with the Prover to verify proofs, and then writes the validated transactions to the blockchain.
- **Prover**: A separate service that the Relayer relies on to verify and create cryptographic proofs of the data extracted from emails.

## Build the Relayer

### Running the Prover

#### ☞  Using Modal

1. **Set Up Modal Tokens**: Follow the instructions in `Relayer.Dockerfile` to set up your Modal tokens.

2. **Start the Prover Service**: Run the following command to start the Prover service using Modal:


```bash
modal run python packages/prover/local.py
```

<!-- ```bash
cd packages/relayer
cargo build   # output binary is in target/debug/relayer
``` -->

### Running the Relayer

 1. Create a `.env` file in `packages/relayer` by taking a copy from `.env.example`.

```bash
cp packages/relayer/.env.example packages/relayer/.env
```

2. Update the `.env` file

```bash
CORE_CONTRACT_ADDRESS=           # Address of the deployed wallet contract.
PRIVATE_KEY=                      # Private key for Relayer's account.
CHAIN_RPC_PROVIDER=http://127.0.0.1:8545
CHAIN_ID=11155111                    # Chain ID of the testnet.

# IMAP + SMTP (Settings will be provided by your email provider)
IMAP_DOMAIN_NAME=imap.gmail.com
IMAP_PORT=993
AUTH_TYPE=password
SMTP_DOMAIN_NAME=smtp.gmail.com
LOGIN_ID=                    # IMAP login id - usually your email address.
LOGIN_PASSWORD=""         # IMAP password - usually your email password.

PROVER_LOCATION=local         # Keep this local for running the prover locally.
PROVER_ADDRESS="http://0.0.0.0:8080"

FEE_PER_GAS=0        # Fee per gas in wei.
DATABASE_URL= "postgres://test@localhost/emailwallet_test"
RELAYER_EMAIL_ADDR=
RELAYER_HOSTNAME="example.com"
WEB_SERVER_ADDRESS="127.0.0.1:4500"
CIRCUITS_DIR_PATH=  #Path to email-wallet/packages/circuits
SUBGRAPH_URL=https://api.thegraph.com/subgraphs/name/zkemail/email-wallet-v1-sepolia-2
INPUT_FILES_DIR_PATH=  #Path to email-wallet/packages/relayer/input_files
EMAIL_TEMPLATES_PATH=  #Path to email templates, e.g. ./packages/relayer/eml_templates/

ONBOARDING_TOKEN_ADDR=
ONBOARDING_TOKEN_AMOUNT=100
ONBOARDING_TOKEN_DISTRIBUTION_LIMIT=10
ONBOARDING_REPLY="You received 100 TEST!"

CANISTER_ID="i73e6-2qaaa-aaaan-qepxa-cai"
PEM_PATH="./.ic.pem"
IC_REPLICA_URL="https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=i73e6-2qaaa-aaaan-qepxa-cai"

JSON_LOGGER=false
```

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

3. Ensure the contract ABIs are up to date in `packages/relayer/abi/` directory.

4. Start the relayer:

```bash
docker run \
-p 80:80 \
-v $(pwd)/.env:/email-wallet/packages/relayer/.env \
email_wallet_v1_relayer:latest
```

10. You can test by sending an email to your relayer account with a subject like `Send 1 ETH to another@email.com`. Relayer will deploy wallet for you for the first time and you will need to fund it externally as the wallet have no balance.


