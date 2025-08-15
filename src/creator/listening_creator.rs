use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use tracing::{debug, warn};

use crate::creator::core::{
    Creator, CreatorConfig, StateProvider, TaskData, TaskDataFactory, TaskQueue, TaskRequest,
};

/// Listening creator that waits for external task requests from a queue
pub struct ListeningCreator<P, F, Q>
where
    P: StateProvider,
    F: TaskDataFactory<State = P::State>,
    Q: TaskQueue,
{
    state_provider: Arc<P>,
    task_data_factory: Arc<F>,
    queue: Arc<Q>,
    config: CreatorConfig,
}

impl<P, F, Q> ListeningCreator<P, F, Q>
where
    P: StateProvider,
    F: TaskDataFactory<State = P::State>,
    Q: TaskQueue,
{
    /// Create a new listening creator
    pub fn new(state_provider: P, task_data_factory: F, queue: Q, config: CreatorConfig) -> Self {
        Self {
            state_provider: Arc::new(state_provider),
            task_data_factory: Arc::new(task_data_factory),
            queue: Arc::new(queue),
            config,
        }
    }

    /// Get a reference to the queue for external access
    #[allow(dead_code)]
    pub fn get_queue(&self) -> Arc<Q> {
        self.queue.clone()
    }

    /// Wait for a task from the queue
    async fn wait_for_task(&self) -> Result<TaskRequest> {
        let mut attempts = 0;
        let max_attempts = self.config.timeout_ms / self.config.polling_interval_ms;

        loop {
            if let Some(task) = self.queue.pop() {
                debug!("Received task from queue: {:?}", task);
                return Ok(task);
            }

            attempts += 1;
            if attempts >= max_attempts {
                return Err(anyhow::anyhow!(
                    "Timeout waiting for task after {}ms",
                    self.config.timeout_ms
                ));
            }

            warn!(
                "No task available, waiting {}ms (attempt {}/{})",
                self.config.polling_interval_ms, attempts, max_attempts
            );
            sleep(Duration::from_millis(self.config.polling_interval_ms)).await;
        }
    }

    /// Create task data from state and request
    async fn create_task_data_from_request(
        &self,
        state: &P::State,
        request: &TaskRequest,
    ) -> Result<F::TaskData> {
        self.task_data_factory
            .create_task_data_from_request(state, request)
            .await
    }

    /// Encode task data into a payload
    fn encode_task_data(&self, task_data: F::TaskData, encoded_state: Vec<u8>) -> Vec<u8> {
        task_data.encode_into_payload(encoded_state)
    }
}

#[async_trait]
impl<P, F, Q> Creator for ListeningCreator<P, F, Q>
where
    P: StateProvider + 'static,
    F: TaskDataFactory<State = P::State> + 'static,
    Q: TaskQueue + 'static,
    P::State: 'static,
    F::TaskData: 'static,
{
    type State = P::State;
    type TaskData = F::TaskData;

    async fn create_payload_and_state(&self) -> Result<(Vec<u8>, Self::State)> {
        // Wait for a task from the queue
        let task = self.wait_for_task().await?;

        // Get current state with retry logic
        let current_state = self.get_current_state_with_retry().await?;

        // Encode the state
        let encoded_state = self.state_provider.encode_state(&current_state).await;

        // Create task data from the request
        let task_data = self
            .create_task_data_from_request(&current_state, &task)
            .await?;

        // Encode into final payload
        let payload = self.encode_task_data(task_data, encoded_state);

        debug!(
            "Created payload for state {:?} with task {:?}",
            current_state, task
        );
        Ok((payload, current_state))
    }

    async fn get_current_state(&self) -> Result<Self::State> {
        self.state_provider.get_current_state().await
    }
}

impl<P, F, Q> ListeningCreator<P, F, Q>
where
    P: StateProvider,
    F: TaskDataFactory<State = P::State>,
    Q: TaskQueue,
{
    /// Get current state with retry logic
    async fn get_current_state_with_retry(&self) -> Result<P::State> {
        let mut last_error = None;

        for attempt in 0..self.config.max_retries {
            match self.state_provider.get_current_state().await {
                Ok(state) => return Ok(state),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.config.max_retries - 1 {
                        sleep(Duration::from_millis(self.config.polling_interval_ms)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            anyhow::anyhow!(
                "Failed to get current state after {} attempts",
                self.config.max_retries
            )
        }))
    }
}
