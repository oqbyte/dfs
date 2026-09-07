use thiserror::Error;

#[derive(Debug, Error)]
pub enum DfsError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to parse map file: {0}")]
    Json(#[from] serde_json::Error),
    #[error("chunk integrity check failed: expected {expected}, found {found}")]
    HashMismatch { expected: String, found: String },
    #[error("invalid file name: {0}")]
    InvalidFileName(String),
    #[error("file already exists: {0}")]
    AlreadyExists(String),
    #[error("the ref.json file has be courrupted")]
    CourruptedCountFile,
}
