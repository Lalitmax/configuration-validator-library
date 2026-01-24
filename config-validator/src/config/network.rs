use crate::validation::Validate;
use crate::validation::rules::validate_interface_name;
use crate::error::ValidationError;

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub interfaces: Vec<NetworkInterface>,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub network_type: NetworkType,
    pub speed_gbps: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkType {
    Ethernet,
    Infiniband,
    OmniPath,
}

// Validate network configuration
impl Validate for NetworkConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        if self.interfaces.is_empty() {
            errors.push(ValidationError::EmptyConfiguration {
                field: "network.interfaces".to_string(),
            });
        }

        for iface in &self.interfaces {
            if let Err(e) = validate_interface_name(&iface.name) {
                errors.push(e);
            }

            if iface.speed_gbps == 0 {
                errors.push(ValidationError::InvalidNetworkInterface {
                    name: iface.name.clone(),
                    reason: "speed_gbps must be greater than 0".to_string(),
                });
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError::MultipleErrors(errors))
        }
    }
}
