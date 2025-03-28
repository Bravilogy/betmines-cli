use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("File not found: {0}")]
    IoError(#[from] io::Error),

    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),

    #[error("Failed to parse JSON: {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("Failed to open URL: {0}")]
    BrowserError(String),

    #[error("Filter import failed: {0}")]
    ImportError(String),

    #[error("Filter deletion failed: {0}")]
    DeletionError(String),
}
