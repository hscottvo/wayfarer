use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
pub mod error;
use error::Result;
use git2::Repository;
use tracing::instrument;

use crate::git::error::GitError::{self as Error};

pub struct GitRepo(Repository);
impl GitRepo {
    fn new(path: PathBuf) -> Result<Self> {
        if !is_git_repo(&path)? {
            return Err(Error::NotAGitRepo(path));
        }
        Ok(Self(Repository::open(path)?))
    }

    /// Gets the name of a git repo. Precedence is
    /// 1. repo name on the remote
    /// 2. name of the checkout's directory
    /// # Errors
    ///
    /// Will return `Err` if repo name cannot be formatted to a &str.
    pub fn name(&self) -> Result<String> {
        let remote = self.0.find_remote("origin")?;
        let url = remote.url()?.to_owned();
        Ok(parse_repo_name(&url)?.to_owned())
    }

    /// Gets the branch of a git repo.
    ///
    /// # Errors
    ///
    /// Will return `Err` if head cannot be found or head is detached.
    pub fn branch(&self) -> Result<String> {
        let head = self.0.head()?;
        if head.is_branch() {
            Ok(head.shorthand()?.to_owned())
        } else {
            Err(Error::DetachedHead)
        }
    }

    // /// # Errors
    // ///
    // /// Will return `Err` if repo owner cannot be parsed to a &str.
    // pub fn owner(&self) -> Result<&str> {
    //     self.remote
    //         .as_ref()
    //         .map(GitRemote::owner)
    //         .ok_or(Error::Format)
    // }

    // /// # Errors
    // ///
    // /// Will return `Err` if host cannot be parsed into a &str.
    // pub fn host(&self) -> Result<&str> {
    //     self.remote
    //         .as_ref()
    //         .map(GitRemote::host)
    //         .ok_or(Error::Format)
    // }
}

fn parse_repo_name(url: &str) -> Result<&str> {
    if !url.contains('.') || !url.contains('/') {
        dbg!(url);
        dbg!("fuck");
        return Err(Error::RemoteParse);
    }
    url.rsplit(['/', '.']).nth(1).ok_or(Error::RemoteParse)
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
#[instrument(skip(dir), fields(dir = ?dir.as_ref()))]
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

    // #[test]
    // fn returns_only_git_repos() -> Result<()> {
    //     let dir = tempdir()?;
    //
    //     let a_path = dir.path().join("a");
    //     fs::create_dir(&a_path)?;
    //     Command::new("git").arg("init").arg(&a_path).output()?;
    //
    //     let b_path = dir.path().join("b");
    //     fs::create_dir(&b_path)?;
    //     Command::new("git").arg("init").arg(&b_path).output()?;
    //
    //     let c_path = dir.path().join("c");
    //     fs::create_dir(&c_path)?;
    //     Command::new("git").arg("init").arg(&c_path).output()?;
    //
    //     let d_path = dir.path().join("d");
    //     fs::create_dir(&d_path)?;
    //
    //     let e_path = dir.path().join("e");
    //     fs::create_dir(&e_path)?;
    //
    //     let mut repos = git_repos(dir.path())?;
    //     repos.sort();
    //
    //     assert_eq!(
    //         vec![
    //             GitRepo::new(a_path)?,
    //             GitRepo::new(b_path)?,
    //             GitRepo::new(c_path)?,
    //         ],
    //         repos
    //     );
    //
    //     Ok(())
    // }

    #[test]
    fn parse_repo_name_from_url() -> Result<()> {
        let valid_repo_name = parse_repo_name("git@github.com:hscottvo/waypoint.git")?;
        assert_eq!(valid_repo_name, "waypoint");

        let no_delimiters = parse_repo_name("waypoint");
        assert!(no_delimiters.is_err());

        let missing_slash = parse_repo_name("waypoint.git");
        assert!(missing_slash.is_err());

        let missing_period = parse_repo_name("hscottvo/waypoint");
        assert!(missing_period.is_err());
        Ok(())
    }

    #[test]
    fn bruh() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path();
        Command::new("git").arg("init").arg(path).output()?;
        Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("config")
            .arg("--get")
            .arg("remote.origin.url")
            .output()?;
        Ok(())
    }
}
