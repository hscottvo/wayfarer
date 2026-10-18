use eyre::Result;
use wayfarer_core::git::git_repos;

use crate::commands::CommandRunner;

impl CommandRunner {
    pub(super) fn list_repos(&self) -> Result<()> {
        let repos = git_repos(self.config.base_directory())?;
        for repo in repos {
            println!("{repo}");
        }
        Ok(())
    }
}
