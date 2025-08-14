use crate::creator::{
    base::ContractProviderTrait,
    implementations::{default::Creator, listening::ListeningCreator, queue::SimpleTaskQueue},
    interface::TaskCreatorTrait,
    types::TaskQueue,
};
use crate::ingress::TaskRequest;
use anyhow::Result;

/// Mock contract provider for testing creator implementations
#[derive(Debug)]
struct MockProvider {
    state: u64,
    should_fail: bool,
}

impl MockProvider {
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
impl ContractProviderTrait for MockProvider {
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
async fn test_creator_new() {
    let provider = MockProvider::new(42);
    let creator = Creator::new(provider);

    // Test that the creator was created successfully
    assert!(creator.get_current_state().await.is_ok());
}

#[tokio::test]
async fn test_creator_get_current_state() {
    let expected_state = 123;
    let provider = MockProvider::new(expected_state);
    let creator = Creator::new(provider);

    let result = creator.get_current_state().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_state);
}

#[tokio::test]
async fn test_creator_get_current_state_failure() {
    let provider = MockProvider::new_failing();
    let creator = Creator::new(provider);

    let result = creator.get_current_state().await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Mock provider failure");
}

#[tokio::test]
async fn test_creator_get_payload_and_state() {
    let expected_state = 456;
    let provider = MockProvider::new(expected_state);
    let creator = Creator::new(provider);

    let result = creator.get_payload_and_state().await;
    assert!(result.is_ok());

    let (payload, state) = result.unwrap();
    assert_eq!(state, expected_state);

    // Verify payload contains the encoded state
    let expected_payload = expected_state.to_le_bytes().to_vec();
    assert!(payload.len() > expected_payload.len()); // Should contain task data + state
}

#[tokio::test]
async fn test_creator_get_payload_and_state_failure() {
    let provider = MockProvider::new_failing();
    let creator = Creator::new(provider);

    let result = creator.get_payload_and_state().await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Creator error"));
}

#[tokio::test]
async fn test_listening_creator_new() {
    let provider = MockProvider::new(42);
    let queue = SimpleTaskQueue::new();
    let creator = ListeningCreator::new(provider, queue);

    // Test that the creator was created successfully
    assert!(creator.get_current_state().await.is_ok());
}

#[tokio::test]
async fn test_listening_creator_get_current_state() {
    let expected_state = 123;
    let provider = MockProvider::new(expected_state);
    let queue = SimpleTaskQueue::new();
    let creator = ListeningCreator::new(provider, queue);

    let result = creator.get_current_state().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_state);
}

#[tokio::test]
async fn test_listening_creator_get_current_state_failure() {
    let provider = MockProvider::new_failing();
    let queue = SimpleTaskQueue::new();
    let creator = ListeningCreator::new(provider, queue);

    let result = creator.get_current_state().await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Mock provider failure");
}

#[tokio::test]
async fn test_listening_creator_get_next_task_empty_queue() {
    let provider = MockProvider::new(42);
    let queue = SimpleTaskQueue::new();
    let creator = ListeningCreator::new(provider, queue);

    // Queue should be empty initially
    let task = creator.get_next_task().await;
    assert!(task.is_none());
}

#[tokio::test]
async fn test_listening_creator_get_next_task_with_task() {
    let provider = MockProvider::new(42);
    let queue = SimpleTaskQueue::new();
    let creator = ListeningCreator::new(provider, queue);

    // Add a task to the queue
    let task_request = TaskRequest {
        body: crate::ingress::TaskRequestBody {
            var1: "test1".to_string(),
            var2: "test2".to_string(),
            var3: "test3".to_string(),
        },
    };
    creator.queue.push(task_request.clone());

    // Should get the task from the queue
    let task = creator.get_next_task().await;
    assert!(task.is_some());
    assert_eq!(task.unwrap().body.var1, "test1");
}

#[tokio::test]
async fn test_listening_creator_get_payload_for_state() {
    let provider = MockProvider::new(42);
    let queue = SimpleTaskQueue::new();
    let creator = ListeningCreator::new(provider, queue);
    let state = 789;

    let result = creator.get_payload_for_state(state).await;
    assert!(result.is_ok());

    let (payload, returned_state) = result.unwrap();
    assert_eq!(returned_state, state);

    // Verify payload contains the encoded state
    let expected_payload = state.to_le_bytes().to_vec();
    assert!(payload.len() >= expected_payload.len());
}

#[tokio::test]
async fn test_listening_creator_get_payload_and_state_with_task() {
    let provider = MockProvider::new(456);
    let queue = SimpleTaskQueue::new();
    let creator = ListeningCreator::new(provider, queue);

    // Add a task to the queue
    let task_request = TaskRequest {
        body: crate::ingress::TaskRequestBody {
            var1: "task1".to_string(),
            var2: "task2".to_string(),
            var3: "task3".to_string(),
        },
    };
    creator.queue.push(task_request);

    // Should get payload and state successfully
    let result = creator.get_payload_and_state().await;
    assert!(result.is_ok());

    let (payload, state) = result.unwrap();
    assert_eq!(state, 456);
    assert!(!payload.is_empty());
}

#[tokio::test]
async fn test_listening_creator_get_payload_and_state_failure() {
    let provider = MockProvider::new_failing();
    let queue = SimpleTaskQueue::new();
    let creator = ListeningCreator::new(provider, queue);

    // Test that get_current_state fails (which is called by get_payload_and_state)
    let result = creator.get_current_state().await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Mock provider failure");
}

#[tokio::test]
async fn test_task_creator_trait_object_safety() {
    // Test that we can use trait objects
    let provider = MockProvider::new(42);
    let creator = Creator::new(provider);
    let creator_ref: &dyn TaskCreatorTrait<u64> = &creator;

    // Test that we can call methods on the trait object
    let state = creator_ref.get_current_state().await;
    assert!(state.is_ok());
    assert_eq!(state.unwrap(), 42);
}
