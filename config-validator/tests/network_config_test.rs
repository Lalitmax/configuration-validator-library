use config_validator::{
    NetworkConfig,
    NetworkInterface,
    NetworkType,
    Validate,
};

#[test]
fn valid_network_config_passes_validation() {
    // Arrange: create a valid network configuration
    let network = NetworkConfig {
        interfaces: vec![
            NetworkInterface {
                name: "eth0".to_string(),
                network_type: NetworkType::Ethernet,
                speed_gbps: 10,
            },
        ],
    };

    // Act: run validation
    let result = network.validate();

    // Assert: validation should succeed
    assert!(result.is_ok());
}
