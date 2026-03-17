use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

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
}

impl SiteConfig {
    pub fn posts_per_page(&self) -> usize {
        self.site
            .as_ref()
            .and_then(|s| s.posts_per_page)
            .filter(|v| *v > 0)
            .unwrap_or(10)
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
