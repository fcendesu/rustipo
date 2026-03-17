use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    pub base_url: String,
    pub theme: String,
    pub description: String,
    // Reserved for template contexts and future metadata outputs.
    #[allow(dead_code)]
    pub author: Option<AuthorConfig>,
    pub site: Option<SiteOptions>,
}

#[derive(Debug, Deserialize)]
// Author keys are accepted in config now even though rendering does not consume them yet.
#[allow(dead_code)]
pub struct AuthorConfig {
    pub name: Option<String>,
    pub email: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SiteOptions {
    pub posts_per_page: Option<usize>,
    pub favicon: Option<String>,
}

impl SiteConfig {
    pub fn posts_per_page(&self) -> usize {
        self.site
            .as_ref()
            .and_then(|s| s.posts_per_page)
            .filter(|v| *v > 0)
            .unwrap_or(10)
    }

    pub fn resolve_favicon_links(&self, project_root: impl AsRef<Path>) -> Result<FaviconLinks> {
        let project_root = project_root.as_ref();
        let static_dir = project_root.join("static");

        let mut links = FaviconLinks {
            icon_href: Some("/favicon.ico".to_string()),
            ico_href: Some("/favicon.ico".to_string()),
            svg_href: None,
            apple_touch_icon_href: None,
        };

        if static_dir.join("favicon.svg").is_file() {
            links.svg_href = Some("/favicon.svg".to_string());
        }
        if static_dir.join("apple-touch-icon.png").is_file() {
            links.apple_touch_icon_href = Some("/apple-touch-icon.png".to_string());
        }

        if let Some(configured) = self
            .site
            .as_ref()
            .and_then(|site| site.favicon.as_deref())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            let href = normalize_favicon_href(configured);
            let expected = static_dir.join(href.trim_start_matches('/'));
            if !expected.is_file() {
                bail!(
                    "configured favicon file not found: '{}' (expected at '{}')",
                    href,
                    expected.display()
                );
            }

            links.icon_href = Some(href.clone());
            if href.ends_with(".svg") {
                links.svg_href = Some(href);
            } else if href.ends_with(".ico") {
                links.ico_href = Some(href);
            } else if href.ends_with("apple-touch-icon.png") {
                links.apple_touch_icon_href = Some(href);
            }
        }

        Ok(links)
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FaviconLinks {
    pub icon_href: Option<String>,
    pub svg_href: Option<String>,
    pub ico_href: Option<String>,
    pub apple_touch_icon_href: Option<String>,
}

fn normalize_favicon_href(value: &str) -> String {
    if value.starts_with('/') {
        value.to_string()
    } else {
        format!("/{value}")
    }
}

pub fn load(path: impl AsRef<Path>) -> Result<SiteConfig> {
    let path = path.as_ref();

    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;

    let config = toml::from_str::<SiteConfig>(&raw)
        .with_context(|| format!("failed to parse config file: {}", path.display()))?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{SiteConfig, SiteOptions};

    fn base_config() -> SiteConfig {
        SiteConfig {
            title: "Rustipo".to_string(),
            base_url: "https://example.com".to_string(),
            theme: "default".to_string(),
            description: "Portfolio".to_string(),
            author: None,
            site: None,
        }
    }

    #[test]
    fn resolves_default_favicon_links() {
        let dir = tempdir().expect("tempdir should be created");
        fs::create_dir_all(dir.path().join("static")).expect("static dir should be created");
        fs::write(dir.path().join("static/favicon.svg"), "<svg/>")
            .expect("svg favicon should be written");

        let config = base_config();
        let links = config
            .resolve_favicon_links(dir.path())
            .expect("favicon links should resolve");

        assert_eq!(links.ico_href.as_deref(), Some("/favicon.ico"));
        assert_eq!(links.icon_href.as_deref(), Some("/favicon.ico"));
        assert_eq!(links.svg_href.as_deref(), Some("/favicon.svg"));
    }

    #[test]
    fn fails_when_configured_favicon_is_missing() {
        let dir = tempdir().expect("tempdir should be created");
        fs::create_dir_all(dir.path().join("static")).expect("static dir should be created");

        let mut config = base_config();
        config.site = Some(SiteOptions {
            posts_per_page: None,
            favicon: Some("/favicon.ico".to_string()),
        });

        let error = config
            .resolve_favicon_links(dir.path())
            .expect_err("missing favicon should fail");
        assert!(
            error
                .to_string()
                .contains("configured favicon file not found")
        );
    }
}
