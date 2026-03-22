use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeSource {
    BuiltIn,
    Local,
}

impl ThemeSource {
    pub fn label(self, directory_name: &str) -> String {
        match self {
            Self::BuiltIn => "built-in".to_string(),
            Self::Local => directory_name.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeMetadata {
    pub id: Option<String>,
    pub name: String,
    pub version: String,
    // Preserved from theme contract for listing and future template exposure.
    #[allow(dead_code)]
    pub author: String,
    pub description: String,
    pub extends: Option<String>,
}

#[derive(Debug)]
pub struct ThemeSummary {
    pub theme_id: String,
    pub directory_name: String,
    pub source: ThemeSource,
    pub metadata: ThemeMetadata,
}

#[derive(Debug)]
pub struct Theme {
    // Retained for future theme-relative resolution (includes, diagnostics, tooling).
    #[allow(dead_code)]
    pub root_dir: PathBuf,
    pub template_dirs: Vec<PathBuf>,
    pub static_dirs: Vec<PathBuf>,
    pub metadata: ThemeMetadata,
}
