use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::config::SiteConfig;
use crate::content::pages::{Page, PublicationMode, build_pages, build_pages_for_mode};
use crate::palette::models::Palette;
use crate::render::templates::RenderedPage;
use crate::theme::models::Theme;

pub(crate) struct PreparedSite {
    pub config: SiteConfig,
    pub theme: Theme,
    pub palette: Palette,
    pub pages: Vec<Page>,
    pub rendered_pages: Vec<RenderedPage>,
    pub not_found_html: String,
}

pub(crate) fn prepare_site(
    verbose: bool,
    publication_mode: PublicationMode,
) -> Result<PreparedSite> {
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
    let asset_version = compute_asset_version(".", &theme.static_dirs, &palette)?;
    let pages = match publication_mode {
        PublicationMode::Production => build_pages("content")?,
        PublicationMode::Preview => build_pages_for_mode("content", PublicationMode::Preview)?,
    };
    if verbose {
        println!("Built pages from content: {}", pages.len());
    }

    let site_context = crate::render::templates::SiteRenderContext {
        favicon_links: &favicon_links,
        site_style: &site_style,
        site_has_custom_css,
        site_font_faces_css: site_font_faces_css.as_deref(),
        asset_version: &asset_version,
        palette: &palette,
    };
    let rendered_pages =
        crate::render::templates::render_pages(&theme, &config, &pages, &site_context)?;
    let not_found_html =
        crate::render::templates::render_not_found_page(&theme, &config, &pages, &site_context)?;
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
        not_found_html,
    })
}

fn compute_asset_version(
    project_root: impl AsRef<Path>,
    theme_static_dirs: &[std::path::PathBuf],
    palette: &Palette,
) -> Result<String> {
    let mut hash = 0xcbf29ce484222325_u64;
    hash_static_dir(&mut hash, project_root.as_ref().join("static"))?;

    for dir in theme_static_dirs {
        hash_static_dir(&mut hash, dir)?;
    }

    let palette_json = serde_json::to_vec(palette).context("failed to serialize palette")?;
    hash_bytes(&mut hash, &palette_json);

    Ok(format!("{hash:016x}"))
}

fn hash_static_dir(hash: &mut u64, dir: impl AsRef<Path>) -> Result<()> {
    let dir = dir.as_ref();
    if !dir.exists() {
        return Ok(());
    }

    let mut files = WalkDir::new(dir)
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("failed to walk static asset directory: {}", dir.display()))?;
    files.sort_by(|left, right| left.path().cmp(right.path()));

    for entry in files {
        if !entry.file_type().is_file() {
            continue;
        }

        let relative = entry.path().strip_prefix(dir).with_context(|| {
            format!(
                "failed to compute static asset relative path: {}",
                entry.path().display()
            )
        })?;
        hash_bytes(hash, relative.to_string_lossy().as_bytes());
        hash_bytes(hash, &[0]);
        let bytes = fs::read(entry.path())
            .with_context(|| format!("failed to read static asset: {}", entry.path().display()))?;
        hash_bytes(hash, &bytes);
        hash_bytes(hash, &[0xff]);
    }

    Ok(())
}

fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }
}
