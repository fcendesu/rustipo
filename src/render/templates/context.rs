use std::collections::BTreeMap;

use serde::Serialize;
use tera::Context as TeraContext;

use crate::content::pages::{Page, PageKind};

#[derive(Clone, Serialize)]
pub(super) struct NavItem {
    pub title: String,
    pub route: String,
    pub active: bool,
}

#[derive(Clone, Serialize)]
pub(super) struct AdjacentPost {
    pub title: String,
    pub route: String,
    pub summary: Option<String>,
    pub date: Option<String>,
}

#[derive(Default)]
pub(super) struct SharedTemplateData {
    nav_entries: Vec<NavEntry>,
    adjacent_posts: BTreeMap<String, AdjacentPosts>,
}

#[derive(Clone)]
struct NavEntry {
    title: String,
    route: String,
    kind: NavEntryKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum NavEntryKind {
    Home,
    StandalonePage,
    Blog,
    Projects,
}

#[derive(Clone, Default)]
struct AdjacentPosts {
    previous_post: Option<AdjacentPost>,
    next_post: Option<AdjacentPost>,
}

pub(super) fn build_shared_template_data(pages: &[Page]) -> SharedTemplateData {
    SharedTemplateData {
        nav_entries: build_nav_entries(pages),
        adjacent_posts: build_adjacent_posts(pages),
    }
}

pub(super) fn insert_page_context(
    context: &mut TeraContext,
    shared: &SharedTemplateData,
    route: &str,
    page_kind: &str,
    current_section: &str,
) {
    context.insert("page_kind", page_kind);
    context.insert("current_section", current_section);
    context.insert(
        "site_nav",
        &site_nav_for_route(shared, route, current_section),
    );

    let adjacent = shared
        .adjacent_posts
        .get(route)
        .cloned()
        .unwrap_or_default();
    context.insert("previous_post", &adjacent.previous_post);
    context.insert("next_post", &adjacent.next_post);
}

fn build_nav_entries(pages: &[Page]) -> Vec<NavEntry> {
    let mut entries = Vec::new();

    if pages.iter().any(|page| page.kind == PageKind::Index) {
        entries.push(NavEntry {
            title: "Home".to_string(),
            route: "/".to_string(),
            kind: NavEntryKind::Home,
        });
    }

    let mut standalone_pages = pages
        .iter()
        .filter(|page| page.kind == PageKind::Page)
        .map(|page| {
            (
                page.frontmatter.order.unwrap_or(i64::MAX),
                NavEntry {
                    title: page
                        .frontmatter
                        .title
                        .clone()
                        .unwrap_or_else(|| page.slug.clone()),
                    route: page.route.clone(),
                    kind: NavEntryKind::StandalonePage,
                },
            )
        })
        .collect::<Vec<_>>();
    standalone_pages.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then_with(|| a.1.title.cmp(&b.1.title))
            .then_with(|| a.1.route.cmp(&b.1.route))
    });
    entries.extend(standalone_pages.into_iter().map(|(_, entry)| entry));

    if pages.iter().any(|page| page.kind == PageKind::BlogPost) {
        entries.push(NavEntry {
            title: "Blog".to_string(),
            route: "/blog/".to_string(),
            kind: NavEntryKind::Blog,
        });
    }

    if pages.iter().any(|page| page.kind == PageKind::Project) {
        entries.push(NavEntry {
            title: "Projects".to_string(),
            route: "/projects/".to_string(),
            kind: NavEntryKind::Projects,
        });
    }

    entries
}

fn site_nav_for_route(
    shared: &SharedTemplateData,
    route: &str,
    current_section: &str,
) -> Vec<NavItem> {
    shared
        .nav_entries
        .iter()
        .map(|entry| NavItem {
            title: entry.title.clone(),
            route: entry.route.clone(),
            active: nav_entry_is_active(entry, route, current_section),
        })
        .collect()
}

fn nav_entry_is_active(entry: &NavEntry, route: &str, current_section: &str) -> bool {
    match entry.kind {
        NavEntryKind::Home => route == "/",
        NavEntryKind::StandalonePage => route == entry.route,
        NavEntryKind::Blog => matches!(current_section, "blog" | "archive" | "tags"),
        NavEntryKind::Projects => current_section == "projects",
    }
}

fn build_adjacent_posts(pages: &[Page]) -> BTreeMap<String, AdjacentPosts> {
    let mut posts = pages
        .iter()
        .filter(|page| page.kind == PageKind::BlogPost)
        .map(|page| AdjacentPost {
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

    posts.sort_by(|a, b| {
        a.date
            .cmp(&b.date)
            .then_with(|| a.title.cmp(&b.title))
            .then_with(|| a.route.cmp(&b.route))
    });

    let mut adjacent = BTreeMap::new();
    for (index, post) in posts.iter().enumerate() {
        adjacent.insert(
            post.route.clone(),
            AdjacentPosts {
                previous_post: index
                    .checked_sub(1)
                    .and_then(|prev| posts.get(prev).cloned()),
                next_post: posts.get(index + 1).cloned(),
            },
        );
    }

    adjacent
}
