use crate::creator::base::ContractProviderTrait;
use anyhow::Result;

/// Mock contract provider with u64 state
#[derive(Debug)]
struct MockU64Provider {
    state: u64,
    should_fail: bool,
}

impl MockU64Provider {
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
impl ContractProviderTrait for MockU64Provider {
    type State = u64;

    async fn get_current_state(&self) -> Result<u64> {
        if self.should_fail {
            Err(anyhow::anyhow!("Mock u64 provider failure"))
        } else {
            Ok(self.state)
        }
    }

    async fn encode_state_call(&self, state: &u64) -> Vec<u8> {
        state.to_le_bytes().to_vec()
    }
}

/// Mock contract provider with String state
#[derive(Debug)]
struct MockStringProvider {
    state: String,
    should_fail: bool,
}

impl MockStringProvider {
    fn new(state: String) -> Self {
        Self {
            state,
            should_fail: false,
        }
    }

    fn new_failing() -> Self {
        Self {
            state: String::new(),
            should_fail: true,
        }
    }
}

#[async_trait::async_trait]
impl ContractProviderTrait for MockStringProvider {
    type State = String;

    async fn get_current_state(&self) -> Result<String> {
        if self.should_fail {
            Err(anyhow::anyhow!("Mock string provider failure"))
        } else {
            Ok(self.state.clone())
        }
    }

    async fn encode_state_call(&self, state: &String) -> Vec<u8> {
        state.as_bytes().to_vec()
    }
}

#[tokio::test]
async fn test_u64_provider_get_current_state() {
    let expected_state = 42;
    let provider = MockU64Provider::new(expected_state);

    let result = provider.get_current_state().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_state);
}

#[tokio::test]
async fn test_u64_provider_get_current_state_failure() {
    let provider = MockU64Provider::new_failing();

    let result = provider.get_current_state().await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Mock u64 provider failure");
}

#[tokio::test]
async fn test_u64_provider_encode_state_call() {
    let provider = MockU64Provider::new(0);
    let state = 123;

    let encoded = provider.encode_state_call(&state).await;
    let expected = state.to_le_bytes().to_vec();

    assert_eq!(encoded, expected);
}

#[tokio::test]
async fn test_string_provider_get_current_state() {
    let expected_state = "test_state".to_string();
    let provider = MockStringProvider::new(expected_state.clone());

    let result = provider.get_current_state().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_state);
}

#[tokio::test]
async fn test_string_provider_get_current_state_failure() {
    let provider = MockStringProvider::new_failing();

    let result = provider.get_current_state().await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Mock string provider failure"
    );
}

#[tokio::test]
async fn test_string_provider_encode_state_call() {
    let provider = MockStringProvider::new(String::new());
    let state = "test_string".to_string();

    let encoded = provider.encode_state_call(&state).await;
    let expected = state.as_bytes().to_vec();

    assert_eq!(encoded, expected);
}

#[tokio::test]
async fn test_provider_trait_object_safety() {
    // Test that we can use trait objects (Send + Sync requirements)
    let provider = MockU64Provider::new(42);
    let provider_ref: &dyn ContractProviderTrait<State = u64> = &provider;

    // Test that we can call methods on the trait object
    let state = provider_ref.get_current_state().await;
    assert!(state.is_ok());
    assert_eq!(state.unwrap(), 42);
}
