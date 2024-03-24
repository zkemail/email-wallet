use anyhow::anyhow;

use candid::CandidType;
use ic_agent::agent::http_transport::ReqwestTransport;
use ic_agent::agent::*;
use ic_agent::identity::*;
use ic_utils::canister::*;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct DkimOracleClient<'a> {
    pub canister: Canister<'a>,
}

#[derive(Default, CandidType, Deserialize, Debug, Clone)]
pub struct SignedDkimPublicKey {
    pub selector: String,
    pub domain: String,
    pub signature: String,
    pub public_key: String,
    pub public_key_hash: String,
}

impl<'a> DkimOracleClient<'a> {
    pub fn gen_agent(pem_path: &str, replica_url: &str) -> anyhow::Result<Agent> {
        let identity = Secp256k1Identity::from_pem_file(pem_path)?;
        let transport = ReqwestTransport::create(replica_url)?;
        let agent = AgentBuilder::default()
            .with_identity(identity)
            .with_transport(transport)
            .build()?;
        Ok(agent)
    }

    pub fn new(canister_id: &str, agent: &'a Agent) -> anyhow::Result<Self> {
        let canister = CanisterBuilder::new()
            .with_canister_id(canister_id)
            .with_agent(&agent)
            .build()?;
        Ok(Self { canister })
    }

    pub async fn request_signature(
        &self,
        selector: &str,
        domain: &str,
    ) -> anyhow::Result<SignedDkimPublicKey> {
        let request = self
            .canister
            .update("sign_dkim_public_key")
            .with_args((selector, domain))
            .build::<(Result<SignedDkimPublicKey, String>,)>();
        let response = request
            .call_and_wait_one::<Result<SignedDkimPublicKey, String>>()
            .await?
            .map_err(|e| anyhow!(format!("Error from canister: {:?}", e)))?;

        Ok(response)
    }
}
