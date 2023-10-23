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
- `tests/Integration.t.sol` - Contains all integration tests. Integration tests generate the proof using the circuit and verify it using contracts.


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

Run the below commands to deploy each contracts. Ensure address of WETH and Uniswap Price oracle on the target chain.

#### Deploy Token Registry
```
PRIVATE_KEY="" \
forge script script/DeployTokenRegistry.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "" \
--verify
```

Copy the address from log `TokenRegistry deployed at: 0x9f44be9F69aF1e049dCeCDb2d9296f36C49Ceafb`


#### Deploy DKIM Registry
```
PRIVATE_KEY="" \
forge script script/DeployDKIMRegistry.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "" \
--verify
```

Copy the address from log `DKIMRegistry deployed at: 0xbE66454b0Fa9E6b3D53DC1b0f9D21978bb864531`


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

Copy the address from log `UniswapTWAPOracle deployed at: 0xF5f40B12aa15286F0DE5610C4e29d87a97997ee7`


#### Deploy Email Wallet Core
```
PRIVATE_KEY="" \
WETH=0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9 \
TOKEN_REGISTRY=0x9f44be9F69aF1e049dCeCDb2d9296f36C49Ceafb \
DKIM_REGISTRY=0xbE66454b0Fa9E6b3D53DC1b0f9D21978bb864531 \
PRICE_ORACLE=0xF5f40B12aa15286F0DE5610C4e29d87a97997ee7 \
forge script script/DeployEmailWallet.s.sol:Deploy \
-vvvv \
--rpc-url https://ethereum-sepolia.publicnode.com	 \
--chain-id 11155111 \
--broadcast \
--etherscan-api-key "" \
--verify
```

Copy the addresses from log:
```
  Verifier deployed at: 0x6885A42bB98eE80eEca9de93Aaf8ee5CEd6096be
  Wallet implementation deployed at: 0xb48fb94ba07fa279337091C29fb4dACA324A9690
  EmailWalletCore deployed at: 0x21E7FDC3A6ac59124b8AF2dc2c13E118EfE8248f
  NFT Extension deployed at: 0xb7F8bD28719aA118EcD8D01293acEe0E804b2EE6
  Uniswap Extension deployed at: 0x6CE6893f06A438A85686DC1104688ad3b032de05
```
