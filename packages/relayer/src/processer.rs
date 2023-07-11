use anyhow::{anyhow, Result};
use fancy_regex::Regex;
use imap::types::Fetch;

use crate::imap_client::ImapClient;

#[derive(Debug)]
pub struct EmailProcesser {
    receiver: ImapClient,
}

impl EmailProcesser {
    const SUBJECT_REGEX: &'static str = r"Email Wallet Manipulation \d+";
    pub fn new(receiver: ImapClient) -> Self {
        Self { receiver }
    }

    fn process_one_fetched(fetch: Fetch) -> Result<()> {
        let envelope = fetch.envelope().ok_or(anyhow!("No envelope"))?;
        let subject = envelope.subject.as_ref().ok_or(anyhow!("No subject"))?;
        let subject_str = String::from_utf8(subject.to_vec())?;
        let subject_regex = Regex::new(Self::SUBJECT_REGEX)?;
        let manipulation_id = subject_regex
            .find(&subject_str)?
            .ok_or(anyhow!("No manipulation id"))?
            .as_str()
            .parse::<usize>()?;

        Ok(())
    }
}
