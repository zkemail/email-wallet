use crate::*;

use std::path::Path;

use sqlx::{sqlite::SqlitePool, Pool, Row, Sqlite};

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
                email TEXT PRIMARY KEY
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

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS claims (
                email_address TEXT PRIMARY KEY,
                random TEXT NOT NULL, 
                commit TEXT UNIQUE
            );",
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub(crate) async fn get_unhandled_emails(&self) -> Result<Vec<String>> {
        let mut vec = Vec::new();

        let rows = sqlx::query("SELECT email FROM emails")
            .fetch_all(&self.db)
            .await?;

        for row in rows {
            let email: String = row.get("email");
            vec.push(email)
        }

        Ok(vec)
    }

    pub(crate) async fn insert_email(&self, key: &str) -> Result<()> {
        sqlx::query("INSERT INTO emails (email) VALUES (?)")
            .bind(key)
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
    pub(crate) async fn contains_email(&self, key: &str) -> Result<bool> {
        let result = sqlx::query("SELECT 1 FROM emails WHERE email = ?")
            .bind(key)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.is_some())
    }

    pub(crate) async fn insert_user(&self, email_address: &str, viewing_key: &str) -> Result<()> {
        sqlx::query("INSERT INTO users (email_address, viewing_key) VALUES (?, ?)")
            .bind(email_address)
            .bind(viewing_key)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub(crate) async fn insert_claim(
        &self,
        email_address: &str,
        random: &str,
        commit: &str,
    ) -> Result<()> {
        sqlx::query("INSERT INTO claims (email_address, random, commit) VALUES (?, ?, ?)")
            .bind(email_address)
            .bind(random)
            .bind(commit)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub(crate) async fn contains_user(&self, email_address: &str) -> Result<bool> {
        let result = sqlx::query("SELECT 1 FROM users WHERE email_address = ?")
            .bind(email_address)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.is_some())
    }

    pub(crate) async fn get_viewing_key(&self, email_address: &str) -> Result<Option<String>> {
        let row_result = sqlx::query("SELECT viewing_key FROM users WHERE email_address = ?")
            .bind(email_address)
            .fetch_one(&self.db)
            .await;

        match row_result {
            Ok(row) => {
                let viewing_key: String = row.get("viewing_key");
                Ok(Some(viewing_key))
            }
            Err(sqlx::error::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e).map_err(|e| anyhow::anyhow!(e))?,
        }
    }
}
