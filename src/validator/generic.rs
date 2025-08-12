use anyhow::Result;
use commonware_cryptography::sha256::Digest;

use super::{blockchain::BlockchainValidator, interface::ValidatorTrait};

/// Generic validator that can work with any implementation of ValidatorTrait.
/// 
/// This struct provides a flexible wrapper around validator implementations,
/// allowing different validation strategies to be swapped without changing
/// the consuming code. It follows the Dependency Inversion Principle by
/// depending on the ValidatorTrait abstraction rather than concrete implementations.
/// 
/// # Type Parameters
/// * `T` - The validator implementation type that implements ValidatorTrait
pub struct Validator<T: ValidatorTrait> {
    pub validator_impl: T,
}

impl<T: ValidatorTrait> Validator<T> {
    /// Creates a new Validator instance with the given implementation.
    /// 
    /// This constructor takes ownership of a validator implementation
    /// and wraps it in the generic Validator struct.
    /// 
    /// # Arguments
    /// * `validator_impl` - The validator implementation to wrap
    /// 
    /// # Returns
    /// * `Self` - The new Validator instance
    pub fn new(validator_impl: T) -> Self {
        Self { validator_impl }
    }

    /// Validates a message and returns the expected hash.
    /// 
    /// This method delegates to the underlying validator implementation
    /// to perform the actual validation logic.
    /// 
    /// # Arguments
    /// * `msg` - The message bytes to validate
    /// 
    /// # Returns
    /// * `Result<Digest>` - The expected hash if validation succeeds, or an error
    pub async fn validate_and_return_expected_hash(&self, msg: &[u8]) -> Result<Digest> {
        self.validator_impl.validate_and_return_expected_hash(msg).await
    }

    /// Extracts and hashes the payload from a message.
    /// 
    /// This method delegates to the underlying validator implementation
    /// to extract and hash the message payload.
    /// 
    /// # Arguments
    /// * `msg` - The message bytes to extract payload from
    /// 
    /// # Returns
    /// * `Result<Digest>` - The hash of the extracted payload, or an error
    pub async fn get_payload_from_message(&self, msg: &[u8]) -> Result<Digest> {
        self.validator_impl.get_payload_from_message(msg).await
    }
}

// Convenience implementation for creating a BlockchainValidator
impl Validator<BlockchainValidator> {
    /// Creates a new Validator with a BlockchainValidator implementation.
    /// 
    /// This is a convenience method that creates a BlockchainValidator
    /// and wraps it in a Validator struct.
    /// 
    /// # Returns
    /// * `Result<Self>` - The new Validator instance or an error
    pub async fn new_blockchain() -> Result<Self> {
        let blockchain_validator = BlockchainValidator::new().await?;
        Ok(Self::new(blockchain_validator))
    }
}
