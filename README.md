# Configuration Validator Library

A Rust library for validating server configurations with comprehensive validation rules for hosts, storage, and network settings.

## Features

- Server configuration validation with detailed error messages
- Host roles: Manager, Storage, Client, Gateway
- Network types: Ethernet, Infiniband, OmniPath
- Human-readable byte size parsing (K, M, G, T)
- Type-safe configuration structures

## Installation

```toml
[dependencies]
config-validator = "0.1.0"
```

## Usage

```rust
use config_validator::{
    ServerConfig, Host, HostRole, StorageConfig, Filesystem,
    NetworkConfig, NetworkInterface, NetworkType, ByteSize, Validate,
};

let server = ServerConfig {
    name: "prod-cluster".to_string(),
    hosts: vec![
        Host {
            hostname: "manager-01".to_string(),
            ip_address: "192.168.1.10".to_string(),
            role: HostRole::Manager,
            enabled: true,
        },
    ],
    storage: StorageConfig {
        filesystems: vec![
            Filesystem {
                name: "data_fs".to_string(),
                mount_point: "/data".to_string(),
                size: "100G".parse::<ByteSize>().unwrap(),
            },
        ],
        default_size: "50G".parse::<ByteSize>().unwrap(),
    },
    network: NetworkConfig {
        interfaces: vec![
            NetworkInterface {
                name: "eth0".to_string(),
                network_type: NetworkType::Ethernet,
                speed_gbps: 10,
            },
        ],
    },
};

match server.validate() {
    Ok(_) => println!("Valid configuration"),
    Err(e) => println!("Invalid: {}", e),
}
```

## API Overview

### ServerConfig
Top-level configuration containing name, hosts, storage, and network settings. Requires at least one Manager host.

### Host
Represents a cluster machine with hostname, IP address, role, and enabled status.

Validation:
- Hostname: 1-255 chars, alphanumeric/hyphens/dots, no leading/trailing hyphens
- IP: Valid IPv4 format

### StorageConfig
Contains filesystems and default size. Requires at least one filesystem.

### Filesystem
Defines name, mount point, and size.

Validation:
- Name: 1-64 chars, alphanumeric and underscores only
- Mount point: Must start with '/'

### NetworkConfig
Contains network interfaces. Requires at least one interface.

### NetworkInterface
Defines name, type, and speed.

Validation:
- Name: Letters followed by digits (e.g., eth0, ib0)
- Speed: Must be greater than 0

### ByteSize
Parses human-readable sizes: B, K (1024), M (1048576), G (1073741824), T (1099511627776)

```rust
let size: ByteSize = "100G".parse().unwrap();
```

## Validation

All types implement the `Validate` trait:

```rust
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}
```

## Error Types

- InvalidHostname, InvalidIpAddress, InvalidFilesystemName
- InvalidMountPoint, InvalidNetworkInterface, InvalidByteSize
- NoManagerHost, NoFilesystems, EmptyConfiguration
- MultipleErrors

## Testing

```bash
cd config-validator
cargo test
```

## License

Open source.
