# Email Wallet Deployment Guide

This guide provides step-by-step instructions for deploying and running the Email Wallet system, including smart contracts and the relayer service.

## Overview

Email Wallet is an Ethereum contract wallet that can be controlled by sending emails. The deployment process involves two main steps:

1. Deploying the smart contracts
2. Setting up and running the relayer service

## Prerequisites

Before starting, ensure you have the following:

- [Foundry](https://github.com/foundry-rs/foundry) installed for contract deployment
- [Rust and Cargo](https://www.rust-lang.org/tools/install) for running the relayer
- [Docker](https://docs.docker.com/get-docker/) for running the database, SMTP, and IMAP services
- A private key with funds for contract deployment and relayer operations

## Step 1: Deploy Smart Contracts

### 1.1 Set Up Environment

Navigate to the contracts directory:

```sh
cd packages/contracts
```

Create a `.env` file:

```sh
cp .env.sample .env
```

Edit the `.env` file and set the following variables:
- `PRIVATE_KEY`: Your private key for deployment
- `RPC_URL`: RPC URL for the target network
- `CHAIN_ID`: Chain ID of the target network
- `CHAIN_NAME`: Name of the target network

Other default values work for Base Sepolia, but adjust them if you're deploying to a different chain.

### 1.2 Deploy All Contracts

Run the deployment script:

```sh
source .env

forge script script/DefaultSetupScript.s.sol:Deploy \
--rpc-url $RPC_URL \
--chain-id $CHAIN_ID \
--broadcast \
-vvvv
```

You'll get a response with the addresses of all deployed contracts. Save these addresses as you'll need them for setting up the relayer.

Example output:
```
== Logs ==
  TokenRegistry proxy deployed at: 0xB895744454C62e9052A4332C29c4F19048054B56
  TokenRegistry implementation deployed at: 0xee62Ca2257E98284eEBf66c8849cA00489Ff1E03
  AllVerifiers implementation deployed at: 0x8350e89C182dD211180A8773cd91F79f2822BC9a
  ECDSAOwnedDKIMRegistry deployed at: 0xA11D302310bAd2Da10216B0FA91e5A1bca631c19
  Wallet implementation deployed at: 0x7a6361d28b1121EB8a030778f597A85443833cc9
  Oauth core deployed at: 0x50bba18c7289bf4189E2d8e5ed40Ae86be020567
  RelayerHandler proxy deployed at: 0x9F68880B9A760d15734872EbA3D519549C3996BB
  RelayerHandler implementation deployed at: 0x9e119689Fc87be723886dA2dD012bF43D66c7BA4
  ExtensionHandler proxy deployed at: 0x6D6bA7Ee9Ed199F67C6C26dfb3B6E56b60D9fdef
  <TRUNCATED>
  TestERC20 deployed at: 0x5635e6A652bE734F858dAA769e215cdb516eaca4
  EmailWalletCore proxy deployed at: 0xeA1001AE28da904744fdd5167A4EbF1f2f546fab
```

### 1.3 Register Relayer Data

Before proceeding to deploy the relayer, you need to register the relayer data in the `RelayerHandler` contract.

Set the following environment variables:

```sh
RPC_URL=<the RPC URL of the chain where you deployed the contracts>
PRIVATE_KEY=<this is the relayer's private key>
RELAYER_HANDLER=<address of the RelayerHandler contract from the previous step>
RELAYER_EMAIL=email@example.com # used for relayer communication later
RELAYER_HOSTNAME=example.com
```

Then run:

```sh
forge script script/RegisterRelayer.s.sol --rpc-url $RPC_URL --broadcast
```

## Step 2: Set Up and Run the Relayer

### 2.1 Set Up Environment

Navigate to the relayer directory:

```sh
cd packages/relayer
```

Create a `.env` file:

```sh
cp .env.example .env
```

Update the following key variables in the `.env` file:

- `CORE_CONTRACT_ADDRESS`: Address of the EmailWalletCore proxy from Step 1.2
- `ONBOARDING_TOKEN_ADDR`: Address of the TestERC20 from Step 1.2
- `PRIVATE_KEY`: The registered relayer's private key for submitting on-chain transactions
- `CHAIN_RPC_PROVIDER`, `CHAIN_RPC_EXPLORER`, `CHAIN_ID`: Set according to your target chain
- `RELAYER_EMAIL_ADDR`, `RELAYER_HOSTNAME`: Fill out according to your relayer setup

Other variables like `CANISTER_ID`, `IC_REPLICA_URL`, `SMTP_SERVER`, and `DATABASE_URL` can use default values for local testing.

### 2.2 Run Docker Compose for Local Services

Ensure you have set proper values in `.env` for SMTP and IMAP services, then run:

```sh
docker compose up --build -d
```

Confirm all services are running:

```sh
docker ps
```

You should see output similar to:

```
CONTAINER ID   IMAGE          STATUS          PORTS
97f6b65dd0b7   relayer-imap   Up 1 second     
3f2fc12589c9   postgres:15    Up 1 second     0.0.0.0:5432->5432/tcp
e5e7c6a7c434   relayer-smtp   Up 1 second     0.0.0.0:3000->3000/tcp
```

### 2.3 Register the Relayer On-Chain

Run:

```sh
cargo run --release -- setup
```

This step registers the relayer's details on the blockchain, preparing it for operation.

### 2.4 Start the Relayer

Run:

```sh
cargo run --release
```

## Step 3: Testing the System

You can test the system by sending an email to your relayer account with a subject like:

```
Send 1 ETH to another@email.com
```

For the first transaction, the relayer will deploy a wallet for you, but you will need to fund it externally as the wallet will have no balance initially.

## Step 4: Running the Frontend (Optional)

For a complete experience, you can run the frontend from the [emailwallet.org](https://github.com/zkemail/emailwallet.org/tree/refactor/v1.1) repository.

## Troubleshooting

### Common Issues

1. **Contract Deployment Fails**: Ensure your private key has sufficient funds and the RPC URL is correct.

2. **Relayer Registration Fails**: Verify that you're using the correct RelayerHandler address and that your private key has permission to register.

3. **Relayer Service Doesn't Start**: Check the logs for errors. Common issues include:
   - Incorrect environment variables
   - Database connection problems
   - SMTP/IMAP configuration issues

4. **Emails Not Being Processed**: Verify that:
   - The SMTP and IMAP services are running
   - The relayer has the correct email credentials
   - The email format follows the expected pattern

### Getting Help

If you encounter issues not covered in this guide, please refer to the detailed documentation in the respective README files or open an issue on the GitHub repository.

## Features

With your Email Wallet system deployed, you can:

- Send ETH and ERC20 tokens to email addresses and Ethereum addresses
- Execute any calldata on target contracts
- Install extensions for additional functionalities
- Use Uniswap extension to swap tokens
- Use NFT extension to mint NFTs
- Set custom DKIM Registry
- Transfer wallet ownership

All by simply sending emails with the appropriate subject lines! 