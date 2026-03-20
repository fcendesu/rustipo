use anyhow::{Context, Result};
use tera::{Context as TeraContext, Tera};

use crate::content::pages::{Page, PageKind};

use super::{CommonRenderContext, RenderEnvironment, RenderedPage};

const MERMAID_SNIPPET_MARKER: &str = "data-rustipo-mermaid";
const MERMAID_SNIPPET: &str = r#"<script type="module" data-rustipo-mermaid>
import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs";
mermaid.initialize({ startOnLoad: false });
await mermaid.run({ querySelector: ".mermaid" });
</script>"#;

pub(super) fn render_content_pages(
    tera: &Tera,
    pages: &[Page],
    env: &RenderEnvironment<'_>,
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
        context.insert("page_has_mermaid", &page.has_mermaid);
        let render_context = CommonRenderContext {
            shared: env.shared,
            route: &page.route,
            page_kind: page_kind_name(page.kind),
            current_section: current_section_name(page.kind),
            favicon_links: env.favicon_links,
            site_style: env.site_style,
            site_has_custom_css: env.site_has_custom_css,
            palette: env.palette,
        };
        super::insert_common_site_context(&mut context, env.config, &render_context);
        context.insert(
            "page_title",
            &page
                .frontmatter
                .title
                .clone()
                .unwrap_or_else(|| env.config.title.clone()),
        );

        let html = tera.render(template, &context).with_context(|| {
            format!(
                "failed to render template '{template}' for route '{}'",
                page.route
            )
        })?;
        let html = inject_mermaid_runtime(html, page.has_mermaid);

        rendered.push(RenderedPage {
            route: page.route.clone(),
            html,
        });
    }

    Ok(rendered)
}

fn inject_mermaid_runtime(html: String, page_has_mermaid: bool) -> String {
    if !page_has_mermaid || html.contains(MERMAID_SNIPPET_MARKER) {
        return html;
    }

    if let Some(pos) = html.rfind("</body>") {
        let mut output = html;
        output.insert_str(pos, MERMAID_SNIPPET);
        output
    } else {
        let mut output = html;
        output.push_str(MERMAID_SNIPPET);
        output
    }
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
