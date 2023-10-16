#![allow(clippy::upper_case_acronyms)]

use crate::*;

pub(crate) enum SubjectCommand {
    Send {},
    NFT {},
    Swap {},
    Exit {},
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
