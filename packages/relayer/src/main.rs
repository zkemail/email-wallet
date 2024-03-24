use anyhow::Result;
use relayer::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "setup" {
        return setup().await;
    } else {
        let (sender, rx) = EmailForwardSender::new();
        run(RelayerConfig::new(), event_consumer, sender, rx).await?;
    }
    Ok(())
}
