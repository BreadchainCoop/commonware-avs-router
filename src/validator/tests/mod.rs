use crate::wire;
use alloy::sol_types::SolValue;
use alloy_primitives::U256;
use commonware_codec::Encode;
use commonware_cryptography::sha256::Digest;
use commonware_cryptography::{Hasher, Sha256};
use std::collections::HashMap;

use super::{generic::Validator, mock::MockValidator};

/// Test helper for creating test messages
fn create_test_message(round: u64) -> Vec<u8> {
    let mut metadata = HashMap::new();
    metadata.insert("var1".to_string(), "test_var1".to_string());
    metadata.insert("var2".to_string(), "test_var2".to_string());
    metadata.insert("var3".to_string(), "test_var3".to_string());

    let aggregation = wire::Aggregation {
        round,
        metadata,
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
async fn test_create_mock_validator() {
    let expected_round = 25;
    let mock_validator = MockValidator::new_success(expected_round);
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
async fn test_create_failing_mock_validator() {
    let error_message = "Failure test".to_string();
    let mock_validator = MockValidator::new_failure(error_message.clone());
    let validator = Validator::new(mock_validator);

    let test_message = create_test_message(1);

    let result = validator
        .validate_and_return_expected_hash(&test_message)
        .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains(&error_message));
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
