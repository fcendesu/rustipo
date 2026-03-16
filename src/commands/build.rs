use anyhow::Result;

pub fn run() -> Result<()> {
    let config = crate::config::load("config.toml")?;
    println!(
        "Loaded config: title='{}', theme='{}'",
        config.title, config.theme
    );
    println!("`rustipo build` is not implemented yet");
    Ok(())
}
