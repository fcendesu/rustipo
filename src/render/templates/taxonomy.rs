use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::BTreeMap;
use tera::{Context as TeraContext, Tera};

use crate::content::pages::{Page, PageKind};
use crate::taxonomy::{
    TAGS_TAXONOMY, TaxonomyValue, frontmatter_tags, taxonomy_route, taxonomy_term_route,
    taxonomy_title,
};

use super::{CommonRenderContext, RenderEnvironment, RenderedPage};

#[derive(Clone, Serialize)]
struct SectionItem {
    title: String,
    route: String,
    summary: Option<String>,
    date: Option<String>,
}

#[derive(Clone, Serialize)]
struct TaxonomyBucket {
    term: TaxonomyValue,
    items: Vec<SectionItem>,
}

#[derive(Clone, Serialize)]
struct TaxonomyTermSummary {
    name: String,
    slug: String,
    route: String,
    count: usize,
}

pub(super) fn render_taxonomy_pages(
    tera: &Tera,
    pages: &[Page],
    env: &RenderEnvironment<'_>,
) -> Result<Vec<RenderedPage>> {
    let tag_title = taxonomy_title(TAGS_TAXONOMY).expect("built-in taxonomy should exist");
    let taxonomy_route =
        taxonomy_route(TAGS_TAXONOMY).expect("built-in taxonomy route should exist");
    let mut buckets: BTreeMap<String, TaxonomyBucket> = BTreeMap::new();

    for page in pages.iter().filter(|page| page.kind == PageKind::BlogPost) {
        let tags = frontmatter_tags(&page.frontmatter);
        if tags.is_empty() {
            continue;
        }

        let item = SectionItem {
            title: page
                .frontmatter
                .title
                .clone()
                .unwrap_or_else(|| page.slug.clone()),
            route: env.config.public_url_path(&page.route),
            summary: page.frontmatter.summary.clone(),
            date: page.frontmatter.date.as_ref().map(ToString::to_string),
        };

        for tag in tags {
            buckets
                .entry(tag.slug.clone())
                .and_modify(|bucket| bucket.items.push(item.clone()))
                .or_insert_with(|| TaxonomyBucket {
                    term: TaxonomyValue {
                        route: env.config.public_url_path(&tag.route),
                        ..tag
                    },
                    items: vec![item.clone()],
                });
        }
    }

    if buckets.is_empty() {
        return Ok(Vec::new());
    }

    let taxonomy_terms = buckets
        .values()
        .map(|bucket| TaxonomyTermSummary {
            name: bucket.term.name.clone(),
            slug: bucket.term.slug.clone(),
            route: bucket.term.route.clone(),
            count: bucket.items.len(),
        })
        .collect::<Vec<_>>();

    let mut rendered = Vec::new();
    rendered.push(render_taxonomy_index_page(
        tera,
        env,
        &taxonomy_route,
        tag_title,
        &taxonomy_terms,
    )?);

    for bucket in buckets.into_values() {
        rendered.push(render_taxonomy_term_page(
            tera,
            env,
            tag_title,
            &taxonomy_terms,
            bucket,
        )?);
    }

    Ok(rendered)
}

fn render_taxonomy_index_page(
    tera: &Tera,
    env: &RenderEnvironment<'_>,
    route: &str,
    taxonomy_title: &str,
    taxonomy_terms: &[TaxonomyTermSummary],
) -> Result<RenderedPage> {
    let mut context = TeraContext::new();
    context.insert("route", &env.config.public_url_path(route));
    context.insert("section_name", TAGS_TAXONOMY);
    context.insert("section_title", taxonomy_title);
    context.insert("taxonomy_name", TAGS_TAXONOMY);
    context.insert("taxonomy_title", taxonomy_title);
    context.insert("taxonomy_terms", taxonomy_terms);
    context.insert("items", &Vec::<SectionItem>::new());
    let render_context = CommonRenderContext {
        shared: env.shared,
        route,
        page_kind: "section",
        current_section: "tags",
        site: env.site,
    };
    super::insert_common_site_context(&mut context, env.config, &render_context);
    context.insert(
        "page_title",
        &format!("{taxonomy_title} | {}", env.config.title),
    );
    context.insert("page_extra", &super::resolved_page_extra(None));
    context.insert(
        "page_description",
        &super::resolved_page_description(None, env.config),
    );
    context.insert("content_html", "");
    context.insert("page_has_mermaid", &false);
    context.insert("page_has_math", &false);
    context.insert("current_page", &1usize);
    context.insert("total_pages", &1usize);
    context.insert("prev_url", &Option::<String>::None);
    context.insert("next_url", &Option::<String>::None);

    let html = tera
        .render("section.html", &context)
        .with_context(|| format!("failed to render taxonomy section template for '{route}'"))?;

    Ok(RenderedPage {
        route: route.to_string(),
        html,
    })
}

fn render_taxonomy_term_page(
    tera: &Tera,
    env: &RenderEnvironment<'_>,
    taxonomy_title: &str,
    taxonomy_terms: &[TaxonomyTermSummary],
    mut bucket: TaxonomyBucket,
) -> Result<RenderedPage> {
    bucket
        .items
        .sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.title.cmp(&b.title)));

    let route = taxonomy_term_route(TAGS_TAXONOMY, &bucket.term.slug)
        .expect("built-in taxonomy term route should exist");
    let mut context = TeraContext::new();
    context.insert("route", &env.config.public_url_path(&route));
    context.insert("section_name", TAGS_TAXONOMY);
    context.insert("section_title", &format!("Tag: {}", bucket.term.name));
    context.insert("taxonomy_name", TAGS_TAXONOMY);
    context.insert("taxonomy_title", taxonomy_title);
    context.insert("taxonomy_term", &bucket.term);
    context.insert("taxonomy_terms", taxonomy_terms);
    context.insert("taxonomy_items", &bucket.items);
    context.insert("items", &bucket.items);
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
        &format!("Tag: {} | {}", bucket.term.name, env.config.title),
    );
    context.insert("page_extra", &super::resolved_page_extra(None));
    context.insert(
        "page_description",
        &super::resolved_page_description(None, env.config),
    );
    context.insert("content_html", "");
    context.insert("page_has_mermaid", &false);
    context.insert("page_has_math", &false);
    context.insert("current_page", &1usize);
    context.insert("total_pages", &1usize);
    context.insert("prev_url", &Option::<String>::None);
    context.insert("next_url", &Option::<String>::None);

    let html = tera.render("section.html", &context).with_context(|| {
        format!(
            "failed to render taxonomy term template for '{}'",
            bucket.term.slug
        )
    })?;

    Ok(RenderedPage { route, html })
}
