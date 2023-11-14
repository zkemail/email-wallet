use crate::*;

use std::sync::atomic::Ordering;

use axum::Router;
use log::trace;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tower_http::cors::{AllowHeaders, AllowMethods, Any, CorsLayer};

#[derive(Deserialize)]
struct EmailAddrCommitRequest {
    email_address: String,
    random: String,
}

#[derive(Deserialize)]
struct UnclaimRequest {
    email_address: String,
    random: String,
    expiry_time: i64,
    is_fund: bool,
    tx_hash: String,
}

#[derive(Deserialize)]
struct AccountRegistrationRequest {
    email_address: String,
    account_key: String,
}

#[derive(Serialize)]
struct AccountRegistrationResponse {
    account_key: String,
    wallet_addr: String,
    tx_hash: String,
}

#[derive(Serialize)]
struct StatResponse {
    onboarding_tokens_distributed: u32,
    onboarding_tokens_left: u32,
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
    let id = chain_client
        .get_unclaim_id_from_tx_hash(&payload.tx_hash, payload.is_fund)
        .await?;
    info!("id {:?}", id);
    let psi_client = PSIClient::new(
        Arc::clone(&chain_client),
        payload.email_address.clone(),
        id,
        payload.is_fund,
    )
    .await?;
    psi_client
        .check_and_reveal(db.clone(), chain_client.clone(), &payload.email_address)
        .await?;
    let claim = Claim {
        id,
        email_address: payload.email_address.clone(),
        random: payload.random.clone(),
        commit: field2hex(&commit),
        expiry_time: payload.expiry_time,
        is_fund: payload.is_fund,
        is_announced: false,
    };
    tx_claimer.send(claim)?;
    trace!("claim sent to tx_claimer");

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
    let chain_client_check_clone = Arc::clone(&chain_client);
    let chain_client_reveal_clone = Arc::clone(&chain_client);
    let tx_claimer_reveal_clone = tx_claimer.clone();

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
                unclaim(json, db, chain_client, tx_claimer)
                    .await
                    .map_err(|err| {
                        error!("Failed to accept unclaim: {}", err);
                        err.to_string()
                    })
            }),
        )
        .route(
            "/api/stats",
            axum::routing::get(move || async move {
                let stats = StatResponse {
                    onboarding_tokens_distributed: ONBOARDING_COUNTER.load(Ordering::SeqCst),
                    onboarding_tokens_left: *ONBOARDING_TOKEN_DISTRIBUTION_LIMIT.get().unwrap()
                        - ONBOARDING_COUNTER.load(Ordering::SeqCst),
                };
                axum::Json(stats)
            }),
        )
        .route(
            "/api/serveCheck/",
            axum::routing::post(move |payload: String| async move {
                info!("/serveCheck Received payload: {}", payload);
                let json = serde_json::from_str::<CheckRequest>(&payload)
                    .map_err(|_| "Invalid payload json".to_string())?;
                serve_check_request(json, chain_client_check_clone)
                    .await
                    .map_err(|err| {
                        error!("Failed PSI check serve: {}", err);
                        err.to_string()
                    })
            }),
        )
        .route(
            "/api/serveReveal/",
            axum::routing::post(move |payload: String| async move {
                info!("/serveCheck Received payload: {}", payload);
                let json = serde_json::from_str::<RevealRequest>(&payload)
                    .map_err(|_| "Invalid payload json".to_string())?;
                serve_reveal_request(json, chain_client_reveal_clone, tx_claimer_reveal_clone)
                    .await
                    .map_err(|err| {
                        error!("Failed PSI reveal serve: {}", err);
                        err.to_string()
                    })
            }),
        )
        .layer(
            CorsLayer::new()
                .allow_methods(AllowMethods::any())
                .allow_headers(AllowHeaders::any())
                .allow_origin(Any),
        );

    trace!("Listening API at {}", addr);
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
