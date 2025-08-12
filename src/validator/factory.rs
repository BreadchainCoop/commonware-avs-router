use anyhow::Result;
use std::env;

use super::{
    blockchain::BlockchainValidator,
    generic::Validator,
    mock::MockValidator,
};

/// Configuration for validator factory creation.
#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    /// The type of validator to create
    pub validator_type: ValidatorType,
    /// Expected round number for mock validators
    pub expected_round: Option<u64>,
    /// Custom error message for mock validators
    pub error_message: Option<String>,
}

/// Types of validators that can be created by the factory.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidatorType {
    /// Blockchain-based validator for production use
    Blockchain,
    /// Mock validator for testing
    Mock,
    /// Mock validator that always succeeds
    MockSuccess,
    /// Mock validator that always fails
    MockFailure,
}

impl Default for ValidatorConfig {
    fn default() -> Self {
        Self {
            validator_type: ValidatorType::Blockchain,
            expected_round: None,
            error_message: None,
        }
    }
}

impl ValidatorConfig {
    /// Creates a new ValidatorConfig with the specified validator type.
    pub fn new(validator_type: ValidatorType) -> Self {
        Self {
            validator_type,
            expected_round: None,
            error_message: None,
        }
    }

    /// Sets the expected round number for mock validators.
    pub fn with_expected_round(mut self, round: u64) -> Self {
        self.expected_round = Some(round);
        self
    }

    /// Sets the error message for mock validators.
    pub fn with_error_message(mut self, message: String) -> Self {
        self.error_message = Some(message);
        self
    }

    /// Creates a validator configuration from environment variables.
    /// 
    /// Reads the `VALIDATOR_TYPE` environment variable to determine
    /// the type of validator to create. Valid values are:
    /// - "blockchain" -> ValidatorType::Blockchain
    /// - "mock" -> ValidatorType::Mock
    /// - "mock_success" -> ValidatorType::MockSuccess
    /// - "mock_failure" -> ValidatorType::MockFailure
    /// 
    /// # Returns
    /// * `Result<Self>` - The configuration or an error if invalid
    pub fn from_env() -> Result<Self> {
        let validator_type = match env::var("VALIDATOR_TYPE")
            .unwrap_or_else(|_| "blockchain".to_string())
            .to_lowercase()
            .as_str()
        {
            "blockchain" => ValidatorType::Blockchain,
            "mock" => ValidatorType::Mock,
            "mock_success" => ValidatorType::MockSuccess,
            "mock_failure" => ValidatorType::MockFailure,
            invalid => {
                return Err(anyhow::anyhow!(
                    "Invalid VALIDATOR_TYPE: {}. Valid values are: blockchain, mock, mock_success, mock_failure",
                    invalid
                ));
            }
        };

        let expected_round = env::var("VALIDATOR_EXPECTED_ROUND")
            .ok()
            .and_then(|s| s.parse::<u64>().ok());

        let error_message = env::var("VALIDATOR_ERROR_MESSAGE").ok();

        Ok(Self {
            validator_type,
            expected_round,
            error_message,
        })
    }
}

/// Creates a blockchain validator instance.
/// 
/// This is a convenience function for creating a blockchain validator
/// without needing to create a full configuration.
/// 
/// # Returns
/// * `Result<Validator<BlockchainValidator>>` - The created validator or an error
pub async fn create_blockchain_validator() -> Result<Validator<BlockchainValidator>> {
    Validator::new_blockchain().await
}

/// Creates a mock validator instance for testing.
/// 
/// This is a convenience function for creating a mock validator
/// that always succeeds, useful for testing scenarios.
/// 
/// # Arguments
/// * `expected_round` - The round number to use for hash generation
/// 
/// # Returns
/// * `Validator<MockValidator>` - The created validator
pub fn create_mock_validator(expected_round: u64) -> Validator<MockValidator> {
    let mock_validator = MockValidator::new_success(expected_round);
    Validator::new(mock_validator)
}

/// Creates a mock validator instance that always fails.
/// 
/// This is a convenience function for creating a mock validator
/// that always fails, useful for testing error handling.
/// 
/// # Arguments
/// * `error_message` - The error message to return on failure
/// 
/// # Returns
/// * `Validator<MockValidator>` - The created validator
pub fn create_failing_mock_validator(error_message: String) -> Validator<MockValidator> {
    let mock_validator = MockValidator::new_failure(error_message);
    Validator::new(mock_validator)
}
