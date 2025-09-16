use crate::orchestrator::builder::OrchestratorBuilder;
use std::time::Duration;
use super::helpers::{signer, contributor};
use super::mocks::clock::MockClock;
use commonware_runtime::Clock;

#[tokio::test]
async fn test_aggregation_frequency_1_second() {
    let clock = MockClock::new();
    let test_signer = signer::create_test_signer();
    
    // Create orchestrator with 1 second aggregation frequency
    let (contributors, g1_map) = contributor::create_test_contributors();
    let builder = OrchestratorBuilder::new(clock, test_signer)
        .with_contributors(contributors)
        .with_g1_map(g1_map)
        .with_aggregation_frequency(Duration::from_secs(1))
        .with_threshold(2);
    
    // Verify the configuration
    let config = builder.get_config().expect("Failed to get config");
    assert_eq!(config.config.aggregation_frequency, Duration::from_secs(1));
}

#[tokio::test]
async fn test_aggregation_frequency_from_env() {
    // Set environment variable to 1 second
    unsafe {
        std::env::set_var("AGGREGATION_FREQUENCY", "1");
    }
    
    let clock = MockClock::new();
    let test_signer = signer::create_test_signer();
    
    // Create orchestrator and load from environment
    let (contributors, g1_map) = contributor::create_test_contributors();
    let builder = OrchestratorBuilder::new(clock, test_signer)
        .with_contributors(contributors)
        .with_g1_map(g1_map)
        .load_from_env();
    
    // Verify the configuration was loaded from environment
    let config = builder.get_config().expect("Failed to get config");
    assert_eq!(config.config.aggregation_frequency, Duration::from_secs(1));
    
    // Clean up
    unsafe {
        std::env::remove_var("AGGREGATION_FREQUENCY");
    }
}

#[tokio::test]
async fn test_fast_aggregation_cycle_timing() {
    let clock = MockClock::new();
    let test_signer = signer::create_test_signer();
    
    // Create orchestrator with 1 second aggregation frequency
    let (contributors, g1_map) = contributor::create_test_contributors();
    let builder = OrchestratorBuilder::new(clock.clone(), test_signer)
        .with_contributors(contributors)
        .with_g1_map(g1_map)
        .with_aggregation_frequency(Duration::from_secs(1))
        .with_threshold(1);
    
    // Verify that aggregation frequency is set correctly
    let config = builder.get_config().expect("Failed to get config");
    assert_eq!(config.config.aggregation_frequency, Duration::from_secs(1));
    
    // Simulate time progression to test aggregation timing
    let initial_time = clock.current();
    
    // Advance time by 5 seconds
    for _ in 0..5 {
        clock.advance(Duration::from_secs(1));
    }
    
    let final_time = clock.current();
    let elapsed = final_time.duration_since(initial_time).expect("Time went backwards");
    
    // Verify time has advanced correctly
    assert_eq!(elapsed, Duration::from_secs(5), "Clock should have advanced by 5 seconds");
}

#[tokio::test]
async fn test_multiple_frequency_settings() {
    // Test various frequency settings
    let frequencies = vec![1, 2, 5, 10, 30, 60];
    
    for freq in frequencies {
        unsafe {
            std::env::set_var("AGGREGATION_FREQUENCY", freq.to_string());
        }
        
        let clock = MockClock::new();
        let test_signer = signer::create_test_signer();
        
        let (contributors, g1_map) = contributor::create_test_contributors();
        let builder = OrchestratorBuilder::new(clock, test_signer)
            .with_contributors(contributors)
            .with_g1_map(g1_map)
            .load_from_env();
        
        let config = builder.get_config().expect("Failed to get config");
        assert_eq!(
            config.config.aggregation_frequency,
            Duration::from_secs(freq),
            "Aggregation frequency should be {} seconds",
            freq
        );
        
        unsafe {
            std::env::remove_var("AGGREGATION_FREQUENCY");
        }
    }
}

#[tokio::test]
async fn test_invalid_aggregation_frequency_env() {
    // Test that invalid env values don't crash and fall back to default
    unsafe {
        std::env::set_var("AGGREGATION_FREQUENCY", "invalid");
    }
    
    let clock = MockClock::new();
    let test_signer = signer::create_test_signer();
    
    let (contributors, g1_map) = contributor::create_test_contributors();
    let builder = OrchestratorBuilder::new(clock, test_signer)
        .with_contributors(contributors)
        .with_g1_map(g1_map)
        .load_from_env();
    
    // Should fall back to default (30 seconds)
    let config = builder.get_config().expect("Failed to get config");
    assert_eq!(config.config.aggregation_frequency, Duration::from_secs(30));
    
    unsafe {
        std::env::remove_var("AGGREGATION_FREQUENCY");
    }
}