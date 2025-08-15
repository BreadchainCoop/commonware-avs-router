use crate::creator::core::TaskData;

/// Test-specific task data type
#[derive(Debug, Clone)]
pub struct TestTaskData {
    pub var1: String,
    pub var2: String,
    pub var3: String,
}

impl TaskData for TestTaskData {
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
