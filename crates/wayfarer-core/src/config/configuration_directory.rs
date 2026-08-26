use std::{
    env,
    path::{Path, PathBuf},
};

use crate::config::error::Result;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ConfigurationDirectory(PathBuf);

impl ConfigurationDirectory {
    pub fn try_new() -> Result<Self> {
        // let x = PathBuf::from_str(env::var("XDG_CONFIG_HOME")?)?;
        // let x = PathBuf::from("iserntiern");
        let x = PathBuf::from(&env::var("XDG_CONFIG_HOME")?).join("wayfarer");
        dbg!(x);
        Ok(Self("".into()))
    }

    #[cfg(test)]
    pub unsafe fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub fn to_path_buf(&self) -> PathBuf {
        self.0.clone()
    }
}

impl AsRef<Path> for ConfigurationDirectory {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bruh() -> Result<()> {
        let x = ConfigurationDirectory::try_new()?;
        Ok(())
    }
}
