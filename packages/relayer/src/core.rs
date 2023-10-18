#![allow(clippy::upper_case_acronyms)]

use ethers::types::U256;
use tokio::sync::mpsc::UnboundedSender;

use crate::*;

pub(crate) enum SubjectCommand {
    Send {
        amount: U256,
        token: String,
        recipient: String,
    },
    NFT {},
    Swap {},
    Exit {},
}

pub(crate) async fn handle_email(
    email: String,
    db: Arc<Database>,
    tx: UnboundedSender<(String, String)>,
) -> Result<()> {
    let parsed_email = ParsedEmail::new_from_raw_email(&email).await?;

    let from_address = parsed_email.get_from_addr()?;
    let viewing_key = match db.get_viewing_key(&from_address).await? {
        Some(viewing_key) => viewing_key,
        None => {
            db.remove_email(&email).await?;
            bail!(NOT_MY_SENDER);
        }
    };

    let subject_command = validate_subject(&parsed_email.get_subject_all()?)?;
    match subject_command {
        SubjectCommand::Send {
            amount,
            token,
            recipient,
        } => {
            if !is_enough_balance(&from_address, &viewing_key, &parsed_email).await? {
                bail!(INSUFFICIENT_BALANCE);
            }

            let recipient_viewing_key = match db.get_viewing_key(&recipient).await? {
                Some(viewing_key) => viewing_key,
                None => {
                    todo!("PSI")
                }
            };

            let input = std::process::Command::new("").output()?.stdout;
            let input = String::from_utf8(input)?;

            let proof = generate_proof(&input).await?;

            let response = send_to_chain(&proof).await?;

            tx.send((
                format!("Transaction was sent! That's the tx hash: {}", response),
                from_address,
            ))?;
        }
        _ => todo!(),
    }

    db.remove_email(&email).await?;

    Ok(())
}

pub(crate) fn validate_subject(subject: &str) -> Result<SubjectCommand> {
    todo!()
}

pub(crate) async fn generate_proof(input: &str) -> Result<String> {
    todo!()
}
