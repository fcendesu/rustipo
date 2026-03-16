use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

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

pub fn build_pages(content_dir: impl AsRef<Path>) -> Result<Vec<Page>> {
    let content_dir = content_dir.as_ref();
    let markdown_files = crate::content::loader::discover_markdown_files(content_dir)?;

    let mut pages = Vec::new();

    for file in markdown_files {
        let raw = fs::read_to_string(&file)
            .with_context(|| format!("failed to read markdown file: {}", file.display()))?;

        let parsed = crate::content::frontmatter::parse(&raw)
            .with_context(|| format!("failed to parse markdown file: {}", file.display()))?;

        if parsed.frontmatter.draft == Some(true) {
            continue;
        }

        let rel_path = file
            .strip_prefix(content_dir)
            .with_context(|| {
                format!(
                    "failed to compute content-relative path for: {}",
                    file.display()
                )
            })?
            .to_path_buf();

        let page_meta = derive_page_meta(&rel_path, parsed.frontmatter.slug.as_deref())?;
        let html = crate::content::markdown::render_html(&parsed.content);

        pages.push(Page {
            source_path: file,
            route: page_meta.route,
            slug: page_meta.slug,
            kind: page_meta.kind,
            frontmatter: parsed.frontmatter,
            markdown: parsed.content,
            html,
        });
    }

    Ok(pages)
}

struct PageMeta {
    route: String,
    slug: String,
    kind: PageKind,
}

fn derive_page_meta(rel_path: &Path, frontmatter_slug: Option<&str>) -> Result<PageMeta> {
    let raw_stem = rel_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .context("invalid UTF-8 filename")?;

    let preferred_slug = frontmatter_slug.unwrap_or(raw_stem);
    let slug = normalize_slug(preferred_slug);

    let components = rel_path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    match components.as_slice() {
        [name] if name == "index.md" => Ok(PageMeta {
            route: "/".to_string(),
            slug,
            kind: PageKind::Index,
        }),
        [name] if name.ends_with(".md") => Ok(PageMeta {
            route: format!("/{slug}/"),
            slug,
            kind: PageKind::Page,
        }),
        [section, name] if section == "blog" && name.ends_with(".md") => Ok(PageMeta {
            route: format!("/blog/{slug}/"),
            slug,
            kind: PageKind::BlogPost,
        }),
        [section, name] if section == "projects" && name.ends_with(".md") => Ok(PageMeta {
            route: format!("/projects/{slug}/"),
            slug,
            kind: PageKind::Project,
        }),
        _ => bail!("unsupported content path structure: {}", rel_path.display()),
    }
}

fn normalize_slug(input: &str) -> String {
    let mut slug = String::with_capacity(input.len());
    let mut previous_dash = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            slug.push('-');
            previous_dash = true;
        }
    }

    slug.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{PageKind, build_pages, derive_page_meta};

    #[test]
    fn derives_routes_for_supported_paths() {
        let index =
            derive_page_meta(Path::new("index.md"), None).expect("index route should parse");
        let about =
            derive_page_meta(Path::new("about.md"), None).expect("about route should parse");
        let blog = derive_page_meta(Path::new("blog/hello-rust.md"), None)
            .expect("blog route should parse");
        let project = derive_page_meta(Path::new("projects/solar-map.md"), None)
            .expect("project route should parse");

        assert_eq!(index.route, "/");
        assert_eq!(index.kind, PageKind::Index);

        assert_eq!(about.route, "/about/");
        assert_eq!(about.kind, PageKind::Page);

        assert_eq!(blog.route, "/blog/hello-rust/");
        assert_eq!(blog.kind, PageKind::BlogPost);

        assert_eq!(project.route, "/projects/solar-map/");
        assert_eq!(project.kind, PageKind::Project);
    }

    #[test]
    fn frontmatter_slug_overrides_filename() {
        let page =
            derive_page_meta(Path::new("blog/hello.md"), Some("My Custom Slug")).expect("meta");
        assert_eq!(page.slug, "my-custom-slug");
        assert_eq!(page.route, "/blog/my-custom-slug/");
    }

    #[test]
    fn excludes_draft_pages() {
        let dir = tempdir().expect("tempdir should be created");
        let content_dir = dir.path().join("content");
        fs::create_dir_all(content_dir.join("blog")).expect("blog dir should be created");

        fs::write(content_dir.join("index.md"), "# Home").expect("index should be written");
        fs::write(
            content_dir.join("blog/draft.md"),
            "---\ndraft: true\n---\n\n# Draft",
        )
        .expect("draft should be written");

        let pages = build_pages(&content_dir).expect("build_pages should succeed");
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].route, "/");
    }

    #[test]
    fn builds_page_with_frontmatter_and_html() {
        let dir = tempdir().expect("tempdir should be created");
        let content_dir = dir.path().join("content");
        fs::create_dir_all(content_dir.join("blog")).expect("blog dir should be created");

        fs::write(
            content_dir.join("blog/post.md"),
            "---\ntitle: Post\nsummary: Example\n---\n\n# Hello",
        )
        .expect("post should be written");

        let pages = build_pages(&content_dir).expect("build_pages should succeed");
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].kind, PageKind::BlogPost);
        assert_eq!(pages[0].route, "/blog/post/");
        assert_eq!(pages[0].frontmatter.title.as_deref(), Some("Post"));
        assert!(pages[0].html.contains("<h1>Hello</h1>"));
    }

    use std::path::Path;
}
