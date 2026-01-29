use crate::config::{Host, StorageConfig, NetworkConfig};
use crate::validation::Validate;
use crate::error::ValidationError;
use crate::config::HostRole;
// Top-level server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub hosts: Vec<Host>,
    pub storage: StorageConfig,
    pub network: NetworkConfig,
}

impl Validate for ServerConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        // Must have at least one Manager host
        let has_manager = self
            .hosts
            .iter()
            .any(|host| host.role == HostRole::Manager);

        if !has_manager {
            errors.push(ValidationError::NoManagerHost);
        }

        // Validate all hosts
        for host in &self.hosts {
            if let Err(e) = host.validate() {
                errors.push(e);
            }
        }

        // Validate storage config
        if let Err(e) = self.storage.validate() {
            errors.push(e);
        }

        // Validate network config
        if let Err(e) = self.network.validate() {
            errors.push(e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError::MultipleErrors(errors))
        }
    }
}