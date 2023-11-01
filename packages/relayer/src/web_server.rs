use crate::*;

use axum::Router;
use log::trace;
use serde::Deserialize;
use tokio::sync::mpsc::UnboundedSender;
use tower_http::cors::{AllowMethods, Any, CorsLayer};

#[derive(Deserialize)]
struct EmailAddrCommitRequest {
    email_address: String,
    random: String,
}

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
    payload: UnclaimRequest,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    tx_claimer: UnboundedSender<Claim>,
) -> Result<String> {
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&payload.email_address);
    info!(
        "padded email address fields: {:?}",
        padded_email_addr.to_email_addr_fields()
    );
    let commit = padded_email_addr.to_commitment(&hex2field(&payload.random)?)?;
    info!("commit {:?}", commit);
    info!(
        "commit derived from the provided email address and randomness: {}",
        field2hex(&commit)
    );
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

pub(crate) async fn run_server(
    addr: &str,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    tx_claimer: UnboundedSender<Claim>,
) -> Result<()> {
    let app = Router::new()
        .route(
            "/api/emailAddrCommit",
            axum::routing::post(move |payload: String| async move {
                info!("/emailAddrCommit Received payload: {}", payload);
                let json = serde_json::from_str::<EmailAddrCommitRequest>(&payload)
                    .map_err(|_| "Invalid payload json".to_string())
                    .unwrap();
                let padded_email_addr = PaddedEmailAddr::from_email_addr(&json.email_address);
                let commit = padded_email_addr
                    .to_commitment(&hex2field(&json.random).unwrap())
                    .unwrap();
                info!("commit {:?}", commit);
                field2hex(&commit)
            }),
        )
        .route(
            "/api/unclaim",
            axum::routing::post(move |payload: String| async move {
                info!("/unclaim Received payload: {}", payload);
                let json = serde_json::from_str::<UnclaimRequest>(&payload)
                    .map_err(|_| "Invalid payload json".to_string())?;
                unclaim(json, Arc::clone(&db), Arc::clone(&chain_client), tx_claimer)
                    .await
                    .map_err(|err| {
                        error!("Failed to accept unclaim: {}", err);
                        err.to_string()
                    })
            }),
        )
        .layer(
            CorsLayer::new()
                .allow_methods(AllowMethods::any())
                .allow_origin(Any),
        );

    trace!("Listening API at {}", addr);
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
