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
    let config = crate::config::load("config.toml")?;
    if verbose {
        println!(
            "Loaded config: title='{}', theme='{}', palette='{}'",
            config.title,
            config.theme,
            config.selected_palette()
        );
    }
    let theme = crate::theme::loader::load_active_theme(".", &config.theme)?;
    let palette = crate::palette::loader::load_palette(".", config.selected_palette())?;
    if verbose {
        println!(
            "Loaded theme: {} ({})",
            theme.metadata.name, theme.metadata.version
        );
        println!("Loaded palette: {}", palette.name);
    }
    let favicon_links = config.resolve_favicon_links(".")?;
    let site_style = config.style_options();
    let (_site_fonts, font_faces) = config.resolve_fonts(".", &theme.static_dirs)?;
    let site_font_faces_css =
        (!font_faces.is_empty()).then(|| crate::config::fonts::render_font_faces_css(&font_faces));
    let site_has_custom_css = config.has_custom_css(".");
    let pages = crate::content::pages::build_pages("content")?;
    if verbose {
        println!("Built pages from content: {}", pages.len());
    }
    let rendered_pages = crate::render::templates::render_pages(
        &theme,
        &config,
        &pages,
        &crate::render::templates::SiteRenderContext {
            favicon_links: &favicon_links,
            site_style: &site_style,
            site_has_custom_css,
            site_font_faces_css: site_font_faces_css.as_deref(),
            palette: &palette,
        },
    )?;
    if verbose {
        println!("Rendered pages with templates: {}", rendered_pages.len());
    }
    crate::output::writer::write_rendered_pages("dist", &rendered_pages)?;
    crate::output::palette::ensure_palette_output_path_available("static", &theme.static_dirs)?;
    crate::output::palette::write_palette_css("dist", &palette)?;
    let copied_assets = crate::output::assets::copy_assets_with_collision_check(
        "static",
        &theme.static_dirs,
        "dist",
    )?;
    let rss_items = crate::output::rss::write_rss_feed("dist", &config, &pages)?;
    let search_documents = crate::output::search::write_search_index("dist", &pages)?;
    let sitemap_urls =
        crate::output::sitemap::write_sitemap("dist", &config.base_url, &rendered_pages)?;
    if verbose {
        println!("Generated palette CSS: dist/palette.css ({})", palette.id);
        println!("Copied assets: {}", copied_assets);
        println!("Generated RSS items: {}", rss_items);
        println!("Generated search documents: {}", search_documents);
        println!("Generated sitemap URLs: {}", sitemap_urls);
        println!("Build completed: dist/");
    }
    Ok(())
}
