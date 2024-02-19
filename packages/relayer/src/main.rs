use relayer::*;

use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 2 && args[1] == "setup" {
        return setup().await;
    } else {
        let (_, rx) = EmailForwardSender::new();
        let event_consumer_fn = |event: EmailWalletEvent| {
            match event {
                EmailWalletEvent::PsiRegistered {
                    email_addr,
                    account_key,
                    tx_hash,
                } => {}
                _ => {}
            }
            Ok(())
        };
        let event_consumer = EventConsumer::new(Box::new(event_consumer_fn));
        let routes = vec![];
        run(RelayerConfig::new(), event_consumer, rx, routes).await?;
    }
    Ok(())
}
