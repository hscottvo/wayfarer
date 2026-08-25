mod eyre_setup;
mod tracing_setup;
use eyre::Result;

use eyre_setup::setup_eyre;
use tracing_setup::setup_tracing;

pub fn setup_env() -> Result<()> {
    setup_eyre()?;
    setup_tracing();
    Ok(())
}
