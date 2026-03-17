use anyhow::Result;

pub fn run() -> Result<()> {
    build_site()
}

pub fn build_site() -> Result<()> {
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
    let favicon_links = config.resolve_favicon_links(".")?;
    let pages = crate::content::pages::build_pages("content")?;
    println!("Built pages from content: {}", pages.len());
    let rendered_pages =
        crate::render::templates::render_pages(&theme, &config, &pages, &favicon_links)?;
    println!("Rendered pages with templates: {}", rendered_pages.len());
    crate::output::writer::write_rendered_pages("dist", &rendered_pages)?;
    let copied_assets = crate::output::assets::copy_assets_with_collision_check(
        "static",
        &theme.static_dirs,
        "dist",
    )?;
    let rss_items = crate::output::rss::write_rss_feed("dist", &config, &pages)?;
    let search_documents = crate::output::search::write_search_index("dist", &pages)?;
    let sitemap_urls =
        crate::output::sitemap::write_sitemap("dist", &config.base_url, &rendered_pages)?;
    println!("Copied assets: {}", copied_assets);
    println!("Generated RSS items: {}", rss_items);
    println!("Generated search documents: {}", search_documents);
    println!("Generated sitemap URLs: {}", sitemap_urls);
    println!("Build completed: dist/");
    Ok(())
}
