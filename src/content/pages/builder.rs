use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::NaiveDate;

use crate::content::pages::model::Page;
use crate::content::pages::routing::derive_page_meta;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationMode {
    Production,
    Preview,
}

pub fn build_pages_for_mode(
    content_dir: impl AsRef<Path>,
    publication_mode: PublicationMode,
) -> Result<Vec<Page>> {
    let today = chrono::Utc::now().date_naive();
    build_pages_for_date(content_dir, publication_mode, today)
}

fn build_pages_for_date(
    content_dir: impl AsRef<Path>,
    publication_mode: PublicationMode,
    today: NaiveDate,
) -> Result<Vec<Page>> {
    let content_dir = content_dir.as_ref();
    let markdown_files = crate::content::loader::discover_markdown_files(content_dir)?;

    let mut pages = Vec::new();

    for file in markdown_files {
        let raw = fs::read_to_string(&file)
            .with_context(|| format!("failed to read markdown file: {}", file.display()))?;

        let parsed = crate::content::frontmatter::parse(&raw)
            .with_context(|| format!("failed to parse markdown file: {}", file.display()))?;

        if should_exclude_from_output(&parsed.frontmatter, publication_mode, today) {
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
        let rendered = crate::content::markdown::render_html(&parsed.content);

        pages.push(Page {
            source_path: file,
            route: page_meta.route,
            slug: page_meta.slug,
            kind: page_meta.kind,
            has_mermaid: rendered.has_mermaid,
            has_math: rendered.has_math,
            shortcode_assets: rendered.shortcode_assets,
            toc: rendered.toc,
            frontmatter: parsed.frontmatter,
            markdown: parsed.content,
            html: rendered.html,
        });
    }

    Ok(pages)
}

fn should_exclude_from_output(
    frontmatter: &crate::content::frontmatter::Frontmatter,
    publication_mode: PublicationMode,
    today: NaiveDate,
) -> bool {
    match publication_mode {
        PublicationMode::Preview => false,
        PublicationMode::Production => {
            if frontmatter.draft == Some(true) {
                return true;
            }

            frontmatter
                .date
                .as_ref()
                .is_some_and(|date| date.as_naive_date() > today)
        }
    }
}

#[cfg(test)]
pub(super) fn build_pages_for_test_date(
    content_dir: impl AsRef<Path>,
    publication_mode: PublicationMode,
    today: NaiveDate,
) -> Result<Vec<Page>> {
    build_pages_for_date(content_dir, publication_mode, today)
}
