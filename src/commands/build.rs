use anyhow::Result;

pub fn run() -> Result<()> {
    let config = crate::config::load("config.toml")?;
    println!(
        "Loaded config: title='{}', theme='{}'",
        config.title, config.theme
    );
    let pages = crate::content::pages::build_pages("content")?;
    println!("Built pages from content: {}", pages.len());
    println!("`rustipo build` is not implemented yet");
    Ok(())
}
