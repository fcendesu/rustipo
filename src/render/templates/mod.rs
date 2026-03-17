use anyhow::{Context, Result};
use tera::Tera;

use crate::config::SiteConfig;
use crate::content::pages::Page;
use crate::theme::models::Theme;

mod archive;
mod page;
mod section;
mod tags;

#[derive(Debug)]
#[allow(dead_code)]
pub struct RenderedPage {
    pub route: String,
    pub html: String,
}

pub fn render_pages(
    theme: &Theme,
    config: &SiteConfig,
    pages: &[Page],
) -> Result<Vec<RenderedPage>> {
    let glob = format!("{}/**/*.html", theme.templates_dir.display());
    let tera = Tera::new(&glob).with_context(|| {
        format!(
            "failed to load templates from directory: {}",
            theme.templates_dir.display()
        )
    })?;

    let mut rendered = page::render_content_pages(&tera, config, pages)?;
    rendered.extend(section::render_sections(&tera, config, pages)?);
    rendered.extend(archive::render_blog_archive_page(&tera, config, pages)?);
    rendered.extend(tags::render_tag_pages(&tera, config, pages)?);

    Ok(rendered)
}

#[cfg(test)]
mod tests;
