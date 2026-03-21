use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TocItem {
    pub level: u8,
    pub id: String,
    pub title: String,
    pub children: Vec<TocItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatTocItem {
    pub level: u8,
    pub id: String,
    pub title: String,
}

pub fn normalize_heading_title(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn slugify_heading(input: &str) -> String {
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

pub fn unique_heading_id(title: &str, ids: &mut BTreeMap<String, usize>) -> String {
    let base = slugify_heading(title);
    let base = if base.is_empty() {
        "section".to_string()
    } else {
        base
    };

    let count = ids.entry(base.clone()).or_insert(0);
    *count += 1;

    if *count == 1 {
        base
    } else {
        format!("{base}-{}", *count)
    }
}

pub fn build_nested_toc(items: Vec<FlatTocItem>) -> Vec<TocItem> {
    let mut nested = Vec::new();

    for item in items {
        insert_toc_item(&mut nested, item);
    }

    nested
}

fn insert_toc_item(items: &mut Vec<TocItem>, item: FlatTocItem) {
    if let Some(last) = items.last_mut()
        && item.level > last.level
    {
        insert_toc_item(&mut last.children, item);
        return;
    }

    items.push(TocItem {
        level: item.level,
        id: item.id,
        title: item.title,
        children: Vec::new(),
    });
}

#[cfg(test)]
mod tests {
    use super::{FlatTocItem, build_nested_toc, normalize_heading_title, slugify_heading};

    #[test]
    fn normalizes_heading_title_whitespace() {
        assert_eq!(
            normalize_heading_title("  Hello   Rustipo \n\n World  "),
            "Hello Rustipo World"
        );
    }

    #[test]
    fn slugifies_heading_titles() {
        assert_eq!(slugify_heading("What is Rustipo?"), "what-is-rustipo");
        assert_eq!(slugify_heading("C++ / Rust"), "c-rust");
    }

    #[test]
    fn builds_nested_toc_from_flat_headings() {
        let toc = build_nested_toc(vec![
            FlatTocItem {
                level: 1,
                id: "intro".to_string(),
                title: "Intro".to_string(),
            },
            FlatTocItem {
                level: 2,
                id: "getting-started".to_string(),
                title: "Getting Started".to_string(),
            },
            FlatTocItem {
                level: 3,
                id: "install".to_string(),
                title: "Install".to_string(),
            },
            FlatTocItem {
                level: 2,
                id: "next".to_string(),
                title: "Next".to_string(),
            },
        ]);

        assert_eq!(toc.len(), 1);
        assert_eq!(toc[0].title, "Intro");
        assert_eq!(toc[0].children.len(), 2);
        assert_eq!(toc[0].children[0].title, "Getting Started");
        assert_eq!(toc[0].children[0].children[0].title, "Install");
        assert_eq!(toc[0].children[1].title, "Next");
    }
}
