use crate::creator::core::{State, TaskData};

/// Counter state (u64)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CounterState(pub u64);

impl State for CounterState {}

impl std::fmt::Display for CounterState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Default task data for counter operations
#[derive(Debug)]
pub struct DefaultTaskData {
    pub var1: String,
    pub var2: String,
    pub var3: String,
}

impl TaskData for DefaultTaskData {
    fn encode_into_payload(self, mut payload: Vec<u8>) -> Vec<u8> {
        payload.extend_from_slice(self.var1.as_bytes());
        payload.push(0); // null terminator
        payload.extend_from_slice(self.var2.as_bytes());
        payload.push(0); // null terminator
        payload.extend_from_slice(self.var3.as_bytes());
        payload.push(0); // null terminator
        payload
    }
}

impl Default for DefaultTaskData {
    fn default() -> Self {
        Self {
            var1: "default_var1".to_string(),
            var2: "default_var2".to_string(),
            var3: "default_var3".to_string(),
        }
    }
}
