use anyhow::Result;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{error, warn};

use super::factories::DefaultTaskDataFactory;
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
    factory: Arc<DefaultTaskDataFactory>,
}

impl CounterCreator {
    pub fn new(provider: CounterProvider, factory: DefaultTaskDataFactory) -> Self {
        Self {
            provider: Arc::new(provider),
            factory: Arc::new(factory),
        }
    }
}

#[async_trait]
impl Creator for CounterCreator {
    async fn get_payload_and_round(&self) -> Result<(Vec<u8>, u64)> {
        let round = self.provider.get_current_round().await?;
        let _task = self.factory.create_task_data().await?;
        // Domain decision: payload is ABI-encoded round
        let payload = self.provider.encode_round(round);
        Ok((payload, round))
    }
}

/// Creator for the counter usecase that listens for external requests.
pub struct ListeningCounterCreator<Q: TaskQueue + Send + Sync + 'static> {
    provider: Arc<CounterProvider>,
    factory: Arc<DefaultTaskDataFactory>,
    queue: Arc<Q>,
    config: CreatorConfig,
}

impl<Q: TaskQueue + Send + Sync + 'static> ListeningCounterCreator<Q> {
    pub fn new(
        provider: CounterProvider,
        factory: DefaultTaskDataFactory,
        queue: Q,
        config: CreatorConfig,
    ) -> Self {
        Self {
            provider: Arc::new(provider),
            factory: Arc::new(factory),
            queue: Arc::new(queue),
            config,
        }
    }

    async fn wait_for_task(&self) -> Result<TaskRequest> {
        use tokio::time::{Duration, sleep};
        let mut attempts = 0;
        let max_attempts = self.config.timeout_ms / self.config.polling_interval_ms;
        loop {
            if let Some(task) = self.queue.pop() {
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
        let _task_data = self.factory.create_task_data_from_request(&_task).await?;
        let payload = self.provider.encode_round(round);
        Ok((payload, round))
    }
}
