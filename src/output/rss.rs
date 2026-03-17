use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::config::SiteConfig;
use crate::content::pages::{Page, PageKind};

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
        .filter_map(to_feed_item)
        .collect::<Vec<_>>();

    // Most recent first when date format is YYYY-MM-DD.
    entries.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

    let feed = render_feed(config, &entries);
    let output = dist_dir.join("rss.xml");
    fs::write(&output, feed)
        .with_context(|| format!("failed to write RSS feed: {}", output.display()))?;

    Ok(entries.len())
}

struct FeedItem {
    title: String,
    link: String,
    description: String,
    pub_date: String,
}

fn to_feed_item(page: &Page) -> Option<FeedItem> {
    let date = page.frontmatter.date.as_deref()?;
    let pub_date = normalize_date(date)?;

    let title = page
        .frontmatter
        .title
        .clone()
        .unwrap_or_else(|| page.slug.clone());

    let description = page
        .frontmatter
        .summary
        .clone()
        .unwrap_or_else(|| title.clone());

    Some(FeedItem {
        title,
        link: page.route.clone(),
        description,
        pub_date,
    })
}

fn normalize_date(date: &str) -> Option<String> {
    if !is_valid_date(date) {
        return None;
    }
    Some(format!("{date}T00:00:00Z"))
}

fn is_valid_date(date: &str) -> bool {
    let parts = date.split('-').collect::<Vec<_>>();
    if parts.len() != 3 {
        return false;
    }

    let year = match parts[0].parse::<u32>() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let month = match parts[1].parse::<u32>() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let day = match parts[2].parse::<u32>() {
        Ok(v) => v,
        Err(_) => return false,
    };

    if !(1..=12).contains(&month) || day == 0 {
        return false;
    }

    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => return false,
    };

    day <= max_day
}

fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn render_feed(config: &SiteConfig, entries: &[FeedItem]) -> String {
    let mut xml = String::new();

    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<rss version=\"2.0\">\n");
    xml.push_str("  <channel>\n");
    xml.push_str(&format!(
        "    <title>{}</title>\n",
        escape_xml(&config.title)
    ));
    xml.push_str(&format!(
        "    <description>{}</description>\n",
        escape_xml(&config.description)
    ));
    xml.push_str(&format!(
        "    <link>{}</link>\n",
        escape_xml(&config.base_url)
    ));

    for entry in entries {
        let absolute_link = format!("{}{}", config.base_url.trim_end_matches('/'), entry.link);
        xml.push_str("    <item>\n");
        xml.push_str(&format!(
            "      <title>{}</title>\n",
            escape_xml(&entry.title)
        ));
        xml.push_str(&format!(
            "      <link>{}</link>\n",
            escape_xml(&absolute_link)
        ));
        xml.push_str(&format!(
            "      <description>{}</description>\n",
            escape_xml(&entry.description)
        ));
        xml.push_str(&format!("      <pubDate>{}</pubDate>\n", entry.pub_date));
        xml.push_str("    </item>\n");
    }

    xml.push_str("  </channel>\n");
    xml.push_str("</rss>\n");
    xml
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
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
            description: "Portfolio".to_string(),
            author: None,
        };

        let count = write_rss_feed(root.join("dist"), &config, &pages).expect("rss should write");
        assert_eq!(count, 1);

        let rss = fs::read_to_string(root.join("dist/rss.xml")).expect("rss should exist");
        assert!(rss.contains("<rss version=\"2.0\">"));
        assert!(rss.contains("<title>Hello</title>"));
        assert!(rss.contains("<link>https://example.com/blog/post/</link>"));
    }

    #[test]
    fn skips_posts_without_valid_date() {
        let dir = tempdir().expect("tempdir should be created");
        let root = dir.path();

        fs::create_dir_all(root.join("content/blog")).expect("blog dir should be created");
        fs::write(
            root.join("content/blog/post.md"),
            "---\ntitle: Hello\ndate: 2026-13-17\n---\n\n# Hello",
        )
        .expect("post should be written");

        let pages = build_pages(root.join("content")).expect("pages should build");
        let config = SiteConfig {
            title: "Rustipo".to_string(),
            base_url: "https://example.com".to_string(),
            theme: "default".to_string(),
            description: "Portfolio".to_string(),
            author: None,
        };

        let count = write_rss_feed(root.join("dist"), &config, &pages).expect("rss should write");
        assert_eq!(count, 0);

        let rss = fs::read_to_string(root.join("dist/rss.xml")).expect("rss should exist");
        assert!(!rss.contains("<item>"));
    }
}
