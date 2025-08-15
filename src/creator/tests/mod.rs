pub mod mocks;
pub mod types;

use crate::creator::core::{Creator, DefaultState, StateProvider, TaskQueue};
use crate::creator::{CreatorConfig, DefaultCreator, ListeningCreator, SimpleTaskQueue};
use crate::ingress::types::{TaskRequest, TaskRequestBody};
use mocks::{MockStateProvider, MockTaskDataFactory, MockTaskQueue};

// Use DefaultState directly instead of type alias
type TestState = DefaultState<u64>;

#[tokio::test]
async fn test_default_creator_create_payload_and_state_success() {
    let provider = MockStateProvider::new(DefaultState(7));
    let factory = MockTaskDataFactory;
    let creator = DefaultCreator::new(provider, factory, CreatorConfig::default());

    let (payload, state) = creator
        .create_payload_and_state()
        .await
        .expect("should succeed");

    // State should match what provider returns
    assert_eq!(state, DefaultState(7));

    // Encoded state prefix from MockStateProvider is [1,2,3,4]
    assert!(payload.starts_with(&[1, 2, 3, 4]));

    // Payload should contain default var strings separated by null bytes
    let expected_tail = b"default_var1\0default_var2\0default_var3\0";
    assert!(payload.ends_with(expected_tail));
}

#[tokio::test]
async fn test_default_creator_get_current_state_delegates() {
    let provider = MockStateProvider::new(DefaultState(12345));
    let factory = MockTaskDataFactory;
    let creator = DefaultCreator::new(provider, factory, CreatorConfig::default());

    let state = creator.get_current_state().await.expect("should succeed");
    assert_eq!(state, DefaultState(12345));
}

#[tokio::test]
async fn test_listening_creator_processes_task_from_queue() {
    // Arrange
    let provider = MockStateProvider::new(DefaultState(5));
    let factory = MockTaskDataFactory;
    let queue = MockTaskQueue::new();
    let config = CreatorConfig {
        polling_interval_ms: 10,
        max_retries: 3,
        timeout_ms: 100,
    };
    // Enqueue a task
    let request = TaskRequest {
        body: TaskRequestBody {
            var1: "a".into(),
            var2: "b".into(),
            var3: "c".into(),
        },
    };
    queue.push(request.clone());

    let creator = ListeningCreator::new(provider, factory, queue, config);

    // Act
    let (payload, state) = creator
        .create_payload_and_state()
        .await
        .expect("should succeed");

    // Assert
    assert_eq!(state, DefaultState(5));
    // The payload will start with mock encoded state [1,2,3,4] and then the request vars
    assert!(payload.starts_with(&[1, 2, 3, 4]));
    let expected_tail = b"a\0b\0c\0";
    assert!(payload.ends_with(expected_tail));
}

#[tokio::test]
async fn test_listening_creator_times_out_when_no_task() {
    let provider = MockStateProvider::new(DefaultState(1));
    let factory = MockTaskDataFactory;
    let queue = MockTaskQueue::new();
    let config = CreatorConfig {
        polling_interval_ms: 5,
        max_retries: 2,
        timeout_ms: 10,
    };
    let creator = ListeningCreator::new(provider, factory, queue, config);

    let err = creator
        .create_payload_and_state()
        .await
        .expect_err("should timeout");
    assert!(err.to_string().contains("Timeout waiting for task"));
}

#[test]
fn test_simple_task_queue_with_timeout_and_retries() {
    // Test the improved SimpleTaskQueue with timeout and retry logic
    let queue = SimpleTaskQueue::with_config(100, 5); // 100ms timeout, 5 retries

    // Test basic push and pop
    let task = TaskRequest {
        body: TaskRequestBody {
            var1: "test1".into(),
            var2: "test2".into(),
            var3: "test3".into(),
        },
    };

    queue.push(task.clone());
    let popped = queue.pop();
    assert!(popped.is_some());
    assert_eq!(popped.unwrap().body.var1, "test1");

    // Test pop when empty
    let empty_pop = queue.pop();
    assert!(empty_pop.is_none());
}

#[test]
fn test_simple_task_queue_multiple_tasks() {
    let queue = SimpleTaskQueue::new();

    // Push multiple tasks
    for i in 0..5 {
        let task = TaskRequest {
            body: TaskRequestBody {
                var1: format!("task{}", i),
                var2: "test".into(),
                var3: "test".into(),
            },
        };
        queue.push(task);
    }

    // Pop them in order (LIFO behavior)
    for i in (0..5).rev() {
        let popped = queue.pop();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap().body.var1, format!("task{}", i));
    }

    // Queue should be empty
    assert!(queue.pop().is_none());
}

#[tokio::test]
async fn test_mock_state_provider_failing() {
    // Test that the fixed MockStateProvider::new_failing() works correctly
    let provider = MockStateProvider::<TestState>::new_failing();

    // Should fail with an error
    let result = provider.get_current_state().await;
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Mock provider failure")
    );

    // Should still have a valid default state (not undefined behavior)
    assert_eq!(provider.get_state(), &DefaultState(0));
}
