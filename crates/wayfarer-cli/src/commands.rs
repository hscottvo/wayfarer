pub mod print_configuration;

pub mod list_repos;

use clap::Subcommand;
use eyre::Result;
use wayfarer_core::config::Configuration;
#[derive(Subcommand, Clone, Copy)]
pub enum Commands {
    /// Lists git repos in the configured base directory
    List,
    /// Prints loaded configuration
    PrintConfiguration,
}

pub struct CommandRunner {
    config: Configuration,
}

impl CommandRunner {
    pub const fn new(config: Configuration) -> Self {
        Self { config }
    }
    pub fn run(&self, command: Commands) -> Result<()> {
        match command {
            Commands::List => self.list_repos()?,
            Commands::PrintConfiguration => self.print_configuration(),
        }
        Ok(())
    }
}
