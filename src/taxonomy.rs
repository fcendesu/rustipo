use std::collections::BTreeMap;

use serde::Serialize;

use crate::config::SiteConfig;
use crate::content::frontmatter::Frontmatter;

pub const TAGS_TAXONOMY: &str = "tags";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TaxonomyDefinition {
    pub name: String,
    pub title: String,
    pub route: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TaxonomyValue {
    pub name: String,
    pub slug: String,
    pub route: String,
}

pub fn taxonomy_title(name: &str) -> Option<&'static str> {
    match name {
        TAGS_TAXONOMY => Some("Tags"),
        _ => None,
    }
}

pub fn taxonomy_route(name: &str) -> Option<String> {
    match name {
        TAGS_TAXONOMY => Some("/tags/".to_string()),
        _ => None,
    }
}

pub fn taxonomy_term_route(name: &str, slug: &str) -> Option<String> {
    if slug.is_empty() {
        return None;
    }

    let base = taxonomy_route(name)?;
    Some(format!("{base}{slug}/"))
}

pub fn taxonomy_definitions() -> Vec<TaxonomyDefinition> {
    [TAGS_TAXONOMY]
        .into_iter()
        .filter_map(|name| {
            Some(TaxonomyDefinition {
                name: name.to_string(),
                title: taxonomy_title(name)?.to_string(),
                route: taxonomy_route(name)?,
            })
        })
        .collect()
}

pub fn page_taxonomies(
    frontmatter: &Frontmatter,
    config: &SiteConfig,
) -> BTreeMap<String, Vec<TaxonomyValue>> {
    let mut taxonomies = BTreeMap::new();
    let tags = page_tags(frontmatter, config);
    if !tags.is_empty() {
        taxonomies.insert(TAGS_TAXONOMY.to_string(), tags);
    }
    taxonomies
}

pub fn page_tags(frontmatter: &Frontmatter, config: &SiteConfig) -> Vec<TaxonomyValue> {
    frontmatter_tags(frontmatter)
        .into_iter()
        .map(|value| TaxonomyValue {
            route: config.public_url_path(&value.route),
            ..value
        })
        .collect()
}

pub fn frontmatter_tags(frontmatter: &Frontmatter) -> Vec<TaxonomyValue> {
    let mut values = Vec::new();
    let mut seen = std::collections::BTreeSet::new();

    for tag in frontmatter
        .tags
        .as_ref()
        .into_iter()
        .flatten()
        .map(String::as_str)
    {
        let slug = slugify_term(tag);
        if slug.is_empty() || !seen.insert(slug.clone()) {
            continue;
        }

        values.push(TaxonomyValue {
            name: tag.to_string(),
            route: taxonomy_term_route(TAGS_TAXONOMY, &slug).expect("tag route should exist"),
            slug,
        });
    }

    values
}

pub fn slugify_term(input: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::{
        TAGS_TAXONOMY, frontmatter_tags, page_taxonomies, slugify_term, taxonomy_definitions,
        taxonomy_route, taxonomy_term_route,
    };
    use crate::config::SiteConfig;
    use crate::content::frontmatter::Frontmatter;

    #[test]
    fn exposes_builtin_tags_taxonomy_definition() {
        let definitions = taxonomy_definitions();
        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].name, TAGS_TAXONOMY);
        assert_eq!(definitions[0].title, "Tags");
        assert_eq!(definitions[0].route, "/tags/");
    }

    #[test]
    fn builds_tag_page_taxonomies() {
        let config = SiteConfig {
            title: "My Site".to_string(),
            base_url: "https://example.com/docs/".to_string(),
            theme: "default".to_string(),
            palette: None,
            menus: None,
            description: "A site".to_string(),
            author: None,
            site: None,
        };
        let frontmatter = Frontmatter {
            tags: Some(vec!["Site Gen".to_string(), "Rust".to_string()]),
            ..Frontmatter::default()
        };

        let taxonomies = page_taxonomies(&frontmatter, &config);
        let tags = taxonomies.get(TAGS_TAXONOMY).expect("tags should exist");
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].route, "/docs/tags/site-gen/");
        assert_eq!(tags[1].route, "/docs/tags/rust/");
    }

    #[test]
    fn builds_internal_frontmatter_taxonomy_routes() {
        let frontmatter = Frontmatter {
            tags: Some(vec!["Site Gen".to_string(), "Rust".to_string()]),
            ..Frontmatter::default()
        };

        let tags = frontmatter_tags(&frontmatter);
        assert_eq!(tags[0].route, "/tags/site-gen/");
        assert_eq!(tags[1].route, "/tags/rust/");
    }

    #[test]
    fn slugifies_taxonomy_terms() {
        assert_eq!(slugify_term("Site Gen"), "site-gen");
        assert_eq!(slugify_term("C++ / Rust"), "c-rust");
    }

    #[test]
    fn resolves_tags_routes() {
        assert_eq!(taxonomy_route(TAGS_TAXONOMY).as_deref(), Some("/tags/"));
        assert_eq!(
            taxonomy_term_route(TAGS_TAXONOMY, "site-gen").as_deref(),
            Some("/tags/site-gen/")
        );
    }
}
