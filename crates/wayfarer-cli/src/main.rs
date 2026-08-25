mod util;
use eyre::Result;
use tracing::{info, instrument};
use util::setup_env;

fn main() -> Result<()> {
    setup_env()?;
    bar(5);
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
