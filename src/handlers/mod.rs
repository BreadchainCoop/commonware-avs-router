pub mod creator;
pub mod counter_validator;
pub mod executor;
pub mod listening_creator;
pub mod orchestrator;

pub use orchestrator::Orchestrator;

use crate::handlers::{creator::Creator, listening_creator::ListeningCreator};
use std::sync::Arc;

use alloy::{network::EthereumWallet, providers::fillers::FillProvider};
use alloy_provider::{
    RootProvider,
    fillers::{BlobGasFiller, ChainIdFiller, GasFiller, JoinFill, NonceFiller, WalletFiller},
};

// Type alias for the complex provider type used across handlers
pub type CounterProvider = FillProvider<
    JoinFill<
        JoinFill<
            alloy_provider::Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

// Type alias for view-only provider (without wallet)
pub type ViewOnlyProvider = FillProvider<
    JoinFill<
        alloy_provider::Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

/// Shared trait for creators that can generate payloads and round numbers
pub trait TaskCreator: Send + Sync {
    /// Get the current payload and round number
    fn get_payload_and_round(&self) -> impl std::future::Future<Output = anyhow::Result<(Vec<u8>, u64)>> + Send;
}
enum TaskCreatorEnum {
    Creator(Creator),
    ListeningCreator(Arc<ListeningCreator>),
}

impl TaskCreator for TaskCreatorEnum {
    async fn get_payload_and_round(&self) -> anyhow::Result<(Vec<u8>, u64)> {
        match self {
            TaskCreatorEnum::Creator(creator) => creator
                .get_payload_and_round()
                .await
                .map_err(|e| anyhow::anyhow!("Creator error: {}", e)),
            TaskCreatorEnum::ListeningCreator(listening_creator) => listening_creator
                .get_payload_and_round()
                .await
                .map_err(|e| anyhow::anyhow!("ListeningCreator error: {}", e)),
        }
    }
}
