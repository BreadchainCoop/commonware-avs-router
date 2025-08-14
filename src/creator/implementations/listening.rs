use tracing::info;

use crate::creator::{
    base::BaseCreator, interface::TaskCreatorTrait, types::TaskData, types::TaskQueue,
};
use crate::ingress::TaskRequest;

use super::super::base::StateProviderTrait;

/// Listening creator implementation that uses a task queue.
///
/// This creator is generic over the state provider's state type and queue type,
/// allowing it to work with any state provider and queue implementation.
pub struct ListeningCreator<P: StateProviderTrait, Q: TaskQueue> {
    base: BaseCreator<P>,
    pub queue: Q,
}

impl<P: StateProviderTrait, Q: TaskQueue> ListeningCreator<P, Q> {
    pub fn new(state_provider: P, queue: Q) -> Self {
        let base = BaseCreator::new(state_provider);
        Self { base, queue }
    }

    // Pulls the next task from the queue, or returns None if empty
    pub async fn get_next_task(&self) -> Option<TaskRequest> {
        self.queue.pop()
    }

    // Single entry point that can be called by the orchestrator
    // This is where queue requests would be pulled from
    pub async fn get_payload_and_state(&self) -> anyhow::Result<(Vec<u8>, P::State)> {
        // Wait for a task to be available
        let task = loop {
            if let Some(task) = self.get_next_task().await {
                break task;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        };
        let current_state = self.get_current_state().await?;
        let encoded_state = self.base.encode_state(&current_state).await;
        let task_data = self.get_task_data(task).await?;
        let payload = task_data.encode_into_payload(encoded_state);
        Ok((payload, current_state))
    }

    // Optional: Method to get payload for a specific state
    #[allow(dead_code)]
    pub async fn get_payload_for_state(
        &self,
        state: P::State,
    ) -> anyhow::Result<(Vec<u8>, P::State)> {
        let payload = self.base.encode_state(&state).await;
        info!("Created payload for specific state: {:?}", state);
        Ok((payload, state))
    }

    async fn get_task_data(&self, task: TaskRequest) -> anyhow::Result<TaskData> {
        Ok(TaskData::new(
            task.body.var1,
            task.body.var2,
            task.body.var3,
        ))
    }
}

#[async_trait::async_trait]
impl<P: StateProviderTrait, Q: TaskQueue> TaskCreatorTrait<P::State> for ListeningCreator<P, Q> {
    async fn get_payload_and_state(&self) -> anyhow::Result<(Vec<u8>, P::State)> {
        self.get_payload_and_state()
            .await
            .map_err(|e| anyhow::anyhow!("ListeningCreator error: {}", e))
    }

    async fn get_current_state(&self) -> anyhow::Result<P::State> {
        self.base.get_current_state().await
    }
}
