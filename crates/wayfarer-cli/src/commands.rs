pub mod print_configuration;

use clap::Subcommand;
#[derive(Subcommand)]
pub enum Commands {
    /// Lists git repos in the configured base directory
    List,
    /// Prints loaded configuration
    PrintConfiguration,
}
