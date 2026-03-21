use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::config::SiteConfig;
use crate::content::pages::{Page, PageKind};

mod builder;
mod model;
mod writer;

pub fn write_rss_feed(
    dist_dir: impl AsRef<Path>,
    config: &SiteConfig,
    pages: &[Page],
) -> Result<usize> {
    let dist_dir = dist_dir.as_ref();
    fs::create_dir_all(dist_dir)
        .with_context(|| format!("failed to create output directory: {}", dist_dir.display()))?;

    let mut entries = pages
        .iter()
        .filter(|page| page.kind == PageKind::BlogPost)
        .filter_map(builder::to_feed_item)
        .collect::<Vec<_>>();

    // Most recent first when date format is YYYY-MM-DD.
    entries.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

    let feed = writer::render_feed(config, &entries);
    let output = dist_dir.join("rss.xml");
    fs::write(&output, feed)
        .with_context(|| format!("failed to write RSS feed: {}", output.display()))?;

    Ok(entries.len())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::config::SiteConfig;
    use crate::content::pages::build_pages;

    use super::write_rss_feed;

    #[test]
    fn writes_rss_with_blog_posts() {
        let dir = tempdir().expect("tempdir should be created");
        let root = dir.path();

        fs::create_dir_all(root.join("content/blog")).expect("blog dir should be created");
        fs::write(
            root.join("content/blog/post.md"),
            "---\ntitle: Hello\ndate: 2026-03-17\nsummary: First post\n---\n\n# Hello",
        )
        .expect("post should be written");

        let pages = build_pages(root.join("content")).expect("pages should build");
        let config = SiteConfig {
            title: "Rustipo".to_string(),
            base_url: "https://example.com".to_string(),
            theme: "default".to_string(),
            palette: None,
            menus: None,
            description: "Site".to_string(),
            author: None,
            site: None,
        };

        let count = write_rss_feed(root.join("dist"), &config, &pages).expect("rss should write");
        assert_eq!(count, 1);

        let rss = fs::read_to_string(root.join("dist/rss.xml")).expect("rss should exist");
        assert!(rss.contains("<rss version=\"2.0\">"));
        assert!(rss.contains("<title>Hello</title>"));
        assert!(rss.contains("<link>https://example.com/blog/post/</link>"));
    }

    #[test]
    fn fails_when_post_has_invalid_date() {
        let dir = tempdir().expect("tempdir should be created");
        let root = dir.path();

        fs::create_dir_all(root.join("content/blog")).expect("blog dir should be created");
        fs::write(
            root.join("content/blog/post.md"),
            "---\ntitle: Hello\ndate: 2026-13-17\n---\n\n# Hello",
        )
        .expect("post should be written");

        let error = build_pages(root.join("content")).expect_err("invalid date should fail");
        assert!(
            error.to_string().contains("failed to parse markdown file"),
            "unexpected error: {error}"
        );
    }
}
