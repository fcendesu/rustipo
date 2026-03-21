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
mod not_found;
mod page;
mod section;
mod tags;

#[derive(Debug)]
pub struct RenderedPage {
    pub route: String,
    pub html: String,
}

pub struct SiteRenderContext<'a> {
    pub favicon_links: &'a FaviconLinks,
    pub site_style: &'a SiteStyleOptions,
    pub site_has_custom_css: bool,
    pub site_font_faces_css: Option<&'a str>,
    pub palette: &'a Palette,
}

pub(super) struct CommonRenderContext<'a> {
    shared: &'a context::SharedTemplateData,
    route: &'a str,
    page_kind: &'a str,
    current_section: &'a str,
    site: &'a SiteRenderContext<'a>,
}

pub(super) struct RenderEnvironment<'a> {
    pub(super) config: &'a SiteConfig,
    pub(super) shared: &'a context::SharedTemplateData,
    pub(super) site: &'a SiteRenderContext<'a>,
}

pub fn render_pages(
    theme: &Theme,
    config: &SiteConfig,
    pages: &[Page],
    site: &SiteRenderContext<'_>,
) -> Result<Vec<RenderedPage>> {
    let tera = load_theme_templates(theme, config)?;
    let shared = context::build_shared_template_data(pages, config);
    let env = RenderEnvironment {
        config,
        shared: &shared,
        site,
    };

    let mut rendered = page::render_content_pages(&tera, pages, &env)?;
    rendered.extend(section::render_sections(&tera, pages, &env)?);
    rendered.extend(archive::render_blog_archive_page(&tera, pages, &env)?);
    rendered.extend(tags::render_tag_pages(&tera, pages, &env)?);

    Ok(rendered)
}

pub fn render_not_found_page(
    theme: &Theme,
    config: &SiteConfig,
    pages: &[Page],
    site: &SiteRenderContext<'_>,
) -> Result<String> {
    let tera = load_theme_templates(theme, config)?;
    let shared = context::build_shared_template_data(pages, config);
    let env = RenderEnvironment {
        config,
        shared: &shared,
        site,
    };

    not_found::render_not_found_page(&tera, &env)
}

fn insert_common_site_context(
    context: &mut TeraContext,
    config: &SiteConfig,
    render_context: &CommonRenderContext<'_>,
) {
    context.insert("site_title", &config.title);
    context.insert("site_description", &config.description);
    context.insert("site_favicon", &render_context.site.favicon_links.icon_href);
    context.insert(
        "site_favicon_svg",
        &render_context.site.favicon_links.svg_href,
    );
    context.insert(
        "site_favicon_ico",
        &render_context.site.favicon_links.ico_href,
    );
    context.insert(
        "site_apple_touch_icon",
        &render_context.site.favicon_links.apple_touch_icon_href,
    );
    context.insert("site_style", render_context.site.site_style);
    context.insert("site_palette", render_context.site.palette);
    context.insert(
        "site_has_custom_css",
        &render_context.site.site_has_custom_css,
    );
    context.insert(
        "site_font_faces_css",
        &render_context.site.site_font_faces_css,
    );
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

    let template_files = template_map
        .iter()
        .map(|(name, path)| (path.as_path(), Some(name.as_str())))
        .collect::<Vec<_>>();

    let mut tera = Tera::default();
    tera.add_template_files(template_files)
        .context("failed to load theme templates into Tera")?;
    helpers::register(&mut tera, config);

    Ok(tera)
}

#[cfg(test)]
mod tests;
