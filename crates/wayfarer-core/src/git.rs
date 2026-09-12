use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
pub mod error;
use error::Result;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GitRepo(PathBuf);
impl GitRepo {
    const fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

fn is_git_repo(dir: impl AsRef<Path>) -> Result<bool> {
    let ret = Command::new("git")
        .arg("-C")
        .arg(dir.as_ref())
        .arg("rev-parse")
        .arg("--git-dir")
        .status()?
        .success();
    Ok(ret)
}

/// # Errors
///
/// Will return `Err` if rust is unable to spawn the child `git` process
/// that is used to check if a directory is a git repo
pub fn git_repos(dir: impl AsRef<Path>) -> Result<Vec<GitRepo>> {
    let mut repos = Vec::new();
    for dir in fs::read_dir(dir.as_ref())? {
        let dir = dir?;
        if is_git_repo(dir.path())? {
            repos.push(GitRepo::new(dir.path()));
        }
    }
    Ok(repos)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use eyre::Result;
    use tempfile::tempdir;

    use super::*;
    #[test]
    fn valid_git_repo() -> Result<()> {
        let dir = tempdir()?;
        Command::new("git").arg("init").arg(dir.path()).status()?;
        assert!(is_git_repo(dir)?);
        Ok(())
    }
    #[test]
    fn missing_dot_git() -> Result<()> {
        let dir = tempdir()?;
        assert!(!is_git_repo(dir)?);
        Ok(())
    }
    #[test]
    fn bad_dot_git() -> Result<()> {
        let dir = tempdir()?;
        fs::create_dir(dir.path().join(".git"))?;
        assert!(!is_git_repo(dir)?);
        Ok(())
    }
}
