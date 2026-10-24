use crate::git::error::GitError;

use super::Result;
use std::{path::Path, process::Command};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GitRemote {
    url: String,
    host: (usize, usize),
    owner: (usize, usize),
    repo_name: (usize, usize),
}

impl GitRemote {
    pub fn try_new(url: impl AsRef<str>) -> Result<Self> {
        let url = url.as_ref().to_owned();
        if url.is_empty() {
            return Err(GitError::NoRemote);
        }
        let host_end = url.find(':').ok_or(GitError::RemoteParse)?;
        if host_end >= url.len() {
            return Err(GitError::RemoteParse);
        }
        let owner_end = url[host_end..].find('/').ok_or(GitError::RemoteParse)? + host_end;
        if owner_end >= url.len() {
            return Err(GitError::RemoteParse);
        }
        let repo_name_end = url[owner_end..].find('.').ok_or(GitError::RemoteParse)? + owner_end;
        if repo_name_end >= url.len() {
            return Err(GitError::RemoteParse);
        }

        Ok(Self {
            url,
            host: (0, host_end),
            owner: (host_end + 1, owner_end),
            repo_name: (owner_end + 1, repo_name_end),
        })
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        if !path.as_ref().is_dir() {
            return Err(GitError::NotAGitRepo(path.as_ref().to_path_buf()));
        }
        let command_output = Command::new("git")
            .arg("-C")
            .arg(path.as_ref())
            .arg("config")
            .arg("--get")
            .arg("remote.origin.url")
            .output()?;
        Self::try_new(String::from_utf8(command_output.stdout)?.trim())
    }

    pub fn host(&self) -> &str {
        &self.url[self.host.0..self.host.1]
    }

    pub fn owner(&self) -> &str {
        &self.url[self.owner.0..self.owner.1]
    }

    pub fn repo_name(&self) -> &str {
        &self.url[self.repo_name.0..self.repo_name.1]
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn parse_valid_git_remote() -> Result<()> {
        let remote = GitRemote::try_new("git@github.com:hscottvo/wayfarer.git")?;
        assert_eq!(remote.host(), "git@github.com");
        assert_eq!(remote.owner(), "hscottvo");
        assert_eq!(remote.repo_name(), "wayfarer");
        Ok(())
    }

    #[test]
    fn missing_delimiter_returns_error() {
        let result = GitRemote::try_new("no_atsign");
        assert_matches!(result, Err(GitError::RemoteParse));
    }

    #[test]
    fn no_remote() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path();
        Command::new("git").arg("init").arg(path).output()?;
        // Command::new("git")
        //     .arg("-C")
        //     .arg(path)
        //     .arg("remote")
        //     .arg("add")
        //     .arg("origin")
        //     .arg("git@github.com:hscottvo/http-forward-proxy.git")
        //     .output()?;

        let remote = GitRemote::from_path(path);
        assert_matches!(remote, Err(GitError::NoRemote));
        Ok(())
    }
}
