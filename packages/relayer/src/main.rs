use relayer::*;

use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "setup" {
        return setup().await;
    } else {
        run(RelayerConfig::new()).await?;
    }
    Ok(())
}
