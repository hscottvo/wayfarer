use std::path::PathBuf;

use clap::Parser;

use crate::commands::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub const fn command(&self) -> &Commands {
        &self.command
    }
}
