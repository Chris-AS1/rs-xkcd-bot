use anyhow;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("link hasn't been found in the parsed page")]
    LinkNotFoundError,
}
