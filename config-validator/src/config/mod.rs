pub mod host;
pub mod network;
pub mod storage;

pub use host::{Host, HostRole};
pub use network::{NetworkConfig, NetworkInterface, NetworkType};
pub use storage::{StorageConfig, Filesystem};
