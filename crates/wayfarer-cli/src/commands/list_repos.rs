use eyre::Result;
use std::fmt::Write;
use wayfarer_core::git::{GitRepo, git_repos};

use crate::commands::CommandRunner;

impl CommandRunner {
    pub(super) fn list_repos(&self) -> Result<()> {
        let repos = git_repos(self.config.base_directory())?;
        let formatted = format_repos(repos)?;
        println!("{formatted}");
        Ok(())
    }
}
fn format_repos(repos: Vec<GitRepo>) -> Result<String> {
    let mut output = String::new();
    for repo in repos {
        repo.write_to(&mut output, 10)?;
        writeln!(output)?;
    }
    Ok(output)
}
