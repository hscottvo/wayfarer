use std::{io, path::PathBuf, string};

use git2::Error;
use thiserror::Error;
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum GitError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("path at {0} is not a git repo")]
    NotAGitRepo(PathBuf),
    #[error(transparent)]
    Utf8Conversion(#[from] string::FromUtf8Error),
    #[error("failed to parse git remote")]
    RemoteParse,
    #[error("repo has no remote")]
    NoRemote,
    #[error("failed to format git repo for display")]
    Format,
    #[error(transparent)]
    Git(#[from] Error),
}

pub type Result<T> = std::result::Result<T, GitError>;
