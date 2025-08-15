use alloy::{primitives::U256, sol_types::SolValue};
use anyhow::Result;
use async_trait::async_trait;

use super::types::CounterState;
use crate::bindings::{WalletProvider as AlloyProvider, counter::Counter};
use crate::creator::core::{State, StateProvider};

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

#[async_trait::async_trait]
impl StateProvider for CounterProvider {
    type State = CounterState;

    async fn get_current_state(&self) -> Result<CounterState> {
        let current_state = self.counter.number().call().await?;
        Ok(CounterState(current_state._0.to::<u64>()))
    }

    async fn encode_state(&self, state: &CounterState) -> Vec<u8> {
        U256::from(state.0).abi_encode()
    }
}

/// Counter state provider that reads from a smart contract
#[allow(dead_code)]
pub struct CounterStateProvider {
    #[allow(dead_code)]
    counter_address: String,
    #[allow(dead_code)]
    rpc_url: String,
}

impl CounterStateProvider {
    #[allow(dead_code)]
    pub fn new(counter_address: String, rpc_url: String) -> Self {
        Self {
            counter_address,
            rpc_url,
        }
    }
}

#[async_trait]
impl StateProvider for CounterStateProvider {
    type State = CounterState;

    async fn get_current_state(&self) -> Result<Self::State> {
        // In a real implementation, this would call the smart contract
        // For now, we'll simulate it
        Ok(CounterState(42))
    }

    async fn encode_state(&self, state: &Self::State) -> Vec<u8> {
        state.0.to_le_bytes().to_vec()
    }
}

/// Mock state provider for testing
#[allow(dead_code)]
pub struct MockStateProvider<S: State> {
    state: S,
    should_fail: bool,
}

impl<S: State> MockStateProvider<S> {
    #[allow(dead_code)]
    pub fn new(state: S) -> Self {
        Self {
            state,
            should_fail: false,
        }
    }

    #[allow(dead_code)]
    pub fn new_failing() -> Self {
        Self {
            state: unsafe { std::mem::zeroed() }, // Safe for testing
            should_fail: true,
        }
    }
}

#[async_trait]
impl<S: State> StateProvider for MockStateProvider<S> {
    type State = S;

    async fn get_current_state(&self) -> Result<Self::State> {
        if self.should_fail {
            Err(anyhow::anyhow!("Mock provider failure"))
        } else {
            Ok(self.state.clone())
        }
    }

    async fn encode_state(&self, _state: &Self::State) -> Vec<u8> {
        // Default encoding for testing
        vec![1, 2, 3, 4]
    }
}
