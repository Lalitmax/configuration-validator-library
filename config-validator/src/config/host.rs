use crate::validation::Validate;
use crate::validation::rules::{validate_hostname, validate_ipv4};
use crate::error::ValidationError;

// Represents a single machine in the cluster
#[derive(Debug, Clone)]
pub struct Host {
    pub hostname: String,
    pub ip_address: String,
    pub role: HostRole,
    pub enabled: bool,
}

// Defines what responsibility a host has
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostRole {
    Manager,
    Storage,
    Client,
    Gateway,
}

impl Validate for Host {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        if let Err(err) = validate_hostname(&self.hostname) {
            errors.push(err);
        }

        if let Err(err) = validate_ipv4(&self.ip_address) {
            errors.push(err);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError::MultipleErrors(errors))
        }
    }
}