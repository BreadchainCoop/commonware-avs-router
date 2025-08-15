use crate::creator::core::{State, TaskData};

/// Test-specific state type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestState {
    pub value: u64,
}

impl State for TestState {}

/// Test-specific task data type
#[derive(Debug, Clone)]
pub struct TestTaskData {
    pub var1: String,
    pub var2: String,
    pub var3: String,
}

impl TaskData for TestTaskData {
    fn encode_into_payload(self, encoded_state: Vec<u8>) -> Vec<u8> {
        let mut payload = encoded_state;
        payload.extend_from_slice(self.var1.as_bytes());
        payload.push(0); // null terminator
        payload.extend_from_slice(self.var2.as_bytes());
        payload.push(0); // null terminator
        payload.extend_from_slice(self.var3.as_bytes());
        payload.push(0); // null terminator
        payload
    }
}
