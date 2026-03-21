use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

pub fn write_not_found_page(dist_dir: impl AsRef<Path>, html: &str) -> Result<&'static str> {
    let dist_dir = dist_dir.as_ref();
    let output = dist_dir.join("404.html");
    fs::write(&output, html)
        .with_context(|| format!("failed to write not-found page: {}", output.display()))?;
    Ok("404.html")
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::write_not_found_page;

    #[test]
    fn writes_not_found_page() {
        let dir = tempdir().expect("tempdir should be created");
        let dist = dir.path().join("dist");
        fs::create_dir_all(&dist).expect("dist should be created");

        let file_name =
            write_not_found_page(&dist, "<h1>Page not found</h1>").expect("404 should write");

        assert_eq!(file_name, "404.html");
        let html = fs::read_to_string(dist.join("404.html")).expect("404 should exist");
        assert!(html.contains("Page not found"));
    }
}
