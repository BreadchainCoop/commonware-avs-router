use alloy::{
    primitives::{Address, U256}, sol_types::SolValue
    
};
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use anyhow::Result;
use std::{env, str::FromStr};
use crate::bindings::counter::Counter;
use crate::handlers::{TaskCreator, CounterProvider};
use commonware_eigenlayer::config::AvsDeployment;
    
pub struct Creator {
    counter: Counter::CounterInstance<(), CounterProvider>,
}

impl Creator {
    pub fn new(provider: CounterProvider, counter_address: Address) -> Self {
        let counter = Counter::new(counter_address, provider.clone());
        Self {
            counter,
        }
    }

    pub async fn get_current_number(&self) -> Result<u64> {
        let current_number = self.counter.number().call().await?;
        Ok(current_number._0.to::<u64>())
    }

    pub async fn encode_number_call(&self, number: U256) -> Vec<u8> {
        number.abi_encode()
    }

    async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64)> {
        let current_number = self.get_current_number().await?;
        let encoded = self.encode_number_call(U256::from(current_number)).await;
        
        // For non-ingress mode, encode default variables into the payload
        let mut payload = encoded;
        let default_vars = ["default_var1", "default_var2", "default_var3"];
        for var in default_vars {
            payload.extend_from_slice(var.as_bytes());
            payload.push(0); // null terminator
        }
        
        Ok((payload, current_number))
    }
}

impl TaskCreator for Creator {
    async fn get_payload_and_round(&self) -> anyhow::Result<(Vec<u8>, u64)> {
        self.get_payload_and_round().await.map_err(|e| anyhow::anyhow!("Creator error: {}", e))
    }
}

// Helper function to create a new Creator instance
pub async fn create_creator() -> anyhow::Result<Creator> {
    let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let signer = PrivateKeySigner::from_str(&private_key).map_err(|e| anyhow::anyhow!("Failed to parse private key: {}", e))?;
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(&http_rpc)
        .await.map_err(|e| anyhow::anyhow!("Failed to connect provider: {}", e))?;
    
    let deployment = AvsDeployment::load().map_err(|e| anyhow::anyhow!("Failed to load deployment: {}", e))?;
    let counter_address = deployment.counter_address().map_err(|e| anyhow::anyhow!("Failed to get counter address: {}", e))?;
    
    Ok(Creator::new(provider, counter_address))
}
