use relayer::*;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    run(RelayerConfig::new()).await?;

    Ok(())
}
