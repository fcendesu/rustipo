use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Palette {
    pub id: String,
    pub name: String,
    pub description: String,
    pub color_scheme: String,
    pub bg: String,
    pub text: String,
    pub surface_muted: String,
    pub border: String,
    pub blockquote_border: String,
    pub link: String,
    pub link_hover: String,
    pub code_bg: String,
    pub code_text: String,
    pub table_header_bg: String,
    #[serde(default, flatten)]
    pub extra_tokens: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PaletteSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source: PaletteSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteSource {
    BuiltIn,
    Local,
}

impl PaletteSource {
    pub fn as_label(self) -> &'static str {
        match self {
            Self::BuiltIn => "built-in",
            Self::Local => "local",
        }
    }
}
