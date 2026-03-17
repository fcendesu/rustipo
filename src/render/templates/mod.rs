use anyhow::{Context, Result};
use tera::Tera;

use crate::config::SiteConfig;
use crate::content::pages::Page;
use crate::theme::models::Theme;

mod page;
mod section;
mod tags;

#[derive(Debug)]
#[allow(dead_code)]
pub struct RenderedPage {
    pub route: String,
    pub html: String,
}

pub fn render_pages(
    theme: &Theme,
    config: &SiteConfig,
    pages: &[Page],
) -> Result<Vec<RenderedPage>> {
    let glob = format!("{}/**/*.html", theme.templates_dir.display());
    let tera = Tera::new(&glob).with_context(|| {
        format!(
            "failed to load templates from directory: {}",
            theme.templates_dir.display()
        )
    })?;

    let mut rendered = page::render_content_pages(&tera, config, pages)?;
    rendered.extend(section::render_sections(&tera, config, pages)?);
    rendered.extend(tags::render_tag_pages(&tera, config, pages)?);

    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::config::SiteConfig;
    use crate::content::pages::build_pages;
    use crate::theme::loader::load_active_theme;

    use super::render_pages;

    #[test]
    fn renders_pages_with_theme_templates() {
        let dir = tempdir().expect("tempdir should be created");
        let project_root = dir.path();

        fs::create_dir_all(project_root.join("content/blog"))
            .expect("content dir should be created");
        fs::write(project_root.join("content/index.md"), "# Welcome")
            .expect("index should be written");
        fs::write(project_root.join("content/blog/post.md"), "# Post")
            .expect("post should be written");
        fs::write(
            project_root.join("content/blog/post-with-tags.md"),
            "---\ntitle: Tagged\ntags: [\"Rust\", \"Site Gen\"]\n---\n\n# Tagged",
        )
        .expect("tagged post should be written");

        let theme_root = project_root.join("themes/default");
        fs::create_dir_all(theme_root.join("templates")).expect("templates should be created");
        fs::create_dir_all(theme_root.join("static")).expect("static should be created");

        fs::write(
            theme_root.join("templates/base.html"),
            "{% block body %}{% endblock body %}",
        )
        .expect("base template should be written");
        for template in [
            "index.html",
            "page.html",
            "post.html",
            "project.html",
            "section.html",
        ] {
            fs::write(
                theme_root.join("templates").join(template),
                "{% extends \"base.html\" %}{% block body %}<h1>{{ page_title }}</h1>{{ content_html | safe }}{% endblock body %}",
            )
            .expect("template should be written");
        }
        fs::write(
            theme_root.join("theme.toml"),
            "name = \"default\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Default\"\n",
        )
        .expect("theme metadata should be written");

        let config = SiteConfig {
            title: "My Site".to_string(),
            base_url: "https://example.com".to_string(),
            theme: "default".to_string(),
            description: "A portfolio".to_string(),
            author: None,
            site: None,
        };

        let pages = build_pages(project_root.join("content")).expect("pages should build");
        let theme = load_active_theme(project_root, "default").expect("theme should load");

        let rendered = render_pages(&theme, &config, &pages).expect("pages should render");
        assert_eq!(rendered.len(), 7);
        assert!(rendered.iter().any(|p| p.route == "/"));
        assert!(rendered.iter().any(|p| p.route == "/blog/post/"));
        assert!(rendered.iter().any(|p| p.route == "/blog/post-with-tags/"));
        assert!(rendered.iter().any(|p| p.route == "/blog/"));
        assert!(rendered.iter().any(|p| p.route == "/projects/"));
        assert!(rendered.iter().any(|p| p.route == "/tags/rust/"));
        assert!(rendered.iter().any(|p| p.route == "/tags/site-gen/"));
        assert!(rendered.iter().all(|p| p.html.contains("<h1>")));
    }

    #[test]
    fn paginates_blog_section_when_posts_exceed_page_size() {
        let dir = tempdir().expect("tempdir should be created");
        let project_root = dir.path();

        fs::create_dir_all(project_root.join("content/blog"))
            .expect("content dir should be created");
        fs::write(project_root.join("content/index.md"), "# Welcome")
            .expect("index should be written");
        fs::write(project_root.join("content/blog/post-1.md"), "# Post 1")
            .expect("post 1 should be written");
        fs::write(project_root.join("content/blog/post-2.md"), "# Post 2")
            .expect("post 2 should be written");
        fs::write(project_root.join("content/blog/post-3.md"), "# Post 3")
            .expect("post 3 should be written");

        let theme_root = project_root.join("themes/default");
        fs::create_dir_all(theme_root.join("templates")).expect("templates should be created");
        fs::create_dir_all(theme_root.join("static")).expect("static should be created");

        fs::write(
            theme_root.join("templates/base.html"),
            "{% block body %}{% endblock body %}",
        )
        .expect("base template should be written");
        for template in ["index.html", "page.html", "post.html", "project.html"] {
            fs::write(
                theme_root.join("templates").join(template),
                "{% extends \"base.html\" %}{% block body %}<h1>{{ page_title }}</h1>{{ content_html | safe }}{% endblock body %}",
            )
            .expect("template should be written");
        }
        fs::write(
            theme_root.join("templates/section.html"),
            "{% extends \"base.html\" %}{% block body %}<h1>{{ page_title }}</h1>{% for i in items %}<a href=\"{{ i.route }}\">{{ i.title }}</a>{% endfor %}{% if next_url %}<a href=\"{{ next_url }}\">Next</a>{% endif %}{% endblock body %}",
        )
        .expect("section template should be written");
        fs::write(
            theme_root.join("theme.toml"),
            "name = \"default\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Default\"\n",
        )
        .expect("theme metadata should be written");

        let config = SiteConfig {
            title: "My Site".to_string(),
            base_url: "https://example.com".to_string(),
            theme: "default".to_string(),
            description: "A portfolio".to_string(),
            author: None,
            site: Some(crate::config::SiteOptions {
                posts_per_page: Some(2),
            }),
        };

        let pages = build_pages(project_root.join("content")).expect("pages should build");
        let theme = load_active_theme(project_root, "default").expect("theme should load");

        let rendered = render_pages(&theme, &config, &pages).expect("pages should render");
        assert!(rendered.iter().any(|p| p.route == "/blog/"));
        assert!(rendered.iter().any(|p| p.route == "/blog/page/2/"));
    }
}
