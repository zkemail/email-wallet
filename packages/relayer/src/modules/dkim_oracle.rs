use anyhow::anyhow;
use candid::CandidType;
use candid::Encode;
use ic_agent::agent::http_transport::ReqwestTransport;
use ic_agent::agent::*;
use ic_agent::identity::*;
use ic_utils::canister::*;
use ic_utils::interfaces::WalletCanister;
use serde::Deserialize;

/// Amount of cycles charged by the ICP canister
pub const SIGN_CHARGED_CYCLE: u128 = 45_000_000_000;

/// Represents a client for interacting with the DKIM Oracle.
#[derive(Debug, Clone)]
pub struct DkimOracleClient<'a> {
    /// The dkim oracle canister.
    pub dkim_canister: Canister<'a>,
    /// The wallet canister.
    pub wallet_canister: WalletCanister<'a>,
}

/// Represents a signed DKIM public key.
#[derive(Default, CandidType, Deserialize, Debug, Clone)]
pub struct SignedDkimPublicKey {
    /// The selector for the DKIM key
    pub selector: String,
    /// The domain for the DKIM key
    pub domain: String,
    /// The signature of the DKIM key
    pub signature: String,
    /// The public key
    pub public_key: String,
    /// The hash of the public key
    pub public_key_hash: String,
}

impl<'a> DkimOracleClient<'a> {
    /// Generates an agent for the DKIM Oracle Client.
    ///
    /// # Arguments
    ///
    /// * `pem_path` - The path to the PEM file.
    /// * `replica_url` - The URL of the replica.
    ///
    /// # Returns
    ///
    /// An `anyhow::Result<Agent>`.
    pub fn gen_agent(pem_path: &str, replica_url: &str) -> anyhow::Result<Agent> {
        // Create identity from PEM file
        let identity = Secp256k1Identity::from_pem_file(pem_path)?;

        // Create transport using the replica URL
        let transport = ReqwestTransport::create(replica_url)?;

        // Build and return the agent
        let agent = AgentBuilder::default()
            .with_identity(identity)
            .with_transport(transport)
            .build()?;
        Ok(agent)
    }

    /// Creates a new DkimOracleClient.
    ///
    /// # Arguments
    ///
    /// * `dkim_canister_id` - The ID of the dkim canister.
    /// * `wallet_canister_id` - The ID of the wallet canister.
    /// * `agent` - The agent to use for communication.
    ///
    /// # Returns
    ///
    /// An `anyhow::Result<Self>`.
    pub async fn new(
        dkim_canister_id: &str,
        wallet_canister_id: &str,
        agent: &'a Agent,
    ) -> anyhow::Result<Self> {
        // Build the canister using the provided ID and agent
        let dkim_canister = CanisterBuilder::new()
            .with_canister_id(dkim_canister_id)
            .with_agent(agent)
            .build()?;
        let wallet_canister = WalletCanister::from_canister(
            ic_utils::Canister::builder()
                .with_agent(agent)
                .with_canister_id(wallet_canister_id)
                .build()?,
        )
        .await?;
        Ok(Self {
            dkim_canister,
            wallet_canister,
        })
    }

    /// Requests a signature for a DKIM public key.
    ///
    /// # Arguments
    ///
    /// * `selector` - The selector for the DKIM key.
    /// * `domain` - The domain for the DKIM key.
    ///
    /// # Returns
    ///
    /// An `anyhow::Result<SignedDkimPublicKey>`.
    pub async fn request_signature(
        &self,
        selector: &str,
        domain: &str,
    ) -> anyhow::Result<SignedDkimPublicKey> {
        // Build the request to sign the DKIM public key
        let mut arg = Argument::new();
        arg.set_raw_arg(candid::Encode!(&selector, &domain).unwrap());
        let (response,) = self
            .wallet_canister
            .call128::<(Result<SignedDkimPublicKey, String>,), _>(
                *self.dkim_canister.canister_id(),
                "sign_dkim_public_key",
                arg,
                SIGN_CHARGED_CYCLE,
            )
            .call_and_wait()
            .await?;
        let sign = response.map_err(|e| anyhow!(format!("Error from canister: {:?}", e)))?;
        Ok(sign)
    }
}
