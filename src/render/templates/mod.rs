use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::PathBuf;
use tera::{Context as TeraContext, Tera};
use walkdir::WalkDir;

use crate::config::{FaviconLinks, SiteConfig, SiteStyleOptions};
use crate::content::pages::Page;
use crate::palette::models::Palette;
use crate::theme::models::Theme;

mod archive;
mod context;
mod helpers;
mod page;
mod section;
mod tags;

#[derive(Debug)]
pub struct RenderedPage {
    pub route: String,
    pub html: String,
}

pub(super) struct CommonRenderContext<'a> {
    shared: &'a context::SharedTemplateData,
    route: &'a str,
    page_kind: &'a str,
    current_section: &'a str,
    favicon_links: &'a FaviconLinks,
    site_style: &'a SiteStyleOptions,
    site_has_custom_css: bool,
    palette: &'a Palette,
}

pub(super) struct RenderEnvironment<'a> {
    pub(super) config: &'a SiteConfig,
    pub(super) shared: &'a context::SharedTemplateData,
    pub(super) favicon_links: &'a FaviconLinks,
    pub(super) site_style: &'a SiteStyleOptions,
    pub(super) site_has_custom_css: bool,
    pub(super) palette: &'a Palette,
}

pub fn render_pages(
    theme: &Theme,
    config: &SiteConfig,
    pages: &[Page],
    favicon_links: &FaviconLinks,
    site_style: &SiteStyleOptions,
    site_has_custom_css: bool,
    palette: &Palette,
) -> Result<Vec<RenderedPage>> {
    let tera = load_theme_templates(theme, config)?;
    let shared = context::build_shared_template_data(pages);
    let env = RenderEnvironment {
        config,
        shared: &shared,
        favicon_links,
        site_style,
        site_has_custom_css,
        palette,
    };

    let mut rendered = page::render_content_pages(&tera, pages, &env)?;
    rendered.extend(section::render_sections(&tera, pages, &env)?);
    rendered.extend(archive::render_blog_archive_page(&tera, pages, &env)?);
    rendered.extend(tags::render_tag_pages(&tera, pages, &env)?);

    Ok(rendered)
}

fn insert_common_site_context(
    context: &mut TeraContext,
    config: &SiteConfig,
    render_context: &CommonRenderContext<'_>,
) {
    context.insert("site_title", &config.title);
    context.insert("site_description", &config.description);
    context.insert("site_favicon", &render_context.favicon_links.icon_href);
    context.insert("site_favicon_svg", &render_context.favicon_links.svg_href);
    context.insert("site_favicon_ico", &render_context.favicon_links.ico_href);
    context.insert(
        "site_apple_touch_icon",
        &render_context.favicon_links.apple_touch_icon_href,
    );
    context.insert("site_style", render_context.site_style);
    context.insert("site_palette", render_context.palette);
    context.insert("site_has_custom_css", &render_context.site_has_custom_css);
    context::insert_page_context(
        context,
        render_context.shared,
        render_context.route,
        render_context.page_kind,
        render_context.current_section,
    );
}

fn load_theme_templates(theme: &Theme, config: &SiteConfig) -> Result<Tera> {
    let mut template_map: BTreeMap<String, PathBuf> = BTreeMap::new();

    for dir in &theme.template_dirs {
        for entry in WalkDir::new(dir) {
            let entry = entry.with_context(|| {
                format!("failed to walk theme template directory: {}", dir.display())
            })?;
            if !entry.file_type().is_file() {
                continue;
            }

            if entry.path().extension().and_then(|ext| ext.to_str()) != Some("html") {
                continue;
            }

            let rel = entry.path().strip_prefix(dir).with_context(|| {
                format!(
                    "failed to compute theme template relative path: {}",
                    entry.path().display()
                )
            })?;
            let name = rel.to_string_lossy().replace('\\', "/");
            template_map.insert(name, entry.path().to_path_buf());
        }
    }

    let mut tera = Tera::default();
    for (name, path) in template_map {
        tera.add_template_file(&path, Some(&name))
            .with_context(|| format!("failed to load template '{}': {}", name, path.display()))?;
    }
    helpers::register(&mut tera, config);

    Ok(tera)
}

#[cfg(test)]
mod tests;
