use alloy::{
    network::EthereumWallet, primitives::{Address, U256}, providers::fillers::FillProvider, sol, sol_types::SolCall
    
};
use alloy_provider::{fillers::{BlobGasFiller, ChainIdFiller, GasFiller, JoinFill, NonceFiller, WalletFiller}, ProviderBuilder, RootProvider};
use alloy_signer_local::PrivateKeySigner;
use std::{env, str::FromStr};
use NumberEncoder::yourNumbFuncCall;

use crate::bindings::counter::Counter;

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

    pub async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64), Box<dyn std::error::Error>> {
        let current_number = self.get_current_number().await?;
        let encoded = self.encode_number_call(U256::from(current_number)).await;
        Ok((encoded, current_number))
    }
}

// Helper function to create a new Creator instance
pub async fn create_creator() -> Result<Creator, Box<dyn std::error::Error>> {
    let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let signer = PrivateKeySigner::from_str(&private_key)?;
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(&http_rpc)
        .await?;
    
    let counter_address = Address::from_str(&env::var("COUNTER_ADDRESS").expect("COUNTER_ADDRESS must be set"))?;
    
    Ok(Creator::new(provider, counter_address))
}
