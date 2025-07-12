use alloy::{
    network::EthereumWallet, primitives::{Address, U256}, providers::fillers::FillProvider, sol, sol_types::SolCall
    
};
use alloy_provider::{fillers::{BlobGasFiller, ChainIdFiller, GasFiller, JoinFill, NonceFiller, WalletFiller}, ProviderBuilder, RootProvider};
use alloy_signer_local::PrivateKeySigner;
use std::{env, str::FromStr};
use NumberEncoder::yourNumbFuncCall;

use crate::bindings::counter::Counter;
use crate::handlers::TaskCreator;
use commonware_eigenlayer::config::AvsDeployment;

sol! {
    contract NumberEncoder {
        #[derive(Debug)]
        function yourNumbFunc(uint256 number) public returns (bytes memory);
    }
}
    
pub struct Creator {
    counter: Counter::CounterInstance<(), FillProvider<JoinFill<JoinFill<alloy_provider::Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>, WalletFiller<EthereumWallet>>, RootProvider>>,
}

impl Creator {
    pub fn new(provider: FillProvider<JoinFill<JoinFill<alloy_provider::Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>, WalletFiller<EthereumWallet>>, RootProvider>, counter_address: Address) -> Self {
        let counter = Counter::new(counter_address, provider.clone());
        Self {
            counter,
        }
    }

    pub async fn get_current_number(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let current_number = self.counter.number().call().await?;
        Ok(current_number._0.to::<u64>())
    }

    pub async fn encode_number_call(&self, number: U256) -> Vec<u8> {
        yourNumbFuncCall {
            number,
        }
        .abi_encode()[4..].to_vec()
    }

    async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64), Box<dyn std::error::Error>> {
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
    async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64), Box<dyn std::error::Error>> {
        self.get_payload_and_round().await
    }
}

// Helper function to create a new Creator instance
pub async fn create_creator() -> Result<Creator, Box<dyn std::error::Error + Send + Sync>> {
    let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let signer = PrivateKeySigner::from_str(&private_key)?;
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(&http_rpc)
        .await?;
    
    let deployment = AvsDeployment::load()?;
    let counter_address = deployment.counter_address()?;
    
    Ok(Creator::new(provider, counter_address))
}
