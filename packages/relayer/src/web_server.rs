use crate::*;

use axum::{extract::Path, Json, Router};
use log::{debug, error, info, trace, warn};
use serde::Deserialize;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Deserialize)]
struct UnclaimRequest {
    email_address: String,
    random: String,
    expire_time: i64,
    is_fund: bool,
    tx_hash: String,
}

#[derive(Deserialize)]
struct BalanceOfRequest {
    email_address: String,
    token: String,
}

async fn unclaim(
    Json(payload): Json<UnclaimRequest>,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    tx_claimer: UnboundedSender<Claim>,
) -> Result<String> {
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&payload.email_address);
    let commit = padded_email_addr.to_commitment(&hex2field(&payload.random)?)?;
    let claim = Claim {
        email_address: payload.email_address.clone(),
        random: payload.random.clone(),
        commit: field2hex(&commit),
        expire_time: payload.expire_time,
        is_fund: payload.is_fund,
        is_announced: false,
    };
    tx_claimer.send(claim)?;
    Ok(format!(
        "Unclaimed {} for {} is accepted",
        if payload.is_fund { "fund" } else { "state" },
        payload.email_address
    ))
}

// async fn balance_of(Path(params): Path<BalanceOfRequest>) {
//     trace!(
//         "Getting balance of token: {} for email: {}",
//         params.token,
//         params.email_address
//     );
// }

pub(crate) async fn run_server(
    addr: &str,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    tx_claimer: UnboundedSender<Claim>,
) -> Result<()> {
    let app = Router::new().route(
        "/unclaim",
        axum::routing::post(move |payload: Json<UnclaimRequest>| async move {
            unclaim(
                payload,
                Arc::clone(&db),
                Arc::clone(&chain_client),
                tx_claimer,
            )
            .await
            .map_err(|err| err.to_string())
        }),
    );

    trace!("Listening API at {}", addr);

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
