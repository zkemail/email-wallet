use crate::*;

use ethers::prelude::*;

pub(crate) async fn get_latest_block_number() -> Result<U64> {
    let provider = Provider::<Http>::try_from("infura_url")?;
    Ok(provider.get_block_number().await?)
}

pub(crate) async fn send_to_chain(proof: &str) -> Result<()> {
    todo!()
}

pub(crate) async fn is_enough_balance(user: &User, parsed_email: &ParsedEmail) -> Result<bool> {
    todo!()
}
