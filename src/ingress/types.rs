use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskRequestBody {
    pub address: String, // hex string
    pub number: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskRequest {
    pub requester: String, // hex string address
    pub signature: String, // hex string signature
    pub body: TaskRequestBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub success: bool,
    pub message: String,
} 