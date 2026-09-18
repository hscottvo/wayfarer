use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
pub mod error;
use error::Result;

use crate::git::error::GitError as Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GitRepo(PathBuf);
impl GitRepo {
    fn new(path: PathBuf) -> Result<Self> {
        if !is_git_repo(&path)? {
            return Err(Error::NotAGitRepo(path));
        }
        Ok(Self(path))
    }
}
impl Display for GitRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}

fn is_git_repo(dir: impl AsRef<Path>) -> Result<bool> {
    let ret = Command::new("git")
        .arg("-C")
        .arg(dir.as_ref())
        .arg("rev-parse")
        .arg("--git-dir")
        .output()?
        .status
        .success();
    Ok(ret)
}

/// # Errors
///
/// Will return `Err` if rust is unable to read the directory or spawn the child `git` process
/// that is used to check if a directory is a git repo
pub fn git_repos(dir: impl AsRef<Path>) -> Result<Vec<GitRepo>> {
    let mut repos = Vec::new();
    for dir in fs::read_dir(dir.as_ref())? {
        let dir = dir?;
        match GitRepo::new(dir.path()) {
            Ok(repo) => repos.push(repo),
            Err(Error::NotAGitRepo(_)) => {}
            Err(e) => return Err(e),
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
    fn returns_true_for_git_dir() -> Result<()> {
        let dir = tempdir()?;
        Command::new("git").arg("init").arg(dir.path()).output()?;
        assert!(is_git_repo(dir)?);
        Ok(())
    }

    #[test]
    fn returns_false_for_empty_dir() -> Result<()> {
        let dir = tempdir()?;
        assert!(!is_git_repo(dir)?);
        Ok(())
    }

    #[test]
    fn returns_false_for_fake_git_dir() -> Result<()> {
        let dir = tempdir()?;
        fs::create_dir(dir.path().join(".git"))?;
        assert!(!is_git_repo(dir)?);
        Ok(())
    }

    #[test]
    fn returns_only_git_repos() -> Result<()> {
        let dir = tempdir()?;

        let a_path = dir.path().join("a");
        fs::create_dir(&a_path)?;
        Command::new("git").arg("init").arg(&a_path).output()?;

        let b_path = dir.path().join("b");
        fs::create_dir(&b_path)?;
        Command::new("git").arg("init").arg(&b_path).output()?;

        let c_path = dir.path().join("c");
        fs::create_dir(&c_path)?;
        Command::new("git").arg("init").arg(&c_path).output()?;

        let d_path = dir.path().join("d");
        fs::create_dir(&d_path)?;

        let e_path = dir.path().join("e");
        fs::create_dir(&e_path)?;

        let mut repos = git_repos(dir.path())?;
        repos.sort();

        assert_eq!(
            vec![
                GitRepo::new(a_path)?,
                GitRepo::new(b_path)?,
                GitRepo::new(c_path)?,
            ],
            repos
        );

        Ok(())
    }
}
