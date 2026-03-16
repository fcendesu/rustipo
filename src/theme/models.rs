use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ThemeMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Theme {
    pub root_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub static_dir: PathBuf,
    pub metadata: ThemeMetadata,
}
