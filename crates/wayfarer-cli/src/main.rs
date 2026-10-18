mod cli;
mod commands;
mod util;
use clap::Parser as _;
use eyre::Result;
use util::setup_env;
use wayfarer_core::config::Configuration;

use crate::{cli::Cli, commands::CommandRunner};

fn main() -> Result<()> {
    setup_env()?;
    let cli = Cli::parse();

    let config = Configuration::load_xdg()?;

    let runner = CommandRunner::new(config);
    runner.run(*cli.command())?;
    Ok(())
}
