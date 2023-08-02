use anyhow;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum BotError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
