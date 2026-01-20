#[derive(Debug)]
pub enum ValidationError {
    InvalidServer(String),
    InvalidHost(String),
    InvalidStorage(String),
    InvalidNetwork(String),
}
