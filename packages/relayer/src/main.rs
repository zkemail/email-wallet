use relayer::*;

use anyhow::Result;
use ethers::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    run(RelayerConfig::new()).await?;

    Ok(())
}
