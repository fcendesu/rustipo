use std::path::PathBuf;

use crate::content::frontmatter::Frontmatter;
use crate::content::toc::TocItem;

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
    pub has_mermaid: bool,
    pub has_math: bool,
    pub toc: Vec<TocItem>,
    pub frontmatter: Frontmatter,
    pub markdown: String,
    pub html: String,
}
