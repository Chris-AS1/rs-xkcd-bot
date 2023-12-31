use anyhow;
use config::ConfigError;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("link hasn't been found in the parsed page")]
    LinkNotFoundError,
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error("user is currently rate limited")]
    RateLimitError,
}
