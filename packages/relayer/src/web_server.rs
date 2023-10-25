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
    tx_creator: UnboundedSender<String>,
) -> String {
    if db.contains_user(&payload.email_address).await.unwrap() {
        return "User already exists".to_string();
    }

    println!("Creating account for email: {}", payload.email_address);
    tx_creator
        .send(payload.email_address.clone())
        .map_err(|err| return err.to_string());

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
    tx_creator: UnboundedSender<String>,
) -> Result<()> {
    let app = Router::new()
        .route(
            "/createAccount",
            axum::routing::post(move |payload: Json<CreateAccountRequest>| {
                let tx = tx_sender.clone();
                async move { create_account(payload, Arc::clone(&db), tx_creator).await }
            }),
        )
        .route("/balanceOf/:email/:token", axum::routing::get(balance_of));

    println!("Listening API at {}", addr);

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
