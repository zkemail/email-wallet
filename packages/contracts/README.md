### Deploy Contracts

Run the below commands to deploy each contracts. Ensure address of WETH and Uniswap Price oracle on the target chian.


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

Copy the address from log `
  Verifier deployed at: 0x6885A42bB98eE80eEca9de93Aaf8ee5CEd6096be
  Wallet implementation deployed at: 0xb48fb94ba07fa279337091C29fb4dACA324A9690
  EmailWalletCore deployed at: 0x21E7FDC3A6ac59124b8AF2dc2c13E118EfE8248f
  NFT Extension deployed at: 0xb7F8bD28719aA118EcD8D01293acEe0E804b2EE6
  Uniswap Extension deployed at: 0x6CE6893f06A438A85686DC1104688ad3b032de05
`
