pub mod config;
pub mod error;
pub mod validation;

pub use validation::Validate;
pub use error::ValidationError;
pub use config::{NetworkConfig, NetworkInterface, NetworkType};