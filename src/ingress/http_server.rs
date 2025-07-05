use axum::{extract::State, routing::post, Router, Json, http::StatusCode};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::ingress::types::{TaskRequest, TaskResponse};

// Handler for POST /trigger
pub async fn trigger_task_handler(
    State(state): State<Arc<Mutex<Vec<TaskRequest>>>>,
    Json(req): Json<TaskRequest>,
) -> (StatusCode, Json<TaskResponse>) {
    // Verify signature
    if !verify_request_signature(&req) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(TaskResponse {
                success: false,
                message: "Invalid signature".to_string(),
            }),
        );
    }
    // Add to queue
    {
        let mut queue = state.lock().await;
        queue.push(req.clone());
    }
    (
        StatusCode::OK,
        Json(TaskResponse {
            success: true,
            message: "Task queued".to_string(),
        }),
    )
}

// Dummy ECDSA signature verification (replace with real logic)
pub fn verify_request_signature(_req: &TaskRequest) -> bool {
    // TODO: Implement ECDSA signature verification
    // 1. Hash the body (address, number)
    // 2. Recover the address from the signature
    // 3. Check that recovered address == req.requester
    // For now, always return true for placeholder
    true
}

// Start the HTTP server in a background task
pub async fn start_http_server(queue: Arc<Mutex<Vec<TaskRequest>>>, addr: &str) {
    let app = Router::new()
        .route("/trigger", post(trigger_task_handler))
        .with_state(queue);
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind HTTP server");
    info!("ListeningCreator HTTP server running on {}", addr);
    axum::serve(listener, app).await.expect("HTTP server failed");
} 