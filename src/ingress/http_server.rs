use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use std::sync::{Arc, Mutex};
use tracing::{error, info};

use crate::ingress::types::{TaskRequest, TaskResponse};

// Handler for POST /trigger
pub async fn trigger_task_handler(
    State(state): State<Arc<Mutex<Vec<TaskRequest>>>>,
    Json(req): Json<TaskRequest>,
) -> (StatusCode, Json<TaskResponse>) {
    info!("Received task request via HTTP: {:?}", req.body.metadata);
    // Add business logic here such as api-key verification, ecdsa signature verification, etc retrieved from the TaskRequest
    // for example, if we assume `var1` is the api-key
    // let var1 = req.body.var1;
    // if !is_valid_api_key(var1) {
    //     return (StatusCode::UNAUTHORIZED, Json(TaskResponse {
    //         success: false,
    //         message: "Invalid api-key".to_string(),
    //     }));
    // }
    // Add to queue
    {
        if let Ok(mut queue) = state.lock() {
            let queue_size_before = queue.len();
            queue.push(req.clone());
            info!("Task added to HTTP queue. Queue size: {} -> {}", queue_size_before, queue.len());
        } else {
            error!("Failed to acquire lock on HTTP queue");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(TaskResponse {
                success: false,
                message: "Failed to queue task".to_string(),
            }));
        }
    }
    (
        StatusCode::OK,
        Json(TaskResponse {
            success: true,
            message: "Task queued".to_string(),
        }),
    )
}

// Start the HTTP server in a background task
pub async fn start_http_server(queue: Arc<Mutex<Vec<TaskRequest>>>, addr: &str) {
    let app = Router::new()
        .route("/trigger", post(trigger_task_handler))
        .with_state(queue);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind HTTP server");
    info!("Creator HTTP server running on {}", addr);
    axum::serve(listener, app)
        .await
        .expect("HTTP server failed");
}
