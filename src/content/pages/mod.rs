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
}
