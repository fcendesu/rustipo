use std::collections::{BTreeSet, HashMap};

use anyhow::{Result, bail};
use pulldown_cmark::{Event, Options, Parser, Tag};

use crate::content::pages::Page;
use crate::content::toc::TocItem;
use crate::render::templates::RenderedPage;

pub fn validate_internal_links(pages: &[Page], rendered_pages: &[RenderedPage]) -> Result<()> {
    let rendered_routes = rendered_pages
        .iter()
        .map(|page| page.route.as_str())
        .collect::<BTreeSet<_>>();
    let page_anchors = pages
        .iter()
        .map(|page| (page.route.as_str(), collect_anchor_ids(&page.toc)))
        .collect::<HashMap<_, _>>();

    for page in pages {
        for destination in collect_markdown_links(&page.markdown) {
            let Some(target) = parse_internal_target(&page.route, &destination) else {
                continue;
            };

            if !rendered_routes.contains(target.route.as_str()) {
                bail!(
                    "invalid internal link in '{}': '{}' resolves to '{}' but no generated route exists",
                    page.source_path.display(),
                    destination,
                    target.route
                );
            }

            if let Some(fragment) = target.fragment
                && let Some(anchors) = page_anchors.get(target.route.as_str())
                && !anchors.contains(fragment.as_str())
            {
                bail!(
                    "invalid deep link in '{}': '{}' targets missing heading id '{}' on route '{}'",
                    page.source_path.display(),
                    destination,
                    fragment,
                    target.route
                );
            }
        }
    }

    Ok(())
}

fn collect_markdown_links(markdown: &str) -> Vec<String> {
    let markdown = crate::content::shortcodes::preprocess(markdown).markdown;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_GFM);

    let mut links = Vec::new();

    for event in Parser::new_ext(&markdown, options) {
        if let Event::Start(Tag::Link { dest_url, .. }) = event {
            links.push(dest_url.into_string());
        }
    }

    links
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InternalTarget {
    route: String,
    fragment: Option<String>,
}

fn parse_internal_target(current_route: &str, destination: &str) -> Option<InternalTarget> {
    if destination.is_empty() || has_url_scheme(destination) || destination.starts_with("//") {
        return None;
    }

    let (path_and_query, fragment) = split_fragment(destination);
    let (path, _) = split_query(path_and_query);

    if !path.is_empty() && is_asset_like_path(path) {
        return None;
    }

    let route = if path.is_empty() {
        current_route.to_string()
    } else if path.starts_with('/') {
        normalize_route_path(path)
    } else {
        resolve_relative_route(current_route, path)
    };

    Some(InternalTarget {
        route,
        fragment: fragment.filter(|fragment| !fragment.is_empty()),
    })
}

fn has_url_scheme(value: &str) -> bool {
    let Some((scheme, _)) = value.split_once(':') else {
        return false;
    };

    !scheme.is_empty()
        && scheme
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '+' | '-' | '.'))
}

fn split_fragment(value: &str) -> (&str, Option<String>) {
    match value.split_once('#') {
        Some((path, fragment)) => (path, Some(fragment.to_string())),
        None => (value, None),
    }
}

fn split_query(value: &str) -> (&str, Option<&str>) {
    match value.split_once('?') {
        Some((path, query)) => (path, Some(query)),
        None => (value, None),
    }
}

fn is_asset_like_path(path: &str) -> bool {
    let Some(last_segment) = path.rsplit('/').find(|segment| !segment.is_empty()) else {
        return false;
    };

    if matches!(last_segment, "." | "..") {
        return false;
    }

    match last_segment.rsplit_once('.') {
        Some((_, "html")) => false,
        Some(_) => true,
        None => false,
    }
}

fn resolve_relative_route(current_route: &str, path: &str) -> String {
    let mut segments = current_route
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    for segment in path.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            _ => segments.push(segment.to_string()),
        }
    }

    normalize_route_segments(segments)
}

fn normalize_route_path(path: &str) -> String {
    let segments = path
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    normalize_route_segments(segments)
}

fn normalize_route_segments(mut segments: Vec<String>) -> String {
    if segments.is_empty() {
        return "/".to_string();
    }

    if matches!(segments.last().map(String::as_str), Some("index.html")) {
        segments.pop();
    } else if let Some(last) = segments.last_mut()
        && let Some(stem) = last.strip_suffix(".html")
    {
        *last = stem.to_string();
    }

    if segments.is_empty() {
        "/".to_string()
    } else {
        format!("/{}/", segments.join("/"))
    }
}

fn collect_anchor_ids(toc: &[TocItem]) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    collect_anchor_ids_into(toc, &mut ids);
    ids
}

fn collect_anchor_ids_into(toc: &[TocItem], ids: &mut BTreeSet<String>) {
    for item in toc {
        ids.insert(item.id.clone());
        collect_anchor_ids_into(&item.children, ids);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::content::frontmatter::Frontmatter;
    use crate::content::pages::{Page, PageKind};
    use crate::content::shortcodes::ShortcodeAssets;
    use crate::content::toc::TocItem;
    use crate::render::templates::RenderedPage;

    use super::{parse_internal_target, validate_internal_links};

    fn page(route: &str, markdown: &str, toc: Vec<TocItem>) -> Page {
        Page {
            source_path: PathBuf::from(format!("content{}.md", route.replace('/', "_"))),
            route: route.to_string(),
            slug: route.trim_matches('/').to_string(),
            kind: PageKind::Page,
            has_mermaid: false,
            has_math: false,
            shortcode_assets: ShortcodeAssets::default(),
            toc,
            frontmatter: Frontmatter::default(),
            markdown: markdown.to_string(),
            html: String::new(),
        }
    }

    fn rendered(route: &str) -> RenderedPage {
        RenderedPage {
            route: route.to_string(),
            html: String::new(),
        }
    }

    fn toc_item(title: &str, id: &str, children: Vec<TocItem>) -> TocItem {
        TocItem {
            title: title.to_string(),
            id: id.to_string(),
            level: 2,
            children,
        }
    }

    #[test]
    fn validates_existing_internal_routes_and_heading_fragments() {
        let pages = vec![
            page(
                "/guide/",
                "[Install](#install)\n\n[About](/about/)",
                vec![toc_item("Install", "install", Vec::new())],
            ),
            page("/about/", "# About", Vec::new()),
        ];
        let rendered_pages = vec![rendered("/guide/"), rendered("/about/"), rendered("/blog/")];

        validate_internal_links(&pages, &rendered_pages).expect("links should validate");
    }

    #[test]
    fn ignores_external_and_asset_like_links() {
        let pages = vec![page(
            "/guide/",
            "[Docs](https://example.com)\n\n![Logo](/img/logo.svg)\n\n[Asset](/img/logo.svg)",
            Vec::new(),
        )];
        let rendered_pages = vec![rendered("/guide/")];

        validate_internal_links(&pages, &rendered_pages).expect("non-page links should be ignored");
    }

    #[test]
    fn errors_for_missing_deep_link_anchor() {
        let pages = vec![page(
            "/guide/",
            "[Missing](#oops)",
            vec![toc_item("Install", "install", Vec::new())],
        )];
        let rendered_pages = vec![rendered("/guide/")];

        let error = validate_internal_links(&pages, &rendered_pages)
            .expect_err("missing anchor should fail");
        assert!(error.to_string().contains("invalid deep link"));
        assert!(error.to_string().contains("oops"));
    }

    #[test]
    fn parses_relative_and_root_internal_targets() {
        assert_eq!(
            parse_internal_target("/notes/rust/tips/", "../guide/#install"),
            Some(super::InternalTarget {
                route: "/notes/rust/guide/".to_string(),
                fragment: Some("install".to_string()),
            })
        );
        assert_eq!(
            parse_internal_target("/guide/", "/about"),
            Some(super::InternalTarget {
                route: "/about/".to_string(),
                fragment: None,
            })
        );
        assert_eq!(parse_internal_target("/guide/", "/img/logo.svg"), None);
    }
}
