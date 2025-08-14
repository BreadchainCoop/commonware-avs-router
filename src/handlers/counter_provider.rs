use alloy::{primitives::U256, sol_types::SolValue};
use anyhow::Result;

use crate::bindings::{WalletProvider as AlloyProvider, counter::Counter};
use crate::creator::base::StateProviderTrait;

/// Concrete implementation of StateProvider for alloy-based counter contract.
///
/// This provider interacts with a simple counter contract that maintains
/// a single u64 value as its state.
pub struct CounterProvider {
    counter: Counter::CounterInstance<(), AlloyProvider>,
}

impl CounterProvider {
    pub fn new(counter_address: alloy::primitives::Address, provider: AlloyProvider) -> Self {
        let counter = Counter::new(counter_address, provider);
        Self { counter }
    }
}

#[async_trait::async_trait]
impl StateProviderTrait for CounterProvider {
    type State = u64;

    async fn get_current_state(&self) -> Result<u64> {
        let current_state = self.counter.number().call().await?;
        Ok(current_state._0.to::<u64>())
    }

    async fn encode_state(&self, state: &u64) -> Vec<u8> {
        U256::from(*state).abi_encode()
    }
}
