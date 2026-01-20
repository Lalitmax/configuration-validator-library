#[derive(Debug)]
pub enum ValidationError {
    InvalidHostname { hostname: String, reason: String },
    InvalidIpAddress { ip: String, reason: String },
    InvalidFilesystemName { name: String, reason: String },
    InvalidMountPoint { path: String, reason: String },
    InvalidNetworkInterface { name: String, reason: String },
    InvalidByteSize { input: String, reason: String },
    NoManagerHost,
    NoFilesystems,
    EmptyConfiguration { field: String },
    MultipleErrors(Vec<ValidationError>),
}

