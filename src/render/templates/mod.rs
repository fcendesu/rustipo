use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::PathBuf;
use tera::{Context as TeraContext, Tera};
use walkdir::WalkDir;

use crate::config::{FaviconLinks, SiteConfig, SiteStyleOptions};
use crate::content::pages::Page;
use crate::theme::models::Theme;

mod archive;
mod helpers;
mod page;
mod section;
mod tags;

#[derive(Debug)]
pub struct RenderedPage {
    pub route: String,
    pub html: String,
}

pub fn render_pages(
    theme: &Theme,
    config: &SiteConfig,
    pages: &[Page],
    favicon_links: &FaviconLinks,
    site_style: &SiteStyleOptions,
    site_has_custom_css: bool,
) -> Result<Vec<RenderedPage>> {
    let tera = load_theme_templates(theme, config)?;

    let mut rendered = page::render_content_pages(
        &tera,
        config,
        pages,
        favicon_links,
        site_style,
        site_has_custom_css,
    )?;
    rendered.extend(section::render_sections(
        &tera,
        config,
        pages,
        favicon_links,
        site_style,
        site_has_custom_css,
    )?);
    rendered.extend(archive::render_blog_archive_page(
        &tera,
        config,
        pages,
        favicon_links,
        site_style,
        site_has_custom_css,
    )?);
    rendered.extend(tags::render_tag_pages(
        &tera,
        config,
        pages,
        favicon_links,
        site_style,
        site_has_custom_css,
    )?);

    Ok(rendered)
}

fn insert_common_site_context(
    context: &mut TeraContext,
    config: &SiteConfig,
    favicon_links: &FaviconLinks,
    site_style: &SiteStyleOptions,
    site_has_custom_css: bool,
) {
    context.insert("site_title", &config.title);
    context.insert("site_description", &config.description);
    context.insert("site_favicon", &favicon_links.icon_href);
    context.insert("site_favicon_svg", &favicon_links.svg_href);
    context.insert("site_favicon_ico", &favicon_links.ico_href);
    context.insert(
        "site_apple_touch_icon",
        &favicon_links.apple_touch_icon_href,
    );
    context.insert("site_style", site_style);
    context.insert("site_has_custom_css", &site_has_custom_css);
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
