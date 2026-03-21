use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::render::templates::RenderedPage;

pub fn write_rendered_pages(dist_dir: impl AsRef<Path>, pages: &[RenderedPage]) -> Result<()> {
    let dist_dir = dist_dir.as_ref();

    if dist_dir.exists() {
        fs::remove_dir_all(dist_dir)
            .with_context(|| format!("failed to clean output directory: {}", dist_dir.display()))?;
    }
    fs::create_dir_all(dist_dir)
        .with_context(|| format!("failed to create output directory: {}", dist_dir.display()))?;

    let output_routes = collect_output_routes(pages)?;
    let mut outputs = Vec::with_capacity(output_routes.len());
    for (page, rel_path) in pages.iter().zip(output_routes.iter()) {
        outputs.push((dist_dir.join(rel_path), page.html.as_str()));
    }

    for (output_path, html) in outputs {
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create output path: {}", parent.display()))?;
        }

        fs::write(&output_path, html)
            .with_context(|| format!("failed to write output file: {}", output_path.display()))?;
    }

    Ok(())
}

pub fn validate_rendered_pages(pages: &[RenderedPage]) -> Result<usize> {
    let output_routes = collect_output_routes(pages)?;
    Ok(output_routes.len())
}

fn collect_output_routes(pages: &[RenderedPage]) -> Result<Vec<PathBuf>> {
    let mut outputs = Vec::with_capacity(pages.len());
    let mut seen_output_paths: HashMap<PathBuf, String> = HashMap::new();
    for page in pages {
        let output_path = route_to_output_path(&page.route)?;
        if let Some(existing_route) =
            seen_output_paths.insert(output_path.clone(), page.route.clone())
        {
            bail!(
                "duplicate output route collision: '{}' and '{}' both map to '{}'",
                existing_route,
                page.route,
                output_path.display()
            );
        }
        outputs.push(output_path);
    }

    Ok(outputs)
}

fn route_to_output_path(route: &str) -> Result<PathBuf> {
    if !route.starts_with('/') || !route.ends_with('/') {
        bail!("route must start and end with '/': {route}");
    }

    let trimmed = route.trim_matches('/');
    if trimmed.is_empty() {
        return Ok(PathBuf::from("index.html"));
    }

    Ok(PathBuf::from(trimmed).join("index.html"))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::write_rendered_pages;
    use crate::render::templates::RenderedPage;

    #[test]
    fn writes_pretty_url_paths() {
        let dir = tempdir().expect("tempdir should be created");
        let dist = dir.path().join("dist");

        let pages = vec![
            RenderedPage {
                route: "/".to_string(),
                html: "<h1>Home</h1>".to_string(),
            },
            RenderedPage {
                route: "/about/".to_string(),
                html: "<h1>About</h1>".to_string(),
            },
            RenderedPage {
                route: "/blog/post/".to_string(),
                html: "<h1>Post</h1>".to_string(),
            },
        ];

        write_rendered_pages(&dist, &pages).expect("pages should be written");

        assert!(dist.join("index.html").is_file());
        assert!(dist.join("about/index.html").is_file());
        assert!(dist.join("blog/post/index.html").is_file());

        let about_html =
            fs::read_to_string(dist.join("about/index.html")).expect("about html should exist");
        assert!(about_html.contains("About"));
    }

    #[test]
    fn fails_on_duplicate_output_route_collision() {
        let dir = tempdir().expect("tempdir should be created");
        let dist = dir.path().join("dist");

        let pages = vec![
            RenderedPage {
                route: "/about/".to_string(),
                html: "<h1>About</h1>".to_string(),
            },
            RenderedPage {
                route: "/about/".to_string(),
                html: "<h1>About Duplicate</h1>".to_string(),
            },
        ];

        let error = write_rendered_pages(&dist, &pages).expect_err("duplicate route should fail");
        assert!(
            error
                .to_string()
                .contains("duplicate output route collision")
        );
    }
}
