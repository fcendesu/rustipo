use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Serialize;

use crate::content::pages::{Page, PageKind};

#[derive(Debug, Serialize)]
struct SearchDocument {
    route: String,
    title: String,
    summary: Option<String>,
    tags: Vec<String>,
    section: String,
    content: String,
}

pub fn write_search_index(
    dist_dir: impl AsRef<Path>,
    base_url: &str,
    pages: &[Page],
) -> Result<usize> {
    let dist_dir = dist_dir.as_ref();
    fs::create_dir_all(dist_dir)
        .with_context(|| format!("failed to create output directory: {}", dist_dir.display()))?;

    let mut docs = pages
        .iter()
        .map(|page| to_search_document(page, base_url))
        .collect::<Vec<_>>();
    docs.sort_by(|a, b| a.route.cmp(&b.route));

    let output = dist_dir.join("search-index.json");
    let payload =
        serde_json::to_string_pretty(&docs).context("failed to serialize search index JSON")?;
    fs::write(&output, payload)
        .with_context(|| format!("failed to write search index: {}", output.display()))?;

    Ok(docs.len())
}

fn to_search_document(page: &Page, base_url: &str) -> SearchDocument {
    SearchDocument {
        route: crate::url::public_url_path(base_url, &page.route),
        title: page
            .frontmatter
            .title
            .clone()
            .unwrap_or_else(|| page.slug.clone()),
        summary: page.frontmatter.summary.clone(),
        tags: page.frontmatter.tags.clone().unwrap_or_default(),
        section: section_name(page.kind).to_string(),
        content: normalize_text(&page.markdown),
    }
}

fn section_name(kind: PageKind) -> &'static str {
    match kind {
        PageKind::Index => "index",
        PageKind::Page => "page",
        PageKind::BlogPost => "blog",
        PageKind::Project => "projects",
    }
}

fn normalize_text(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::content::pages::build_pages;

    use super::write_search_index;

    #[test]
    fn writes_search_index_from_pages() {
        let dir = tempdir().expect("tempdir should be created");
        let root = dir.path();
        let content_dir = root.join("content");
        let dist_dir = root.join("dist");

        fs::create_dir_all(content_dir.join("blog")).expect("blog dir should be created");
        fs::write(content_dir.join("index.md"), "# Home").expect("index should be written");
        fs::write(
            content_dir.join("blog/post.md"),
            "---\ntitle: Searchable\nsummary: Test summary\ntags: [\"rust\", \"ssg\"]\n---\n\n# Post Body",
        )
        .expect("post should be written");

        let pages = build_pages(&content_dir).expect("pages should build");
        let count = write_search_index(&dist_dir, "https://example.com/docs/", &pages)
            .expect("search index should write");
        assert_eq!(count, 2);

        let raw =
            fs::read_to_string(dist_dir.join("search-index.json")).expect("search index exists");
        let value: serde_json::Value =
            serde_json::from_str(&raw).expect("search index should be valid json");
        let docs = value.as_array().expect("search index should be array");
        assert_eq!(docs.len(), 2);

        let blog_doc = docs
            .iter()
            .find(|doc| {
                doc.get("route").and_then(|route| route.as_str()) == Some("/docs/blog/post/")
            })
            .expect("blog route should be present");
        assert_eq!(
            blog_doc.get("title").and_then(|v| v.as_str()),
            Some("Searchable")
        );
        assert_eq!(
            blog_doc.get("section").and_then(|v| v.as_str()),
            Some("blog")
        );
        assert!(
            blog_doc
                .get("tags")
                .and_then(|v| v.as_array())
                .is_some_and(|tags| tags.len() == 2)
        );
    }
}
