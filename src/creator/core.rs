use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

/// The main creator trait. Implementations produce a payload and a round number.
#[async_trait]
pub trait Creator: Send + Sync {
    /// Compute and return the payload bytes and associated round.
    async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64)>;

    /// Get task metadata as key-value pairs.
    ///
    /// These metadata fields are used in the wire protocol messages and are typically
    /// specific to the use case being implemented (e.g., counter, voting, etc.).
    /// The metadata is flexible and can contain any number of key-value pairs.
    ///
    /// # Returns
    /// * `HashMap<String, String>` - The task metadata as key-value pairs
    fn get_task_metadata(&self) -> HashMap<String, String>;
}
