use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{error, warn};

/// Represents any type of state that can be retrieved and encoded
pub trait State: Debug + Send + Sync + Clone + Default {}

/// Generic default state implementation for primitive types
/// This can be used by any state type that wraps a primitive with a default value
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct DefaultState<T: Default + Clone + Send + Sync + std::fmt::Debug>(pub T);

impl<T: Default + Clone + Send + Sync + std::fmt::Debug> State for DefaultState<T> {}

/// Represents task data that can be encoded into a payload
pub trait TaskData: Debug + Send + Sync {
    /// Encode this task data into a payload
    fn encode_into_payload(self, encoded_state: Vec<u8>) -> Vec<u8>;
}

/// A provider that can retrieve the current state
#[async_trait]
pub trait StateProvider: Send + Sync {
    type State: State;

    /// Get the current state
    async fn get_current_state(&self) -> Result<Self::State>;

    /// Encode a state value into bytes
    async fn encode_state(&self, state: &Self::State) -> Vec<u8>;
}

/// A factory that creates task data from state
#[async_trait]
pub trait TaskDataFactory: Send + Sync {
    type State: State;
    type TaskData: TaskData;

    /// Create task data from the current state
    async fn create_task_data(&self, state: &Self::State) -> Result<Self::TaskData>;

    /// Create task data from state and an external request
    async fn create_task_data_from_request(
        &self,
        state: &Self::State,
        request: &TaskRequest,
    ) -> Result<Self::TaskData>;
}

/// A queue that can hold and provide task requests
pub trait TaskQueue: Send + Sync {
    /// Add a task to the queue
    #[allow(dead_code)]
    fn push(&self, task: TaskRequest);

    /// Remove and return the next task from the queue
    fn pop(&self) -> Option<TaskRequest>;
}

/// Simple in-memory task queue using Arc<Mutex> with proper error handling
#[derive(Clone)]
pub struct SimpleTaskQueue {
    queue: Arc<Mutex<Vec<TaskRequest>>>,
    timeout_ms: u64,
    max_retries: u32,
}

impl SimpleTaskQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
            timeout_ms: 1000, // 1 second default timeout
            max_retries: 3,   // 3 retries by default
        }
    }

    #[allow(dead_code)]
    pub fn with_timeout(timeout_ms: u64) -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
            timeout_ms,
            max_retries: 3,
        }
    }

    #[allow(dead_code)]
    pub fn with_config(timeout_ms: u64, max_retries: u32) -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
            timeout_ms,
            max_retries,
        }
    }

    pub fn get_queue(&self) -> Arc<Mutex<Vec<TaskRequest>>> {
        self.queue.clone()
    }

    /// Try to acquire the lock with timeout and retries
    fn try_lock_with_timeout(&self) -> Result<std::sync::MutexGuard<'_, Vec<TaskRequest>>, String> {
        let start_time = Instant::now();
        let timeout_duration = Duration::from_millis(self.timeout_ms);

        for attempt in 0..self.max_retries {
            // Try to acquire the lock
            match self.queue.try_lock() {
                Ok(guard) => return Ok(guard),
                Err(_) => {
                    // Check if we've exceeded the timeout
                    if start_time.elapsed() >= timeout_duration {
                        return Err(format!(
                            "Failed to acquire lock after {}ms timeout ({} attempts)",
                            self.timeout_ms,
                            attempt + 1
                        ));
                    }

                    // Small delay before retry to avoid busy waiting
                    std::thread::sleep(Duration::from_millis(1));
                }
            }
        }

        Err(format!(
            "Failed to acquire lock after {} retries",
            self.max_retries
        ))
    }
}

impl Default for SimpleTaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskQueue for SimpleTaskQueue {
    fn push(&self, task: TaskRequest) {
        match self.try_lock_with_timeout() {
            Ok(mut queue) => {
                queue.push(task);
            }
            Err(e) => {
                error!("Failed to push task to queue: {}", e);
                warn!("Task dropped due to lock timeout: {:?}", task);
            }
        }
    }

    fn pop(&self) -> Option<TaskRequest> {
        match self.try_lock_with_timeout() {
            Ok(mut queue) => queue.pop(),
            Err(e) => {
                error!("Failed to pop task from queue: {}", e);
                None
            }
        }
    }
}

/// Configuration for creators
#[derive(Debug, Clone)]
pub struct CreatorConfig {
    pub polling_interval_ms: u64,
    pub max_retries: u32,
    pub timeout_ms: u64,
}

impl Default for CreatorConfig {
    fn default() -> Self {
        Self {
            polling_interval_ms: 100,
            max_retries: 3,
            timeout_ms: 5000,
        }
    }
}

/// Re-export task request types from ingress
pub use crate::ingress::types::TaskRequest;

/// The main creator trait that orchestrates the entire process
#[async_trait]
pub trait Creator: Send + Sync {
    type State: State;
    type TaskData: TaskData;

    /// Create a payload and return the current state
    async fn create_payload_and_state(&self) -> Result<(Vec<u8>, Self::State)>;

    /// Get just the current state
    #[allow(dead_code)]
    async fn get_current_state(&self) -> Result<Self::State>;
}

/// Type alias for boxed creators (enables runtime polymorphism)
pub type BoxedCreator<S, T> = Box<dyn Creator<State = S, TaskData = T>>;
