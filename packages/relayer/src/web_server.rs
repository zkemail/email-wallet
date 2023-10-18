use crate::*;

use axum::{extract::Path, Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateAccountRequest {
    email_address: String,
}

#[derive(Deserialize)]
struct BalanceOfRequest {
    email_address: String,
    token: String,
}

async fn create_account(Json(payload): Json<CreateAccountRequest>) {
    println!("Creating account for email: {}", payload.email_address);
}

async fn balance_of(Path(params): Path<BalanceOfRequest>) {
    println!(
        "Getting balance of token: {} for email: {}",
        params.token, params.email_address
    );
}

pub(crate) async fn run_server(addr: &str) -> Result<()> {
    let app = Router::new()
        .route("/createAccount", axum::routing::post(create_account))
        .route("/balanceOf/:email/:token", axum::routing::get(balance_of));

    println!("Listening API at {}", addr);

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
