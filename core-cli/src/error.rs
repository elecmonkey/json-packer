use std::io;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Compression error: {0}")]
    Compression(#[from] json_packer::Error),
    
    #[error("Invalid input format: {0}")]
    InvalidFormat(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Invalid command line arguments: {0}")]
    InvalidArgs(String),
    
    #[error("Batch processing error: {failed}/{total} files failed")]
    BatchError { failed: usize, total: usize },
}

pub type Result<T> = std::result::Result<T, CliError>;

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::Io(_) => 2,
            CliError::Json(_) => 3,
            CliError::Compression(_) => 4,
            CliError::InvalidFormat(_) => 5,
            CliError::FileNotFound(_) => 6,
            CliError::InvalidArgs(_) => 7,
            CliError::BatchError { .. } => 8,
        }
    }
}