mod util;
use eyre::Result;
use tracing::{info, instrument};
use util::setup_env;
use wayfarer_core::config::Configuration;

fn main() -> Result<()> {
    setup_env()?;
    bar(5);
    let config = Configuration::load_xdg()?;
    Ok(())
}

#[instrument]
fn foo(var: u32) {
    info!("hello from foo");
}

#[instrument]
fn bar(var: u32) {
    info!("bar");
    foo(var);
}
