
use sled;
use crate::coordinator::{ValidationStatus, calculate_hash};
use serde::{Serialize, Deserialize};
use anyhow::{anyhow, Result};

/// This function retrieves the salt associated with an email address and message ID.
/// If the email exists in the database, it returns true and the salt as a string.
/// If the email is not found, it stores the message id and returns false and that as the salt string.
pub async fn get_or_store_salt(email: &str, message_id: &str) -> Result<(bool, String)> {
    let db = match sled::open("./db/email_to_salt") {
        Ok(database) => database,
        Err(e) => return Err(anyhow!("Failed to open database: {}", e)),
    };
    let email_exists = db.get(email)?;
    if let Some(salt) = email_exists {
        Ok((true, std::str::from_utf8(&salt)?.to_string()))
    } else {
        db.insert(email, message_id)?;
        Ok((false, message_id.to_string()))
    }
}

/// Define the EmailData struct that the database will store.
/// Raw email is the raw email body as a string (including headers)
/// From is the raw sender email address
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmailData {
    pub body: String,
    pub from: String,
    pub subject: String,
    pub state: ValidationStatus,
}

/// This database maps ids (hashes) to a struct/JSON with raw emails, from email, subject, and validation status. 
/// This function extracts and returns all emails that have a pending validation status.
pub async fn get_pending_and_unvalidated_emails() -> Result<Vec<EmailData>> {
    let db = match sled::open("./db/email_statuses") {
        Ok(database) => database,
        Err(e) => return Err(anyhow!("Failed to open database: {}", e)),
    };
    let mut pending_emails = Vec::new();

    for result in db.iter() {
        let (_id, value) = result?;
        let email_data: EmailData = serde_json::from_slice(&value)?;
        if email_data.state == ValidationStatus::Pending || email_data.state == ValidationStatus::Unvalidated {
            pending_emails.push(email_data);
        }
    }

    Ok(pending_emails)
}

/// This function sets the email state given the raw email, from, subject, and state.
/// It first opens the database, creates an EmailData object, serializes it, calculates the email hash, and then inserts it into the database.
pub async fn set_email_state(raw_email: &str, from: &str, subject: &str, state: ValidationStatus) -> Result<()> {
    let db = match sled::open("./db/email_statuses") {
        Ok(database) => database,
        Err(e) => return Err(anyhow!("Failed to open database: {}", e)),
    };
    let email_data = EmailData {
        body: raw_email.to_string(),
        from: from.to_string(),
        subject: subject.to_string(),
        state,
    };
    let serialized_email_data = serde_json::to_vec(&email_data)?;
    let email_hash = calculate_hash(&email_data.body);
    db.insert(email_hash.as_bytes(), serialized_email_data)?;
    Ok(())
}

/// This function retrieves the email data from the database given the email hash as the DB ID.
pub async fn get_email_data(email_hash: &str) -> Result<EmailData> {
    let db = match sled::open("./db/email_statuses") {
        Ok(database) => database,
        Err(e) => return Err(anyhow!("Failed to open database: {}", e)),
    };

    let value = db.get(email_hash.as_bytes())?;
    let value_bytes = match value {
        Some(v) => v.clone().as_ref().to_vec(),
        None => return Err(anyhow!("No value found for key")),
    };
    let email_data: EmailData = serde_json::from_slice(&value_bytes)?;
    Ok(email_data)
}

/// This function retrieves the email data from the database given the email hash as the DB ID.
pub async fn get_email_data_from_email(raw_email: &str) -> Result<EmailData> {
    let db = match sled::open("./db/email_statuses") {
        Ok(database) => database,
        Err(e) => return Err(anyhow!("Failed to open database: {}", e)),
    };
    
    let email_hash = calculate_hash(&raw_email.to_string());
    let value = db.get(email_hash.as_bytes())?;
    let value_bytes = match value {
        Some(v) => v.clone().as_ref().to_vec(),
        None => return Err(anyhow!("No value found for key")),
    };
    let email_data: EmailData = serde_json::from_slice(&value_bytes)?;
    Ok(email_data)
}

/// This function updates the email state given the raw email.
/// It first retrieves the email data from the database, updates the state, and then reinserts it into the database.
pub async fn update_email_state_with_raw_email(raw_email: &str, state: ValidationStatus) -> Result<()> {
    let email_hash = calculate_hash(&raw_email.to_string());
    let mut email_data = get_email_data(&email_hash).await?;

    let db = match sled::open("./db/email_statuses") {
        Ok(database) => database,
        Err(e) => return Err(anyhow!("Failed to open database: {}", e)),
    };

    email_data.state = state;
    let serialized_email_data = serde_json::to_vec(&email_data)?;
    let email_hash = calculate_hash(&raw_email.to_string());
    db.insert(email_hash.as_bytes(), serialized_email_data)?;
    Ok(())
}

/// This function updates the email state given the email hash.
/// It first retrieves the email data from the database, updates the state, and then reinserts it into the database.
pub async fn update_email_state_with_hash(email_hash: &str, state: ValidationStatus) -> Result<()> {
    let db = match sled::open("./db/email_statuses") {
        Ok(database) => database,
        Err(e) => return Err(anyhow!("Failed to open database: {}", e)),
    };
    let mut email_data = get_email_data(&email_hash).await?;
    email_data.state = state;
    let serialized_email_data = serde_json::to_vec(&email_data)?;
    db.insert(email_hash.as_bytes(), serialized_email_data)?;
    Ok(())
}
