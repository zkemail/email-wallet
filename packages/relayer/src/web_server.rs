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
    chain_client: Arc<ChainClient>,
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

    let input = generate_account_creation_input(
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

    let data = AccountCreationInput::default();
    let res = chain_client.create_account(data).await.unwrap();

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
}

pub(crate) async fn run_server(
    addr: &str,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    tx_sender: UnboundedSender<EmailMessage>,
) -> Result<()> {
    let app = Router::new()
        .route(
            "/createAccount",
            axum::routing::post(move |payload: Json<CreateAccountRequest>| {
                let tx = tx_sender.clone();
                async move { create_account(payload, Arc::clone(&db), chain_client, tx).await }
            }),
        )
        .route("/balanceOf/:email/:token", axum::routing::get(balance_of));

    println!("Listening API at {}", addr);

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
