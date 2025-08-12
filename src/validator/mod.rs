//! Validator module for the commonware-avs-router.
//!
//! This module provides a flexible, generic validation system that follows
//! the Dependency Inversion Principle. It includes:
//!
//! - **ValidatorTrait**: The core interface for all validator implementations
//! - **BlockchainValidator**: Production validator that uses blockchain state
//! - **MockValidator**: Testing validator for unit tests and development
//! - **Generic Validator<T>**: Wrapper that can work with any ValidatorTrait implementation
//! - **Factory Functions**: Dependency injection utilities for creating validators
//!
//! ## Usage Examples
//!
//! ```rust
//! use crate::validator::factory;
//!
//! // Create a blockchain validator for production
//! let validator = factory::create_blockchain_validator().await?;
//!
//! // Create a mock validator for testing
//! let mock_validator = factory::create_mock_validator(42);
//!
//! // Use environment variables for configuration
//! // Set VALIDATOR_TYPE=mock_success in your environment
//! ```

// Public modules
pub mod blockchain;
pub mod factory;
pub mod generic;
pub mod interface;
pub mod mock;

// Test module (only compiled in test mode)
#[cfg(test)]
pub mod tests;

// Re-export the main types for easy access
pub use blockchain::BlockchainValidator;
pub use factory::{
    ValidatorConfig, ValidatorType, create_blockchain_validator, create_failing_mock_validator,
    create_mock_validator,
};
pub use generic::Validator;
pub use interface::ValidatorTrait;
pub use mock::MockValidator;
