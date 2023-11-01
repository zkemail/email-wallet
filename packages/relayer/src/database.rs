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
        // sqlx::query(
        //     "CREATE TABLE IF NOT EXISTS emails (
        //         email_hash TEXT PRIMARY KEY,
        //         email TEXT NOT NULL
        //     );",
        // )
        // .execute(&self.db)
        // .await?;

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
                expire_time BIGINT NOT NULL,
                is_fund BOOLEAN NOT NULL,
                is_announced BOOLEAN NOT NULL
            );",
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    // pub(crate) async fn get_unhandled_emails(&self) -> Result<Vec<String>> {
    //     let mut vec = Vec::new();

    //     let rows = sqlx::query("SELECT email FROM emails")
    //         .fetch_all(&self.db)
    //         .await?;

    //     for row in rows {
    //         let email: String = row.get("email");
    //         vec.push(email)
    //     }

    //     Ok(vec)
    // }

    // pub(crate) async fn insert_email(&self, email_hash: &str, email: &str) -> Result<()> {
    //     info!("email_hash {}", email_hash);
    //     let row = sqlx::query(
    //         "INSERT INTO emails (email_hash, email) VALUES ($1 $2) REtURNING (email_hash)",
    //     )
    //     .bind(email_hash)
    //     .bind(email)
    //     .fetch_one(&self.db)
    //     .await?;
    //     info!("inserted row: {}", row.get::<String, _>("email_hash"));
    //     Ok(())
    // }

    // pub(crate) async fn delete_email(&self, email_hash: &str) -> Result<()> {
    //     let row_affected = sqlx::query("DELETE FROM emails WHERE email_hash = $1")
    //         .bind(email_hash)
    //         .execute(&self.db)
    //         .await?
    //         .rows_affected();
    //     info!("deleted {} rows", row_affected);

    //     Ok(())
    // }

    // // Result<bool> is bad - fix later (possible solution: to output Result<ReturnStatus>
    // // where, ReturnStatus is some Enum ...
    // pub(crate) async fn contains_email(&self, email_hash: &str) -> Result<bool> {
    //     let result = sqlx::query("SELECT 1 FROM emails WHERE email_hash = $1")
    //         .bind(email_hash)
    //         .fetch_optional(&self.db)
    //         .await?;

    //     Ok(result.is_some())
    // }

    pub(crate) async fn insert_user(&self, email_address: &str, account_key: &str) -> Result<()> {
        let row = sqlx::query(
            "INSERT INTO users (email_address, account_key) VALUES ($1, $2) RETURNING *",
        )
        .bind(email_address)
        .bind(account_key)
        .fetch_one(&self.db)
        .await?;
        info!("inserted row: {}", row.get::<String, _>("email_address"));
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
        info!("now {}", now);
        let rows = sqlx::query("SELECT * FROM claims WHERE expire_time < $1")
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
        info!("expire_time {}", expire_time);
        let row = sqlx::query(
            "INSERT INTO claims (email_address, random, email_addr_commit, expire_time, is_fund, is_announced) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(email_address)
        .bind(random)
        .bind(commit)
        .bind(expire_time)
        .bind(is_fund)
        .bind(is_announced)
        .fetch_one(&self.db)
        .await?;
        info!(
            "inserted row: {}",
            row.get::<String, _>("email_addr_commit")
        );
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
