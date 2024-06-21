use crate::*;

use sqlx::{postgres::PgPool, Row};

pub struct Database {
    db: PgPool,
}

impl Database {
    pub async fn open(path: &str) -> Result<Self> {
        let res = Self {
            db: PgPool::connect(path)
                .await
                .map_err(|e| anyhow::anyhow!(e))?,
        };

        res.setup_database().await?;

        Ok(res)
    }

    pub async fn setup_database(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                email_address TEXT PRIMARY KEY,
                account_code TEXT NOT NULL,
                tx_hash TEXT NOT NULL,
                is_onborded BOOLEAN NOT NULL DEFAULT FALSE,
                wallet_addr TEXT NOT NULL
            );",
        )
        .execute(&self.db)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS claims (
                tx_hash TEXT PRIMARY KEY,
                id TEXT NOT NULL,
                email_address TEXT NOT NULL,
                random TEXT NOT NULL,
                email_addr_commit TEXT NOT NULL,
                expiry_time BIGINT NOT NULL,
                is_fund BOOLEAN NOT NULL,
                is_announced BOOLEAN NOT NULL,
                is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
                is_seen BOOLEAN NOT NULL DEFAULT FALSE
            );",
        )
        .execute(&self.db)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS safe (
                wallet_addr TEXT PRIMARY KEY,
                safe_addr TEXT[] NOT NULL
            );",
        )
        .execute(&self.db)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS safe_txs (
                tx_hash TEXT NOT NULL,
                wallet_addr TEXT NOT NULL,
                PRIMARY KEY (tx_hash, wallet_addr)
            );",
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    #[named]
    pub async fn insert_user(
        &self,
        email_address: &str,
        account_code: &str,
        tx_hash: &str,
        is_onborded: bool,
        wallet_addr: &str,
    ) -> Result<()> {
        let row = sqlx::query(
            "INSERT INTO users (email_address, account_code, tx_hash, is_onborded, wallet_addr) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(email_address)
        .bind(account_code)
        .bind(tx_hash)
        .bind(is_onborded)
        .bind(wallet_addr)
        .fetch_one(&self.db)
        .await?;
        info!(
            LOG,
            "inserted row: {}",
            row.get::<String, _>("email_address"); "func" => function_name!()
        );
        Ok(())
    }

    #[named]
    pub async fn user_onborded(&self, email_address: &str, tx_hash: &str) -> Result<()> {
        info!(LOG, "email_address {}", email_address; "func" => function_name!());
        let res = sqlx::query(
            "UPDATE users SET is_onborded = TRUE, tx_hash = $1 WHERE email_address = $2",
        )
        .bind(tx_hash)
        .bind(email_address)
        .execute(&self.db)
        .await?;
        info!(
            LOG,
            "updated result: {:?}",
            res; "func" => function_name!()
        );
        Ok(())
    }

    pub async fn get_claims_by_id(&self, id: &U256) -> Result<Vec<Claim>> {
        let mut vec = Vec::new();

        let rows = sqlx::query("SELECT * FROM claims WHERE id = $1 AND is_deleted = FALSE")
            .bind(u256_to_hex(id))
            .fetch_all(&self.db)
            .await?;

        for row in rows {
            let tx_hash: String = row.get("tx_hash");
            let commit: String = row.get("email_addr_commit");
            let email_address: String = row.get("email_address");
            let random: String = row.get("random");
            let expiry_time: i64 = row.get("expiry_time");
            let is_fund: bool = row.get("is_fund");
            let is_announced: bool = row.get("is_announced");
            let is_seen: bool = row.get("is_seen");
            vec.push(Claim {
                tx_hash,
                id: *id,
                email_address,
                random,
                commit,
                expiry_time,
                is_fund,
                is_announced,
                is_seen,
            })
        }
        Ok(vec)
    }

    pub async fn get_claims_by_email_addr(&self, email_addr: &str) -> Result<Vec<Claim>> {
        let mut vec = Vec::new();

        let rows =
            sqlx::query("SELECT * FROM claims WHERE email_address = $1 AND is_deleted = FALSE")
                .bind(email_addr)
                .fetch_all(&self.db)
                .await?;

        for row in rows {
            let tx_hash: String = row.get("tx_hash");
            let id: String = row.get("id");
            let commit: String = row.get("email_addr_commit");
            let email_address: String = row.get("email_address");
            let random: String = row.get("random");
            let expiry_time: i64 = row.get("expiry_time");
            let is_fund: bool = row.get("is_fund");
            let is_announced: bool = row.get("is_announced");
            let is_seen: bool = row.get("is_seen");
            vec.push(Claim {
                tx_hash,
                id: hex_to_u256(&id)?,
                email_address,
                random,
                commit,
                expiry_time,
                is_fund,
                is_announced,
                is_seen,
            })
        }
        Ok(vec)
    }

    pub async fn get_claims_unexpired(&self, now: i64) -> Result<Vec<Claim>> {
        let mut vec = Vec::new();
        let rows =
            sqlx::query("SELECT * FROM claims WHERE expiry_time > $1 AND is_deleted = FALSE")
                .bind(now)
                .fetch_all(&self.db)
                .await?;

        for row in rows {
            let tx_hash: String = row.get("tx_hash");
            let id: String = row.get("id");
            let commit: String = row.get("email_addr_commit");
            let email_address: String = row.get("email_address");
            let random: String = row.get("random");
            let expiry_time: i64 = row.get("expiry_time");
            let is_fund: bool = row.get("is_fund");
            let is_announced: bool = row.get("is_announced");
            let is_seen: bool = row.get("is_seen");
            vec.push(Claim {
                tx_hash,
                id: hex_to_u256(&id)?,
                email_address,
                random,
                commit,
                expiry_time,
                is_fund,
                is_announced,
                is_seen,
            })
        }
        Ok(vec)
    }

    pub async fn get_claims_expired(&self, now: i64) -> Result<Vec<Claim>> {
        let mut vec = Vec::new();
        let rows =
            sqlx::query("SELECT * FROM claims WHERE expiry_time < $1 AND is_deleted = FALSE")
                .bind(now)
                .fetch_all(&self.db)
                .await?;

        for row in rows {
            let tx_hash: String = row.get("tx_hash");
            let id: String = row.get("id");
            let commit: String = row.get("email_addr_commit");
            let email_address: String = row.get("email_address");
            let random: String = row.get("random");
            let expiry_time: i64 = row.get("expiry_time");
            let is_fund: bool = row.get("is_fund");
            let is_announced: bool = row.get("is_announced");
            let is_seen: bool = row.get("is_seen");
            vec.push(Claim {
                tx_hash,
                id: hex_to_u256(&id)?,
                email_address,
                random,
                commit,
                expiry_time,
                is_fund,
                is_announced,
                is_seen,
            })
        }
        Ok(vec)
    }

    #[named]
    pub async fn insert_claim(&self, claim: &Claim) -> Result<()> {
        info!(LOG, "expiry_time {}", claim.expiry_time; "func" => function_name!());
        let row = sqlx::query(
            "INSERT INTO claims (tx_hash, id, email_address, random, email_addr_commit, expiry_time, is_fund, is_announced, is_seen) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
        )
        .bind(claim.tx_hash.clone())
        .bind(u256_to_hex(&claim.id))
        .bind(claim.email_address.clone())
        .bind(claim.random.clone())
        .bind(claim.commit.clone())
        .bind(claim.expiry_time)
        .bind(claim.is_fund)
        .bind(claim.is_announced)
        .bind(claim.is_seen)
        .fetch_one(&self.db)
        .await?;
        info!(
            LOG,
            "inserted row: {}",
            row.get::<String, _>("email_addr_commit"); "func" => function_name!()
        );
        Ok(())
    }

    pub async fn delete_claim(&self, id: &U256, is_fund: bool) -> Result<()> {
        sqlx::query("UPDATE claims SET is_deleted=TRUE WHERE id = $1 AND is_fund = $2 AND is_deleted = FALSE")
            .bind(u256_to_hex(id))
            .bind(is_fund)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    pub async fn contains_user(&self, email_address: &str) -> Result<bool> {
        let result = sqlx::query("SELECT 1 FROM users WHERE email_address = $1")
            .bind(email_address)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.is_some())
    }

    pub async fn is_user_onborded(&self, email_address: &str) -> Result<bool> {
        let result = sqlx::query("SELECT is_onborded FROM users WHERE email_address = $1")
            .bind(email_address)
            .fetch_one(&self.db)
            .await?;
        Ok(result.get("is_onborded"))
    }

    pub async fn get_account_code(&self, email_address: &str) -> Result<Option<String>> {
        let row_result = sqlx::query("SELECT account_code FROM users WHERE email_address = $1")
            .bind(email_address)
            .fetch_one(&self.db)
            .await;

        match row_result {
            Ok(row) => {
                let account_code: String = row.get("account_code");
                Ok(Some(account_code))
            }
            Err(sqlx::error::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e).map_err(|e| anyhow::anyhow!(e))?,
        }
    }

    pub async fn get_creation_tx_hash(&self, email_address: &str) -> Result<Option<String>> {
        let row_result = sqlx::query("SELECT tx_hash FROM users WHERE email_address = $1")
            .bind(email_address)
            .fetch_one(&self.db)
            .await;

        match row_result {
            Ok(row) => {
                let tx_hash: String = row.get("tx_hash");
                Ok(Some(tx_hash))
            }
            Err(sqlx::error::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e).map_err(|e| anyhow::anyhow!(e))?,
        }
    }

    pub async fn is_wallet_addr_exist(&self, wallet_addr: &str) -> Result<bool> {
        let result = sqlx::query("SELECT 1 FROM users WHERE LOWER(wallet_addr) = LOWER($1)")
            .bind(wallet_addr)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.is_some())
    }

    pub async fn add_user_with_safe(&self, wallet_addr: &str, safe_addr: &str) -> Result<()> {
        let query = "
            INSERT INTO safe (wallet_addr, safe_addr) 
            VALUES ($1, ARRAY[$2::TEXT]) 
            ON CONFLICT (wallet_addr) 
            DO UPDATE SET safe_addr = array_append(safe.safe_addr, EXCLUDED.safe_addr[1])
            RETURNING *;
        ";
        sqlx::query(query)
            .bind(wallet_addr)
            .bind(safe_addr)
            .fetch_one(&self.db)
            .await?;
        Ok(())
    }

    pub async fn get_users_with_safe(&self) -> Result<Vec<String>> {
        let mut vec = Vec::new();
        let rows = sqlx::query("SELECT wallet_addr FROM safe")
            .fetch_all(&self.db)
            .await?;

        for row in rows {
            let wallet_addr: String = row.get("wallet_addr");
            vec.push(wallet_addr);
        }
        Ok(vec)
    }

    pub async fn get_safes_by_user(&self, wallet_addr: &str) -> Result<Vec<String>> {
        let row = sqlx::query("SELECT safe_addr FROM safe WHERE wallet_addr = $1")
            .bind(wallet_addr)
            .fetch_one(&self.db)
            .await?;
        Ok(row.get("safe_addr"))
    }

    pub async fn remove_safe_from_user(&self, wallet_addr: &str, safe_addr: &str) -> Result<()> {
        let query = "
            UPDATE safe 
            SET safe_addr = array_remove(safe_addr, $2) 
            WHERE wallet_addr = $1;
        ";
        sqlx::query(query)
            .bind(wallet_addr)
            .bind(safe_addr)
            .execute(&self.db)
            .await?;

        // If the safe_addr is empty, remove the wallet_addr from the safe table
        let row = sqlx::query("SELECT safe_addr FROM safe WHERE wallet_addr = $1")
            .bind(wallet_addr)
            .fetch_one(&self.db)
            .await?;
        let safe_addr: Vec<String> = row.get("safe_addr");
        if safe_addr.is_empty() {
            sqlx::query("DELETE FROM safe WHERE wallet_addr = $1")
                .bind(wallet_addr)
                .execute(&self.db)
                .await?;
        }

        Ok(())
    }

    pub async fn get_email_by_wallet(&self, wallet_addr: &str) -> Result<String> {
        let row =
            sqlx::query("SELECT email_address FROM users WHERE LOWER(wallet_addr) = LOWER($1)")
                .bind(wallet_addr)
                .fetch_one(&self.db)
                .await?;
        Ok(row.get("email_address"))
    }

    pub async fn insert_safe_tx(&self, tx_hash: &str, wallet_addr: &str) -> Result<()> {
        sqlx::query("INSERT INTO safe_txs (tx_hash, wallet_addr) VALUES ($1, $2) RETURNING *")
            .bind(tx_hash)
            .bind(wallet_addr)
            .fetch_one(&self.db)
            .await?;
        Ok(())
    }

    pub async fn has_safe_tx_by_addr(&self, tx_hash: &str, wallet_addr: &str) -> Result<bool> {
        let result = sqlx::query("SELECT 1 FROM safe_txs WHERE LOWER(tx_hash) = LOWER($1) AND LOWER(wallet_addr) = LOWER($2)")
            .bind(tx_hash)
            .bind(wallet_addr)
            .fetch_optional(&self.db)
            .await?;

        Ok(result.is_some())
    }
}
