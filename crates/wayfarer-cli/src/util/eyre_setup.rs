use eyre::Result;

pub fn setup_eyre() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .add_frame_filter(Box::new(|frames| {
            frames.retain(|frame| {
                frame
                    .filename
                    .as_ref()
                    .is_some_and(|f| f.starts_with(env!("CARGO_MANIFEST_DIR")))
            });
        }))
        .install()?;

    Ok(())
}
