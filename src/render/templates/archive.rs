use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::BTreeMap;
use tera::{Context as TeraContext, Tera};

use crate::config::{FaviconLinks, SiteConfig, SiteStyleOptions};
use crate::content::date::ContentDate;
use crate::content::pages::{Page, PageKind};

use super::RenderedPage;
use super::context::SharedTemplateData;

#[derive(Clone, Serialize)]
struct ArchiveItem {
    title: String,
    route: String,
    summary: Option<String>,
    date: Option<String>,
}

#[derive(Serialize)]
struct ArchiveGroup {
    key: String,
    label: String,
    items: Vec<ArchiveItem>,
}

pub(super) fn render_blog_archive_page(
    tera: &Tera,
    config: &SiteConfig,
    pages: &[Page],
    shared: &SharedTemplateData,
    favicon_links: &FaviconLinks,
    site_style: &SiteStyleOptions,
    site_has_custom_css: bool,
) -> Result<Vec<RenderedPage>> {
    let mut grouped: BTreeMap<String, Vec<ArchiveItem>> = BTreeMap::new();
    let mut all_items = Vec::new();

    for page in pages.iter().filter(|page| page.kind == PageKind::BlogPost) {
        let item = ArchiveItem {
            title: page
                .frontmatter
                .title
                .clone()
                .unwrap_or_else(|| page.slug.clone()),
            route: page.route.clone(),
            summary: page.frontmatter.summary.clone(),
            date: page.frontmatter.date.as_ref().map(ToString::to_string),
        };

        let group_key =
            month_key(page.frontmatter.date.as_ref()).unwrap_or_else(|| "undated".to_string());
        grouped.entry(group_key).or_default().push(item.clone());
        all_items.push(item);
    }

    all_items.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.title.cmp(&b.title)));

    let mut archive_groups = grouped
        .into_iter()
        .map(|(key, mut items)| {
            items.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.title.cmp(&b.title)));
            let label = if key == "undated" {
                "Undated".to_string()
            } else {
                key.clone()
            };
            ArchiveGroup { key, label, items }
        })
        .collect::<Vec<_>>();

    archive_groups.sort_by(|a, b| b.key.cmp(&a.key));

    let route = "/blog/archive/".to_string();
    let mut context = TeraContext::new();
    context.insert("route", &route);
    context.insert("section_name", "archive");
    context.insert("section_title", "Archive");
    context.insert("items", &all_items);
    context.insert("archive_groups", &archive_groups);
    super::insert_common_site_context(
        &mut context,
        config,
        shared,
        &route,
        "section",
        "archive",
        favicon_links,
        site_style,
        site_has_custom_css,
    );
    context.insert("page_title", &format!("Archive | {}", config.title));
    context.insert("content_html", "");
    context.insert("current_page", &1usize);
    context.insert("total_pages", &1usize);
    context.insert("prev_url", &Option::<String>::None);
    context.insert("next_url", &Option::<String>::None);

    let html = tera
        .render("section.html", &context)
        .with_context(|| "failed to render section template for 'archive'".to_string())?;

    Ok(vec![RenderedPage { route, html }])
}

fn month_key(date: Option<&ContentDate>) -> Option<String> {
    date.map(ContentDate::month_key)
}
