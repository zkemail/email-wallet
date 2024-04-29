use crate::*;

use std::sync::atomic::Ordering;

use axum::Router;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tower_http::cors::{AllowHeaders, AllowMethods, Any, CorsLayer};

#[derive(Serialize, Deserialize)]
pub struct EmailAddrCommitRequest {
    pub email_address: String,
    pub random: String,
}

#[derive(Serialize, Deserialize)]
pub struct UnclaimRequest {
    pub email_address: String,
    pub random: String,
    pub expiry_time: i64,
    pub is_fund: bool,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountRegistrationRequest {
    pub email_address: String,
    pub account_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountRegistrationResponse {
    pub account_key: String,
    pub wallet_addr: String,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatResponse {
    pub onboarding_tokens_distributed: u32,
    pub onboarding_tokens_left: u32,
}

#[named]
async fn unclaim(
    payload: UnclaimRequest,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    tx_claimer: UnboundedSender<Claim>,
) -> Result<String> {
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&payload.email_address);
    info!(
        LOG,
        "padded email address fields: {:?}",
        padded_email_addr.to_email_addr_fields(); "func" => function_name!()
    );
    let commit = padded_email_addr.to_commitment(&hex2field(&payload.random)?)?;
    info!(LOG, "commit {:?}", commit; "func" => function_name!());
    let id = chain_client
        .get_unclaim_id_from_tx_hash(&payload.tx_hash, payload.is_fund)
        .await?;
    info!(LOG, "id {:?}", id; "func" => function_name!());
    let claim = Claim {
        tx_hash: payload.tx_hash.clone(),
        id,
        email_address: payload.email_address.clone(),
        random: payload.random.clone(),
        commit: field2hex(&commit),
        expiry_time: payload.expiry_time,
        is_fund: payload.is_fund,
        is_announced: false,
        is_seen: false,
    };
    tx_claimer.send(claim)?;
    trace!(LOG, "claim sent to tx_claimer"; "func" => function_name!());

    Ok(format!(
        "Unclaimed {} for {} is accepted",
        if payload.is_fund { "fund" } else { "state" },
        payload.email_address
    ))
}

#[named]
pub async fn run_server(
    addr: &str,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    tx_claimer: UnboundedSender<Claim>,
    email_sender: EmailForwardSender,
) -> Result<()> {
    let chain_client_check_clone = Arc::clone(&chain_client);
    let chain_client_reveal_clone = Arc::clone(&chain_client);
    let tx_claimer_reveal_clone = tx_claimer.clone();

    let sender_for_nft_transfer_api = email_sender.clone();
    let sender_for_create_account_api = email_sender.clone();
    let sender_for_send_fn_api = email_sender.clone();
    let sender_for_recover_account_key_fn_api = email_sender.clone();

    let mut app = Router::new()
        .route(
            "/api/emailAddrCommit",
            axum::routing::post(move |payload: String| async move {
                info!(LOG, "/emailAddrCommit Received payload: {}", payload; "func" => function_name!());
                let json = serde_json::from_str::<EmailAddrCommitRequest>(&payload)
                    .map_err(|_| "Invalid payload json".to_string())
                    .unwrap();
                let padded_email_addr = PaddedEmailAddr::from_email_addr(&json.email_address);
                let commit = padded_email_addr
                    .to_commitment(&hex2field(&json.random).unwrap())
                    .unwrap();
                info!(LOG, "commit {:?}", commit; "func" => function_name!());
                field2hex(&commit)
            }),
        )
        .route(
            "/api/unclaim",
            axum::routing::post(move |payload: String| async move {
                info!(LOG, "/unclaim Received payload: {}", payload; "func" => function_name!());
                let json = serde_json::from_str::<UnclaimRequest>(&payload)
                    .map_err(|_| "Invalid payload json".to_string())?;
                unclaim(json, db, chain_client, tx_claimer)
                    .await
                    .map_err(|err| {
                        error!(LOG, "Failed to accept unclaim: {}", err; "func" => function_name!());
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
                info!(LOG, "/serveCheck Received payload: {}", payload; "func" => function_name!());
                let json = serde_json::from_str::<CheckRequest>(&payload)
                    .map_err(|_| "Invalid payload json".to_string())?;
                serve_check_request(json, chain_client_check_clone)
                    .await
                    .map_err(|err| {
                        error!(LOG, "Failed PSI check serve: {}", err; "func" => function_name!());
                        err.to_string()
                    })
            }),
        )
        .route(
            "/api/serveReveal/",
            axum::routing::post(move |payload: String| async move {
                info!(LOG, "/serveCheck Received payload: {}", payload; "func" => function_name!());
                let json = serde_json::from_str::<RevealRequest>(&payload)
                    .map_err(|_| "Invalid payload json".to_string())?;
                serve_reveal_request(json, chain_client_reveal_clone, tx_claimer_reveal_clone)
                    .await
                    .map_err(|err| {
                        error!(LOG, "Failed PSI reveal serve: {}", err; "func" => function_name!());
                        err.to_string()
                    })
            }),
        )
        .route(
            "/api/echo",
            axum::routing::get(move || async move { "Hello, world!" }),
        )
        .route(
            "/api/recoverAccountKey",
            axum::routing::post(move |payload: String| async move {
                info!(LOG, "/recoverAccountKey Received payload: {}", payload; "func" => function_name!());
                match recover_account_key_api_fn(payload).await {
                    Ok((request_id, email)) => {
                        sender_for_recover_account_key_fn_api.send(email).unwrap();
                        request_id.to_string()
                    }
                    Err(err) => {
                        error!(
                            LOG,
                            "Failed to accept recover account key: {}", err
                        );
                        err.to_string()
                    }
                }
            }),
        )
        .route(
            "/api/getWalletAddress",
            axum::routing::post::<_, _, (), _>(move |payload: String| async move {
                info!(LOG, "Get wallet address payload: {}", payload);
                match get_wallet_address_api_fn(payload).await {
                    Ok(wallet_addr) => wallet_addr,
                    Err(err) => {
                        error!(LOG, "Failed to accept get wallet address: {}", err);
                        err.to_string()
                    }
                }
            }),
        )
        .route(
            "/api/send",
            axum::routing::post::<_, _, (), _>(move |payload: String| async move {
                info!(LOG, "Send payload: {}", payload);
                match send_api_fn(payload).await {
                    Ok((request_id, email)) => {
                        sender_for_send_fn_api.send(email).unwrap();
                        request_id.to_string()
                    }
                    Err(err) => {
                        error!(LOG, "Failed to accept send: {}", err);
                        err.to_string()
                    }
                }
            }),
        )
        .route(
            "/api/createAccount",
            axum::routing::post::<_, _, (), _>(move |payload: String| async move {
                info!(LOG, "Create account payload: {}", payload);
                match create_account_api_fn(payload).await {
                    Ok((request_id, email)) => {
                        sender_for_create_account_api.send(email).unwrap();
                        request_id.to_string()
                    }
                    Err(err) => {
                        error!(LOG, "Failed to accept create account: {}", err);
                        err.to_string()
                    }
                }
            }),
        )
        .route(
            "/api/isAccountCreated",
            axum::routing::post::<_, _, (), _>(move |payload: String| async move {
                info!(LOG, "Is account created payload: {}", payload);
                match is_account_created_api_fn(payload).await {
                    Ok(status) => status.to_string(),
                    Err(err) => {
                        error!(LOG, "Failed to accept is account created: {}", err);
                        err.to_string()
                    }
                }
            }),
        )
        .route(
            "/api/nftTransfer",
            axum::routing::post::<_, _, (), _>(move |payload: String| async move {
                info!(LOG, "NFT transfer payload: {}", payload);
                match nft_transfer_api_fn(payload).await {
                    Ok((request_id, email)) => {
                        sender_for_nft_transfer_api.send(email).unwrap();
                        request_id.to_string()
                    }
                    Err(err) => {
                        error!(LOG, "Failed to accept nft transfer: {}", err);
                        err.to_string()
                    }
                }
            }),
        );

    app = app.layer(
        CorsLayer::new()
            .allow_methods(AllowMethods::any())
            .allow_headers(AllowHeaders::any())
            .allow_origin(Any),
    );

    trace!(LOG, "Listening API at {}", addr; "func" => function_name!());
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
