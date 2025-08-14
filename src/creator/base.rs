use anyhow::Result;

/// Trait defining the interface for contract provider implementations.
///
/// This trait provides a generic interface for contract interactions,
/// allowing different contract implementations to be swapped without changing
/// the consuming code. Each implementation can define its own state type
/// through the associated `State` type.
#[async_trait::async_trait]
pub trait ContractProviderTrait: Send + Sync {
    /// The state type returned by this contract provider.
    ///
    /// This can be any type that represents the current state of the contract.
    /// For simple contracts, this might be a number (u64, u32, etc.).
    /// For complex contracts, this could be a struct or enum.
    /// The state type must implement Debug for logging purposes.
    /// The state type must also be Send + Sync for async operations.
    type State: std::fmt::Debug + Send + Sync;

    /// Gets the current state from the contract.
    ///
    /// This method retrieves the current state value from the underlying contract.
    /// The exact meaning of "state" depends on the contract implementation.
    ///
    /// # Returns
    /// * `Result<Self::State>` - The current state value, or an error
    async fn get_current_state(&self) -> Result<Self::State>;

    /// Encodes a state value for contract interaction.
    ///
    /// This method encodes a state value into the format expected by the contract.
    /// The encoding format depends on the contract's ABI.
    ///
    /// # Arguments
    /// * `state` - The state value to encode
    ///
    /// # Returns
    /// * `Vec<u8>` - The encoded state data
    async fn encode_state_call(&self, state: &Self::State) -> Vec<u8>;
}

/// Base implementation for creators that use contract providers.
///
/// This struct provides common functionality for creators that need to interact
/// with contracts through a provider interface.
pub struct BaseCreator<P: ContractProviderTrait> {
    contract_provider: P,
}

impl<P: ContractProviderTrait> BaseCreator<P> {
    /// Creates a new BaseCreator with the given contract provider.
    pub fn new(contract_provider: P) -> Self {
        Self { contract_provider }
    }

    /// Gets the current state from the contract provider.
    pub async fn get_current_state(&self) -> Result<P::State> {
        self.contract_provider.get_current_state().await
    }

    /// Encodes a state call using the contract provider.
    pub async fn encode_state_call(&self, state: &P::State) -> Vec<u8> {
        self.contract_provider.encode_state_call(state).await
    }

    /// Gets task data with the current state.
    #[allow(dead_code)] // Kept for binary target, used in tests
    pub async fn get_task_data(&self) -> Result<crate::creator::types::TaskData> {
        let _current_state = self.get_current_state().await?;
        Ok(crate::creator::types::TaskData::default())
    }
}
