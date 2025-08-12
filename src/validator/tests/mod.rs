use crate::wire;
use alloy::sol_types::SolValue;
use alloy_primitives::U256;
use commonware_codec::Encode;
use commonware_cryptography::sha256::Digest;
use commonware_cryptography::{Hasher, Sha256};
use std::env;

use super::{
    factory::{self, ValidatorConfig, ValidatorType},
    generic::Validator,
    mock::MockValidator,
};

/// Test helper for creating test messages
fn create_test_message(round: u64) -> Vec<u8> {
    let aggregation = wire::Aggregation {
        round,
        var1: "test_var1".to_string(),
        var2: "test_var2".to_string(),
        var3: "test_var3".to_string(),
        payload: None,
    };
    aggregation.encode().to_vec()
}

/// Test helper for creating test payload hash
fn create_expected_payload_hash(round: u64) -> Digest {
    let payload = U256::from(round).abi_encode();
    let mut hasher = Sha256::new();
    hasher.update(&payload);
    hasher.finalize()
}

#[tokio::test]
async fn test_mock_validator_success() {
    let expected_round = 42;
    let mock_validator = MockValidator::new_success(expected_round);
    let validator = Validator::new(mock_validator);

    let test_message = create_test_message(expected_round);
    let expected_hash = create_expected_payload_hash(expected_round);

    // Test validate_and_return_expected_hash
    let result = validator
        .validate_and_return_expected_hash(&test_message)
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_hash);

    // Test get_payload_from_message
    let result = validator.get_payload_from_message(&test_message).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_hash);
}

#[tokio::test]
async fn test_mock_validator_failure() {
    let error_message = "Test validation failure".to_string();
    let mock_validator = MockValidator::new_failure(error_message.clone());
    let validator = Validator::new(mock_validator);

    let test_message = create_test_message(1);

    // Test validate_and_return_expected_hash
    let result = validator
        .validate_and_return_expected_hash(&test_message)
        .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains(&error_message));

    // Test get_payload_from_message
    let result = validator.get_payload_from_message(&test_message).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains(&error_message));
}

#[tokio::test]
async fn test_mock_validator_custom_success() {
    let expected_round = 100;
    let mock_validator = MockValidator::new_custom(expected_round, true, None);
    let validator = Validator::new(mock_validator);

    let test_message = create_test_message(expected_round);
    let expected_hash = create_expected_payload_hash(expected_round);

    let result = validator
        .validate_and_return_expected_hash(&test_message)
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_hash);
}

#[tokio::test]
async fn test_mock_validator_custom_failure() {
    let error_message = "Custom validation failure".to_string();
    let mock_validator = MockValidator::new_custom(1, false, Some(error_message.clone()));
    let validator = Validator::new(mock_validator);

    let test_message = create_test_message(1);

    let result = validator
        .validate_and_return_expected_hash(&test_message)
        .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains(&error_message));
}

#[tokio::test]
async fn test_mock_validator_mutability() {
    let mock_validator = MockValidator::new_success(1);
    let mut validator = Validator::new(mock_validator);

    // Test initial state
    let test_message = create_test_message(1);
    let result = validator
        .validate_and_return_expected_hash(&test_message)
        .await;
    assert!(result.is_ok());

    // Change to failure state
    validator.validator_impl.set_should_succeed(false);
    validator
        .validator_impl
        .set_error_message(Some("Changed to failure".to_string()));

    let result = validator
        .validate_and_return_expected_hash(&test_message)
        .await;
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Changed to failure")
    );

    // Change back to success state
    validator.validator_impl.set_should_succeed(true);
    validator.validator_impl.set_error_message(None);

    let result = validator
        .validate_and_return_expected_hash(&test_message)
        .await;
    assert!(result.is_ok());

    // Change expected round
    validator.validator_impl.set_expected_round(50);
    let expected_hash = create_expected_payload_hash(50);
    let result = validator.get_payload_from_message(&test_message).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_hash);
}

#[tokio::test]
async fn test_factory_create_mock_validator() {
    let expected_round = 25;
    let validator = factory::create_mock_validator(expected_round);

    let test_message = create_test_message(expected_round);
    let expected_hash = create_expected_payload_hash(expected_round);

    let result = validator
        .validate_and_return_expected_hash(&test_message)
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_hash);
}

#[tokio::test]
async fn test_factory_create_failing_mock_validator() {
    let error_message = "Factory failure test".to_string();
    let validator = factory::create_failing_mock_validator(error_message.clone());

    let test_message = create_test_message(1);

    let result = validator
        .validate_and_return_expected_hash(&test_message)
        .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains(&error_message));
}

#[test]
fn test_validator_config_default() {
    let config = ValidatorConfig::default();
    assert_eq!(config.validator_type, ValidatorType::Blockchain);
    assert_eq!(config.expected_round, None);
    assert_eq!(config.error_message, None);
}

#[test]
fn test_validator_config_builder_pattern() {
    let config = ValidatorConfig::new(ValidatorType::MockSuccess)
        .with_expected_round(42)
        .with_error_message("test error".to_string());

    assert_eq!(config.validator_type, ValidatorType::MockSuccess);
    assert_eq!(config.expected_round, Some(42));
    assert_eq!(config.error_message, Some("test error".to_string()));
}

#[test]
fn test_validator_config_from_env_default() {
    // Clear any existing environment variables
    unsafe {
        env::remove_var("VALIDATOR_TYPE");
        env::remove_var("VALIDATOR_EXPECTED_ROUND");
        env::remove_var("VALIDATOR_ERROR_MESSAGE");
    }

    let config = ValidatorConfig::from_env().unwrap();
    assert_eq!(config.validator_type, ValidatorType::Blockchain);
    assert_eq!(config.expected_round, None);
    assert_eq!(config.error_message, None);
}

#[test]
fn test_validator_config_from_env_mock_success() {
    unsafe {
        env::set_var("VALIDATOR_TYPE", "mock_success");
        env::set_var("VALIDATOR_EXPECTED_ROUND", "123");
        env::remove_var("VALIDATOR_ERROR_MESSAGE");
    }

    let config = ValidatorConfig::from_env().unwrap();
    assert_eq!(config.validator_type, ValidatorType::MockSuccess);
    assert_eq!(config.expected_round, Some(123));
    assert_eq!(config.error_message, None);

    // Clean up
    unsafe {
        env::remove_var("VALIDATOR_TYPE");
        env::remove_var("VALIDATOR_EXPECTED_ROUND");
    }
}

#[test]
fn test_validator_config_from_env_mock_failure() {
    unsafe {
        env::set_var("VALIDATOR_TYPE", "mock_failure");
        env::set_var("VALIDATOR_ERROR_MESSAGE", "environment error");
        env::remove_var("VALIDATOR_EXPECTED_ROUND");
    }

    let config = ValidatorConfig::from_env().unwrap();
    assert_eq!(config.validator_type, ValidatorType::MockFailure);
    assert_eq!(config.expected_round, None);
    assert_eq!(config.error_message, Some("environment error".to_string()));

    // Clean up
    unsafe {
        env::remove_var("VALIDATOR_TYPE");
        env::remove_var("VALIDATOR_ERROR_MESSAGE");
    }
}

#[test]
fn test_validator_config_from_env_invalid_type() {
    unsafe {
        env::set_var("VALIDATOR_TYPE", "invalid_type");
    }

    let result = ValidatorConfig::from_env();
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid VALIDATOR_TYPE")
    );

    // Clean up
    unsafe {
        env::remove_var("VALIDATOR_TYPE");
    }
}

#[test]
fn test_validator_config_from_env_case_insensitive() {
    unsafe {
        env::set_var("VALIDATOR_TYPE", "MOCK_SUCCESS");
    }

    let config = ValidatorConfig::from_env().unwrap();
    assert_eq!(config.validator_type, ValidatorType::MockSuccess);

    // Clean up
    unsafe {
        env::remove_var("VALIDATOR_TYPE");
    }
}

#[tokio::test]
async fn test_validator_generic_constraints() {
    // Test that the generic Validator works with different implementations
    let mock_validator = MockValidator::new_success(5);
    let validator = Validator::new(mock_validator);

    let test_message = create_test_message(5);
    let expected_hash = create_expected_payload_hash(5);

    let result = validator
        .validate_and_return_expected_hash(&test_message)
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_hash);
}

#[test]
fn test_validator_type_equality() {
    assert_eq!(ValidatorType::Blockchain, ValidatorType::Blockchain);
    assert_eq!(ValidatorType::Mock, ValidatorType::Mock);
    assert_eq!(ValidatorType::MockSuccess, ValidatorType::MockSuccess);
    assert_eq!(ValidatorType::MockFailure, ValidatorType::MockFailure);

    assert_ne!(ValidatorType::Blockchain, ValidatorType::Mock);
    assert_ne!(ValidatorType::MockSuccess, ValidatorType::MockFailure);
}

#[test]
fn test_validator_type_debug() {
    assert_eq!(format!("{:?}", ValidatorType::Blockchain), "Blockchain");
    assert_eq!(format!("{:?}", ValidatorType::Mock), "Mock");
    assert_eq!(format!("{:?}", ValidatorType::MockSuccess), "MockSuccess");
    assert_eq!(format!("{:?}", ValidatorType::MockFailure), "MockFailure");
}

#[test]
fn test_validator_config_debug() {
    let config = ValidatorConfig::new(ValidatorType::MockSuccess).with_expected_round(42);

    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("MockSuccess"));
    assert!(debug_str.contains("42"));
}

#[test]
fn test_validator_config_clone() {
    let config = ValidatorConfig::new(ValidatorType::Mock)
        .with_expected_round(100)
        .with_error_message("test".to_string());

    let cloned_config = config.clone();

    assert_eq!(config.validator_type, cloned_config.validator_type);
    assert_eq!(config.expected_round, cloned_config.expected_round);
    assert_eq!(config.error_message, cloned_config.error_message);
}
