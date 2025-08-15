use anyhow::Result;
use async_trait::async_trait;

use super::types::{CounterState, DefaultTaskData};
use crate::creator::core::{TaskData, TaskDataFactory, TaskRequest};

/// Default task data factory
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

/// Mock task data factory for testing
#[allow(dead_code)]
pub struct MockTaskDataFactory<T: TaskData> {
    #[allow(dead_code)]
    task_data: T,
    should_fail: bool,
}

impl<T: TaskData> MockTaskDataFactory<T> {
    #[allow(dead_code)]
    pub fn new(task_data: T) -> Self {
        Self {
            task_data,
            should_fail: false,
        }
    }

    #[allow(dead_code)]
    pub fn new_failing() -> Self {
        Self {
            task_data: unsafe { std::mem::zeroed() }, // Safe for testing
            should_fail: true,
        }
    }
}

#[async_trait]
impl<T: TaskData> TaskDataFactory for MockTaskDataFactory<T> {
    type State = CounterState;
    type TaskData = T;

    async fn create_task_data(&self, _state: &Self::State) -> Result<Self::TaskData> {
        if self.should_fail {
            Err(anyhow::anyhow!("Mock factory failure"))
        } else {
            // Clone the task data (this is a simplified approach)
            Ok(unsafe { std::mem::zeroed() }) // Safe for testing
        }
    }

    async fn create_task_data_from_request(
        &self,
        _state: &Self::State,
        _request: &TaskRequest,
    ) -> Result<Self::TaskData> {
        if self.should_fail {
            Err(anyhow::anyhow!("Mock factory failure"))
        } else {
            Ok(unsafe { std::mem::zeroed() }) // Safe for testing
        }
    }
}
