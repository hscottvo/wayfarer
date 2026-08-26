use std::{env, io, path::PathBuf};

use config::ConfigError;
use thiserror::Error;
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Env(#[from] env::VarError),
    #[error(transparent)]
    Parse(#[from] toml::de::Error),
    #[error(transparent)]
    ConfigLibrary(#[from] ConfigError),
    #[error("failed to retrieve user config directory")]
    ConfigDirectory,
    #[error("failed to write config to {}", .0.display())]
    ConfigWrite(PathBuf),
    #[error(transparent)]
    Serialization(#[from] toml::ser::Error),
    #[error("path contains path traversal: {}", .0.display())]
    PathTraversal(PathBuf),
    #[error("path is not a directory: {}", .0.display())]
    NotADirectory(PathBuf),
    #[error("failed to retrieve parent of path: {}", .0.display())]
    Parent(PathBuf),
}

pub type Result<T> = std::result::Result<T, ConfigurationError>;
