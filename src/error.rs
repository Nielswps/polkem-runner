use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("CLI argument parsing failed: {0}")]
    CLI(String),

    #[error("interaction with Substrate resulted in error: {0}")]
    SUBSTRATE(String),
}