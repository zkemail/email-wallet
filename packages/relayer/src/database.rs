use crate::*;

use std::path::Path;

use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Pool, Row, Sqlite};

#[derive(Serialize, Deserialize)]
pub(crate) struct EmailAndStatus {
    pub(crate) email: String,
    pub(crate) status: EmailStatus,
}

impl EmailAndStatus {
    pub(crate) fn new(email: String, status: EmailStatus) -> Self {
        Self { email, status }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub(crate) enum EmailStatus {
    Unchecked,
    Checked,
    Executed,
    Finalized,
}

pub(crate) struct Database {
    db: Pool<Sqlite>,
}

impl Database {
    pub(crate) async fn open(path: &Path) -> Result<Self> {
        let res = Self {
            db: SqlitePool::connect(path.to_str().unwrap())
                .await
                .map_err(|e| anyhow::anyhow!(e))?,
        };

        res.setup_database().await?;

        Ok(res)
    }

    pub(crate) async fn setup_database(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS emails (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT NOT NULL,
                status TEXT NOT NULL
            );",
        )
        .execute(&self.db)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                email_address TEXT PRIMARY KEY,
                viewing_key TEXT NOT NULL
            );",
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub(crate) async fn get_unhandled_emails(&self) -> Result<Vec<EmailAndStatus>> {
        let mut vec = Vec::new();

        let rows = sqlx::query("SELECT email, status FROM emails WHERE status != 'Finalized'")
            .fetch_all(&self.db)
            .await?;

        for row in rows {
            let email: String = row.get("email");
            let status = serde_json::from_str(row.get("status"))?;

            vec.push(EmailAndStatus::new(email, status))
        }

        Ok(vec)
    }

    pub(crate) async fn insert_email(&self, value: &EmailAndStatus) -> Result<()> {
        sqlx::query("INSERT INTO emails (email, status) VALUES (?, ?);")
            .bind(&value.email)
            .bind(serde_json::to_string(&value.status)?)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    // Result<bool> is bad - fix later
    pub(crate) async fn contains_finalized_email(&self, key: &str) -> Result<bool> {
        Ok(false)
    }
}
