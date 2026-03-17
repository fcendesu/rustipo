use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::BTreeMap;
use tera::{Context as TeraContext, Tera};

use crate::config::SiteConfig;
use crate::content::pages::{Page, PageKind};

use super::RenderedPage;

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
            date: page.frontmatter.date.clone(),
        };

        let group_key = month_key(item.date.as_deref()).unwrap_or_else(|| "undated".to_string());
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
    context.insert("site_title", &config.title);
    context.insert("site_description", &config.description);
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

fn month_key(date: Option<&str>) -> Option<String> {
    let date = date?;
    let mut parts = date.split('-');
    let year = parts.next()?;
    let month = parts.next()?;
    let day = parts.next()?;
    if parts.next().is_some() {
        return None;
    }

    if !year.chars().all(|c| c.is_ascii_digit()) || year.len() != 4 {
        return None;
    }
    if !month.chars().all(|c| c.is_ascii_digit()) || month.len() != 2 {
        return None;
    }
    if !day.chars().all(|c| c.is_ascii_digit()) || day.len() != 2 {
        return None;
    }

    Some(format!("{year}-{month}"))
}
