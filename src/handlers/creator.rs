use crate::creator::Creator;
use crate::handlers::counter_provider::CounterProvider;
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use commonware_eigenlayer::config::AvsDeployment;
use std::{env, str::FromStr};

pub type DefaultCreator = Creator<CounterProvider>;

// Helper function to create a new Creator instance
pub async fn create_creator() -> anyhow::Result<DefaultCreator> {
    let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let signer = PrivateKeySigner::from_str(&private_key)
        .map_err(|e| anyhow::anyhow!("Failed to parse private key: {}", e))?;
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(&http_rpc)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect provider: {}", e))?;

    let deployment =
        AvsDeployment::load().map_err(|e| anyhow::anyhow!("Failed to load deployment: {}", e))?;
    let counter_address = deployment
        .counter_address()
        .map_err(|e| anyhow::anyhow!("Failed to get counter address: {}", e))?;

    let state_provider = CounterProvider::new(counter_address, provider.clone());
    Ok(Creator::new(state_provider))
}
