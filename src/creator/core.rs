use anyhow::Result;
use async_trait::async_trait;

/// Re-export task request types from ingress
pub use crate::ingress::types::TaskRequest;

/// The main creator trait. Implementations produce a payload and a round number.
#[async_trait]
pub trait Creator: Send + Sync {
    /// Compute and return the payload bytes and associated round.
    async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64)>;
}

/// Type alias for boxed creators (enables runtime polymorphism)
pub type BoxedCreator = Box<dyn Creator>;
