use crate::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct EmailAndStatus(pub(crate) String, pub(crate) EmailStatus);

#[derive(Serialize, Deserialize)]
pub(crate) enum EmailStatus {
    Unchecked,
    Checked,
    Executed,
}

pub(crate) fn is_valid(email: &ParsedEmail) -> bool {
    println!("{}", email.canonicalized_body);
    println!("{}", email.canonicalized_header);
    println!("{}", email.signature_string());
    println!("{}", email.public_key_string());

    true
}

pub(crate) async fn send_to_coordinator(email: &str) -> Result<()> {
    todo!()
}

pub(crate) async fn response_from_modal() -> Result<String> {
    todo!()
}

pub(crate) async fn send_response() -> Result<()> {
    todo!()
}
