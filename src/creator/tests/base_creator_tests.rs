use crate::creator::base::{BaseCreator, ContractProviderTrait};
use anyhow::Result;

/// Mock contract provider for testing BaseCreator
#[derive(Debug)]
struct MockContractProvider {
    state: u64,
    should_fail: bool,
}

impl MockContractProvider {
    fn new(state: u64) -> Self {
        Self {
            state,
            should_fail: false,
        }
    }

    fn new_failing() -> Self {
        Self {
            state: 0,
            should_fail: true,
        }
    }
}

#[async_trait::async_trait]
impl ContractProviderTrait for MockContractProvider {
    type State = u64;

    async fn get_current_state(&self) -> Result<u64> {
        if self.should_fail {
            Err(anyhow::anyhow!("Mock provider failure"))
        } else {
            Ok(self.state)
        }
    }

    async fn encode_state_call(&self, state: &u64) -> Vec<u8> {
        state.to_le_bytes().to_vec()
    }
}

#[tokio::test]
async fn test_base_creator_new() {
    let provider = MockContractProvider::new(42);
    let base_creator = BaseCreator::new(provider);

    // Test that the creator was created successfully
    assert!(base_creator.get_current_state().await.is_ok());
}

#[tokio::test]
async fn test_base_creator_get_current_state() {
    let expected_state = 123;
    let provider = MockContractProvider::new(expected_state);
    let base_creator = BaseCreator::new(provider);

    let result = base_creator.get_current_state().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_state);
}

#[tokio::test]
async fn test_base_creator_get_current_state_failure() {
    let provider = MockContractProvider::new_failing();
    let base_creator = BaseCreator::new(provider);

    let result = base_creator.get_current_state().await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Mock provider failure");
}

#[tokio::test]
async fn test_base_creator_encode_state_call() {
    let provider = MockContractProvider::new(42);
    let base_creator = BaseCreator::new(provider);
    let state = 456;

    let encoded = base_creator.encode_state_call(&state).await;
    let expected = state.to_le_bytes().to_vec();

    assert_eq!(encoded, expected);
}

#[tokio::test]
async fn test_base_creator_get_task_data() {
    let provider = MockContractProvider::new(42);
    let base_creator = BaseCreator::new(provider);

    let result = base_creator.get_task_data().await;
    assert!(result.is_ok());

    let task_data = result.unwrap();
    assert_eq!(task_data.var1, "default_var1");
    assert_eq!(task_data.var2, "default_var2");
    assert_eq!(task_data.var3, "default_var3");
}

#[tokio::test]
async fn test_base_creator_get_task_data_failure() {
    let provider = MockContractProvider::new_failing();
    let base_creator = BaseCreator::new(provider);

    let result = base_creator.get_task_data().await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Mock provider failure");
}
