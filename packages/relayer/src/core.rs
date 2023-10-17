#![allow(clippy::upper_case_acronyms)]

use tokio::sync::mpsc::UnboundedSender;

use crate::*;

pub(crate) enum SubjectCommand {
    Send {},
    NFT {},
    Swap {},
    Exit {},
}

pub(crate) async fn handle_email(
    email: String,
    db: Arc<Database>,
    tx: UnboundedSender<String>,
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
        SubjectCommand::Send {} => {
            if !is_enough_balance(&from_address, &viewing_key, &parsed_email).await? {
                bail!(INSUFFICIENT_BALANCE);
            }
        }
        _ => todo!(),
    }

    db.remove_email(&email).await?;

    Ok(())
}

pub(crate) fn validate_subject(subject: &str) -> Result<SubjectCommand> {
    todo!()
}

pub(crate) async fn send_to_coordinator(email: &str) -> Result<()> {
    Ok(())
}

pub(crate) async fn response_from_modal() -> Result<String> {
    Ok(String::new())
}

pub(crate) async fn send_response() -> Result<()> {
    Ok(())
}
