use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

pub fn write_default_robots_txt(
    dist_dir: impl AsRef<Path>,
    base_url: &str,
) -> Result<&'static str> {
    let dist_dir = dist_dir.as_ref();
    fs::create_dir_all(dist_dir)
        .with_context(|| format!("failed to create output directory: {}", dist_dir.display()))?;

    let robots = render_robots_txt(base_url);
    let output = dist_dir.join("robots.txt");
    fs::write(&output, robots)
        .with_context(|| format!("failed to write robots.txt: {}", output.display()))?;

    Ok("robots.txt")
}

fn render_robots_txt(base_url: &str) -> String {
    let base = base_url.trim_end_matches('/');
    format!("User-agent: *\nAllow: /\nSitemap: {base}/sitemap.xml\n")
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::write_default_robots_txt;

    #[test]
    fn writes_default_robots_txt() {
        let dir = tempdir().expect("tempdir should be created");
        let dist = dir.path().join("dist");

        let file_name = write_default_robots_txt(&dist, "https://example.com/")
            .expect("robots should be written");
        assert_eq!(file_name, "robots.txt");

        let robots = fs::read_to_string(dist.join("robots.txt")).expect("robots.txt should exist");
        assert_eq!(
            robots,
            "User-agent: *\nAllow: /\nSitemap: https://example.com/sitemap.xml\n"
        );
    }
}
