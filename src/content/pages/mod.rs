mod builder;
mod model;
mod routing;

pub use builder::build_pages;
pub use model::{Page, PageKind};

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use super::routing::derive_page_meta;
    use super::{PageKind, build_pages};

    #[test]
    fn derives_routes_for_supported_paths() {
        let index =
            derive_page_meta(Path::new("index.md"), None).expect("index route should parse");
        let about =
            derive_page_meta(Path::new("about.md"), None).expect("about route should parse");
        let nested = derive_page_meta(Path::new("notes/rust/tips.md"), None)
            .expect("nested page route should parse");
        let nested_index = derive_page_meta(Path::new("notes/index.md"), None)
            .expect("nested index route should parse");
        let blog = derive_page_meta(Path::new("blog/hello-rust.md"), None)
            .expect("blog route should parse");
        let project = derive_page_meta(Path::new("projects/solar-map.md"), None)
            .expect("project route should parse");

        assert_eq!(index.route, "/");
        assert_eq!(index.kind, PageKind::Index);

        assert_eq!(about.route, "/about/");
        assert_eq!(about.kind, PageKind::Page);

        assert_eq!(nested.route, "/notes/rust/tips/");
        assert_eq!(nested.kind, PageKind::Page);

        assert_eq!(nested_index.route, "/notes/");
        assert_eq!(nested_index.kind, PageKind::Page);

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
    fn frontmatter_slug_overrides_nested_leaf_filename() {
        let page = derive_page_meta(Path::new("notes/rust/tips.md"), Some("Quick Start"))
            .expect("nested page meta");
        assert_eq!(page.slug, "quick-start");
        assert_eq!(page.route, "/notes/rust/quick-start/");
    }

    #[test]
    fn nested_index_ignores_frontmatter_slug_for_route() {
        let page = derive_page_meta(Path::new("notes/rust/index.md"), Some("Quick Start"))
            .expect("nested index meta");
        assert_eq!(page.slug, "quick-start");
        assert_eq!(page.route, "/notes/rust/");
    }

    #[test]
    fn errors_when_slug_normalizes_to_empty() {
        match derive_page_meta(Path::new("blog/hello.md"), Some("!!!")) {
            Ok(_) => panic!("slug should be rejected"),
            Err(error) => {
                assert!(
                    error
                        .to_string()
                        .contains("slug must contain at least one ASCII letter or digit"),
                    "unexpected error: {error}"
                );
            }
        }
    }

    #[test]
    fn rejects_nested_blog_and_project_paths() {
        for (path, expected) in [
            ("blog/rust/tips.md", "unsupported nested blog content path"),
            (
                "projects/tools/app.md",
                "unsupported nested project content path",
            ),
        ] {
            match derive_page_meta(Path::new(path), None) {
                Ok(_) => panic!("path should be rejected: {path}"),
                Err(error) => {
                    assert!(
                        error.to_string().contains(expected),
                        "unexpected error for {path}: {error}"
                    );
                }
            }
        }
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

    #[test]
    fn builds_nested_custom_pages() {
        let dir = tempdir().expect("tempdir should be created");
        let content_dir = dir.path().join("content");
        fs::create_dir_all(content_dir.join("notes/rust"))
            .expect("nested notes dir should be created");

        fs::write(
            content_dir.join("notes/index.md"),
            "---\ntitle: Notes Home\nslug: ignored\n---\n\n# Notes",
        )
        .expect("nested index should be written");
        fs::write(
            content_dir.join("notes/rust/tips.md"),
            "---\ntitle: Tips\nslug: Quick Start\n---\n\n# Tips",
        )
        .expect("nested page should be written");

        let pages = build_pages(&content_dir).expect("build_pages should succeed");
        assert_eq!(pages.len(), 2);
        assert!(pages.iter().any(|page| page.route == "/notes/"));
        assert!(
            pages
                .iter()
                .any(|page| page.route == "/notes/rust/quick-start/")
        );
        assert!(pages.iter().all(|page| page.kind == PageKind::Page));
    }

    #[test]
    fn builds_pages_with_mermaid_flag() {
        let dir = tempdir().expect("tempdir should be created");
        let content_dir = dir.path().join("content");
        fs::create_dir_all(&content_dir).expect("content dir should be created");

        fs::write(
            content_dir.join("diagram.md"),
            "---\ntitle: Diagram\n---\n\n```mermaid\ngraph TD\n  A --> B\n```",
        )
        .expect("diagram page should be written");
        fs::write(content_dir.join("plain.md"), "# Plain").expect("plain page should be written");

        let pages = build_pages(&content_dir).expect("build_pages should succeed");
        let diagram = pages
            .iter()
            .find(|page| page.route == "/diagram/")
            .expect("diagram page should exist");
        let plain = pages
            .iter()
            .find(|page| page.route == "/plain/")
            .expect("plain page should exist");

        assert!(diagram.has_mermaid);
        assert!(!plain.has_mermaid);
    }
}
