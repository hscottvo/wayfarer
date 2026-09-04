use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::{ConfigurationError, Result};

use super::expand_tilde;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "PathBuf")]
pub struct BaseDirectory(PathBuf);

impl AsRef<Path> for BaseDirectory {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl TryFrom<PathBuf> for BaseDirectory {
    type Error = ConfigurationError;

    fn try_from(path: PathBuf) -> Result<Self> {
        let path = expand_tilde(path)?;
        if path.components().any(|c| c == Component::ParentDir) {
            return Err(ConfigurationError::PathTraversal(path));
        }

        Ok(Self(path))
    }
}
