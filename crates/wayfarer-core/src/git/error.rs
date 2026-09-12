use std::io;

use thiserror::Error;
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum GitError {
    #[error(transparent)]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, GitError>;
