pub mod print_configuration;

pub mod list_repos;

use clap::Subcommand;
use tracing::debug;
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
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }
    pub fn run(&self, command: Commands) {
        match command {
            Commands::List => debug!("passed in command List"),
            Commands::PrintConfiguration => self.print_configuration(),
        }
    }
}
