# ZK-Email Wallet Documentation

The ZK-Email Wallet is a unique type of non-custodial cryptocurrency wallet that is managed via email. This document provides an overview of its features and future development plans. You can try our demo at https://sendeth.org.

## Features

The ZK-Email Wallet currently supports the following functions:

- **ETH Transfers**: Users can transfer ETH from standard ETH addresses to email wallets, from email wallets to standard ETH addresses, and between email wallets.
- **ERC20 Transfers**: The wallet also supports transfers of ERC20 tokens.

Currently, the wallet uniquely provides the following features:
- **Send money to people without accounts**: Because transfers are authorized by an email address, you can mass send money or assets to people who have never used email wallet before but do have an email address. If the email address does not claim the money in 1 year (or another preset time), it can be returned to the sender. This avoids problems with dead email addresses or transactions the recipient does not want to recieve.
- **No user interaction with Ethereum**: By delegating control of their account to an account abstraction wallet under their email provider (which they already trust with their bank account recovery and social media password reset control), users can avoid ever having to think about private keys or the complexity of the crypto ecosystem.

## How It Works (User Perspective)
For each transaction, you send an email. This email can be formatted for you on https://sendeth.org, or you can simply email "relayer@sendeth.org" with a subject like "Send 2 DAI to friend@gmail.com". The relayer will automatically deploy a new wallet for you (if you don't have one) that can only be authorized via emails from you. If you want to test without any Goerli assets, you can start trying to send TEST token like "Send 2 TEST to friend@gmail.com", which will automatically spend from a free 10 token starting balance.

This money is sent to a smart contract wallet authorized to your friend's email address (in this case, friend@gmail.com), meaning only emails from them can spend the money. This transaction occurs in wallets on-chain, and neither the relayer nor us can steal your funds (since we can't generate a fake email from you with the correct cryptographic signatures, only your email provider can). Currently, transfers happen on Goerli -- dm us if you need free Goerli ETH/DAI/USDC to test it out!

If you want to mass-send to a lot of email addresses at once, right now you have to send one email per transaction.

## How It Works (Technical Summary)

When you send a transaction by a relayer (which can be self-hosted if desired), the relayer retrieves your raw email and signature. It then makes a zk proof showing you sent an email authorizing a transaction, and masks out all of the email addresses to ensure that they are not published publicly on-chain. These email addresses are salted via the message ID of your email, so you can always retrieve your own on-chain address without the relayer's help. This authorizes a transaction only if the email is formatted exactly correctly, or else the entire transaction will revert.

For a more detailed technical breakdown of the cryptographic technology used in emails, read the [technical blog post behind zk-email](https://blog.aayushg.com/posts/zkemail/)! For a more in-depth technical breakdown of our code, read through our [Github repository](https://github.com/zkemail/email-wallet)! For a detailed breakdown of the protocol, read through our [presentation slides](https://docs.google.com/presentation/d/1k8NNkdjZ47RbztI1Nn6rx0EcWRxPC2YI/edit#slide=id.p27).

## Future Development

In the next month or two, we intend to prioritize:
- **KYC-Free Onboarding and Offboarding**: We intend to build a native integration with [ZK P2P](https://zkp2p.xyz/), which uses the same base email proving technology to atomically swap off onto Venmo or Revolut.
- **Native Swaps**: We also intend to natively build out a swapping integration, where i.e. Uniswap is integrated into the wallet for seamless token swaps between currencies.
- **Bulk Sending**: We intend to include bulk-sending a list of amounts to a list of email addresses, in one email or transaction.

The initial version of the ZK-Email Wallet does not support privacy features. However, we intend to build a full UTXO model by the end of 2023. The core components of the wallet will be redesigned and reimplemented to support these features, but the user flow will remain the same.