use anyhow::Result;
use async_trait::async_trait;

use super::types::DefaultTaskData;
use crate::creator::core::{DefaultState, TaskDataFactory, TaskRequest};

// Use DefaultState directly instead of type alias
type CounterState = DefaultState<u64>;

/// Default task data factory for counter operations
pub struct DefaultTaskDataFactory;

#[async_trait]
impl TaskDataFactory for DefaultTaskDataFactory {
    type State = CounterState;
    type TaskData = DefaultTaskData;

    async fn create_task_data(&self, _state: &Self::State) -> Result<Self::TaskData> {
        Ok(DefaultTaskData::default())
    }

    async fn create_task_data_from_request(
        &self,
        _state: &Self::State,
        request: &TaskRequest,
    ) -> Result<Self::TaskData> {
        Ok(DefaultTaskData {
            var1: request.body.var1.clone(),
            var2: request.body.var2.clone(),
            var3: request.body.var3.clone(),
        })
    }
}
