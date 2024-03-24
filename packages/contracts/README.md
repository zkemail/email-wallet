## Contract Architecture

#### Core

- `EmailWalletCore.sol` - The main contract that implements functions like `handleEmailOp`. To keep the contract simple, we have split other functionalities to many "handlers".
- `handlers/RelayerHandler.sol` - Handles operations to register and update Relayer config.
- `handlers/AccountHandler.sol` - Handles account related operations like `createAccount`, `initAccount`, `transpotAccount`, and wallet related functions.
- `handlers/UnclaimsHandler.sol` - Handles operations related to Unclaimed Funds and Unclaimed States.
- `handlers/ExtensionHandler.sol` - Handles operations related to publishing and installing Extensions.

#### Wallet

- `Wallet.sol` - Wallet contract deployed for each user (account). The owner of the wallet contract can execute any call data on a target. By default Core contract is the owner of the wallet contract.

#### Interfaces
- `interfaces/Types.sol` - Contains all the structs used in the contract including EmailOp.
- `interfaces/Commands.sol` - Defines the commands used for EmailOps.
- `interfaces/Events.sol` - Defines all events emitted by the contract.
- `interfaces/IVerifier.sol` - Defines the interface for the ZK proof verifier contract.
- `interfaces/IPriceOracle.sol` - Defines the interface for price oracle used for fee calculation.
- `interfaces/Extension.sol` - Defines the interface for an Extension contract.

#### Libraries
- `libraries/SubjectUtils.sol` - Contains helper functions to compute the subject line from an EmailOp.
- `libraries/DecimalUtils.sol` - Contains helper functions to convert uint to decimal string. 

#### Util contracts

- `utils/TokenRegistry.sol` - TokenRegistry contract that stores the address of ERC20 tokens by their names (and reverse) across multiple chains.
- `utils/UniswapTWAPOracle.sol` - UniswapTWAPOracle contract that provides the TWAP price of a token in ETH.
- `utils/ECDSAOwnedDKIMRegistry.sol` - A custom DKIM registry that allows a ECDSA signer to set the DKIM public key for a domain.

#### Default extensions

- `extensions/NFTExtension.sol` - NFT wallet extensions to send NFTs.
- `extensions/UniswapExtension.sol` - Uniswap wallet extension to swap tokens on Uniswap.

#### Circuit verifiers

- `verifier/Verifier.sol` - All verifiers
- `verifier/AccountCreationVerifier.sol` - Verifier for `AccountCreation` circuit.
- `verifier/AccountInitVerifier.sol` - Verifier for `AccountInit` circuit.
- `verifier/EmailSenderVerifier.sol` - Verifier for `EmailSender` circuit.
- `verifier/ClaimVerifier.sol` - Verifier for `Claim` circuit.
- `verifier/AccountTransportVerifier.sol` - Verifier for `AccountTransport` circuit.
- `verifier/AnnouncementVerifier.sol` - Verifier for `Announcement` circuit.

#### Tests
- `tests/*.t.sol` - Contains unit tests for all contracts/functionalities.
- `tests/Integration.t.sol` - Contains all integration tests. Integration tests generate the proof using the circuit and verify it using contracts. Before running those tests, you need to make a `packages/contracts/test/build_integration` directory, download the zip file from the following link, and place its unziped files under that directory.
https://drive.google.com/file/d/1sYlhq4tU7ZZGN1r9HM0pJcA4q6pHErXb/view?usp=sharing

## Build and Test

Make sure you have [Foundry](https://github.com/foundry-rs/foundry) installed

Build the contracts using the below command.

```bash
forge build
```

Run unit tests
```bash
forge test --no-match-test "testIntegration"
```

Run integration tests
Run each integration tests **one by one** as each test will consume lot of memory.
```bash
Eg: forge test --match-test 'testIntegration_Swap_Tokens' -vvv --ffi
```

## Deploy Contracts

You can either deploy all contracts at once or deploy each contract separately.

### Deploy all contracts at once

##### Create .env

```sh
cp .env.sample .env
```

##### Run 

```
source .env

forge script script/DefaultSetupScript.s.sol:Deploy \
--rpc-url $RPC_URL \
--chain-id $CHAIN_ID \
--broadcast \
-vvvv
```

### Deploy each contract separately

Run the below commands to deploy each contracts. Ensure address of WETH and Uniswap Price oracle on the target chain.

#### Deploy Token Registry
```
PRIVATE_KEY="" \
forge script script/01_DeployTokenRegistry.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "" \
--verify
```

Copy the address from log `TokenRegistry implementation deployed at: 0x9f44be9F69aF1e049dCeCDb2d9296f36C49Ceafb`

#### Deploy All Verifiers
```
PRIVATE_KEY="" \
forge script script/02_DeployAllVerifiers.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "" \
--verify
```

Copy the address from log `AllVerifiers implementation deployed at: 0x9f44be9F69aF1e049dCeCDb2d9296f36C49Ceafb`

#### Deploy DKIM Registry
```
PRIVATE_KEY="" \
forge script script/03_DeployDKIMRegistry.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "" \
--verify
```

Copy the address from log `DKIMRegistry implementation deployed at: 0xbE66454b0Fa9E6b3D53DC1b0f9D21978bb864531`


#### Deploy Uniswap TWAP Oracle
```
PRIVATE_KEY="" \
WETH=0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9 \
UNISWAP_FACTORY=0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f \
forge script script/DeployUniswapTWAPOracle.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "" \
--verify
```

Copy the address from log `UniswapTWAPOracle deployed at: 0x0000000000000000000000000000000000000000`

#### Deploy Wallet
```
PRIVATE_KEY="" \
WETH=0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9 \
forge script script/04_DeployWallet.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "" \
--verify
```

Copy the address from log `Wallet proxy deployed at: 0x0000000000000000000000000000000000000000`

#### Deploy Handlers
```
PRIVATE_KEY="" \
WETH=0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9 \
forge script script/05_DeployHandlers.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "" \
--verify
```

Copy the addresses from log 
```
  RelayerHandler proxy deployed at: 0x0000000000000000000000000000000000000000
  ExtensionHandler proxy deployed at: 0x0000000000000000000000000000000000000000
  AccountHandler proxy deployed at: 0x0000000000000000000000000000000000000000
  UnclaimsHandler proxy deployed at: 0x0000000000000000000000000000000000000000

```

#### Deploy Email Wallet Core

```
PRIVATE_KEY="" \
WETH=0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9 \
TOKEN_REGISTRY=0x9f44be9F69aF1e049dCeCDb2d9296f36C49Ceafb \
DKIM_REGISTRY=0xbE66454b0Fa9E6b3D53DC1b0f9D21978bb864531 \
PRICE_ORACLE=0xF5f40B12aa15286F0DE5610C4e29d87a97997ee7 \
forge script script/06_DeployEmailWalletCore.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "" \
--verify
```

Copy the addresses from log:
```
  EmailWalletCore proxy deployed at: 0x0000000000000000000000000000000000000000
```

#### Deploy Extnsions

```
PRIVATE_KEY="" \
WETH=0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9 \
TOKEN_REGISTRY=0x9f44be9F69aF1e049dCeCDb2d9296f36C49Ceafb \
DKIM_REGISTRY=0xbE66454b0Fa9E6b3D53DC1b0f9D21978bb864531 \
PRICE_ORACLE=0xF5f40B12aa15286F0DE5610C4e29d87a97997ee7 \
forge script script/07_SetDefaultExtensions.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "" \
--verify
```

Copy the addresses from log:
```
  NFTExtension proxy deployed at: 0x0000000000000000000000000000000000000000
  UniswapExtension proxy deployed at: 0x0000000000000000000000000000000000000000
```

#### Deploy ECDSAOwnedDKIMRegistry
Set the `SIGNER` to the address of the Ethereum wallet who will be setting the DKIM public key for a domain.

```
PRIVATE_KEY="" \
SIGNER=0x2f6e79a6e1a982a49ca248b70b02f76e921af400 \
forge script script/DeployECDSAOwnedDKIMRegistry.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "S7RWR1ENYB73HZY7WTV3EER7U8CQNBBTAJ" \
--verify
```

Copy the address from log `ECDSAOwnedDKIMRegistry deployed at: 0xB50a02E2Da524feC1209542985b2ae2917aF7265`

## Upgrade Contracts

#### Upgrade Token Registry

##### Run 

```
TOKEN_REGISTRY=0xF1d24E5f7f0Ca617F0c1f3AA34A77EcFfaFedE8f \
PRIVATE_KEY=0x00 \
forge script script/XX_UpgradeTokenRegistry.s.sol:Deploy \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
-vvvv
```