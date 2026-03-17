use anyhow::{Context, Result};
use serde::Serialize;
use tera::{Context as TeraContext, Tera};

use crate::config::{FaviconLinks, SiteConfig};
use crate::content::pages::{Page, PageKind};

use super::RenderedPage;

#[derive(Clone, Serialize)]
struct SectionItem {
    title: String,
    route: String,
    summary: Option<String>,
    date: Option<String>,
}

pub(super) fn render_sections(
    tera: &Tera,
    config: &SiteConfig,
    pages: &[Page],
    favicon_links: &FaviconLinks,
) -> Result<Vec<RenderedPage>> {
    let mut rendered = Vec::new();
    rendered.extend(render_blog_section_pages(
        tera,
        config,
        pages,
        favicon_links,
    )?);
    rendered.push(render_projects_section_page(
        tera,
        config,
        pages,
        favicon_links,
    )?);

    Ok(rendered)
}

fn render_blog_section_pages(
    tera: &Tera,
    config: &SiteConfig,
    pages: &[Page],
    favicon_links: &FaviconLinks,
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
            date: page.frontmatter.date.as_ref().map(ToString::to_string),
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
        context.insert("site_favicon", &favicon_links.icon_href);
        context.insert("site_favicon_svg", &favicon_links.svg_href);
        context.insert("site_favicon_ico", &favicon_links.ico_href);
        context.insert(
            "site_apple_touch_icon",
            &favicon_links.apple_touch_icon_href,
        );
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
    favicon_links: &FaviconLinks,
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
            date: page.frontmatter.date.as_ref().map(ToString::to_string),
        })
        .collect::<Vec<_>>();

    let mut context = TeraContext::new();
    context.insert("route", "/projects/");
    context.insert("section_name", "projects");
    context.insert("section_title", "Projects");
    context.insert("items", &items);
    context.insert("site_title", &config.title);
    context.insert("site_description", &config.description);
    context.insert("site_favicon", &favicon_links.icon_href);
    context.insert("site_favicon_svg", &favicon_links.svg_href);
    context.insert("site_favicon_ico", &favicon_links.ico_href);
    context.insert(
        "site_apple_touch_icon",
        &favicon_links.apple_touch_icon_href,
    );
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
