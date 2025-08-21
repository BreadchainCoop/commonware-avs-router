//! Executor module for the commonware-avs-router.

// Public modules
pub mod bls;
pub mod core;

// Re-export the main types for easy access
#[allow(unused_imports)]
pub use bls::{BlsEigenlayerExecutor, convert_non_signer_data};
#[allow(unused_imports)]
pub use core::{ExecutionResult, VerificationData, VerificationExecutor};
