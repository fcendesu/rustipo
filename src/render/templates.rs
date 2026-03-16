use anyhow::{Context, Result};
use serde::Serialize;
use tera::{Context as TeraContext, Tera};

use crate::config::SiteConfig;
use crate::content::pages::{Page, PageKind};
use crate::theme::models::Theme;

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

    let mut rendered = Vec::with_capacity(pages.len());
    for page in pages {
        let template = template_for_kind(page.kind);
        let mut context = TeraContext::new();

        context.insert("route", &page.route);
        context.insert("slug", &page.slug);
        context.insert("content_html", &page.html);
        context.insert("site_title", &config.title);
        context.insert("site_description", &config.description);
        context.insert(
            "page_title",
            &page
                .frontmatter
                .title
                .clone()
                .unwrap_or_else(|| config.title.clone()),
        );

        let html = tera.render(template, &context).with_context(|| {
            format!(
                "failed to render template '{template}' for route '{}'",
                page.route
            )
        })?;

        rendered.push(RenderedPage {
            route: page.route.clone(),
            html,
        });
    }

    rendered.extend(render_sections(&tera, config, pages)?);

    Ok(rendered)
}

fn template_for_kind(kind: PageKind) -> &'static str {
    match kind {
        PageKind::Index => "index.html",
        PageKind::Page => "page.html",
        PageKind::BlogPost => "post.html",
        PageKind::Project => "project.html",
    }
}

#[derive(Serialize)]
struct SectionItem {
    title: String,
    route: String,
    summary: Option<String>,
    date: Option<String>,
}

fn render_sections(tera: &Tera, config: &SiteConfig, pages: &[Page]) -> Result<Vec<RenderedPage>> {
    let sections = [
        ("blog", "Blog", PageKind::BlogPost),
        ("projects", "Projects", PageKind::Project),
    ];

    let mut rendered = Vec::new();
    for (section_name, section_title, kind) in sections {
        let items = pages
            .iter()
            .filter(|page| page.kind == kind)
            .map(|page| SectionItem {
                title: page
                    .frontmatter
                    .title
                    .clone()
                    .unwrap_or_else(|| page.slug.clone()),
                route: page.route.clone(),
                summary: page.frontmatter.summary.clone(),
                date: page.frontmatter.date.clone(),
            })
            .collect::<Vec<_>>();

        let mut context = TeraContext::new();
        context.insert("route", &format!("/{section_name}/"));
        context.insert("section_name", &section_name);
        context.insert("section_title", &section_title);
        context.insert("items", &items);
        context.insert("site_title", &config.title);
        context.insert("site_description", &config.description);
        context.insert("page_title", &format!("{section_title} | {}", config.title));
        context.insert("content_html", "");

        let html = tera
            .render("section.html", &context)
            .with_context(|| format!("failed to render section template for '{section_name}'"))?;

        rendered.push(RenderedPage {
            route: format!("/{section_name}/"),
            html,
        });
    }

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
        };

        let pages = build_pages(project_root.join("content")).expect("pages should build");
        let theme = load_active_theme(project_root, "default").expect("theme should load");

        let rendered = render_pages(&theme, &config, &pages).expect("pages should render");
        assert_eq!(rendered.len(), 4);
        assert!(rendered.iter().any(|p| p.route == "/"));
        assert!(rendered.iter().any(|p| p.route == "/blog/post/"));
        assert!(rendered.iter().any(|p| p.route == "/blog/"));
        assert!(rendered.iter().any(|p| p.route == "/projects/"));
        assert!(rendered.iter().all(|p| p.html.contains("<h1>")));
    }
}
