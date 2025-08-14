use crate::creator::{implementations::queue::SimpleTaskQueue, types::TaskQueue};
use crate::ingress::TaskRequest;

/// Mock task queue implementation for testing
#[derive(Debug)]
struct MockTaskQueue {
    tasks: Vec<TaskRequest>,
    should_fail: bool,
}

impl MockTaskQueue {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            should_fail: false,
        }
    }

    fn new_failing() -> Self {
        Self {
            tasks: Vec::new(),
            should_fail: true,
        }
    }

    fn with_tasks(tasks: Vec<TaskRequest>) -> Self {
        Self {
            tasks,
            should_fail: false,
        }
    }
}

impl TaskQueue for MockTaskQueue {
    fn push(&self, _task: TaskRequest) {
        if !self.should_fail {
            // In a real implementation, this would be thread-safe
            // For testing, we'll just simulate the behavior
        }
    }

    fn pop(&self) -> Option<TaskRequest> {
        if self.should_fail {
            None
        } else {
            self.tasks.last().cloned()
        }
    }
}

#[test]
fn test_simple_task_queue_new() {
    let queue = SimpleTaskQueue::new();

    // Test that the queue was created successfully
    assert!(queue.pop().is_none()); // Should be empty initially
}

#[test]
fn test_simple_task_queue_default() {
    let queue = SimpleTaskQueue::default();

    // Test that the default implementation works
    assert!(queue.pop().is_none()); // Should be empty initially
}

#[test]
fn test_simple_task_queue_push_and_pop() {
    let queue = SimpleTaskQueue::new();

    // Create a test task
    let task = TaskRequest {
        body: crate::ingress::TaskRequestBody {
            var1: "test1".to_string(),
            var2: "test2".to_string(),
            var3: "test3".to_string(),
        },
    };

    // Push the task
    queue.push(task.clone());

    // Pop the task
    let popped_task = queue.pop();
    assert!(popped_task.is_some());
    assert_eq!(popped_task.unwrap().body.var1, "test1");
}

#[test]
fn test_simple_task_queue_multiple_tasks() {
    let queue = SimpleTaskQueue::new();

    // Create multiple test tasks
    let task1 = TaskRequest {
        body: crate::ingress::TaskRequestBody {
            var1: "task1".to_string(),
            var2: "task2".to_string(),
            var3: "task3".to_string(),
        },
    };

    let task2 = TaskRequest {
        body: crate::ingress::TaskRequestBody {
            var1: "task4".to_string(),
            var2: "task5".to_string(),
            var3: "task6".to_string(),
        },
    };

    // Push tasks
    queue.push(task1.clone());
    queue.push(task2.clone());

    // Pop tasks (should be in LIFO order for Vec::pop)
    let popped_task2 = queue.pop();
    assert!(popped_task2.is_some());
    assert_eq!(popped_task2.unwrap().body.var1, "task4");

    let popped_task1 = queue.pop();
    assert!(popped_task1.is_some());
    assert_eq!(popped_task1.unwrap().body.var1, "task1");

    // Queue should be empty now
    assert!(queue.pop().is_none());
}

#[test]
fn test_simple_task_queue_empty_pop() {
    let queue = SimpleTaskQueue::new();

    // Pop from empty queue
    let task = queue.pop();
    assert!(task.is_none());
}

#[test]
fn test_simple_task_queue_get_queue() {
    let queue = SimpleTaskQueue::new();
    let queue_arc = queue.get_queue();

    // Test that we can get the underlying queue
    assert!(queue_arc.lock().is_ok());
}

#[test]
fn test_mock_task_queue_new() {
    let queue = MockTaskQueue::new();

    // Test that the mock queue was created successfully
    assert!(queue.pop().is_none()); // Should be empty initially
}

#[test]
fn test_mock_task_queue_with_tasks() {
    let task = TaskRequest {
        body: crate::ingress::TaskRequestBody {
            var1: "test1".to_string(),
            var2: "test2".to_string(),
            var3: "test3".to_string(),
        },
    };

    let queue = MockTaskQueue::with_tasks(vec![task.clone()]);

    // Should return the task
    let popped_task = queue.pop();
    assert!(popped_task.is_some());
    assert_eq!(popped_task.unwrap().body.var1, "test1");
}

#[test]
fn test_mock_task_queue_failing() {
    let queue = MockTaskQueue::new_failing();

    // Should always return None when failing
    assert!(queue.pop().is_none());
}

#[test]
fn test_task_queue_trait_object_safety() {
    // Test that we can use trait objects
    let queue = SimpleTaskQueue::new();
    let queue_ref: &dyn TaskQueue = &queue;

    // Test that we can call methods on the trait object
    queue_ref.push(TaskRequest {
        body: crate::ingress::TaskRequestBody {
            var1: "test".to_string(),
            var2: "test".to_string(),
            var3: "test".to_string(),
        },
    });

    let task = queue_ref.pop();
    assert!(task.is_some());
    assert_eq!(task.unwrap().body.var1, "test");
}

#[test]
fn test_task_queue_send_sync() {
    // Test that TaskQueue implementations are Send + Sync
    let queue = SimpleTaskQueue::new();

    // This test ensures the trait bounds are satisfied
    fn assert_send_sync<T: Send + Sync>(_t: T) {}
    assert_send_sync(queue);

    // Additional verification that the queue actually works
    let queue2 = SimpleTaskQueue::new();
    assert!(queue2.pop().is_none());
}

#[test]
fn test_simple_task_queue_concurrent_access() {
    use std::sync::Arc;
    use std::thread;

    let queue = Arc::new(SimpleTaskQueue::new());
    let queue_clone = queue.clone();

    // Spawn a thread to push a task
    let handle = thread::spawn(move || {
        let task = TaskRequest {
            body: crate::ingress::TaskRequestBody {
                var1: "thread_task".to_string(),
                var2: "test2".to_string(),
                var3: "test3".to_string(),
            },
        };
        queue_clone.push(task);
    });

    // Wait for the thread to complete
    handle.join().unwrap();

    // Pop the task from the main thread
    let task = queue.pop();
    assert!(task.is_some());
    assert_eq!(task.unwrap().body.var1, "thread_task");
}
