use crate::error::ValidationError;

// Common trait for all config validation
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}