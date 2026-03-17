use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct ThemeMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub extends: Option<String>,
}

#[derive(Debug)]
pub struct ThemeSummary {
    pub directory_name: String,
    pub metadata: ThemeMetadata,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Theme {
    pub root_dir: PathBuf,
    pub template_dirs: Vec<PathBuf>,
    pub static_dirs: Vec<PathBuf>,
    pub metadata: ThemeMetadata,
}
