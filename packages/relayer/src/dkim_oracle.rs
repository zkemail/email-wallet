use anyhow::anyhow;
use candid::utils::encode_args;
use candid::{CandidType, Principal};
use ic_agent::agent::*;
use ic_agent::identity::*;
use ic_utils::canister::*;
use ic_utils::interfaces::wallet::*;
use ic_utils::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct DkimOracleClient<'a> {
    pub(crate) canister: Canister<'a>,
}

#[derive(Default, CandidType, Deserialize, Debug, Clone)]
pub struct SignedDkimPublicKey {
    pub selector: String,
    pub domain: String,
    pub chain_id: u64,
    pub signature: String,
    pub public_key: String,
    pub public_key_hash: String,
}

impl<'a> DkimOracleClient<'a> {
    pub fn gen_agent(pem_path: &str) -> anyhow::Result<Agent> {
        let identity = Secp256k1Identity::from_pem_file(pem_path)?;
        let agent = AgentBuilder::default().with_identity(identity).build()?;
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
            .with_arg(&encode_args((selector, domain))?)
            .build::<(Result<SignedDkimPublicKey, String>,)>();
        let response = request
            .call_and_wait_one::<Result<SignedDkimPublicKey, String>>()
            .await?
            .map_err(|e| anyhow!(format!("Error from canister: {:?}", e)))?;

        // let result = Decode!(&response, String)?;
        Ok(response)
    }
}
