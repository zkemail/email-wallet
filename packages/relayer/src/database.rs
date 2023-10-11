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
        let mut values = vec![];
        for result in self.db.iter() {
            let (_, value) = result?;
            let value = String::from_utf8(value.to_vec())?;
            let EmailAndStatus(raw_email, status) = serde_json::from_str(&value)?;
            if EmailStatus::Finalized != status {
                values.push(value);
            }
        }

        Ok(values)
    }

    pub(crate) fn insert(&self, key: &[u8], value: &str) -> Result<()> {
        self.db.insert(key, value)?;
        Ok(())
    }

    pub(crate) fn insert_raw_email(&self, value: &str) -> Result<()> {
        self.db.insert(calculate_hash(value), value)?;
        Ok(())
    }

    pub(crate) fn remove(&self, value: &str) -> Result<()> {
        self.db.remove(calculate_hash(value))?;
        Ok(())
    }

    // Result<bool> is bad - fix later
    pub(crate) fn contains_finalized(&self, key: &str) -> Result<bool> {
        let key = calculate_hash(key);
        if !self.db.contains_key(&key).unwrap() {
            return Ok(false);
        }

        let value = self.db.get(&key).unwrap().unwrap();
        let value = String::from_utf8(value.to_vec()).unwrap();
        let EmailAndStatus(raw_email, status) = serde_json::from_str(&value)?;
        Ok(status == EmailStatus::Finalized)
    }
}
