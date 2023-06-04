use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("CLI argument parsing failed: {0}")]
    CLI(String),

    #[error("Interaction with Substrate resulted in error: {0}")]
    Substrate(String),

    #[error("Attempt at POST request to the logging agent resulted in error: {0}")]
    Log(String),
}