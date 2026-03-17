use anyhow::{Context, Result};
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};
use serde::{Deserialize, Serialize};

use crate::content::date::ContentDate;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub date: Option<ContentDate>,
    pub summary: Option<String>,
    pub tags: Option<Vec<String>>,
    pub draft: Option<bool>,
    pub slug: Option<String>,
    pub order: Option<i64>,
    pub links: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct ParsedContent {
    pub frontmatter: Frontmatter,
    pub content: String,
}

pub fn parse(input: &str) -> Result<ParsedContent> {
    let matter = Matter::<YAML>::new();
    let parsed: ParsedEntity<Frontmatter> = matter
        .parse(input)
        .context("failed to parse YAML frontmatter")?;

    Ok(ParsedContent {
        frontmatter: parsed.data.unwrap_or_default(),
        content: parsed.content,
    })
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parses_frontmatter_and_content() {
        let input = r#"---
title: Hello
summary: Test
draft: false
---

# Heading
"#;

        let parsed = parse(input).expect("frontmatter should parse");
        assert_eq!(parsed.frontmatter.title.as_deref(), Some("Hello"));
        assert_eq!(parsed.frontmatter.summary.as_deref(), Some("Test"));
        assert_eq!(parsed.frontmatter.draft, Some(false));
        assert!(parsed.content.contains("# Heading"));
    }

    #[test]
    fn supports_content_without_frontmatter() {
        let parsed = parse("# Heading").expect("content without frontmatter should parse");
        assert!(parsed.frontmatter.title.is_none());
        assert_eq!(parsed.content, "# Heading");
    }

    #[test]
    fn rejects_invalid_date_in_frontmatter() {
        let input = r#"---
title: Bad Date
date: 2026-13-17
---

# Heading
"#;
        parse(input).expect_err("invalid date should fail");
    }
}
