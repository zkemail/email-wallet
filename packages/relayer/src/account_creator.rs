#![allow(clippy::upper_case_acronyms)]

use crate::*;

use tokio::sync::mpsc::UnboundedSender;

#[named]
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
    let is_account_key_none = account_key.is_none();
    let account_key = if let Some(account_key) = account_key {
        account_key
    } else {
        AccountKey::new(rand::thread_rng())
    };
    let account_key_str = field2hex(&account_key.0);

    trace!(LOG, "Generated account_key {account_key_str}"; "func" => function_name!());

    let input = generate_account_creation_input(
        CIRCUITS_DIR_PATH.get().unwrap(),
        &email_address,
        RELAYER_RAND.get().unwrap(),
        &account_key_str,
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
    info!(LOG, "Account creation data {:?}", data; "func" => function_name!());
    let res = chain_client.create_account(data).await?;
    info!(LOG, "account creation tx hash: {}", res; "func" => function_name!());
    let wallet_salt = account_key.to_wallet_salt()?;
    let wallet_addr = chain_client
        .get_wallet_addr_from_salt(&wallet_salt.0)
        .await?;
    let token_transfered = chain_client
        .free_mint_test_erc20(wallet_addr, ethers::utils::parse_ether("100")?)
        .await?;

    db.insert_user(&email_address, &account_key_str, &res, false)
        .await?;
    if is_account_key_none {
        tx.send(EmailMessage {
            to: email_address.to_string(),
            email_args: EmailArgs::AccountCreation {
                user_email_addr: email_address,
            },
            account_key: Some(account_key_str),
            wallet_addr: None,
            tx_hash: Some(res),
        })
        .unwrap();
    }
    Ok(())
}
