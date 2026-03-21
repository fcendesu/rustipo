use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use tera::Context as TeraContext;

use crate::config::SiteConfig;
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

#[derive(Clone, Serialize)]
pub(super) struct BreadcrumbItem {
    pub title: String,
    pub route: String,
    pub active: bool,
    pub linkable: bool,
}

#[derive(Default)]
pub(in crate::render) struct SharedTemplateData {
    auto_nav_entries: Vec<NavEntry>,
    configured_menus: BTreeMap<String, Vec<ConfiguredMenuEntry>>,
    breadcrumb_titles: BTreeMap<String, String>,
    linkable_breadcrumb_routes: BTreeSet<String>,
    adjacent_posts: BTreeMap<String, AdjacentPosts>,
}

#[derive(Clone)]
struct NavEntry {
    title: String,
    route: String,
    kind: NavEntryKind,
}

#[derive(Clone)]
struct ConfiguredMenuEntry {
    title: String,
    route: String,
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

pub(super) fn build_shared_template_data(
    pages: &[Page],
    config: &SiteConfig,
) -> SharedTemplateData {
    SharedTemplateData {
        auto_nav_entries: build_nav_entries(pages),
        configured_menus: build_configured_menus(config),
        breadcrumb_titles: build_breadcrumb_titles(pages),
        linkable_breadcrumb_routes: build_linkable_breadcrumb_routes(pages),
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
    context.insert("site_menus", &site_menus_for_route(shared, route));
    context.insert("breadcrumbs", &breadcrumbs_for_route(shared, route));

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

fn build_configured_menus(config: &SiteConfig) -> BTreeMap<String, Vec<ConfiguredMenuEntry>> {
    config
        .menus
        .as_ref()
        .map(|menus| {
            menus
                .iter()
                .map(|(name, entries)| {
                    let items = entries
                        .iter()
                        .map(|entry| ConfiguredMenuEntry {
                            title: entry.title.trim().to_string(),
                            route: entry.route.trim().to_string(),
                        })
                        .collect::<Vec<_>>();
                    (name.clone(), items)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn build_breadcrumb_titles(pages: &[Page]) -> BTreeMap<String, String> {
    let mut titles = BTreeMap::new();

    for page in pages {
        titles.insert(page.route.clone(), breadcrumb_title_for_page(page));
    }

    if pages.iter().any(|page| page.kind == PageKind::BlogPost) {
        titles
            .entry("/blog/".to_string())
            .or_insert_with(|| "Blog".to_string());
        titles
            .entry("/blog/archive/".to_string())
            .or_insert_with(|| "Archive".to_string());

        for page in pages.iter().filter(|page| page.kind == PageKind::BlogPost) {
            let Some(tags) = page.frontmatter.tags.as_ref() else {
                continue;
            };

            for tag in tags {
                let slug = normalize_tag_slug(tag);
                if slug.is_empty() {
                    continue;
                }

                titles
                    .entry(format!("/tags/{slug}/"))
                    .or_insert_with(|| tag.clone());
            }
        }
    }

    if pages.iter().any(|page| page.kind == PageKind::Project) {
        titles
            .entry("/projects/".to_string())
            .or_insert_with(|| "Projects".to_string());
    }

    titles
}

fn build_linkable_breadcrumb_routes(pages: &[Page]) -> BTreeSet<String> {
    build_breadcrumb_titles(pages).into_keys().collect()
}

fn breadcrumb_title_for_page(page: &Page) -> String {
    match page.kind {
        PageKind::Index => "Home".to_string(),
        _ => page
            .frontmatter
            .title
            .clone()
            .unwrap_or_else(|| titleize_segment(&page.slug)),
    }
}

fn site_nav_for_route(
    shared: &SharedTemplateData,
    route: &str,
    current_section: &str,
) -> Vec<NavItem> {
    if let Some(main_menu) = shared.configured_menus.get("main") {
        return configured_menu_for_route(main_menu, route);
    }

    shared
        .auto_nav_entries
        .iter()
        .map(|entry| NavItem {
            title: entry.title.clone(),
            route: entry.route.clone(),
            active: nav_entry_is_active(entry, route, current_section),
        })
        .collect()
}

fn site_menus_for_route(
    shared: &SharedTemplateData,
    route: &str,
) -> BTreeMap<String, Vec<NavItem>> {
    shared
        .configured_menus
        .iter()
        .map(|(name, entries)| (name.clone(), configured_menu_for_route(entries, route)))
        .collect()
}

fn breadcrumbs_for_route(shared: &SharedTemplateData, route: &str) -> Vec<BreadcrumbItem> {
    let Some(segments) = breadcrumb_segments(route) else {
        return Vec::new();
    };

    if route == "/" {
        return vec![BreadcrumbItem {
            title: shared
                .breadcrumb_titles
                .get(route)
                .cloned()
                .unwrap_or_else(|| "Home".to_string()),
            route: route.to_string(),
            active: true,
            linkable: shared.linkable_breadcrumb_routes.contains(route),
        }];
    }

    let mut items = Vec::new();

    if let Some(home_title) = shared.breadcrumb_titles.get("/") {
        items.push(BreadcrumbItem {
            title: home_title.clone(),
            route: "/".to_string(),
            active: false,
            linkable: shared.linkable_breadcrumb_routes.contains("/"),
        });
    }

    for (index, segment) in segments.iter().enumerate() {
        let crumb_route = format!("/{}/", segments[..=index].join("/"));
        items.push(BreadcrumbItem {
            title: shared
                .breadcrumb_titles
                .get(&crumb_route)
                .cloned()
                .unwrap_or_else(|| titleize_segment(segment)),
            route: crumb_route.clone(),
            active: crumb_route == route,
            linkable: shared.linkable_breadcrumb_routes.contains(&crumb_route),
        });
    }

    items
}

fn breadcrumb_segments(route: &str) -> Option<Vec<String>> {
    if !route.starts_with('/') || !route.ends_with('/') {
        return None;
    }

    let segments = route
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    Some(segments)
}

fn configured_menu_for_route(entries: &[ConfiguredMenuEntry], route: &str) -> Vec<NavItem> {
    entries
        .iter()
        .map(|entry| NavItem {
            title: entry.title.clone(),
            route: entry.route.clone(),
            active: configured_menu_entry_is_active(entry, route),
        })
        .collect()
}

fn configured_menu_entry_is_active(entry: &ConfiguredMenuEntry, route: &str) -> bool {
    if !entry.route.starts_with('/') {
        return false;
    }

    if entry.route == "/" {
        return route == "/";
    }

    let menu_route = entry.route.trim_end_matches('/');
    let current_route = route.trim_end_matches('/');
    current_route == menu_route || current_route.starts_with(&format!("{menu_route}/"))
}

fn titleize_segment(segment: &str) -> String {
    segment
        .split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(titleize_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn titleize_word(word: &str) -> String {
    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };

    format!(
        "{}{}",
        first.to_uppercase().collect::<String>(),
        chars.as_str().to_ascii_lowercase()
    )
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
