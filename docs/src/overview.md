# Overview

The ZK-Email Wallet is a non-custodial cryptocurrency wallet managed via email. 

The core component of this wallet is a [smart-contract](https://github.com/zkemail/zk-email-wallet/tree/feat/v1/packages/contracts). The V1 contract supports:
* Transfers of ETH
* Transfers of ERC20 tokens
* Extensions for using external contracts, such as Uniswap, and other functionalities

---

## ZK-Email

The wallet is managed by users through email, leveraging **zk-email** or **proof of email** technology for trustless operations. 

ZK-Email is a mechanism that validates the existence of an email message and verifies statements about any data contained within it. This technology can be used to build a trustless bridge from Web2 to Web3, without the need for third-party intermediaries like oracles such as Chainlink. In this context, email serves as a source of oracle data.

### Core Technical Components

The core components of ZK-Email include:
* [ZK-RSA](https://mirror.xyz/privacy-scaling-explorations.eth/mmkG4uB2PR_peGucULAa7zHag-jz1Y5biZH8W6K2LYM) - RSA signature verification in Zero-Knowledge (ZK)
* [ZK-Regex](https://katat.me/blog/ZK+Regex) - Regex pattern matching in ZK

Most modern mail servers sign email messages with a [DKIM signature](https://www.emailonacid.com/blog/article/email-deliverability/what_is_dkim_everything_you_need_to_know_about_digital_signatures/) (essentially RSA) to mitigate various attack vectors. By verifying the DKIM signature of an email, we can confirm that it was processed by a specific mail server. This alone makes it possible to build a Web2 <-> Web3 bridge. However, in most cases, we need to conceal certain information contained in the message. Removing this information from the message would invalidate the signature, so we use ZK proving systems to check the signature in ZK and keep all data private, except for what needs to be public. This approach enhances privacy and significantly reduces the size of the calldata.

One challenge is that email messages often don't adhere to a specific format. To effectively retrieve necessary information, we use ZK-Regex.

For more information on ZK-Email, refer to [this blog post](https://blog.aayushg.com/posts/zkemail/) by Aayush.

ZK-Email can be used to build various applications, including:
* [Proof of Twitter](https://zkemail.xyz/) - Proving ownership of a Twitter account/username on-chain
* [ZK-P2P](https://github.com/zkp2p/zk-p2p-v1) - A trustless bridge from fiat to crypto

The ZK-Email Wallet is one such application built using ZK-Email.
