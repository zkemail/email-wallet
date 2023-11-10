use crate::*;

use axum::{extract::Path, Json, Router};
use serde::Deserialize;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Deserialize)]
struct CreateAccountRequest {
    email_address: String,
}

#[derive(Deserialize)]
struct BalanceOfRequest {
    email_address: String,
    token: String,
}

async fn create_account(
    Json(payload): Json<CreateAccountRequest>,
    db: Arc<Database>,
    tx_sender: UnboundedSender<EmailMessage>,
) -> String {
    if db.contains_user(&payload.email_address).await.unwrap() {
        return "User already exists".to_string();
    }

    println!("Creating account for email: {}", payload.email_address);

    let account_key = AccountKey::new(rand::thread_rng());
    let account_key = field2hex(&account_key.0);

    println!("Generated account_key {account_key}");

    db.insert_user(&payload.email_address, &account_key)
        .await
        .unwrap();

    let input = generate_creation_input(
        CIRCUITS_DIR_PATH.get().unwrap(),
        &payload.email_address,
        RELAYER_RAND.get().unwrap(),
        &account_key,
    )
    .await
    .unwrap();

    let proof = generate_proof(
        &input,
        "generateCreationProof",
        PROVER_ADDRESS.get().unwrap(),
    )
    .await
    .unwrap();

    let data = AccountCreation::default();
    let res = call_account_creation_op(data).await.unwrap();

    tx_sender
        .send(EmailMessage {
            subject: format!("New Account - CODE:{}", account_key),
            body: format!(
                "New Account Was Created For You with Account Key set to {}",
                account_key
            ),
            to: payload.email_address.clone(),
            message_id: Some(account_key),
        })
        .unwrap();

    format!("Created account for emailaddress {}", payload.email_address)
}

async fn balance_of(Path(params): Path<BalanceOfRequest>) {
    println!(
        "Getting balance of token: {} for email: {}",
        params.token, params.email_address
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
    tx_sender: UnboundedSender<EmailMessage>,
) -> Result<()> {
    let chain_client_check_clone = Arc::clone(&chain_client);
    let chain_client_reveal_clone = Arc::clone(&chain_client);
    let tx_claimer_reveal_clone = tx_claimer.clone();

    let app = Router::new()
        .route(
            "/createAccount",
            axum::routing::post(move |payload: Json<CreateAccountRequest>| {
                let tx = tx_sender.clone();
                async move { create_account(payload, Arc::clone(&db), tx).await }
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

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
