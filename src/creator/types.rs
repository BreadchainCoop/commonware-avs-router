use crate::ingress::TaskRequest;

/// Trait for task queue operations
#[allow(dead_code)]
pub trait TaskQueue: Send + Sync {
    fn push(&self, task: TaskRequest);
    fn pop(&self) -> Option<TaskRequest>;
}

/// Task data structure for payload generation
#[derive(Debug, Clone)]
pub struct TaskData {
    pub var1: String,
    pub var2: String,
    pub var3: String,
}

impl TaskData {
    pub fn new(var1: String, var2: String, var3: String) -> Self {
        Self { var1, var2, var3 }
    }

    pub fn encode_into_payload(&self, mut payload: Vec<u8>) -> Vec<u8> {
        payload.extend_from_slice(self.var1.as_bytes());
        payload.push(0); // null terminator
        payload.extend_from_slice(self.var2.as_bytes());
        payload.push(0); // null terminator
        payload.extend_from_slice(self.var3.as_bytes());
        payload.push(0); // null terminator
        payload
    }
}

impl Default for TaskData {
    fn default() -> Self {
        Self {
            var1: "default_var1".to_string(),
            var2: "default_var2".to_string(),
            var3: "default_var3".to_string(),
        }
    }
}
