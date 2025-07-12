use crate::handlers::{wire, ViewOnlyProvider};
use commonware_eigenlayer::config::AvsDeployment;
use alloy::{sol, sol_types::{SolCall}};
use alloy_primitives::U256;
use alloy_provider::ProviderBuilder;
use commonware_codec::DecodeExt;
use commonware_cryptography::{Hasher, Sha256};
use NumberEncoder::yourNumbFuncCall;
use std::env;
use crate::bindings::counter::Counter;
use commonware_cryptography::sha256::Digest;

sol! {
    contract NumberEncoder {
        #[derive(Debug)]
        function yourNumbFunc(uint256 number) public returns (bytes memory);
    }
}
pub struct Validator {
    counter: Counter::CounterInstance<(), ViewOnlyProvider>,
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
        // perform some operations on the variables to ensure validity of the request
        // in this case, we just check if the requested number is the current number on the counter
        self.validate_message(msg).await?;

        let payload_hash = self.get_hashed_payload(msg).await?;

        Ok(payload_hash)
    }


    /// Decode the wire message once and return the aggregation struct
    async fn decode_message(&self, msg: &[u8]) -> Result<wire::Aggregation, Box<dyn std::error::Error + Send + Sync>> {
        wire::Aggregation::decode(msg).map_err(|e| e.into())
    }
    async fn validate_message(&self, msg: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let aggregation = self.decode_message(msg).await?;
        let requested_num = aggregation.round;
        let current_number = self.counter.number().call().await?;
        let current_number = current_number._0.to::<u64>();

        if requested_num != current_number {
            return Err(format!(
                "Invalid round number in message. Expected {}, got {}",
                current_number, requested_num
            ).into());
        }
        Ok(())
    }

    async fn get_hashed_payload(&self, msg: &[u8]) -> Result<Digest, Box<dyn std::error::Error + Send + Sync>> {
        // in this case, we are abi encoding the requested number and hashing the result 
        // in a way that is compatible with the counter contract onchain signature validation
        let aggregation = self.decode_message(msg).await?;
        let requested_num = aggregation.round;
        let payload = yourNumbFuncCall {
            number: U256::from(requested_num),
        }
        .abi_encode()[4..].to_vec();
        let mut hasher = Sha256::new();
        hasher.update(&payload);
        let payload_hash = hasher.finalize();
        Ok(payload_hash)
    }
}
