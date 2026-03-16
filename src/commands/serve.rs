use anyhow::Result;

pub fn run() -> Result<()> {
    let config = crate::config::load("config.toml")?;
    println!(
        "Loaded config: title='{}', theme='{}'",
        config.title, config.theme
    );
    println!("`rustipo serve` is not implemented yet");
    Ok(())
}
