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
    )
    .await?;

    let (proof, pub_signals) =
        generate_proof(&input, "account_creation", PROVER_ADDRESS.get().unwrap()).await?;

    let domain = pub_signals[0..9].to_vec();

    let email_proof = EmailProof {
        domain: domain
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("."),
        dkim_public_key_hash: u256_to_bytes32(&pub_signals[9]),
        nullifier: u256_to_bytes32(&pub_signals[10]),
        timestamp: pub_signals[11],
        proof: proof,
    };

    let data = AccountCreationInput {
        wallet_salt: u256_to_bytes32(&pub_signals[12]),
        psi_point: get_psi_point_bytes(pub_signals[13], pub_signals[14]),
        proof: email_proof,
    };
    info!(LOG, "Account creation data {:?}", data; "func" => function_name!());
    let res = chain_client.create_account(data).await?;
    info!(LOG, "account creation tx hash: {}", res; "func" => function_name!());
    let wallet_salt = WalletSalt::new(
        &PaddedEmailAddr::from_email_addr(&email_address),
        account_key,
    )
    .unwrap();
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
