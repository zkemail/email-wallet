# ZK Email Contracts

These are the V0 contracts accessible via emailwallet.org.

Feature | V0 | V1
---|---|---
Inter-relayer communication | No | Yes
Support extensions | No | Yes
Support simple ERC20 transfers | Yes | Yes
On-chain Anonymity of Email Addresses | Yes | Yes
Anonymity (default gmail message-ids) | 40-64 bits | 256 bit

## Testing

To get syntax highlighting in VSCode to work, you have to open this directory as the root directory for the Solidity extension to read the remappings properly.

To setup,

```bash
curl -L https://foundry.paradigm.xyz | bash && source ~/.bashrc && foundryup
# forge install foundry-rs/forge-std
# cp -r node_modules/@openzeppelin src/contracts/lib/@openzeppelin
cd src/contracts

forge install openzeppelin/openzeppelin-contracts foundry-rs/forge-std openzeppelin/openzeppelin-contracts-upgradeable dapphub/ds-test --no-commit
```

Maybe add `--no-git` to the end if the last one fails but not rec'd.

To test your own contracts, copy TestTwitter.t.sol into a new test file, and make sure you can compile your proof fine. You can run a specific test with `forge test --match-test test_name`. Then make sure the whole suite passes and isn't above the size limit:

```bash
forge test --fork-url https://eth-goerli.g.alchemy.com/v2/$ALCHEMY_GOERLI_KEY
forge build --sizes # Make sure these are all below 24kB
```

## Deployment

Current Goerli EmailWallet Address: `0x78ea729eba80d7399bbfb1dabd783ada5ae375df`

To deploy contract to local forked mainnet or prod, edit Deploy.s.sol to point to your contracts. In `src/contracts`, you should also edit the `.env` file from cloning `.env.example` to include your own private key.

To do a test deployment, run local chain in tmux window 1:

```bash
tmux
# Run local chain in tmux window 1
anvil --fork-url https://eth-goerli.g.alchemy.com/v2/$ALCHEMY_GOERLI_KEY --port 8548
```

Then deploy the contract, by first testing a deploy to forked goerli. Make sure you've duplicated .env.example into .env and set a private key first.

```bash
# Set terminal to the folder with this README
cd src/contracts
source .env
export MAIN_CONTRACT_NAME=EmailWallet
export RPC_URL="http://127.0.0.1:8548"
# Export to abi for relayers
forge inspect src/wallet/EmailWallet.sol:$MAIN_CONTRACT_NAME abi >> contract.abi
# First, test deploy without actually broadcasting it
forge script script/DeployWallet.s.sol:Deploy -vvvv --rpc-url $RPC_URL
# Then, actually deploy and verify
forge script script/DeployWallet.s.sol:Deploy -vvvv --rpc-url $RPC_URL --broadcast --slow --verify
```

Then, the main EMAIL_ADDR contract will be the one listed under `ERC1967Proxy` in `run-latest.json` under `src/contracts/broadcast/deploy-contract-name/#`. If the verification fails, you can do it again with:

```bash
# Verify the contract with the raw one via Etherscan
forge verify-contract $EMAIL_ADDR $MAIN_CONTRACT_NAME --watch --etherscan-api-key $GOERLI_ETHERSCAN_API_KEY
```

### What if I get an error about request failed and not all the contracts deploy?

Maybe fullnode is on [old geth](https://github.com/ethereum/go-ethereum/issues/26890) endpoint, like Alchemy is. Switch to infura or add `--slow` to deploy script:

```
forge script script/Deploy.s.sol:Deploy -vvvv --rpc-url $RPC_URL --broadcast --slow  --verify
```