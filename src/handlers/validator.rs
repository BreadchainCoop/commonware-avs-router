use crate::handlers::wire;
use crate::config::AvsDeployment;
use alloy::{sol, sol_types::{SolCall}};
use alloy_primitives::{Address, U256};
use alloy_provider::{fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller}, ProviderBuilder, RootProvider};
use commonware_codec::{DecodeExt, ReadExt};
use commonware_cryptography::{Hasher, Sha256};
use NumberEncoder::yourNumbFuncCall;
use std::{env, io::Cursor, str::FromStr};
use crate::bindings::counter::Counter;
use commonware_cryptography::sha256::Digest;

sol! {
    contract NumberEncoder {
        #[derive(Debug)]
        function yourNumbFunc(uint256 number) public returns (bytes memory);
    }
}
pub struct Validator {
    counter: Counter::CounterInstance<(), FillProvider<JoinFill<alloy_provider::Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>, RootProvider>>,
}

impl Validator {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
        let provider = ProviderBuilder::new()
            .on_http(url::Url::parse(&http_rpc).unwrap());
        
        let deployment = AvsDeployment::load()?;
        let counter_address = deployment.counter_address()?;
        let counter = Counter::new(counter_address, provider.clone());
        
        Ok(Self {
            counter,
        })
    }

    pub async fn validate_and_return_expected_hash(&self, msg: &[u8]) -> Result<Digest, Box<dyn std::error::Error + Send + Sync>> {
        // First verify the message round
        self.verify_message_round(msg).await?;
        
        // Then get the payload hash
        self.get_payload_from_message(msg).await
    }

    pub async fn get_payload_from_message(&self, msg: &[u8]) -> Result<Digest, Box<dyn std::error::Error + Send + Sync>> {
        // Decode the wire message
        let aggregation = wire::Aggregation::decode(msg)?;
        
        // Create the payload directly
        let payload = yourNumbFuncCall {
            number: U256::from(aggregation.round),
        }
        .abi_encode()[4..].to_vec();
        
        // Hash the payload
        let mut hasher = Sha256::new();
        hasher.update(&payload);
        let payload_hash = hasher.finalize();
        
        Ok(payload_hash)
    }

    async fn verify_message_round(&self, msg: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let aggregation = wire::Aggregation::read(&mut Cursor::new(msg))?;
        let current_number = self.counter.number().call().await?;
        let current_number = current_number._0.to::<u64>();

        if aggregation.round != current_number {
            return Err(format!(
                "Invalid round number in message. Expected {}, got {}",
                current_number, aggregation.round
            ).into());
        }

        Ok(())
    }
}
