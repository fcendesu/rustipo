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
#[allow(dead_code)]
pub struct Page {
    pub source_path: PathBuf,
    pub route: String,
    pub slug: String,
    pub kind: PageKind,
    pub frontmatter: Frontmatter,
    pub markdown: String,
    pub html: String,
}
