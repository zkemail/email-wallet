use crate::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct EmailAndStatus(pub(crate) String, pub(crate) EmailStatus);

#[derive(Serialize, Deserialize, PartialEq)]
pub(crate) enum EmailStatus {
    Unchecked,
    Checked,
    Executed,
    Finalized,
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

pub(crate) fn calculate_hash(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish().to_string()
}
