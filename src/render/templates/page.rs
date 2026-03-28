use anyhow::{Context, Result};
use tera::{Context as TeraContext, Tera};

use crate::content::pages::{Page, PageKind};
use crate::content::shortcodes::ShortcodeAssets;

use super::{CommonRenderContext, RenderEnvironment, RenderedPage};

const MERMAID_SNIPPET_MARKER: &str = "data-rustipo-mermaid";
const MERMAID_SNIPPET: &str = r#"<script type="module" data-rustipo-mermaid>
import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs";
mermaid.initialize({ startOnLoad: false });
await mermaid.run({ querySelector: ".mermaid" });
</script>"#;
const MATH_SNIPPET_MARKER: &str = "data-rustipo-math";
const MATH_STYLESHEET: &str = r#"<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.38/dist/katex.min.css" data-rustipo-math>"#;
const MATH_SNIPPET: &str = r#"<script type="module" data-rustipo-math>
import katex from "https://cdn.jsdelivr.net/npm/katex@0.16.38/dist/katex.mjs";

for (const node of document.querySelectorAll(".math-inline, .math-display")) {
  katex.render(node.textContent ?? "", node, {
    displayMode: node.classList.contains("math-display"),
    throwOnError: false,
    output: "htmlAndMathml"
  });
}
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
        let public_route = env.config.public_url_path(&page.route);
        let content_html = super::rewrite_public_html_urls(&page.html, env.config);

        context.insert("route", &public_route);
        context.insert("slug", &page.slug);
        context.insert("content_html", &content_html);
        context.insert("frontmatter", &page.frontmatter);
        context.insert(
            "page_extra",
            &super::resolved_page_extra(page.frontmatter.extra.as_ref()),
        );
        context.insert("page_summary", &page.frontmatter.summary);
        context.insert(
            "page_description",
            &super::resolved_page_description(page.frontmatter.summary.as_deref(), env.config),
        );
        context.insert(
            "page_date",
            &page.frontmatter.date.as_ref().map(ToString::to_string),
        );
        context.insert("page_tags", &page.frontmatter.tags);
        context.insert(
            "page_taxonomies",
            &crate::taxonomy::page_taxonomies(&page.frontmatter, env.config),
        );
        context.insert("page_links", &page.frontmatter.links);
        context.insert("page_order", &page.frontmatter.order);
        context.insert("page_has_mermaid", &page.has_mermaid);
        context.insert("page_has_math", &page.has_math);
        context.insert("page_toc", &page.toc);
        let render_context = CommonRenderContext {
            shared: env.shared,
            route: &page.route,
            page_kind: page_kind_name(page.kind),
            current_section: current_section_name(page.kind),
            site: env.site,
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
        let html = inject_shortcode_assets(html, &page.shortcode_assets, env.config);
        let html = inject_math_runtime(html, page.has_math);
        let html = inject_mermaid_runtime(html, page.has_mermaid);

        rendered.push(RenderedPage {
            route: page.route.clone(),
            html,
        });
    }

    Ok(rendered)
}

fn inject_shortcode_assets(
    html: String,
    assets: &ShortcodeAssets,
    config: &crate::config::SiteConfig,
) -> String {
    let mut html = html;

    for stylesheet in &assets.stylesheets {
        let stylesheet = public_shortcode_asset_path(stylesheet, config);
        let snippet = format!(
            "<link rel=\"stylesheet\" href=\"{}\" data-rustipo-shortcode-asset>",
            escape_html_attr(&stylesheet)
        );
        html = inject_before_head_end(html, &snippet);
    }

    for script in &assets.scripts {
        let script = public_shortcode_asset_path(script, config);
        let snippet = format!(
            "<script type=\"module\" src=\"{}\" data-rustipo-shortcode-asset></script>",
            escape_html_attr(&script)
        );
        html = inject_before_body_end(html, &snippet);
    }

    html
}

fn inject_mermaid_runtime(html: String, page_has_mermaid: bool) -> String {
    if !page_has_mermaid || html.contains(MERMAID_SNIPPET_MARKER) {
        return html;
    }

    inject_before_body_end(html, MERMAID_SNIPPET)
}

fn inject_math_runtime(html: String, page_has_math: bool) -> String {
    if !page_has_math || html.contains(MATH_SNIPPET_MARKER) {
        return html;
    }

    let html = inject_before_head_end(html, MATH_STYLESHEET);
    inject_before_body_end(html, MATH_SNIPPET)
}

fn inject_before_head_end(html: String, snippet: &str) -> String {
    if let Some(pos) = html.rfind("</head>") {
        let mut output = html;
        output.insert_str(pos, snippet);
        output
    } else {
        format!("{snippet}{html}")
    }
}

fn inject_before_body_end(html: String, snippet: &str) -> String {
    if let Some(pos) = html.rfind("</body>") {
        let mut output = html;
        output.insert_str(pos, snippet);
        output
    } else {
        let mut output = html;
        output.push_str(snippet);
        output
    }
}

fn public_shortcode_asset_path(path: &str, config: &crate::config::SiteConfig) -> String {
    if path.starts_with('/') {
        config.public_url_path(path)
    } else {
        path.to_string()
    }
}

fn escape_html_attr(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
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
