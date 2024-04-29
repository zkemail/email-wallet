## Overview

The Relayer, a crucial system component, bridges email communication and blockchain transactions, ensuring secure information processing.

Below is a diagram of the Relayer infrastructure and its component interactions.

![Relayer Infrastructure](/docs/images/RelayerInfra.png)


To build the Relayer, you need to understand its three main components:

- **SMTP Server**: The entry point for emails, which the Relayer monitors for incoming messages that trigger blockchain transactions.
- **Relayer**: The core service that listens for emails from the SMTP server, communicates with the Prover to verify proofs, and then writes the validated transactions to the blockchain.
- **Prover**: A separate service that the Relayer relies on to verify and create zk proofs of the data extracted from emails.
- **Database (DB)**: A PostgreSQL database to store account information, facilitating checks on whether an account already exists within the system.

## Building the Relayer

### Running the Prover

#### ☞ Running locally 
To run the prover locally, follow these steps:

1. **Prepare the Environment**:
Ensure your system has Node.js, npm, and Python 3 installed. The prover relies on specific npm packages and Python modules to function correctly.

2. **Install Global npm Packages**:
Install snarkjs globally using npm.
```
   npm install -g snarkjs@latest
```
3. **Install Python Requirements**:
Navigate to the prover directory and install the required Python packages listed in requirements.txt.

```
 pip install -r requirements.txt
```
4. Run the curl commands and script in the `prover/local_setup.sh` file.

5. **Start the prover**

```
python3 packages/prover/local.py
```


#### ☞  Using Modal
The prover leverages Modal, a cost-effective, serverless computing platform activated as needed, for generating zk proofs. Visit site here: https://modal.com

1. **Set Up Modal Tokens**: Follow the instructions in `/Relayer.Dockerfile` to set up your Modal tokens.

2. **Start the Prover Service**: Run the following command to start the Prover service using Modal:


```bash
modal run python packages/prover/local.py
```

## Setting up the DB
**Database Setup Steps:**

1. **Start PostgreSQL Database**:
   ```
   docker run --rm --name email-wallet-db -e POSTGRES_PASSWORD=p@ssw0rd -e POSTGRES_USER=emailwallet -e POSTGRES_DB=emailwallet -p 5432:5432 postgres
   ```

2. **Set Environment Variable**:
   In `.env`:
   ```
   DATABASE_URL=postgresql://emailwallet:p@ssw0rd@localhost:5432/emailwallet
   ```

**Note**: The system triggers on emails with `accountKey` in the subject, for new account setups. Reply-email functionality for account transport will be deprecated.


<!-- ```bash
cd packages/relayer
cargo build   # output binary is in target/debug/relayer
``` -->

## Setting up the Relayer


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

PROVER_ADDRESS="http://0.0.0.0:8080" # local or modal URL

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

### Buidling the Docker image

- #### For local development
```bash
cd ../../
docker buildx build -t email_wallet_v1_relayer:latest-arm -f Relayer.Dockerfile . \
  --build-arg modal_token_id=${MODAL_TOKEN_ID} \
  --build-arg modal_token_secret=${MODAL_TOKEN_SECRET}
```

- #### Production(x86 compatibility):
```bash
docker buildx build -t email_wallet_v1_relayer:latest -f Relayer.Dockerfile . \
  --build-arg modal_token_id=${MODAL_TOKEN_ID} \
  --build-arg modal_token_secret=${MODAL_TOKEN_SECRET} \
  --platform=linux/arm64
```
#### **Running the Relayer**

1. **Start the Docker Container**: Launch the Relayer service in a Docker container with the following command:
```bash
docker run \
-p 80:80 \
-v $(pwd)/.env:/email-wallet/packages/relayer/.env \
email_wallet_v1_relayer:latest
```

2. **Update Contract ABIs**: Ensure the contract ABIs are up to date in `packages/relayer/abi/` directory.

4. **Register the relayer on chain by running**:
```
cargo run --release -- setup
```
This step registers the relayer's details on the blockchain, preparing it for operation.

4. **Start the relayer**:
```
cargo run --release
```



5. You can test by sending an email to your relayer account with a subject like `Send 1 ETH to another@email.com`. Relayer will deploy wallet for you for the first time and you will need to fund it externally as the wallet have no balance.


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

## ☞ Relayer's Incentive 
The Relayer's incentive is transaction fees collected from the sender.
Specifically, the Relayer operator can set a fee per gas in wei to [the ENV file](https://github.com/zkemail/email-wallet/blob/main/packages/relayer/.env.example#L17).
However, that value must be less `maxFeePerGas` parameter defined in our contract, which is 2 gwei now.

When the Relayer posts the email-triggered transaction along with the ZK proof of email, our contract calculates the total amount of fee in wei as follows:
1. If the transaction does not pass the validation function `validateEmailOp`, the fee is zero.
2. The contract measures the consumed gas when executing the transaction. 
3. The gas to refund ERC20 tokens, 55000 gas, is added to 2.
4. If the transaction specifies a recipient's email address, 450000 and 500000 gas are added to 3 for ERC20 tokens transfer and the other use cases, respectively.
5. The total amount of fee in wei is the multiplication between the fee per gas set by the relayer and the total gas in 4.

Note that our contract catches any errors during the execution of the transaction in Step 2 instead of making the transaction fail because the Relayer cannot always simulate if the execution returns errors in general cases before posting it on-chain.
Therefore, the Relayer can always collect the fee as long as the transaction passes the validation in Step 1.
This design refers to the bundler's fee mechanism in [ERC-4337](https://eips.ethereum.org/EIPS/eip-4337).

<br />
