# Overview

ZK-Email Wallet is a non-custodial crypto wallet, which is managed through email. 

Core component is a [smart-contract](https://github.com/zkemail/zk-email-wallet/tree/feat/v1/packages/contracts). 
V1 contract allows:
* ETH transfers
* ERC20 transfers
* Add extensions (usage of external contracts, such as Uniswap and in general any other functionality)

---

## ZK-Email

As it was said the wallet (the contract) is managed by users through email. It's possible to do that trustlessly with **zk-email** or **proof of email** technology. 

Zk-Email is a mechanism that allows to prove the existence of a mail message and also to prove statements about any data contained within it. This can be used to build trustless bridge from Web2 to Web3 (no 3rd party included, s.a. oracles as chainlink, etc.) - email as an oracle data.

### How it works? 

Zk-Email core components are:
* [ZK-RSA](https://mirror.xyz/privacy-scaling-explorations.eth/mmkG4uB2PR_peGucULAa7zHag-jz1Y5biZH8W6K2LYM) - RSA signature verification in ZK;
* [ZK-Regex](https://katat.me/blog/ZK+Regex) - Regex pattern matching in ZK.

It's important to understand, that practically all modern mail servers are signing mail messages with [DKIM signature](https://www.emailonacid.com/blog/article/email-deliverability/what_is_dkim_everything_you_need_to_know_about_digital_signatures/) (basically RSA) to prevent many kind of attack vectors; having some email - we can prove that it's indeed the message that's handled by some message server, by checking the DKIM signature.
Thus, only having this, it's already possible to build a Web2 <-> Web3 bridge. But the thing is that it won't be useful, cause in most cases we want to hide some things, that are contained in the message (and of course we cannot just simply remove it from the message, because it won't be possible to check the signature then). 
In addition message body is usually big, and calldata on L1s and even on many L2s is very expensive. 

We can use ZK proving systems and do the signature check in ZK and make all data private, except the things that need to be public - this will help us with privacy and also it'll critically shrink the calldata. 

The main problem here is that the messages are usually don't follow some specific format. That's why we need something to effectively retrieve necessary info. For that ZK-Regex is used.

To learn more on ZK-Email you can read [this blog post](https://blog.aayushg.com/posts/zkemail/) by Aayush.

ZK-Email allows to build many different applications:
* [Proof of twitter](https://zkemail.xyz/) - proof you own a Twitter account/username on-chain
* [ZK-P2P](https://github.com/zkp2p/zk-p2p-v1) - trustless fiat to crypto bridge

And of course Zk-Email Wallet is also built using ZK-Email.

