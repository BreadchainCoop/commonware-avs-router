use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskRequestBody {
    pub var1: String, 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskRequest {
    pub body: TaskRequestBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub success: bool,
    pub message: String,
} 