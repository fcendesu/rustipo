use std::path::PathBuf;

use crate::content::frontmatter::Frontmatter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageKind {
    Index,
    Page,
    BlogPost,
    Project,
}

#[derive(Debug)]
pub struct Page {
    // Kept for future diagnostics and source-linked features (watch mode, content tracing).
    #[allow(dead_code)]
    pub source_path: PathBuf,
    pub route: String,
    pub slug: String,
    pub kind: PageKind,
    pub frontmatter: Frontmatter,
    pub markdown: String,
    pub html: String,
}
