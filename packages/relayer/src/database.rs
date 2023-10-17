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

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub(crate) enum EmailStatus {
    Unchecked,
    Executed,
    Finalized,
}

pub(crate) struct User {
    pub(crate) email_address: String,
    pub(crate) viewing_key: String,
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
                email TEXT PRIMARY KEY,
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
        sqlx::query(
            "INSERT INTO emails (email, status) VALUES (?, ?)
             ON CONFLICT(email) DO UPDATE SET status = excluded.status;",
        )
        .bind(&value.email)
        .bind(serde_json::to_string(&value.status)?)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub(crate) async fn remove_email(&self, key: &str) -> Result<()> {
        sqlx::query("DELETE FROM emails WHERE email = ?")
            .bind(key)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    // Result<bool> is bad - fix later (possible solution: to output Result<ReturnStatus>
    // where, ReturnStatus is some Enum ...
    pub(crate) async fn contains_finalized_email(&self, key: &str) -> Result<bool> {
        let result = sqlx::query("SELECT 1 FROM emails WHERE email = ? AND status = 'Finalized'")
            .bind(key)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.is_some())
    }

    pub(crate) async fn insert_user(&self, user: User) -> Result<()> {
        sqlx::query("INSERT INTO users (email_address, viewing_key) VALUES (?, ?)")
            .bind(&user.email_address)
            .bind(&user.viewing_key)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub(crate) async fn get_user(&self, email_address: &str) -> Result<Option<User>> {
        let row_result = sqlx::query("SELECT viewing_key FROM users WHERE email_address = ?")
            .bind(email_address)
            .fetch_one(&self.db)
            .await;

        match row_result {
            Ok(row) => {
                let viewing_key: String = row.get("viewing_key");
                Ok(Some(User {
                    email_address: email_address.to_string(),
                    viewing_key,
                }))
            }
            Err(sqlx::error::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e).map_err(|e| anyhow::anyhow!(e))?,
        }
    }
}
