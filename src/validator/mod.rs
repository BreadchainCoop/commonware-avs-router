//! Validator module for the commonware-avs-router.

// Public modules
pub mod generic;
pub mod interface;
pub mod mock;

// Test module (only compiled in test mode)
#[cfg(test)]
pub mod tests;

// Re-export the main types for easy access
#[allow(unused_imports)]
pub use generic::Validator;
#[allow(unused_imports)]
pub use interface::ValidatorTrait;
#[allow(unused_imports)]
pub use mock::MockValidator;
