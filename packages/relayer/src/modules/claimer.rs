#![allow(clippy::upper_case_acronyms)]

use crate::*;

use ethers::types::Address;
use relayer_utils::{
    field2hex, generate_claim_input, hex2field, u256_to_bytes32, AccountCode, AccountSalt,
    PaddedEmailAddr,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub tx_hash: String,
    pub id: U256,
    pub email_address: String,
    pub random: String,
    pub commit: String,
    pub expiry_time: i64,
    pub is_fund: bool,
    pub is_announced: bool,
    pub is_seen: bool,
}

#[named]
pub async fn claim_unclaims(mut claim: Claim) -> Result<EmailWalletEvent> {
    let need_creation = true;
    let is_seen = claim.is_seen;
    if DB
        .get_claims_by_id(&claim.id)
        .await?
        .into_iter()
        .filter(|c: &Claim| c.is_fund == claim.is_fund)
        .collect::<Vec<_>>().is_empty()
    {
        claim.is_seen = true;
        DB.insert_claim(&claim).await?;
        // let psi_client = PSIClient::new(
        //     claim.email_address.to_string(),
        //     claim.tx_hash.clone(),
        //     claim.id,
        //     claim.is_fund,
        // )
        // .await?;
        // need_creation = psi_client.check_and_reveal().await?;
    }
    if need_creation && !DB.contains_user(&claim.email_address).await.unwrap() {
        let account_code = AccountCode::new(rand::thread_rng());
        let account_code_str = field2hex(&account_code.0);
        // let psi_point = compute_psi_point(
        //     CIRCUITS_DIR_PATH.get().unwrap(),
        //     &claim.email_address,
        //     &account_code_str,
        // )
        // .await?;
        let account_salt = AccountSalt::new(
            &PaddedEmailAddr::from_email_addr(&claim.email_address),
            account_code,
        )?;
        // let tx_hash = CLIENT.register_psi_point(&psi_point, &account_salt).await?;
        // info!(LOG, "register psi point tx hash: {}", tx_hash; "func" => function_name!());
        let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;

        DB.insert_user(
            &claim.email_address,
            &account_code_str,
            "",
            false,
            &format!("0x{}", hex::encode(wallet_addr.as_bytes())),
        )
        .await?;
        return Ok(EmailWalletEvent::Invitation {
            email_addr: claim.email_address,
            account_code,
            is_first: true,
            tx_hash: "".to_string(),
        });
    }
    let account_code_str = if let Some(key) = DB.get_account_code(&claim.email_address).await? {
        key
    } else {
        return Ok(EmailWalletEvent::NoOp);
    };
    let is_account_created = CLIENT
        .check_if_account_created_by_account_code(&claim.email_address, &account_code_str)
        .await?;
    if !is_seen && !is_account_created {
        return Ok(EmailWalletEvent::Invitation {
            email_addr: claim.email_address,
            account_code: AccountCode(hex2field(&account_code_str)?),
            is_first: false,
            tx_hash: "".to_string(),
        });
        // return Err(anyhow!("Account not created"));
    } else if !is_account_created {
        return Err(anyhow!("Account not created"));
    }
    let account_code = AccountCode(hex2field(&account_code_str)?);
    let padded_email_addr = PaddedEmailAddr::from_email_addr(&claim.email_address);
    let account_salt = AccountSalt::new(&padded_email_addr, account_code)?;
    let now = now();

    let (unclaimed_fund, unclaimed_state) = if claim.is_fund {
        let unclaimed_fund = CLIENT.query_unclaimed_fund(claim.id).await?;
        if unclaimed_fund.expiry_time.as_u64() < u64::try_from(now).unwrap() {
            return Err(anyhow!("Claim expired"));
        }
        (Some(unclaimed_fund), None)
    } else {
        let unclaimed_state = CLIENT.query_unclaimed_state(claim.id).await?;
        if unclaimed_state.expiry_time.as_u64() < u64::try_from(now).unwrap() {
            return Err(anyhow!("Claim expired"));
        }
        if claim.is_announced
            && is_installed_extension(unclaimed_state.extension_addr, &account_salt).await?
        {
            return Err(anyhow!(
                "Unclaimed state anounces the email address but its extension is not installed."
            ));
        }
        (None, Some(unclaimed_state))
    };

    let input = generate_claim_input(
        &claim.email_address,
        &claim.random,
        &field2hex(&account_code.0),
    )
    .await?;
    let (proof, pub_signals) =
        generate_proof(&input, "claim", PROVER_ADDRESS.get().unwrap()).await?;
    info!(LOG, "original commit {}", claim.commit; "func" => function_name!());
    info!(LOG, "original randomness {}", claim.random; "func" => function_name!());
    info!(LOG, "commit in pub signals: {}", pub_signals[0]; "func" => function_name!());
    let data = ClaimInput {
        id: claim.id,
        recipient_account_salt: u256_to_bytes32(&pub_signals[1]),
        is_fund: claim.is_fund,
        proof,
    };
    let tx_hash = CLIENT.claim(data).await?;
    DB.delete_claim(&claim.id, claim.is_fund).await?;
    let wallet_addr = CLIENT.get_wallet_addr_from_salt(&account_salt.0).await?;
    Ok(EmailWalletEvent::Claimed {
        unclaimed_fund,
        unclaimed_state,
        email_addr: claim.email_address,
        is_fund: claim.is_fund,
        is_announced: claim.is_announced,
        recipient_account_code: account_code,
        tx_hash,
    })
}

async fn is_installed_extension(
    extension_addr: Address,
    account_salt: &AccountSalt,
) -> Result<bool> {
    let subject_templates = CLIENT
        .query_subject_templates_of_extension(extension_addr)
        .await?;
    let command = subject_templates[0][0].as_str();
    let installed_extension = CLIENT
        .query_user_extension_for_command(account_salt, command)
        .await?;
    Ok(installed_extension == extension_addr)
}

#[named]
pub async fn void_unclaims(claim: Claim) -> Result<EmailWalletEvent> {
    let now = now();
    let commit = hex2field(&claim.commit)?;
    DB.delete_claim(&claim.id, claim.is_fund).await?;
    info!(LOG, "claim deleted id {}", claim.id; "func" => function_name!());
    let (reply_msg, sender, tx_hash) = if claim.is_fund {
        let unclaimed_fund = CLIENT.query_unclaimed_fund(claim.id).await?;
        if unclaimed_fund.expiry_time.as_u64() > u64::try_from(now).unwrap() {
            return Err(anyhow!("Claim is not expired"));
        }
        let result = CLIENT.void(claim.id, true).await?;
        (
            format!("Voided fund: {}", unclaimed_fund.token_addr),
            unclaimed_fund.sender,
            result,
        )
    } else {
        let unclaimed_state = CLIENT.query_unclaimed_state(claim.id).await?;
        if unclaimed_state.expiry_time.as_u64() > u64::try_from(now).unwrap() {
            return Err(anyhow!("Claim is not expired"));
        }
        let result = CLIENT.void(claim.id, false).await?;
        (
            format!("Voided state: {}", unclaimed_state.extension_addr),
            unclaimed_state.sender,
            result,
        )
    };
    Ok(EmailWalletEvent::Voided { claim, tx_hash })
}
