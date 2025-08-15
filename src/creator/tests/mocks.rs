use anyhow::Result;
use async_trait::async_trait;
use std::sync::Mutex;

use super::types::TestTaskData;
use crate::creator::core::{
    DefaultState, State, StateProvider, TaskDataFactory, TaskQueue, TaskRequest,
};

// Use DefaultState directly instead of type alias
type TestState = DefaultState<u64>;

/// Mock state provider for testing
#[allow(dead_code)]
pub struct MockStateProvider<S: State> {
    state: S,
    should_fail: bool,
}

impl<S: State> MockStateProvider<S> {
    #[allow(dead_code)]
    pub fn new(state: S) -> Self {
        Self {
            state,
            should_fail: false,
        }
    }

    #[allow(dead_code)]
    pub fn new_failing() -> Self {
        Self {
            state: S::default(),
            should_fail: true,
        }
    }

    #[allow(dead_code)]
    pub fn get_state(&self) -> &S {
        &self.state
    }
}

#[async_trait]
impl<S: State> StateProvider for MockStateProvider<S> {
    type State = S;

    async fn get_current_state(&self) -> Result<Self::State> {
        if self.should_fail {
            Err(anyhow::anyhow!("Mock provider failure"))
        } else {
            Ok(self.state.clone())
        }
    }

    async fn encode_state(&self, _state: &Self::State) -> Vec<u8> {
        // Default encoding for testing
        vec![1, 2, 3, 4]
    }
}

/// Mock task data factory for testing
#[allow(dead_code)]
pub struct MockTaskDataFactory;

#[async_trait]
impl TaskDataFactory for MockTaskDataFactory {
    type State = TestState;
    type TaskData = TestTaskData;

    async fn create_task_data(&self, _state: &TestState) -> Result<TestTaskData> {
        Ok(TestTaskData {
            var1: "default_var1".to_string(),
            var2: "default_var2".to_string(),
            var3: "default_var3".to_string(),
        })
    }

    async fn create_task_data_from_request(
        &self,
        _state: &TestState,
        request: &TaskRequest,
    ) -> Result<TestTaskData> {
        Ok(TestTaskData {
            var1: request.body.var1.clone(),
            var2: request.body.var2.clone(),
            var3: request.body.var3.clone(),
        })
    }
}

/// Simple task queue implementation for testing
#[derive(Clone)]
pub struct MockTaskQueue {
    queue: std::sync::Arc<Mutex<Vec<TaskRequest>>>,
}

impl MockTaskQueue {
    pub fn new() -> Self {
        Self {
            queue: std::sync::Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[allow(dead_code)]
    pub fn get_queue(&self) -> std::sync::Arc<Mutex<Vec<TaskRequest>>> {
        self.queue.clone()
    }
}

impl Default for MockTaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskQueue for MockTaskQueue {
    fn push(&self, task: TaskRequest) {
        if let Ok(mut queue) = self.queue.lock() {
            queue.push(task);
        }
    }

    fn pop(&self) -> Option<TaskRequest> {
        if let Ok(mut queue) = self.queue.lock() {
            queue.pop()
        } else {
            None
        }
    }
}
