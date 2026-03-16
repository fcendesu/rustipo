use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SiteConfig {
    pub title: String,
    pub base_url: String,
    pub theme: String,
    pub description: String,
    pub author: Option<AuthorConfig>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthorConfig {
    pub name: Option<String>,
    pub email: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
}

pub fn load(path: impl AsRef<Path>) -> Result<SiteConfig> {
    let path = path.as_ref();

    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;

    let config = toml::from_str::<SiteConfig>(&raw)
        .with_context(|| format!("failed to parse config file: {}", path.display()))?;

    Ok(config)
}
