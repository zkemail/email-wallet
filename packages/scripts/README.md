## Email Wallet Scripts

Some scripts to help with email wallets.

### 1. populate-token-registry

This will fetch tokens from Uniswap default token list and populate the TokenRegistry contracts.

ENVS:
```
CHAIN_ID= #chain id
RPC_URL= #rpc url of the chain
TOKEN_REGISTRY=  #address of the token registry
PRIVATE_KEY=  #private key of the wallet
```

Run

```bash
cargo run populate-token-registry
```