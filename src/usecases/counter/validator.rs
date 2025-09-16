use crate::{
    bindings::{ReadOnlyProvider, counter::Counter},
    usecases::counter::creator::CounterTaskData,
    wire,
};
use alloy::sol_types::SolValue;
use alloy_primitives::U256;
use alloy_provider::ProviderBuilder;
use anyhow::Result;
use commonware_codec::{DecodeExt, ReadExt};
use commonware_cryptography::sha256::Digest;
use commonware_cryptography::{Hasher, Sha256};
use commonware_eigenlayer::config::AvsDeployment;
use std::{env, io::Cursor};

use crate::validator::interface::ValidatorTrait;

/// Counter-specific validator implementation.
pub struct CounterValidator {
    counter: Counter::CounterInstance<ReadOnlyProvider>,
}

impl CounterValidator {
    /// Creates a new CounterValidator instance.
    pub async fn new() -> Result<Self> {
        let http_rpc = env::var("HTTP_RPC").expect("HTTP_RPC must be set");
        let provider = ProviderBuilder::new().connect_http(url::Url::parse(&http_rpc).unwrap());

        let deployment = AvsDeployment::load()
            .map_err(|e| anyhow::anyhow!("Failed to load AVS deployment: {}", e))?;
        let counter_address = deployment
            .counter_address()
            .map_err(|e| anyhow::anyhow!("Failed to get counter address: {}", e))?;
        let counter = Counter::new(counter_address, provider.clone());

        Ok(Self { counter })
    }

    /// Verifies that the message round number matches the current onchain state.
    async fn verify_message_round(&self, msg: &[u8]) -> Result<()> {
        let aggregation: wire::Aggregation<CounterTaskData> =
            wire::Aggregation::read(&mut Cursor::new(msg))?;
        let current_number = self.counter.number().call().await?;
        let current_number = current_number.to::<u64>();

        if aggregation.round != current_number {
            return Err(anyhow::anyhow!(
                "Invalid round number in message. Expected {}, got {}",
                current_number,
                aggregation.round
            ));
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl ValidatorTrait for CounterValidator {
    async fn validate_and_return_expected_hash(&self, msg: &[u8]) -> Result<Digest> {
        self.verify_message_round(msg).await?;
        self.get_payload_from_message(msg).await
    }

    async fn get_payload_from_message(&self, msg: &[u8]) -> Result<Digest> {
        let aggregation: wire::Aggregation<CounterTaskData> = wire::Aggregation::decode(msg)?;
        let payload = U256::from(aggregation.round).abi_encode();

        // Hash the payload
        let mut hasher = Sha256::new();
        hasher.update(&payload);
        let payload_hash = hasher.finalize();

        Ok(payload_hash)
    }
}
