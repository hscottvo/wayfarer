use anstyle::AnsiColor;
use eyre::{OptionExt as _, Result};
use itertools::izip;
use std::fmt::Write;
use std::string::String;
use wayfarer_core::git::{GitRepo, git_repos};

use crate::commands::CommandRunner;

impl CommandRunner {
    pub(super) fn list_repos(&self) -> Result<()> {
        let repos = git_repos(self.config.base_directory())?;
        let formatted = format_repos(&repos)?;
        println!("{formatted}");
        Ok(())
    }
}
fn format_repos(repos: &Vec<GitRepo>) -> Result<String> {
    let mut output = String::new();

    let mut names = vec!["name"];
    let mut branches = vec!["branch"];
    let mut owners = vec!["owner"];
    let mut hosts = vec!["host"];
    for repo in repos {
        names.push(repo.name().unwrap_or(""));
        branches.push(repo.branch());
        owners.push(repo.owner().unwrap_or(""));
        hosts.push(repo.host().unwrap_or(""));
    }

    let names_width: usize = names
        .iter()
        .map(|s| s.len())
        .max()
        .ok_or_eyre("no repos to format")?
        + 1;
    let branches_width: usize = branches
        .iter()
        .map(|s| s.len())
        .max()
        .ok_or_eyre("no repos to format")?
        + 1;
    let owners_width: usize = owners
        .iter()
        .map(|s| s.len())
        .max()
        .ok_or_eyre("no repos to format")?
        + 1;

    let mut names = names.iter();
    let mut branches = branches.iter();
    let mut owners = owners.iter();
    let mut hosts = hosts.iter();

    let header = AnsiColor::Blue.on_default().bold();

    let name = names.next().ok_or_eyre("no repos to format")?;
    let branch = branches.next().ok_or_eyre("no repos to format")?;
    let owner = owners.next().ok_or_eyre("no repos to format")?;
    let host = hosts.next().ok_or_eyre("no repos to format")?;
    let name_padding = " ".repeat(names_width - name.len());
    let branch_padding = " ".repeat(branches_width - branch.len());
    let owner_padding = " ".repeat(owners_width - owner.len());
    writeln!(
        output,
        "{header}{name}{name_padding}{branch}{branch_padding}{owner}{owner_padding}{host}{header:#}"
    )?;

    for (name, branch, owner, host) in izip!(names, branches, owners, hosts) {
        let name_padding = " ".repeat(names_width - name.len());
        let branch_padding = " ".repeat(branches_width - branch.len());
        let owner_padding = " ".repeat(owners_width - owner.len());
        writeln!(
            output,
            "{name}{name_padding}{branch}{branch_padding}{owner}{owner_padding}{host}"
        )?;
    }

    Ok(output)
}
