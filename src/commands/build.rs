use anyhow::Result;

pub fn run() -> Result<()> {
    let config = crate::config::load("config.toml")?;
    println!(
        "Loaded config: title='{}', theme='{}'",
        config.title, config.theme
    );
    let theme = crate::theme::loader::load_active_theme(".", &config.theme)?;
    println!(
        "Loaded theme: {} ({})",
        theme.metadata.name, theme.metadata.version
    );
    let pages = crate::content::pages::build_pages("content")?;
    println!("Built pages from content: {}", pages.len());
    let rendered_pages = crate::render::templates::render_pages(&theme, &config, &pages)?;
    println!("Rendered pages with templates: {}", rendered_pages.len());
    println!("`rustipo build` is not implemented yet");
    Ok(())
}
