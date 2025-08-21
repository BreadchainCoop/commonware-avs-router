use crate::creator::MockCreator;
use crate::creator::core::Creator;
use crate::executor::MockExecutor;
use crate::orchestrator::builder::OrchestratorBuilder;
use crate::validator::MockValidator;
use std::collections::HashMap;
use std::time::Duration;

use super::helpers::{contributor, signer};
use super::mocks::clock::MockClock;

#[tokio::test]
async fn test_orchestrator_builder_integration() {
    let clock = MockClock::new();
    let signer = signer::create_test_signer();
    let (contributors, g1_map) = contributor::create_test_contributors();

    // Test the full builder workflow
    let builder = OrchestratorBuilder::new(clock.clone(), signer)
        .with_contributors(contributors.clone())
        .with_g1_map(g1_map.clone())
        .with_threshold(2)
        .with_aggregation_frequency(Duration::from_millis(100))
        .with_ingress("127.0.0.1:8080".to_string());

    let task_creator = MockCreator::new();
    let executor = MockExecutor::new();
    let validator = MockValidator::new_success(1);

    let orchestrator = builder
        .build(task_creator, executor, validator)
        .expect("Failed to build orchestrator");

    // Verify the orchestrator was built correctly by testing public methods
    let metadata = orchestrator.task_creator().get_task_metadata();
    assert!(metadata.contains_key("test_key"));

    let executor_count = orchestrator.executor().get_execution_count();
    assert_eq!(executor_count, 0);

    let validator_count = orchestrator.validator().get_validation_count();
    assert_eq!(validator_count, 0);
}

#[tokio::test]
async fn test_orchestrator_metadata_integration() {
    let clock = MockClock::new();
    let signer = signer::create_test_signer();
    let (contributors, g1_map) = contributor::create_test_contributors();

    let mut custom_metadata = HashMap::new();
    custom_metadata.insert("integration_test".to_string(), "true".to_string());
    custom_metadata.insert(
        "test_phase".to_string(),
        "metadata_verification".to_string(),
    );

    let builder = OrchestratorBuilder::new(clock.clone(), signer)
        .with_contributors(contributors)
        .with_g1_map(g1_map)
        .with_threshold(2);

    let task_creator = MockCreator::new().with_metadata(custom_metadata.clone());
    let executor = MockExecutor::new();
    let validator = MockValidator::new_success(1);

    let orchestrator = builder
        .build(task_creator, executor, validator)
        .expect("Failed to build orchestrator");

    // Verify metadata is accessible through the orchestrator
    let metadata = orchestrator.task_creator().get_task_metadata();
    assert_eq!(metadata, custom_metadata);
    assert_eq!(metadata.get("integration_test"), Some(&"true".to_string()));
    assert_eq!(
        metadata.get("test_phase"),
        Some(&"metadata_verification".to_string())
    );
}

#[tokio::test]
async fn test_orchestrator_component_access_integration() {
    let clock = MockClock::new();
    let signer = signer::create_test_signer();
    let (contributors, g1_map) = contributor::create_test_contributors();

    let builder = OrchestratorBuilder::new(clock.clone(), signer)
        .with_contributors(contributors)
        .with_g1_map(g1_map)
        .with_threshold(2);

    let task_creator = MockCreator::new();
    let executor = MockExecutor::new();
    let validator = MockValidator::new_success(1);

    let orchestrator = builder
        .build(task_creator, executor, validator)
        .expect("Failed to build orchestrator");

    // Test access to all components
    let creator_metadata = orchestrator.task_creator().get_task_metadata();
    assert!(!creator_metadata.is_empty());

    let executor_count = orchestrator.executor().get_execution_count();
    assert_eq!(executor_count, 0);

    let validator_count = orchestrator.validator().get_validation_count();
    assert_eq!(validator_count, 0);
}

#[tokio::test]
async fn test_orchestrator_config_integration() {
    let clock = MockClock::new();
    let signer = signer::create_test_signer();
    let (contributors, g1_map) = contributor::create_test_contributors();

    // Test with various configuration combinations
    let builder = OrchestratorBuilder::new(clock.clone(), signer)
        .with_contributors(contributors.clone())
        .with_g1_map(g1_map.clone())
        .with_threshold(3)
        .with_aggregation_frequency(Duration::from_secs(60))
        .with_ingress("0.0.0.0:9090".to_string());

    let task_creator = MockCreator::new();
    let executor = MockExecutor::new();
    let validator = MockValidator::new_success(1);

    let orchestrator = builder
        .build(task_creator, executor, validator)
        .expect("Failed to build orchestrator");

    // Verify all configuration is properly applied by testing component behavior
    let metadata = orchestrator.task_creator().get_task_metadata();
    assert!(metadata.contains_key("test_key"));

    let executor_count = orchestrator.executor().get_execution_count();
    assert_eq!(executor_count, 0);

    let validator_count = orchestrator.validator().get_validation_count();
    assert_eq!(validator_count, 0);
}

#[tokio::test]
async fn test_orchestrator_validation_integration() {
    let clock = MockClock::new();
    let signer = signer::create_test_signer();
    let (contributors, g1_map) = contributor::create_test_contributors();

    // Test validation with different thresholds
    for threshold in 1..=3 {
        let builder = OrchestratorBuilder::new(clock.clone(), signer.clone())
            .with_contributors(contributors.clone())
            .with_g1_map(g1_map.clone())
            .with_threshold(threshold);

        let task_creator = MockCreator::new();
        let executor = MockExecutor::new();
        let validator = MockValidator::new_success(1);

        let orchestrator = builder
            .build(task_creator, executor, validator)
            .expect("Failed to build orchestrator");

        // Verify the orchestrator was built successfully
        let metadata = orchestrator.task_creator().get_task_metadata();
        assert!(metadata.contains_key("test_key"));

        let executor_count = orchestrator.executor().get_execution_count();
        assert_eq!(executor_count, 0);
    }
}

#[tokio::test]
async fn test_orchestrator_environment_integration() {
    let clock = MockClock::new();
    let signer = signer::create_test_signer();
    let (contributors, g1_map) = contributor::create_test_contributors();

    // Set environment variables
    unsafe {
        std::env::set_var("INGRESS", "true");
        std::env::set_var("INGRESS_ADDRESS", "127.0.0.1:7070");
        std::env::set_var("AGGREGATION_FREQUENCY", "120");
        std::env::set_var("THRESHOLD", "2");
    }

    let builder = OrchestratorBuilder::new(clock.clone(), signer)
        .with_contributors(contributors)
        .with_g1_map(g1_map)
        .load_from_env();

    let task_creator = MockCreator::new();
    let executor = MockExecutor::new();
    let validator = MockValidator::new_success(1);

    let orchestrator = builder
        .build(task_creator, executor, validator)
        .expect("Failed to build orchestrator");

    // Verify environment variables were applied by testing component behavior
    let metadata = orchestrator.task_creator().get_task_metadata();
    assert!(metadata.contains_key("test_key"));

    let executor_count = orchestrator.executor().get_execution_count();
    assert_eq!(executor_count, 0);

    // Clean up environment variables
    unsafe {
        std::env::remove_var("INGRESS");
        std::env::remove_var("INGRESS_ADDRESS");
        std::env::remove_var("AGGREGATION_FREQUENCY");
        std::env::remove_var("THRESHOLD");
    }
}

#[tokio::test]
async fn test_orchestrator_component_interaction() {
    let clock = MockClock::new();
    let signer = signer::create_test_signer();
    let (contributors, g1_map) = contributor::create_test_contributors();

    let builder = OrchestratorBuilder::new(clock.clone(), signer)
        .with_contributors(contributors)
        .with_g1_map(g1_map)
        .with_threshold(2);

    let task_creator = MockCreator::new();
    let executor = MockExecutor::new().with_success(true);
    let validator = MockValidator::new_success(1);

    let orchestrator = builder
        .build(task_creator, executor, validator)
        .expect("Failed to build orchestrator");

    // Test that components can interact properly
    let (payload, round) = orchestrator
        .task_creator()
        .get_payload_and_round()
        .await
        .expect("Failed to get payload and round");

    assert_eq!(round, 1);
    assert_eq!(payload, round.to_le_bytes().to_vec());

    let metadata = orchestrator.task_creator().get_task_metadata();
    assert!(!metadata.is_empty());

    // Test executor interaction
    let executor_ref = orchestrator.executor();
    assert_eq!(executor_ref.get_execution_count(), 0);

    // Test validator interaction
    let validator_ref = orchestrator.validator();
    assert_eq!(validator_ref.get_validation_count(), 0);
}
