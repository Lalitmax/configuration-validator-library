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
