use crate::*;
use anyhow::Result;
use async_trait::async_trait;
use std::fs;
use std::path::PathBuf;

#[async_trait]
pub trait EmailsPool {
    async fn get_unhandled_emails(&self) -> Result<Vec<(String, String)>>;

    async fn get_email_by_hash(&self, email_hash: &str) -> Result<String>;

    async fn insert_email(&self, email_hash: &str, email: &str) -> Result<()>;

    async fn delete_email(&self, email_hash: &str) -> Result<()>;

    // Result<bool> is bad - fix later (possible solution: to output Result<ReturnStatus>
    // where, ReturnStatus is some Enum ...
    async fn contains_email(&self, email_hash: &str) -> Result<bool>;
}

pub struct FileEmailsPool {
    dir_path: String,
}

#[async_trait]
impl EmailsPool for FileEmailsPool {
    async fn get_unhandled_emails(&self) -> Result<Vec<(String, String)>> {
        let dir = fs::read_dir(&self.dir_path)?;
        let mut emails = Vec::new();
        for path in dir.into_iter() {
            let path = path?.path();
            let email = fs::read_to_string(path)?;
            emails.push((calculate_default_hash(&email), email));
        }
        Ok(emails)
    }

    async fn get_email_by_hash(&self, email_hash: &str) -> Result<String> {
        let file_path = self.email_hash_to_path(email_hash);
        let email = fs::read_to_string(file_path)?;
        Ok(email)
    }

    async fn insert_email(&self, email_hash: &str, email: &str) -> Result<()> {
        let file_path = self.email_hash_to_path(email_hash);
        fs::write(file_path, email)?;
        Ok(())
    }

    async fn delete_email(&self, email_hash: &str) -> Result<()> {
        let file_path = self.email_hash_to_path(email_hash);
        fs::remove_file(file_path)?;
        Ok(())
    }

    async fn contains_email(&self, email_hash: &str) -> Result<bool> {
        let file_path = self.email_hash_to_path(email_hash);
        Ok(file_path.exists())
    }
}

impl FileEmailsPool {
    #[named]
    pub fn new() -> Self {
        let dir_path = PathBuf::new()
            .join(env!("CARGO_MANIFEST_DIR"))
            .join("received_emails")
            .to_str()
            .unwrap()
            .to_string();
        info!(LOG, "dir_path: {}", dir_path; "func" => function_name!());
        fs::create_dir_all(&dir_path).unwrap();
        Self { dir_path }
    }

    fn email_hash_to_path(&self, email_hash: &str) -> PathBuf {
        PathBuf::from(&self.dir_path).join(format!("{}.eml", email_hash))
    }
}
