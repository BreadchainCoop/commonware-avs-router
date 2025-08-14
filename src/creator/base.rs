use anyhow::Result;

/// Trait defining the interface for state provider implementations.
///
/// This trait provides a generic interface for retrieving and encoding state data,
/// allowing different state sources to be swapped without changing the consuming code.
/// Each implementation can define its own state type through the associated `State` type.
#[async_trait::async_trait]
pub trait StateProviderTrait: Send + Sync {
    /// The state type returned by this provider.
    ///
    /// This can be any type that represents the current state.
    /// For simple cases, this might be a number (u64, u32, etc.).
    /// For complex cases, this could be a struct or enum.
    /// The state type must implement Debug for logging purposes.
    /// The state type must also be Send + Sync for async operations.
    type State: std::fmt::Debug + Send + Sync;

    /// Gets the current state.
    ///
    /// This method retrieves the current state value from the underlying source.
    /// The exact meaning of "state" depends on the implementation.
    ///
    /// # Returns
    /// * `Result<Self::State>` - The current state value, or an error
    async fn get_current_state(&self) -> Result<Self::State>;

    /// Encodes a state value for external use.
    ///
    /// This method encodes a state value into a format suitable for external consumption.
    /// The encoding format depends on the implementation's requirements.
    ///
    /// # Arguments
    /// * `state` - The state value to encode
    ///
    /// # Returns
    /// * `Vec<u8>` - The encoded state data
    async fn encode_state(&self, state: &Self::State) -> Vec<u8>;
}

/// Base implementation for creators that use state providers.
///
/// This struct provides common functionality for creators that need to interact
/// with state sources through a provider interface.
pub struct BaseCreator<P: StateProviderTrait> {
    state_provider: P,
}

impl<P: StateProviderTrait> BaseCreator<P> {
    /// Creates a new BaseCreator with the given state provider.
    pub fn new(state_provider: P) -> Self {
        Self { state_provider }
    }

    /// Gets the current state from the state provider.
    pub async fn get_current_state(&self) -> Result<P::State> {
        self.state_provider.get_current_state().await
    }

    /// Encodes a state using the state provider.
    pub async fn encode_state(&self, state: &P::State) -> Vec<u8> {
        self.state_provider.encode_state(state).await
    }

    /// Gets task data with the current state.
    #[allow(dead_code)] // Kept for binary target, used in tests
    pub async fn get_task_data(&self) -> Result<crate::creator::types::TaskData> {
        let _current_state = self.get_current_state().await?;
        Ok(crate::creator::types::TaskData::default())
    }
}
