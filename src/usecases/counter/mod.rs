// Counter-specific state and task data types
pub mod types;

// Counter state providers
pub mod providers;

// Counter task data factories
pub mod factories;

// Counter validators
pub mod validators;

// Counter executor implementation
pub mod executor;

// Re-export main types for easy access
pub use executor::CounterHandler;
pub use factories::DefaultTaskDataFactory;
pub use providers::CounterProvider;
pub use types::{CounterState, DefaultTaskData};
pub use validators::CounterValidator;
