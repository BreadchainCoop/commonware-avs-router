//! Executor module for the commonware-avs-router.

// Public modules
pub mod contract;
pub mod interface;

// Test module (only compiled in test mode)
#[cfg(test)]
pub mod tests;

// Re-export the main types for easy access
#[allow(unused_imports)]
pub use contract::ContractExecutor;
#[allow(unused_imports)]
pub use interface::{ContractHandler, ExecutionResult, Executor, ExecutorTrait};
