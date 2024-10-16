use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("File not found.")]
    IoError(#[from] io::Error),

    #[error("Failed to parse JSON.")]
    JSONError(#[from] serde_json::Error),
}
