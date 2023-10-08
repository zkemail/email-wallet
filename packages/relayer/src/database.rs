use std::path::Path;

use crate::*;

use sled::Db;

pub(crate) struct Database {
    db: Db,
}

impl Database {
    pub(crate) fn open(path: &Path) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Database { db })
    }

    pub(crate) fn get_or_store_salt(&self, email: &str, message_id: &str) -> Result<String> {
        match self.db.get(email).unwrap() {
            Some(salt) => Ok(std::str::from_utf8(&salt)?.to_string()),
            None => {
                self.db.insert(email, message_id)?;
                Ok(message_id.to_string())
            }
        }
    }

    pub(crate) fn get_unhandled_emails(&self) -> Result<Vec<String>> {
        let mut emails = vec![];
        for result in self.db.iter() {
            let (_, value) = result?;
            let email = String::from_utf8(value.to_vec())?;
            emails.push(email);
        }

        Ok(emails)
    }

    pub(crate) fn insert(&self, email: &str) -> Result<()> {
        todo!()
    }

    pub(crate) fn remove(&self, email: &str) -> Result<()> {
        todo!()
    }
}
