use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::render::templates::RenderedPage;

pub fn write_sitemap(
    dist_dir: impl AsRef<Path>,
    base_url: &str,
    pages: &[RenderedPage],
) -> Result<usize> {
    let dist_dir = dist_dir.as_ref();
    fs::create_dir_all(dist_dir)
        .with_context(|| format!("failed to create output directory: {}", dist_dir.display()))?;

    let urls = collect_urls(base_url, pages);
    let sitemap = render_sitemap(&urls);

    let output = dist_dir.join("sitemap.xml");
    fs::write(&output, sitemap)
        .with_context(|| format!("failed to write sitemap: {}", output.display()))?;

    Ok(urls.len())
}

fn collect_urls(base_url: &str, pages: &[RenderedPage]) -> BTreeSet<String> {
    let base = base_url.trim_end_matches('/');

    pages
        .iter()
        .map(|page| format!("{base}{}", page.route))
        .collect::<BTreeSet<_>>()
}

fn render_sitemap(urls: &BTreeSet<String>) -> String {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    for url in urls {
        xml.push_str("  <url>\n");
        xml.push_str(&format!("    <loc>{}</loc>\n", escape_xml(url)));
        xml.push_str("  </url>\n");
    }

    xml.push_str("</urlset>\n");
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

    use super::write_sitemap;
    use crate::render::templates::RenderedPage;

    #[test]
    fn writes_sitemap_from_rendered_routes() {
        let dir = tempdir().expect("tempdir should be created");
        let dist = dir.path().join("dist");

        let pages = vec![
            RenderedPage {
                route: "/".to_string(),
                html: "".to_string(),
            },
            RenderedPage {
                route: "/about/".to_string(),
                html: "".to_string(),
            },
            RenderedPage {
                route: "/about/".to_string(),
                html: "".to_string(),
            },
            RenderedPage {
                route: "/blog/post/".to_string(),
                html: "".to_string(),
            },
        ];

        let count =
            write_sitemap(&dist, "https://example.com", &pages).expect("sitemap should be written");
        assert_eq!(count, 3);

        let sitemap = fs::read_to_string(dist.join("sitemap.xml")).expect("sitemap should exist");
        assert!(sitemap.contains("<urlset"));
        assert!(sitemap.contains("<loc>https://example.com/</loc>"));
        assert!(sitemap.contains("<loc>https://example.com/about/</loc>"));
        assert!(sitemap.contains("<loc>https://example.com/blog/post/</loc>"));
    }
}
