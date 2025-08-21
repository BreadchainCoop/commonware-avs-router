use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{error, warn};

use super::providers::CounterProvider;

use crate::creator::core::Creator;
use crate::ingress::types::TaskRequest;

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

/// Configuration for listening creators
#[derive(Debug, Clone)]
pub struct CreatorConfig {
    pub polling_interval_ms: u64,
    pub timeout_ms: u64,
}

impl Default for CreatorConfig {
    fn default() -> Self {
        Self {
            polling_interval_ms: 100,
            timeout_ms: 5000,
        }
    }
}

/// Creator for the counter usecase without ingress.
pub struct CounterCreator {
    provider: Arc<CounterProvider>,
}

impl CounterCreator {
    pub fn new(provider: CounterProvider) -> Self {
        Self {
            provider: Arc::new(provider),
        }
    }
}

#[async_trait]
impl Creator for CounterCreator {
    async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64)> {
        let round = self.provider.get_current_round().await?;
        // Task data is now handled directly via get_task_metadata()
        // Domain decision: payload is ABI-encoded round
        let payload = self.provider.encode_round(round);
        Ok((payload, round))
    }

    fn get_task_metadata(&self) -> HashMap<String, String> {
        // Use the default task data values for consistency
        let mut metadata = HashMap::new();
        metadata.insert("var1".to_string(), "default_var1".to_string());
        metadata.insert("var2".to_string(), "default_var2".to_string());
        metadata.insert("var3".to_string(), "default_var3".to_string());
        metadata
    }
}

/// Creator for the counter usecase that listens for external requests.
pub struct ListeningCounterCreator<Q: TaskQueue + Send + Sync + 'static> {
    provider: Arc<CounterProvider>,
    queue: Arc<Q>,
    config: CreatorConfig,
    current_task: std::sync::Mutex<Option<TaskRequest>>,
}

impl<Q: TaskQueue + Send + Sync + 'static> ListeningCounterCreator<Q> {
    pub fn new(provider: CounterProvider, queue: Q, config: CreatorConfig) -> Self {
        Self {
            provider: Arc::new(provider),
            queue: Arc::new(queue),
            config,
            current_task: std::sync::Mutex::new(None),
        }
    }

    async fn wait_for_task(&self) -> Result<TaskRequest> {
        use tokio::time::{Duration, sleep};
        let mut attempts = 0;
        let max_attempts = self.config.timeout_ms / self.config.polling_interval_ms;
        loop {
            if let Some(task) = self.queue.pop() {
                // Store the task for metadata access
                if let Ok(mut current_task) = self.current_task.lock() {
                    *current_task = Some(task.clone());
                }
                return Ok(task);
            }
            attempts += 1;
            if attempts >= max_attempts {
                break;
            }
            sleep(Duration::from_millis(self.config.polling_interval_ms)).await;
        }
        Err(anyhow::anyhow!(
            "Timeout waiting for task after {}ms",
            self.config.timeout_ms
        ))
    }
}

#[async_trait]
impl<Q: TaskQueue + Send + Sync + 'static> Creator for ListeningCounterCreator<Q> {
    async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64)> {
        let _task = self.wait_for_task().await?;
        let round = self.provider.get_current_round().await?;
        // Task data is now handled directly via get_task_metadata() from the request
        let payload = self.provider.encode_round(round);
        Ok((payload, round))
    }

    fn get_task_metadata(&self) -> HashMap<String, String> {
        // Use the default task data values for consistency
        let mut metadata = HashMap::new();
        metadata.insert("var1".to_string(), "default_var1".to_string());
        metadata.insert("var2".to_string(), "default_var2".to_string());
        metadata.insert("var3".to_string(), "default_var3".to_string());
        metadata
    }
}

/// This enum allows us to use concrete types at compile time while still
/// supporting different creator implementations. This enables the generic
/// orchestrator to work without runtime polymorphism.
pub enum CounterCreatorType {
    /// Basic counter creator without ingress
    Basic(CounterCreator),
    /// Listening counter creator with HTTP ingress
    Listening(ListeningCounterCreator<SimpleTaskQueue>),
}

#[async_trait]
impl Creator for CounterCreatorType {
    async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64)> {
        match self {
            CounterCreatorType::Basic(creator) => creator.get_payload_and_round().await,
            CounterCreatorType::Listening(creator) => creator.get_payload_and_round().await,
        }
    }

    fn get_task_metadata(&self) -> HashMap<String, String> {
        match self {
            CounterCreatorType::Basic(creator) => creator.get_task_metadata(),
            CounterCreatorType::Listening(creator) => creator.get_task_metadata(),
        }
    }
}
