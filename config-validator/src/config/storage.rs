use crate::validation::Validate;
use crate::validation::rules::{
    validate_filesystem_name,
    validate_mount_point,
};
use crate::error::ValidationError;
use crate::types::ByteSize;
/// Storage configuration for the server
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub filesystems: Vec<Filesystem>,
    pub default_size: ByteSize,
}

/// Represents a filesystem definition
#[derive(Debug, Clone)]
pub struct Filesystem {
    pub name: String,
    pub mount_point: String,
    pub size: ByteSize,
}

// Validate storage configuration
impl Validate for StorageConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        // Must have at least one filesystem
        if self.filesystems.is_empty() {
            errors.push(ValidationError::NoFilesystems);
        }

        // Validate each filesystem
        for fs in &self.filesystems {
            if let Err(e) = validate_filesystem_name(&fs.name) {
                errors.push(e);
            }

            if let Err(e) = validate_mount_point(&fs.mount_point) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ValidationError::MultipleErrors(errors))
        }
    }
}