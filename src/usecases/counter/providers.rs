use alloy::{primitives::U256, sol_types::SolValue};
use anyhow::Result;
use async_trait::async_trait;

use crate::bindings::{WalletProvider as AlloyProvider, counter::Counter};
use crate::creator::core::{DefaultState, StateProvider};

// Use DefaultState directly instead of type alias
type CounterState = DefaultState<u64>;

/// Real counter state provider that reads from a smart contract
pub struct CounterProvider {
    counter: Counter::CounterInstance<(), AlloyProvider>,
}

impl CounterProvider {
    pub fn new(counter_address: alloy::primitives::Address, provider: AlloyProvider) -> Self {
        let counter = Counter::new(counter_address, provider);
        Self { counter }
    }
}

#[async_trait]
impl StateProvider for CounterProvider {
    type State = CounterState;

    async fn get_current_state(&self) -> Result<CounterState> {
        let current_state = self.counter.number().call().await?;
        Ok(DefaultState(current_state._0.to::<u64>()))
    }

    async fn encode_state(&self, state: &CounterState) -> Vec<u8> {
        U256::from(state.0).abi_encode()
    }
}
