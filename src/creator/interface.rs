/// Shared trait for creators that can generate payloads and state.
///
/// This trait is generic over the state type, allowing different state
/// providers to return different state types while maintaining a consistent
/// interface for payload generation.
#[async_trait::async_trait]
pub trait TaskCreatorTrait<State>: Send + Sync {
    /// Get the current payload and state.
    ///
    /// This method generates a payload based on the current state and returns
    /// both the payload and the current state value.
    ///
    /// # Returns
    /// * `Result<(Vec<u8>, State)>` - The payload and current state, or an error
    async fn get_payload_and_state(&self) -> anyhow::Result<(Vec<u8>, State)>;

    /// Get the current state.
    ///
    /// This method retrieves the current state value from the underlying
    /// state provider.
    ///
    /// # Returns
    /// * `Result<State>` - The current state value, or an error
    async fn get_current_state(&self) -> anyhow::Result<State>;
}
