use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

use crate::creator::core::{Creator, CreatorConfig, StateProvider, TaskData, TaskDataFactory};

/// Default creator that generates payloads without external input
pub struct DefaultCreator<P, F>
where
    P: StateProvider,
    F: TaskDataFactory<State = P::State>,
{
    state_provider: Arc<P>,
    task_data_factory: Arc<F>,
    config: CreatorConfig,
}

impl<P, F> DefaultCreator<P, F>
where
    P: StateProvider,
    F: TaskDataFactory<State = P::State>,
{
    /// Create a new default creator
    pub fn new(state_provider: P, task_data_factory: F, config: CreatorConfig) -> Self {
        Self {
            state_provider: Arc::new(state_provider),
            task_data_factory: Arc::new(task_data_factory),
            config,
        }
    }

    /// Create task data from the current state
    async fn create_task_data(&self, state: &P::State) -> Result<F::TaskData> {
        self.task_data_factory.create_task_data(state).await
    }

    /// Encode task data into a payload
    fn encode_task_data(&self, task_data: &F::TaskData, encoded_state: Vec<u8>) -> Vec<u8> {
        task_data.encode_into_payload(encoded_state)
    }
}

#[async_trait]
impl<P, F> Creator for DefaultCreator<P, F>
where
    P: StateProvider + 'static,
    F: TaskDataFactory<State = P::State> + 'static,
    P::State: 'static,
    F::TaskData: 'static,
{
    type State = P::State;
    type TaskData = F::TaskData;

    async fn create_payload_and_state(&self) -> Result<(Vec<u8>, Self::State)> {
        // Get current state with retry logic
        let current_state = self.get_current_state_with_retry().await?;

        // Encode the state
        let encoded_state = self.state_provider.encode_state(&current_state).await;

        // Create task data
        let task_data = self.create_task_data(&current_state).await?;

        // Encode into final payload
        let payload = self.encode_task_data(&task_data, encoded_state);

        Ok((payload, current_state))
    }

    async fn get_current_state(&self) -> Result<Self::State> {
        self.state_provider.get_current_state().await
    }
}

impl<P, F> DefaultCreator<P, F>
where
    P: StateProvider,
    F: TaskDataFactory<State = P::State>,
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
