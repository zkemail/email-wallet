# ZK Email Wallet Contracts

To setup, simply run `yarn`. It will install the necessary dependencies


## Testing

To build/test locally you need to install foundry

```bash
curl -L https://foundry.paradigm.xyz | bash && source ~/.bashrc && foundryup
```

Then you can run
```bash
forge test
```


## Deploying

You can set private key in `script/DeployWallet.s.sol` and run

```bash
forge script script/DeployWallet.s.sol:Deploy -vvvv --rpc-url http://localhost:8545 
```
