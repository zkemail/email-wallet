use crate::*;

use std::path::Path;

use sled::Db;

pub(crate) struct Database {
    db: Db,
}

impl Database {
    pub(crate) fn open(path: &Path) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Database { db })
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

    pub(crate) fn insert(&self, key: &[u8], value: &str) -> Result<()> {
        // sha256
        // self.db.insert(value)?;
        todo!()
    }

    pub(crate) fn insert_raw_email(&self, value: &str) -> Result<()> {
        // sha256
        // self.db.insert(value)?;
        todo!()
    }

    pub(crate) fn remove(&self, email: &str) -> Result<()> {
        todo!()
    }
}
