use std::{io, path::PathBuf};

use thiserror::Error;
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum GitError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("directory at {0} is not a git repo")]
    NotAGitRepo(PathBuf),
}

pub type Result<T> = std::result::Result<T, GitError>;
