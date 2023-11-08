use crate::*;

use anyhow::Ok;
use axum::Router;
use log::trace;
use serde::{Deserialize, Serialize};
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
    expiry_time: i64,
    is_fund: bool,
    tx_hash: String,
}

#[derive(Deserialize)]
struct AccountRegistrationRequest {
    email_address: String,
}

#[derive(Serialize)]
struct AccountRegistrationResponse {
    account_key: String,
    wallet_addr: String,
    onboard_token_sent: bool,
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
    let mut psi_res = vec![];
    let psi_condition = {
        let account_key = db.get_account_key(&payload.email_address).await?;
        (account_key.is_none()
            || !chain_client
                .check_if_account_initialized_by_account_key(
                    &payload.email_address,
                    &account_key.unwrap(),
                )
                .await?)
            && {
                trace!("Starting PSI");
                psi_res = psi_client.find().await?;
                !psi_res.is_empty()
            }
    };
    if psi_condition {
        trace!("Reveal PSI");
        psi_client
            .reveal(&psi_res.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
            .await?;

        Ok(format!(
            "Unclaimed {} for {} was sent to found relayer",
            if payload.is_fund { "fund" } else { "state" },
            payload.email_address
        ))
    } else {
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
}

async fn onboard(
    payload: AccountRegistrationRequest,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
) -> Result<axum::Json<AccountRegistrationResponse>> {
    let account_key = AccountKey::new(rand::thread_rng());
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&payload.email_address);
    let relayer_rand = RelayerRand(hex2field(RELAYER_RAND.get().unwrap())?);

    let account_key_commit =
        account_key.to_commitment(&padded_email_addr, &relayer_rand.hash()?)?;
    let wallet_salt = account_key.to_wallet_salt().unwrap().0;
    let wallet_addr = chain_client.get_wallet_addr_from_salt(&wallet_salt).await?;

    info!("Counterfactual wallet address for email: {}", wallet_addr);

    // TODO: Implement distribution limit
    let onboard_token_sent = chain_client
        .transfer_onboarding_tokens(wallet_addr.clone())
        .await?;

    return Ok(axum::Json(AccountRegistrationResponse {
        account_key: field2hex(&account_key_commit),
        wallet_addr: format!("{:?}", wallet_addr),
        onboard_token_sent: onboard_token_sent,
    }));
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

    let chain_client_onboard_clone = Arc::clone(&chain_client);
    let db_onboard_clone = Arc::clone(&db);

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
            "/api/onboard",
            axum::routing::post(move |payload: String| async move {
                info!("/onboard Received payload: {}", payload);
                let json = serde_json::from_str::<AccountRegistrationRequest>(&payload)
                    .map_err(|_| "Invalid payload json".to_string())?;

                onboard(json, db_onboard_clone, chain_client_onboard_clone)
                    .await
                    .map_err(|err| {
                        error!("Failed to accept unclaim: {}", err);
                        err.to_string()
                    })
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
                        error!("Failed to accept unclaim: {}", err);
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
