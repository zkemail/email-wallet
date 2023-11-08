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
    expiry_time: i64,
    is_fund: bool,
    tx_hash: String,
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
