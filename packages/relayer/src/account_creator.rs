#![allow(clippy::upper_case_acronyms)]

use crate::*;

use log::trace;
use tokio::sync::mpsc::UnboundedSender;

pub(crate) async fn create_account(
    email_address: String,
    account_key: Option<AccountKey>,
    db: Arc<Database>,
    chain_client: Arc<ChainClient>,
    tx: UnboundedSender<EmailMessage>,
) -> Result<()> {
    if db.contains_user(&email_address).await.unwrap() {
        return Err(anyhow!("User already exists".to_string()));
    }
    let account_key = if let Some(account_key) = account_key {
        account_key
    } else {
        AccountKey::new(rand::thread_rng())
    };
    let account_key = field2hex(&account_key.0);

    trace!("Generated account_key {account_key}");

    let input = generate_account_creation_input(
        CIRCUITS_DIR_PATH.get().unwrap(),
        &email_address,
        RELAYER_RAND.get().unwrap(),
        &account_key,
    )
    .await?;

    let (proof, pub_signals) =
        generate_proof(&input, "account_creation", PROVER_ADDRESS.get().unwrap()).await?;

    let data = AccountCreationInput {
        email_addr_pointer: u256_to_bytes32(&pub_signals[1]),
        account_key_commit: u256_to_bytes32(&pub_signals[2]),
        wallet_salt: u256_to_bytes32(&pub_signals[3]),
        psi_point: get_psi_point_bytes(pub_signals[4], pub_signals[5]),
        proof,
    };
    info!("Account creation data {:?}", data);
    let res = chain_client.create_account(data).await?;
    info!("account creation tx hash: {}", res);
    db.insert_user(&email_address, &account_key, &res).await?;
    tx.send(EmailMessage {
        subject: format!("New Account - CODE:{}", account_key),
        body: email_template_message(
            &format!(
                "New Email Wallet account was created for you; Your Account Key: {}",
                account_key
            ),
            &res,
        )
        .await?,
        to: email_address.to_string(),
        message_id: Some(account_key),
    })
    .unwrap();
    Ok(())
}
