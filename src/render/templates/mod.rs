use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::PathBuf;
use tera::Tera;
use walkdir::WalkDir;

use crate::config::SiteConfig;
use crate::content::pages::Page;
use crate::theme::models::Theme;

mod archive;
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
) -> Result<Vec<RenderedPage>> {
    let tera = load_theme_templates(theme)?;

    let mut rendered = page::render_content_pages(&tera, config, pages)?;
    rendered.extend(section::render_sections(&tera, config, pages)?);
    rendered.extend(archive::render_blog_archive_page(&tera, config, pages)?);
    rendered.extend(tags::render_tag_pages(&tera, config, pages)?);

    Ok(rendered)
}

fn load_theme_templates(theme: &Theme) -> Result<Tera> {
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

    Ok(tera)
}

#[cfg(test)]
mod tests;
