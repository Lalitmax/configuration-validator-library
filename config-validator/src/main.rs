use std::str::FromStr;
use config_validator::{
    StorageConfig,
    Filesystem,
    ByteSize,
    Validate,
};

fn main() {
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

    match storage.validate() {
        Ok(_) => println!("Storage config is valid"),
        Err(e) => println!("Storage config is invalid: {}", e),
    }
}