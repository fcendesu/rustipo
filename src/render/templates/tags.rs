use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::BTreeMap;
use tera::{Context as TeraContext, Tera};

use crate::content::pages::{Page, PageKind};

use super::{CommonRenderContext, RenderEnvironment, RenderedPage};

#[derive(Clone, Serialize)]
struct SectionItem {
    title: String,
    route: String,
    summary: Option<String>,
    date: Option<String>,
}

pub(super) fn render_tag_pages(
    tera: &Tera,
    pages: &[Page],
    env: &RenderEnvironment<'_>,
) -> Result<Vec<RenderedPage>> {
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
            route: env.config.public_url_path(&page.route),
            summary: page.frontmatter.summary.clone(),
            date: page.frontmatter.date.as_ref().map(ToString::to_string),
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
        let route = format!("/tags/{tag_slug}/");
        let mut context = TeraContext::new();
        context.insert("route", &env.config.public_url_path(&route));
        context.insert("section_name", "tags");
        context.insert("section_title", &format!("Tag: {tag_slug}"));
        context.insert("items", &items);
        let render_context = CommonRenderContext {
            shared: env.shared,
            route: &route,
            page_kind: "section",
            current_section: "tags",
            site: env.site,
        };
        super::insert_common_site_context(&mut context, env.config, &render_context);
        context.insert(
            "page_title",
            &format!("Tag: {tag_slug} | {}", env.config.title),
        );
        context.insert(
            "page_description",
            &super::resolved_page_description(None, env.config),
        );
        context.insert("content_html", "");
        context.insert("page_has_mermaid", &false);
        context.insert("page_has_math", &false);

        let html = tera
            .render("section.html", &context)
            .with_context(|| format!("failed to render tag section template for '{tag_slug}'"))?;

        rendered.push(RenderedPage { route, html });
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
