use base_directory::BaseDirectory;
use config::{Config, FileFormat};
use error::{ConfigurationError, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

use crate::config::configuration_directory::ConfigurationDirectory;

mod base_directory;
mod configuration_directory;
pub mod error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct Configuration {
    base_directory: BaseDirectory,
}

impl Configuration {
    /// # Errors
    ///
    /// Will return `Err` if `path` does not exist, the user does not have
    /// permission to read it, or the file cannot be parsed into a valid
    /// `Configuration`.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let settings = Config::builder()
            .add_source(config::File::from(path.as_ref()).format(FileFormat::Toml))
            // .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        Ok(settings.try_deserialize()?)
    }

    /// # Errors
    ///
    /// Will return `Err` if the the user's `XDG_CONFIG_HOME` is not set,
    /// the directory does not exist, or the file cannot be parsed.
    pub fn load_xdg() -> Result<Self> {
        Self::load(&xdg_config_path()?)
    }

    /// # Errors
    ///
    /// Will return `Err` if fails to write to the configuration file.
    pub fn save(&self, path: &ConfigurationDirectory) -> Result<PathBuf> {
        let parent_path = path
            .as_ref()
            .parent()
            .ok_or_else(|| ConfigurationError::Parent(path.to_path_buf()))?;
        fs::create_dir_all(parent_path)?;
        let mut file = NamedTempFile::new_in(parent_path)?;
        let contents = toml::to_string_pretty(self)?;
        file.write_all(contents.as_bytes())?;
        file.flush()?;
        file.as_file().sync_all()?;
        let write_path = parent_path.join("config.toml");
        file.persist(&write_path)
            .map_err(|_| ConfigurationError::ConfigWrite(write_path.clone()))?;
        Ok(write_path)
    }
}

// im pretty sure this is stupid
fn expand_tilde(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    let path = if let Ok(path_from_home) = path.strip_prefix("~") {
        dirs::home_dir()
            .ok_or(ConfigurationError::ConfigDirectory)?
            .join(path_from_home)
    } else {
        path.to_path_buf()
    };
    Ok(path)
}

fn xdg_config_path() -> Result<PathBuf> {
    Ok(ConfigurationDirectory::try_new()?
        .as_ref()
        .join("config.toml"))
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;
    use tempfile::tempdir;
    fn test_configuration() -> Result<Configuration> {
        let path = PathBuf::from_str("./").unwrap();
        let base_directory = BaseDirectory::try_from(path)?;
        Ok(Configuration { base_directory })
    }

    #[test]
    fn valid_toml_deserializes() -> Result<()> {
        let toml_string = r#"base_directory = "/tmp/Repos""#;
        let config: Configuration = Config::builder()
            .add_source(config::File::from_str(
                toml_string,
                config::FileFormat::Toml,
            ))
            .build()?
            .try_deserialize()?;

        assert_eq!(config.base_directory.as_ref(), Path::new("/tmp/Repos"));
        Ok(())
    }

    #[test]
    fn read_toml_from_dir() -> Result<()> {
        let path: PathBuf = tempdir()?.path().join("config.toml");
        let dir = unsafe { ConfigurationDirectory::new(path) };
        let expected = test_configuration()?;
        let saved_dir = expected.save(&dir)?;

        let config = Configuration::load(saved_dir)?;
        assert_eq!(expected, config);
        Ok(())
    }
}
