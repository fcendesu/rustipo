use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::content::pages::model::PageKind;

pub(super) struct PageMeta {
    pub(super) route: String,
    pub(super) slug: String,
    pub(super) kind: PageKind,
}

pub(super) fn derive_page_meta(
    rel_path: &Path,
    frontmatter_slug: Option<&str>,
) -> Result<PageMeta> {
    let raw_stem = rel_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .context("invalid UTF-8 filename")?;

    let preferred_slug = frontmatter_slug.unwrap_or(raw_stem);
    let slug = normalize_slug(preferred_slug);
    if slug.is_empty() {
        bail!(
            "slug must contain at least one ASCII letter or digit: {}",
            rel_path.display()
        );
    }

    let components = rel_path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    match components.as_slice() {
        [name] if name == "index.md" => Ok(PageMeta {
            route: "/".to_string(),
            slug,
            kind: PageKind::Index,
        }),
        [name] if name.ends_with(".md") => Ok(PageMeta {
            route: format!("/{slug}/"),
            slug,
            kind: PageKind::Page,
        }),
        [section, name] if section == "blog" && name.ends_with(".md") => Ok(PageMeta {
            route: format!("/blog/{slug}/"),
            slug,
            kind: PageKind::BlogPost,
        }),
        [section, name] if section == "projects" && name.ends_with(".md") => Ok(PageMeta {
            route: format!("/projects/{slug}/"),
            slug,
            kind: PageKind::Project,
        }),
        [section, ..] if section == "blog" => {
            bail!(
                "unsupported nested blog content path: {}",
                rel_path.display()
            )
        }
        [section, ..] if section == "projects" => {
            bail!(
                "unsupported nested project content path: {}",
                rel_path.display()
            )
        }
        [.., name] if name == "index.md" => Ok(PageMeta {
            route: format!(
                "/{}/",
                join_path_segments(&components[..components.len() - 1])
            ),
            slug,
            kind: PageKind::Page,
        }),
        [.., name] if name.ends_with(".md") => Ok(PageMeta {
            route: format!(
                "/{}/{slug}/",
                join_path_segments(&components[..components.len() - 1])
            ),
            slug,
            kind: PageKind::Page,
        }),
        _ => bail!("unsupported content path structure: {}", rel_path.display()),
    }
}

fn join_path_segments(segments: &[String]) -> String {
    segments.join("/")
}

fn normalize_slug(input: &str) -> String {
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
