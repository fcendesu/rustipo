use anyhow::Result;

pub fn run() -> Result<()> {
    build_site()
}

pub fn build_site() -> Result<()> {
    build_site_with_logging(true)
}

pub fn build_site_quiet() -> Result<()> {
    build_site_with_logging(false)
}

fn build_site_with_logging(verbose: bool) -> Result<()> {
    let prepared = crate::commands::site::prepare_site(verbose)?;
    crate::output::writer::write_rendered_pages("dist", &prepared.rendered_pages)?;
    crate::output::palette::ensure_palette_output_path_available(
        "static",
        &prepared.theme.static_dirs,
    )?;
    crate::output::palette::write_palette_css("dist", &prepared.palette)?;
    let copied_assets = crate::output::assets::copy_assets_with_collision_check(
        "static",
        &prepared.theme.static_dirs,
        "dist",
    )?;
    let rss_items = crate::output::rss::write_rss_feed("dist", &prepared.config, &prepared.pages)?;
    let search_documents = crate::output::search::write_search_index("dist", &prepared.pages)?;
    let sitemap_urls = crate::output::sitemap::write_sitemap(
        "dist",
        &prepared.config.base_url,
        &prepared.rendered_pages,
    )?;
    if verbose {
        println!(
            "Generated palette CSS: dist/palette.css ({})",
            prepared.palette.id
        );
        println!("Copied assets: {}", copied_assets);
        println!("Generated RSS items: {}", rss_items);
        println!("Generated search documents: {}", search_documents);
        println!("Generated sitemap URLs: {}", sitemap_urls);
        println!("Build completed: dist/");
    }
    Ok(())
}
