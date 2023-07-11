pub const CHAIN: &str = "Ethereum Goerli";

pub fn first_reply(amount: &str, currency: &str, recipient: &str) -> String {
    format!(
        "Valid send initiated. Sending {} {} to {} on {}. \
        We will follow up with Etherscan link when finished! \n \nYou are sending with ZK email technology. \
        The relayer will automatically prove, on-chain, that you sent an email authorizing this transaction.  \
        We will deploy a wallet for you if you don't have one. While we're in beta, we'll also give you \
        10 TestERC20 if you don't have any to start, but in the future when we use real currency, \
        we'll send you an address to top up your wallet via Ethereum or other methods.",
        amount, currency, recipient, CHAIN
    )
}

pub fn invalid_reply(reason: &str) -> String {
    format!(
        "Send {}! Please try again with this subject: \"Send _ dai to __@__.___\". \
    You can send dai or eth right now, but it all sends with TestERC20.",
        reason
    )
}


pub fn pending_reply(address: &str, amount: &str, currency: &str, recipient: &str) -> String {
    format!(
        "Creating new wallet for sender at {} -- in order to send this transaction, you must top up \
        that address with enough currency to send. Your send has been queued and will execute once we detect \
        enough balance in that account, and then send {} {} to {} on {}. \
        We will follow up with Etherscan link when finished! \n \nYou are sending with ZK email technology. \
        The relayer will automatically prove, on-chain, that you sent an email authorizing this transaction.  \
        We will deploy a wallet for you if you don't have one. While we're in beta, we'll also give you \
        10 TestERC20 if you don't have any to start, but in the future when we use real currency, \
        we'll send you an address to top up your wallet via Ethereum or other methods.",
        address, amount, currency, recipient, CHAIN
    )
}