use crate::*;

use sqlx::{postgres::PgPool, Row};

pub(crate) struct Database {
    db: PgPool,
}

impl Database {
    pub(crate) async fn open(path: &str) -> Result<Self> {
        let res = Self {
            db: PgPool::connect(path)
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
                account_key TEXT NOT NULL
            );",
        )
        .execute(&self.db)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS claims (
                email_address TEXT NOT NULL,
                random TEXT NOT NULL,
                email_addr_commit TEXT NOT NULL,
                expire_time INTEGER NOT NULL,
                is_fund BOOLEAN NOT NULL,
                is_announced BOOLEAN NOT NULL
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
        sqlx::query("INSERT INTO emails (email) VALUES ($1)")
            .bind(key)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub(crate) async fn delete_email(&self, key: &str) -> Result<()> {
        sqlx::query("DELETE FROM emails WHERE email = $1")
            .bind(key)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    // Result<bool> is bad - fix later (possible solution: to output Result<ReturnStatus>
    // where, ReturnStatus is some Enum ...
    pub(crate) async fn contains_email(&self, key: &str) -> Result<bool> {
        let result = sqlx::query("SELECT 1 FROM emails WHERE email = $1")
            .bind(key)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.is_some())
    }

    pub(crate) async fn insert_user(&self, email_address: &str, account_key: &str) -> Result<()> {
        sqlx::query("INSERT INTO users (email_address, account_key) VALUES ($1, $2)")
            .bind(email_address)
            .bind(account_key)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub(crate) async fn get_claims_by_commit(&self, commit: &str) -> Result<Vec<Claim>> {
        let mut vec = Vec::new();

        let rows = sqlx::query("SELECT * FROM claims WHERE email_addr_commit = $1")
            .bind(commit)
            .fetch_all(&self.db)
            .await?;

        for row in rows {
            let commit: String = row.get("email_addr_commit");
            let email_address: String = row.get("email_address");
            let random: String = row.get("random");
            let expire_time: i64 = row.get("expire_time");
            let is_fund: bool = row.get("is_fund");
            let is_announced: bool = row.get("is_announced");
            vec.push(Claim {
                email_address,
                random,
                commit,
                expire_time,
                is_fund,
                is_announced,
            })
        }
        Ok(vec)
    }

    pub(crate) async fn get_claims_by_email_addr(&self, email_addr: &str) -> Result<Vec<Claim>> {
        let mut vec = Vec::new();

        let rows = sqlx::query("SELECT * FROM claims WHERE email_address = $1")
            .bind(email_addr)
            .fetch_all(&self.db)
            .await?;

        for row in rows {
            let commit: String = row.get("email_addr_commit");
            let email_address: String = row.get("email_address");
            let random: String = row.get("random");
            let expire_time: i64 = row.get("expire_time");
            let is_fund: bool = row.get("is_fund");
            let is_announced: bool = row.get("is_announced");
            vec.push(Claim {
                email_address,
                random,
                commit,
                expire_time,
                is_fund,
                is_announced,
            })
        }
        Ok(vec)
    }

    pub(crate) async fn get_claims_expired(&self, now: i64) -> Result<Vec<Claim>> {
        let mut vec = Vec::new();

        let rows = sqlx::query("SELECT * FROM claims WHERE expire_time > $1")
            .bind(now)
            .fetch_all(&self.db)
            .await?;

        for row in rows {
            let commit: String = row.get("email_addr_commit");
            let email_address: String = row.get("email_address");
            let random: String = row.get("random");
            let expire_time: i64 = row.get("expire_time");
            let is_fund: bool = row.get("is_fund");
            let is_announced: bool = row.get("is_announced");
            vec.push(Claim {
                email_address,
                random,
                commit,
                expire_time,
                is_fund,
                is_announced,
            })
        }
        Ok(vec)
    }

    pub(crate) async fn insert_claim(
        &self,
        email_address: &str,
        random: &str,
        commit: &str,
        expire_time: i64,
        is_fund: bool,
        is_announced: bool,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO claims (email_address, random, email_addr_commit, expire_time, is_fund, is_announced) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(email_address)
        .bind(random)
        .bind(commit)
        .bind(expire_time)
        .bind(is_fund)
        .bind(is_announced)
        .execute(&self.db)
        .await?;
        Ok(())
    }

    pub(crate) async fn delete_claim(&self, commit: &str, is_fund: bool) -> Result<()> {
        sqlx::query("DELETE FROM claims WHERE email_addr_commit = $1 AND is_fund = $2")
            .bind(commit)
            .bind(is_fund)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    pub(crate) async fn contains_user(&self, email_address: &str) -> Result<bool> {
        let result = sqlx::query("SELECT 1 FROM users WHERE email_address = $1")
            .bind(email_address)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.is_some())
    }

    pub(crate) async fn get_account_key(&self, email_address: &str) -> Result<Option<String>> {
        let row_result = sqlx::query("SELECT account_key FROM users WHERE email_address = $1")
            .bind(email_address)
            .fetch_one(&self.db)
            .await;

        match row_result {
            Ok(row) => {
                let account_key: String = row.get("account_key");
                Ok(Some(account_key))
            }
            Err(sqlx::error::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e).map_err(|e| anyhow::anyhow!(e))?,
        }
    }
}
