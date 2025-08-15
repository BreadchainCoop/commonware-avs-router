use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

/// Represents any type of state that can be retrieved and encoded
pub trait State: Debug + Send + Sync + Clone {}

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

/// Simple in-memory task queue using Arc<Mutex>
#[derive(Clone)]
pub struct SimpleTaskQueue {
    queue: Arc<Mutex<Vec<TaskRequest>>>,
}

impl SimpleTaskQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_queue(&self) -> Arc<Mutex<Vec<TaskRequest>>> {
        self.queue.clone()
    }
}

impl Default for SimpleTaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskQueue for SimpleTaskQueue {
    fn push(&self, task: TaskRequest) {
        if let Ok(mut queue) = self.queue.try_lock() {
            queue.push(task);
        }
    }

    fn pop(&self) -> Option<TaskRequest> {
        if let Ok(mut queue) = self.queue.try_lock() {
            queue.pop()
        } else {
            None
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
