mod cli;
mod util;
use clap::Parser as _;
use eyre::Result;
use tracing::debug;
use util::setup_env;
use wayfarer_core::{config::Configuration, git::git_repos};

use crate::cli::{Cli, Commands};

fn main() -> Result<()> {
    setup_env()?;
    let cli = Cli::parse();

    match cli.command {
        Commands::List => debug!("passed in command List"),
        Commands::PrintConfiguration => debug!("passed in command PrintConfiguration"),
    }

    let config = Configuration::load_xdg()?;
    println!("{config:?}");
    let repos = git_repos(config.base_directory())?;
    for repo in &repos {
        println!("{repo}");
    }
    Ok(())
}
