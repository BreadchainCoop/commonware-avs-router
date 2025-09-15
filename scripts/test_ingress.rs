use reqwest;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingress_url = std::env::var("INGRESS_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let endpoint = format!("{}/trigger", ingress_url);
    
    println!("Testing ingress endpoint at: {}", endpoint);
    
    // Wait a bit for the server to be ready
    println!("Waiting for ingress server to be ready...");
    sleep(Duration::from_secs(5)).await;
    
    let client = reqwest::Client::new();
    
    // Try to ping the server multiple times
    let mut success_count = 0;
    let total_attempts = 5;
    
    for i in 1..=total_attempts {
        println!("\nAttempt {}/{}", i, total_attempts);
        
        let payload = json!({
            "id": format!("test-task-{}", i),
            "body": {
                "var1": "test-value-1",
                "var2": "test-value-2",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });
        
        println!("Sending request with payload: {}", serde_json::to_string_pretty(&payload)?);
        
        match client
            .post(&endpoint)
            .json(&payload)
            .timeout(Duration::from_secs(10))
            .send()
            .await
        {
            Ok(response) => {
                let status = response.status();
                let body = response.text().await?;
                
                println!("Response Status: {}", status);
                println!("Response Body: {}", body);
                
                if status.is_success() {
                    success_count += 1;
                    println!("✓ Request {} successful", i);
                    
                    // Parse response to verify it matches expected format
                    if let Ok(json_response) = serde_json::from_str::<serde_json::Value>(&body) {
                        if json_response.get("success") == Some(&json!(true)) {
                            println!("✓ Response indicates task was queued successfully");
                        }
                    }
                } else {
                    println!("✗ Request {} failed with status: {}", i, status);
                }
            }
            Err(e) => {
                println!("✗ Request {} failed with error: {}", i, e);
                
                // Check if it's a connection error
                if e.is_connect() {
                    println!("   Connection error - ingress server may not be running");
                } else if e.is_timeout() {
                    println!("   Request timeout - server may be overloaded");
                }
            }
        }
        
        // Wait between requests
        if i < total_attempts {
            sleep(Duration::from_secs(2)).await;
        }
    }
    
    println!("\n=== Test Results ===");
    println!("Successful requests: {}/{}", success_count, total_attempts);
    
    if success_count == 0 {
        println!("✗ All requests failed - ingress server may not be running or accessible");
        std::process::exit(1);
    } else if success_count < total_attempts {
        println!("⚠ Some requests failed - ingress server may be unstable");
        std::process::exit(2);
    } else {
        println!("✓ All requests successful - ingress server is working properly");
    }
    
    Ok(())
}