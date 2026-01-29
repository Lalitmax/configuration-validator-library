use config_validator::{
    ServerConfig,
    Host,
    HostRole,
    StorageConfig,
    Filesystem,
    NetworkConfig,
    NetworkInterface,
    NetworkType,
    ByteSize,
    Validate,
};

#[test]
fn valid_server_config_passes_validation() {
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

    assert!(server.validate().is_ok());
}
