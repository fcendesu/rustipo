use anyhow::{Context, Result};
use tera::{Context as TeraContext, Tera};

use crate::config::{FaviconLinks, SiteConfig, SiteStyleOptions};
use crate::content::pages::{Page, PageKind};

use super::RenderedPage;
use super::context::SharedTemplateData;

pub(super) fn render_content_pages(
    tera: &Tera,
    config: &SiteConfig,
    pages: &[Page],
    shared: &SharedTemplateData,
    favicon_links: &FaviconLinks,
    site_style: &SiteStyleOptions,
    site_has_custom_css: bool,
) -> Result<Vec<RenderedPage>> {
    let mut rendered = Vec::with_capacity(pages.len());

    for page in pages {
        let template = template_for_kind(page.kind);
        let mut context = TeraContext::new();

        context.insert("route", &page.route);
        context.insert("slug", &page.slug);
        context.insert("content_html", &page.html);
        context.insert("frontmatter", &page.frontmatter);
        context.insert("page_summary", &page.frontmatter.summary);
        context.insert(
            "page_date",
            &page.frontmatter.date.as_ref().map(ToString::to_string),
        );
        context.insert("page_tags", &page.frontmatter.tags);
        context.insert("page_links", &page.frontmatter.links);
        context.insert("page_order", &page.frontmatter.order);
        super::insert_common_site_context(
            &mut context,
            config,
            shared,
            &page.route,
            page_kind_name(page.kind),
            current_section_name(page.kind),
            favicon_links,
            site_style,
            site_has_custom_css,
        );
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

fn page_kind_name(kind: PageKind) -> &'static str {
    match kind {
        PageKind::Index => "index",
        PageKind::Page => "page",
        PageKind::BlogPost => "post",
        PageKind::Project => "project",
    }
}

fn current_section_name(kind: PageKind) -> &'static str {
    match kind {
        PageKind::Index => "home",
        PageKind::Page => "pages",
        PageKind::BlogPost => "blog",
        PageKind::Project => "projects",
    }
}
