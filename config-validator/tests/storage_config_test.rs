use std::str::FromStr;

use config_validator::{
    StorageConfig,
    Filesystem,
    ByteSize,
    Validate,
};

#[test]
fn valid_storage_config_passes_validation() {
    // Arrange: create a valid storage configuration
    let storage = StorageConfig {
        filesystems: vec![
            Filesystem {
                name: "fs1".to_string(),
                mount_point: "/mnt/fs1".to_string(),
                size: ByteSize::from_str("2048B").unwrap(),
            },
        ],
        default_size: ByteSize::from_str("1024K").unwrap(),
    };

    // Act: run validation
    let result = storage.validate();

    // Assert: validation should succeed
    assert!(result.is_ok());
}