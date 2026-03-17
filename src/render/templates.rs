use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::BTreeMap;
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
    rendered.extend(render_tag_pages(&tera, config, pages)?);

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

#[derive(Clone, Serialize)]
struct SectionItem {
    title: String,
    route: String,
    summary: Option<String>,
    date: Option<String>,
}

fn render_sections(tera: &Tera, config: &SiteConfig, pages: &[Page]) -> Result<Vec<RenderedPage>> {
    let mut rendered = Vec::new();
    rendered.extend(render_blog_section_pages(tera, config, pages)?);
    rendered.push(render_projects_section_page(tera, config, pages)?);

    Ok(rendered)
}

fn render_blog_section_pages(
    tera: &Tera,
    config: &SiteConfig,
    pages: &[Page],
) -> Result<Vec<RenderedPage>> {
    let items = pages
        .iter()
        .filter(|page| page.kind == PageKind::BlogPost)
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

    let per_page = config.posts_per_page();
    let total_pages = usize::max(1, items.len().div_ceil(per_page));
    let mut rendered = Vec::with_capacity(total_pages);

    for page_idx in 0..total_pages {
        let start = page_idx * per_page;
        let end = usize::min(start + per_page, items.len());
        let paged_items = if start >= items.len() {
            Vec::new()
        } else {
            items[start..end].to_vec()
        };

        let page_number = page_idx + 1;
        let route = if page_number == 1 {
            "/blog/".to_string()
        } else {
            format!("/blog/page/{page_number}/")
        };
        let prev_url = if page_number <= 1 {
            None
        } else if page_number == 2 {
            Some("/blog/".to_string())
        } else {
            Some(format!("/blog/page/{}/", page_number - 1))
        };
        let next_url = if page_number < total_pages {
            Some(format!("/blog/page/{}/", page_number + 1))
        } else {
            None
        };

        let mut context = TeraContext::new();
        context.insert("route", &route);
        context.insert("section_name", "blog");
        context.insert("section_title", "Blog");
        context.insert("items", &paged_items);
        context.insert("site_title", &config.title);
        context.insert("site_description", &config.description);
        context.insert("page_title", &format!("Blog | {}", config.title));
        context.insert("content_html", "");
        context.insert("current_page", &page_number);
        context.insert("total_pages", &total_pages);
        context.insert("prev_url", &prev_url);
        context.insert("next_url", &next_url);

        let html = tera.render("section.html", &context).with_context(|| {
            format!("failed to render section template for 'blog' page {page_number}")
        })?;

        rendered.push(RenderedPage { route, html });
    }

    Ok(rendered)
}

fn render_projects_section_page(
    tera: &Tera,
    config: &SiteConfig,
    pages: &[Page],
) -> Result<RenderedPage> {
    let items = pages
        .iter()
        .filter(|page| page.kind == PageKind::Project)
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
    context.insert("route", "/projects/");
    context.insert("section_name", "projects");
    context.insert("section_title", "Projects");
    context.insert("items", &items);
    context.insert("site_title", &config.title);
    context.insert("site_description", &config.description);
    context.insert("page_title", &format!("Projects | {}", config.title));
    context.insert("content_html", "");
    context.insert("current_page", &1usize);
    context.insert("total_pages", &1usize);
    context.insert("prev_url", &Option::<String>::None);
    context.insert("next_url", &Option::<String>::None);

    let html = tera
        .render("section.html", &context)
        .with_context(|| "failed to render section template for 'projects'".to_string())?;

    Ok(RenderedPage {
        route: "/projects/".to_string(),
        html,
    })
}

fn render_tag_pages(tera: &Tera, config: &SiteConfig, pages: &[Page]) -> Result<Vec<RenderedPage>> {
    let mut tags: BTreeMap<String, Vec<SectionItem>> = BTreeMap::new();

    for page in pages.iter().filter(|page| page.kind == PageKind::BlogPost) {
        let Some(page_tags) = page.frontmatter.tags.as_ref() else {
            continue;
        };

        let title = page
            .frontmatter
            .title
            .clone()
            .unwrap_or_else(|| page.slug.clone());
        let item = SectionItem {
            title,
            route: page.route.clone(),
            summary: page.frontmatter.summary.clone(),
            date: page.frontmatter.date.clone(),
        };

        for tag in page_tags {
            let tag_slug = normalize_tag_slug(tag);
            if tag_slug.is_empty() {
                continue;
            }
            tags.entry(tag_slug).or_default().push(SectionItem {
                title: item.title.clone(),
                route: item.route.clone(),
                summary: item.summary.clone(),
                date: item.date.clone(),
            });
        }
    }

    let mut rendered = Vec::new();
    for (tag_slug, items) in tags {
        let mut context = TeraContext::new();
        context.insert("route", &format!("/tags/{tag_slug}/"));
        context.insert("section_name", "tags");
        context.insert("section_title", &format!("Tag: {tag_slug}"));
        context.insert("items", &items);
        context.insert("site_title", &config.title);
        context.insert("site_description", &config.description);
        context.insert("page_title", &format!("Tag: {tag_slug} | {}", config.title));
        context.insert("content_html", "");

        let html = tera
            .render("section.html", &context)
            .with_context(|| format!("failed to render tag section template for '{tag_slug}'"))?;

        rendered.push(RenderedPage {
            route: format!("/tags/{tag_slug}/"),
            html,
        });
    }

    Ok(rendered)
}

fn normalize_tag_slug(input: &str) -> String {
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
