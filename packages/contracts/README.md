### Deploy Contracts

Run the below commands to deploy each contracts. Ensure address of WETH and Uniswap Price oracle on the target chian.


#### Deploy Token Registry
```
PRIVATE_KEY="<private-key-int>" \
forge script script/DeployTokenRegistry.s.sol:Deploy \
-vvvv \
--rpc-url https://rpc.ankr.com/eth_goerli \
--chain-id 5 \
--broadcast \
--etherscan-api-key <etherscan-api-key> \
--verify
```

Copy the address from log `TokenRegistry deployed at: 0x8D1998c739A13a4960d7263e0C7F01BD7695794A`


#### Deploy DKIM Registry
```
PRIVATE_KEY="<private-key-int>" \
forge script script/DeployDKIMRegistry.s.sol:Deploy \
-vvvv \
--rpc-url https://rpc.ankr.com/eth_goerli \
--chain-id 5 \
--broadcast \
--etherscan-api-key <etherscan-api-key> \
--verify
```

Copy the address from log `DKIMRegistry deployed at: 0xbC58246c953292BD5187e8EC4dC7067741043800`


#### Deploy Uniswap TWAP Oracle
```
PRIVATE_KEY="<private-key-int>" \
WETH=0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6 \
UNISWAP_FACTORY=0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f \
forge script script/DeployUniswapTWAPOracle.s.sol:Deploy \
-vvvv \
--rpc-url https://rpc.ankr.com/eth_goerli \
--chain-id 5 \
--broadcast \
--etherscan-api-key <etherscan-api-key> \
--verify
```

Copy the address from log `UniswapTWAPOracle deployed at: 0x022A1E1c27343FDf731315DE0655D9e0C7F59Cde`


#### Deploy Email Wallet Core
```
PRIVATE_KEY="<private-key-int>" \
WETH=0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6 \
TOKEN_REGISTRY=0x8D1998c739A13a4960d7263e0C7F01BD7695794A \
DKIM_REGISTRY=0xbC58246c953292BD5187e8EC4dC7067741043800 \
PRICE_ORACLE=0x022A1E1c27343FDf731315DE0655D9e0C7F59Cde \
forge script script/DeployEmailWallet.s.sol:Deploy \
-vvvv \
--rpc-url https://rpc.ankr.com/eth_goerli \
--chain-id 5 \
--broadcast \
--etherscan-api-key <etherscan-api-key> \
--verify
```

Copy the address from log `
  Verifier deployed at: 0xd4Edf5478387b0E0e54F12E2e3CA5B10C60fC531
  Wallet implementation deployed at: 0xcA84fd5B1831Ae0414F25af36fe9fF2b95498652
  EmailWalletCore deployed at: 0xd8E5ab228C0F88dd6b195d0557006AE817c09DF7
  `
