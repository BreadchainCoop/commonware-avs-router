pub mod creator;
pub mod executor;
pub mod validator;
pub mod orchestrator;
pub mod listening_creator;

pub use orchestrator::Orchestrator;
pub mod wire;

use std::{error::Error, sync::Arc};

use crate::handlers::{creator::Creator, listening_creator::ListeningCreator};

/// Shared trait for creators that can generate payloads and round numbers
pub trait TaskCreator: Send + Sync {
    /// Get the current payload and round number
    async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64), Box<dyn Error>>;
}
enum TaskCreatorEnum {
    Creator(Creator),
    ListeningCreator(Arc<ListeningCreator>),
}

impl TaskCreator for TaskCreatorEnum {
    async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64), Box<dyn std::error::Error>> {
        match self {
            TaskCreatorEnum::Creator(creator) => creator.get_payload_and_round().await,
            TaskCreatorEnum::ListeningCreator(listening_creator) => listening_creator.get_payload_and_round().await,
        }
    }
}
