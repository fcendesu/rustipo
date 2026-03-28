use anyhow::{Context, Result};
use serde::Serialize;
use tera::{Context as TeraContext, Tera};

use crate::content::frontmatter::Frontmatter;

use super::{CommonRenderContext, RenderEnvironment};

const NOT_FOUND_ROUTE: &str = "/404/";
const NOT_FOUND_CONTENT_HTML: &str = concat!(
    "<h1>Page not found</h1>",
    "<p>The page you requested could not be found.</p>",
    "<p><a href=\"/\">Return home</a></p>"
);

#[derive(Serialize)]
struct BreadcrumbItem {
    title: String,
    route: String,
    active: bool,
    linkable: bool,
}

pub(super) fn render_not_found_page(tera: &Tera, env: &RenderEnvironment<'_>) -> Result<String> {
    let template = if tera.get_template("404.html").is_ok() {
        "404.html"
    } else {
        "page.html"
    };

    let frontmatter = Frontmatter {
        title: Some("Page not found".to_string()),
        ..Frontmatter::default()
    };

    let mut context = TeraContext::new();
    let public_route = env.config.public_url_path(NOT_FOUND_ROUTE);
    let content_html = super::rewrite_public_html_urls(NOT_FOUND_CONTENT_HTML, env.config);
    context.insert("route", &public_route);
    context.insert("slug", "404");
    context.insert("content_html", &content_html);
    context.insert("frontmatter", &frontmatter);
    context.insert(
        "page_extra",
        &super::resolved_page_extra(frontmatter.extra.as_ref()),
    );
    context.insert("page_summary", &frontmatter.summary);
    context.insert(
        "page_description",
        &super::resolved_page_description(frontmatter.summary.as_deref(), env.config),
    );
    context.insert(
        "page_date",
        &frontmatter.date.as_ref().map(ToString::to_string),
    );
    context.insert("page_tags", &frontmatter.tags);
    context.insert(
        "page_taxonomies",
        &crate::taxonomy::page_taxonomies(&frontmatter, env.config),
    );
    context.insert("page_links", &frontmatter.links);
    context.insert("page_order", &frontmatter.order);
    context.insert("page_has_mermaid", &false);
    context.insert("page_has_math", &false);
    context.insert("page_toc", &Vec::<crate::content::toc::TocItem>::new());
    let render_context = CommonRenderContext {
        shared: env.shared,
        route: NOT_FOUND_ROUTE,
        page_kind: "page",
        current_section: "pages",
        site: env.site,
    };
    super::insert_common_site_context(&mut context, env.config, &render_context);
    context.insert(
        "page_title",
        &format!("Page not found | {}", env.config.title),
    );
    context.insert(
        "breadcrumbs",
        &vec![
            BreadcrumbItem {
                title: "Home".to_string(),
                route: env.config.public_url_path("/"),
                active: false,
                linkable: true,
            },
            BreadcrumbItem {
                title: "Page not found".to_string(),
                route: public_route,
                active: true,
                linkable: false,
            },
        ],
    );

    tera.render(template, &context)
        .with_context(|| format!("failed to render not-found template using '{template}'"))
}
