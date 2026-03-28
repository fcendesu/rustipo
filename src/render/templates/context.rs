use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use tera::Context as TeraContext;

use crate::config::SiteConfig;
use crate::content::pages::{Page, PageKind};
use crate::taxonomy::{TAGS_TAXONOMY, frontmatter_tags, taxonomy_definitions};

#[derive(Clone, Serialize)]
pub(super) struct NavItem {
    pub title: String,
    pub route: String,
    pub active: bool,
}

#[derive(Clone, Serialize)]
pub(super) struct SidebarChildNav {
    pub parent_route: String,
    pub items: Vec<NavItem>,
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

#[derive(Clone, Serialize)]
pub(super) struct SiteTaxonomy {
    pub name: String,
    pub title: String,
    pub route: String,
}

#[derive(Default)]
pub(in crate::render) struct SharedTemplateData {
    auto_nav_entries: Vec<NavEntry>,
    page_entries: Vec<PageEntry>,
    configured_menus: BTreeMap<String, Vec<ConfiguredMenuEntry>>,
    site_taxonomies: Vec<SiteTaxonomy>,
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

#[derive(Clone)]
struct PageEntry {
    title: String,
    route: String,
    order: i64,
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
        page_entries: build_page_entries(pages),
        configured_menus: build_configured_menus(config),
        site_taxonomies: build_site_taxonomies(pages),
        breadcrumb_titles: build_breadcrumb_titles(pages),
        linkable_breadcrumb_routes: build_linkable_breadcrumb_routes(pages),
        adjacent_posts: build_adjacent_posts(pages),
    }
}

pub(super) fn insert_page_context(
    context: &mut TeraContext,
    config: &SiteConfig,
    shared: &SharedTemplateData,
    route: &str,
    page_kind: &str,
    current_section: &str,
) {
    context.insert("page_kind", page_kind);
    context.insert("current_section", current_section);
    context.insert(
        "site_nav",
        &site_nav_for_route(shared, config, route, current_section),
    );
    context.insert("site_menus", &site_menus_for_route(shared, config, route));
    context.insert(
        "site_taxonomies",
        &site_taxonomies_for_config(shared, config),
    );
    context.insert("breadcrumbs", &breadcrumbs_for_route(shared, config, route));
    context.insert(
        "sidebar_child_nav",
        &sidebar_child_nav_for_route(shared, config, route),
    );

    let adjacent = shared
        .adjacent_posts
        .get(route)
        .cloned()
        .unwrap_or_default();
    context.insert(
        "previous_post",
        &adjacent.previous_post.map(|post| AdjacentPost {
            route: config.public_url_path(&post.route),
            ..post
        }),
    );
    context.insert(
        "next_post",
        &adjacent.next_post.map(|post| AdjacentPost {
            route: config.public_url_path(&post.route),
            ..post
        }),
    );
}

fn build_site_taxonomies(pages: &[Page]) -> Vec<SiteTaxonomy> {
    let has_tags = pages
        .iter()
        .filter(|page| page.kind == PageKind::BlogPost)
        .any(|page| !frontmatter_tags(&page.frontmatter).is_empty());

    if !has_tags {
        return Vec::new();
    }

    taxonomy_definitions()
        .into_iter()
        .filter(|taxonomy| taxonomy.name == TAGS_TAXONOMY)
        .map(|taxonomy| SiteTaxonomy {
            name: taxonomy.name,
            title: taxonomy.title,
            route: taxonomy.route,
        })
        .collect()
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

fn build_page_entries(pages: &[Page]) -> Vec<PageEntry> {
    let mut entries = pages
        .iter()
        .filter(|page| page.kind == PageKind::Page)
        .map(|page| PageEntry {
            title: page
                .frontmatter
                .title
                .clone()
                .unwrap_or_else(|| page.slug.clone()),
            route: page.route.clone(),
            order: page.frontmatter.order.unwrap_or(i64::MAX),
        })
        .collect::<Vec<_>>();

    entries.sort_by(|a, b| {
        a.order
            .cmp(&b.order)
            .then_with(|| a.title.cmp(&b.title))
            .then_with(|| a.route.cmp(&b.route))
    });

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
        titles
            .entry("/tags/".to_string())
            .or_insert_with(|| "Tags".to_string());

        for page in pages.iter().filter(|page| page.kind == PageKind::BlogPost) {
            for tag in frontmatter_tags(&page.frontmatter) {
                titles.entry(tag.route).or_insert_with(|| tag.name);
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
    config: &SiteConfig,
    route: &str,
    current_section: &str,
) -> Vec<NavItem> {
    if let Some(main_menu) = shared.configured_menus.get("main") {
        return configured_menu_for_route(main_menu, config, route);
    }

    shared
        .auto_nav_entries
        .iter()
        .map(|entry| NavItem {
            title: entry.title.clone(),
            route: config.public_url_path(&entry.route),
            active: nav_entry_is_active(entry, route, current_section),
        })
        .collect()
}

fn site_menus_for_route(
    shared: &SharedTemplateData,
    config: &SiteConfig,
    route: &str,
) -> BTreeMap<String, Vec<NavItem>> {
    shared
        .configured_menus
        .iter()
        .map(|(name, entries)| {
            (
                name.clone(),
                configured_menu_for_route(entries, config, route),
            )
        })
        .collect()
}

fn sidebar_child_nav_for_route(
    shared: &SharedTemplateData,
    config: &SiteConfig,
    route: &str,
) -> Option<SidebarChildNav> {
    let Some(parent_route) = nearest_nav_parent_with_children(shared, route) else {
        return None;
    };

    let items = direct_child_pages(shared, &parent_route)
        .into_iter()
        .map(|entry| NavItem {
            title: entry.title.clone(),
            route: config.public_url_path(&entry.route),
            active: route == entry.route,
        })
        .collect::<Vec<_>>();

    if items.is_empty() {
        return None;
    }

    Some(SidebarChildNav {
        parent_route: config.public_url_path(&parent_route),
        items,
    })
}

fn breadcrumbs_for_route(
    shared: &SharedTemplateData,
    config: &SiteConfig,
    route: &str,
) -> Vec<BreadcrumbItem> {
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
            route: config.public_url_path(route),
            active: true,
            linkable: shared.linkable_breadcrumb_routes.contains(route),
        }];
    }

    let mut items = Vec::new();

    if let Some(home_title) = shared.breadcrumb_titles.get("/") {
        items.push(BreadcrumbItem {
            title: home_title.clone(),
            route: config.public_url_path("/"),
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
            route: config.public_url_path(&crumb_route),
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

fn configured_menu_for_route(
    entries: &[ConfiguredMenuEntry],
    config: &SiteConfig,
    route: &str,
) -> Vec<NavItem> {
    entries
        .iter()
        .map(|entry| NavItem {
            title: entry.title.clone(),
            route: config.public_url_path(&entry.route),
            active: configured_menu_entry_is_active(entry, route),
        })
        .collect()
}

fn nearest_nav_parent_with_children(shared: &SharedTemplateData, route: &str) -> Option<String> {
    let mut current = route.to_string();

    loop {
        if !direct_child_pages(shared, &current).is_empty() {
            return Some(current);
        }

        let Some(parent) = immediate_parent_route(&current) else {
            return None;
        };

        if parent == "/" {
            return None;
        }

        current = parent;
    }
}

fn direct_child_pages<'a>(
    shared: &'a SharedTemplateData,
    parent_route: &str,
) -> Vec<&'a PageEntry> {
    shared
        .page_entries
        .iter()
        .filter(|entry| entry.route != parent_route)
        .filter(|entry| immediate_parent_route(&entry.route).as_deref() == Some(parent_route))
        .collect()
}

fn immediate_parent_route(route: &str) -> Option<String> {
    let segments = breadcrumb_segments(route)?;

    if segments.is_empty() {
        return None;
    }

    if segments.len() == 1 {
        return Some("/".to_string());
    }

    Some(format!("/{}/", segments[..segments.len() - 1].join("/")))
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

fn site_taxonomies_for_config(
    shared: &SharedTemplateData,
    config: &SiteConfig,
) -> Vec<SiteTaxonomy> {
    shared
        .site_taxonomies
        .iter()
        .map(|taxonomy| SiteTaxonomy {
            route: config.public_url_path(&taxonomy.route),
            ..taxonomy.clone()
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
