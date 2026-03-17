use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::content::pages::model::Page;
use crate::content::pages::routing::derive_page_meta;

pub fn build_pages(content_dir: impl AsRef<Path>) -> Result<Vec<Page>> {
    let content_dir = content_dir.as_ref();
    let markdown_files = crate::content::loader::discover_markdown_files(content_dir)?;

    let mut pages = Vec::new();

    for file in markdown_files {
        let raw = fs::read_to_string(&file)
            .with_context(|| format!("failed to read markdown file: {}", file.display()))?;

        let parsed = crate::content::frontmatter::parse(&raw)
            .with_context(|| format!("failed to parse markdown file: {}", file.display()))?;

        if parsed.frontmatter.draft == Some(true) {
            continue;
        }

        let rel_path = file
            .strip_prefix(content_dir)
            .with_context(|| {
                format!(
                    "failed to compute content-relative path for: {}",
                    file.display()
                )
            })?
            .to_path_buf();

        let page_meta = derive_page_meta(&rel_path, parsed.frontmatter.slug.as_deref())?;
        let html = crate::content::markdown::render_html(&parsed.content);

        pages.push(Page {
            source_path: file,
            route: page_meta.route,
            slug: page_meta.slug,
            kind: page_meta.kind,
            frontmatter: parsed.frontmatter,
            markdown: parsed.content,
            html,
        });
    }

    Ok(pages)
}
