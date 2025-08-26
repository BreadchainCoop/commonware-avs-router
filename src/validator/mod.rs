//! Validator module for the commonware-avs-router.

// Public modules
pub mod generic;
pub mod interface;

// Test module
#[cfg(test)]
pub mod tests;

// Re-export the main types for easy access
#[allow(unused_imports)]
pub use generic::Validator;
#[allow(unused_imports)]
pub use interface::ValidatorTrait;

// Re-export test utilities
#[cfg(test)]
#[allow(unused_imports)]
pub use tests::mock::MockValidator;
