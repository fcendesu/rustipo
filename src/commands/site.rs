use anyhow::Result;

use crate::config::SiteConfig;
use crate::content::pages::Page;
use crate::palette::models::Palette;
use crate::render::templates::RenderedPage;
use crate::theme::models::Theme;

pub(crate) struct PreparedSite {
    pub config: SiteConfig,
    pub theme: Theme,
    pub palette: Palette,
    pub pages: Vec<Page>,
    pub rendered_pages: Vec<RenderedPage>,
}

pub(crate) fn prepare_site(verbose: bool) -> Result<PreparedSite> {
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
    crate::content::links::validate_internal_links(&pages, &rendered_pages)?;
    if verbose {
        println!("Rendered pages with templates: {}", rendered_pages.len());
    }

    Ok(PreparedSite {
        config,
        theme,
        palette,
        pages,
        rendered_pages,
    })
}
