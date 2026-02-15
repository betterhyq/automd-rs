//! Error types and Result alias.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cargo.toml not found")]
    CargoTomlNotFound,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse Cargo.toml: {0}")]
    CargoParse(String),

    #[error("Invalid repository URL: {0}")]
    InvalidRepoUrl(String),

    #[error("Block handler '{0}': {1}")]
    BlockHandler(String, String),
}

pub type Result<T> = std::result::Result<T, Error>;
