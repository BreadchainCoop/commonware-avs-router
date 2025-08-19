//! Executor module for the commonware-avs-router.

// Public modules
pub mod bls_eigenlayer_executor;
pub mod interface;

// Test module (only compiled in test mode)
#[cfg(test)]
pub mod tests;

// Re-export the main types for easy access
#[allow(unused_imports)]
pub use bls_eigenlayer_executor::{BlsEigenlayerExecutor, convert_non_signer_data};
#[allow(unused_imports)]
pub use interface::{BlsSignatureVerificationHandler, ExecutionResult, ExecutorTrait};
