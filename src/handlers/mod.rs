pub mod creator;
pub mod executor;
pub mod validator;
pub mod orchestrator;
pub mod listening_creator;

pub use orchestrator::Orchestrator;
pub mod wire;

use std::sync::Arc;
use crate::handlers::{creator::Creator, listening_creator::ListeningCreator};

use alloy::{
    network::EthereumWallet,
    providers::fillers::FillProvider,
};
use alloy_provider::{
    fillers::{BlobGasFiller, ChainIdFiller, GasFiller, JoinFill, NonceFiller, WalletFiller},
    RootProvider,
};

// Type alias for the complex provider type used across handlers
pub type CounterProvider = FillProvider<
    JoinFill<
        JoinFill<
            alloy_provider::Identity,
            JoinFill<
                GasFiller,
                JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
            >,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

// Type alias for view-only provider (without wallet)
pub type ViewOnlyProvider = FillProvider<
    JoinFill<
        alloy_provider::Identity,
        JoinFill<
            GasFiller,
            JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
        >,
    >,
    RootProvider,
>;

/// Shared trait for creators that can generate payloads and round numbers
pub trait TaskCreator: Send + Sync {
    /// Get the current payload and round number
    async fn get_payload_and_round(&self) -> anyhow::Result<(Vec<u8>, u64)>;
}
enum TaskCreatorEnum {
    Creator(Creator),
    ListeningCreator(Arc<ListeningCreator>),
}

impl TaskCreator for TaskCreatorEnum {
    async fn get_payload_and_round(&self) -> anyhow::Result<(Vec<u8>, u64)> {
        match self {
            TaskCreatorEnum::Creator(creator) => creator.get_payload_and_round().await.map_err(|e| anyhow::anyhow!("Creator error: {}", e)),
            TaskCreatorEnum::ListeningCreator(listening_creator) => listening_creator.get_payload_and_round().await.map_err(|e| anyhow::anyhow!("ListeningCreator error: {}", e)),
        }
    }
}
