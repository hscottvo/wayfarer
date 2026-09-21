use std::{
    env::{self},
    path::{Path, PathBuf},
};

use tracing::debug;

use crate::config::error::{ConfigurationError, Result};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct ConfigurationDirectory(PathBuf);

impl ConfigurationDirectory {
    pub fn try_new() -> Result<Self> {
        let config_dir = env::var_os("XDG_CONFIG_HOME")
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
            .ok_or(ConfigurationError::ConfigDirectory)?
            .join("wayfarer");

        debug!(path = ?config_dir, "using configuration directory");
        Ok(Self(config_dir))
    }

    #[cfg(test)]
    pub const unsafe fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

impl AsRef<Path> for ConfigurationDirectory {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}
