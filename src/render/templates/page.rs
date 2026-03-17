use anyhow::{Context, Result};
use tera::{Context as TeraContext, Tera};

use crate::config::SiteConfig;
use crate::content::pages::{Page, PageKind};

use super::RenderedPage;

pub(super) fn render_content_pages(
    tera: &Tera,
    config: &SiteConfig,
    pages: &[Page],
) -> Result<Vec<RenderedPage>> {
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
