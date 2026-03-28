use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::theme::models::{ThemeMetadata, ThemeSource, ThemeSummary};

struct BuiltInThemeFile {
    relative_path: &'static str,
    contents: &'static str,
}

struct BuiltInTheme {
    directory_name: &'static str,
    files: &'static [BuiltInThemeFile],
}

const JOURNAL_FILES: &[BuiltInThemeFile] = &[
    BuiltInThemeFile {
        relative_path: "theme.toml",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/journal/theme.toml"
        )),
    },
    BuiltInThemeFile {
        relative_path: "static/style.css",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/journal/static/style.css"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/base.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/journal/templates/base.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/index.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/journal/templates/index.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/page.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/journal/templates/page.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/post.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/journal/templates/post.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/project.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/journal/templates/project.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/section.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/journal/templates/section.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/partials/head_assets.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/journal/templates/partials/head_assets.html"
        )),
    },
];

const ATLAS_FILES: &[BuiltInThemeFile] = &[
    BuiltInThemeFile {
        relative_path: "theme.toml",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/atlas/theme.toml"
        )),
    },
    BuiltInThemeFile {
        relative_path: "static/style.css",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/atlas/static/style.css"
        )),
    },
    BuiltInThemeFile {
        relative_path: "static/landing.js",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/atlas/static/landing.js"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/base.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/atlas/templates/base.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/index.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/atlas/templates/index.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/page.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/atlas/templates/page.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/post.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/atlas/templates/post.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/project.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/atlas/templates/project.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/section.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/atlas/templates/section.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/macros/toc.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/atlas/templates/macros/toc.html"
        )),
    },
    BuiltInThemeFile {
        relative_path: "templates/partials/head_assets.html",
        contents: include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/themes/atlas/templates/partials/head_assets.html"
        )),
    },
];

const BUILT_IN_THEMES: &[BuiltInTheme] = &[
    BuiltInTheme {
        directory_name: "atlas",
        files: ATLAS_FILES,
    },
    BuiltInTheme {
        directory_name: "journal",
        files: JOURNAL_FILES,
    },
];

pub fn list_builtin_themes() -> Result<Vec<ThemeSummary>> {
    BUILT_IN_THEMES
        .iter()
        .map(|theme| {
            let metadata = parse_theme_metadata(theme)?;
            let theme_id = metadata
                .id
                .clone()
                .unwrap_or_else(|| theme.directory_name.to_string());

            Ok(ThemeSummary {
                theme_id,
                directory_name: theme.directory_name.to_string(),
                source: ThemeSource::BuiltIn,
                metadata,
            })
        })
        .collect()
}

pub fn materialize_builtin_theme(directory_name: &str) -> Result<PathBuf> {
    let theme = BUILT_IN_THEMES
        .iter()
        .find(|theme| theme.directory_name == directory_name)
        .with_context(|| format!("unknown built-in theme: {directory_name}"))?;

    let root_dir = built_in_theme_root().join(directory_name);
    for file in theme.files {
        let destination = root_dir.join(file.relative_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::write(&destination, file.contents)
            .with_context(|| format!("failed to write {}", destination.display()))?;
    }

    Ok(root_dir)
}

fn parse_theme_metadata(theme: &BuiltInTheme) -> Result<ThemeMetadata> {
    let theme_file = theme
        .files
        .iter()
        .find(|file| file.relative_path == "theme.toml")
        .with_context(|| {
            format!(
                "built-in theme missing theme.toml: {}",
                theme.directory_name
            )
        })?;

    toml::from_str::<ThemeMetadata>(theme_file.contents)
        .with_context(|| format!("failed to parse built-in theme: {}", theme.directory_name))
}

fn built_in_theme_root() -> PathBuf {
    std::env::temp_dir()
        .join("rustipo-built-in-themes")
        .join(env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    use super::{list_builtin_themes, materialize_builtin_theme};

    #[test]
    fn lists_built_in_themes() {
        let themes = list_builtin_themes().expect("built-in themes should load");
        let ids = themes
            .iter()
            .map(|theme| theme.theme_id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(ids, vec!["atlas", "journal"]);
    }

    #[test]
    fn materializes_built_in_theme_assets() {
        let root = materialize_builtin_theme("atlas").expect("theme should materialize");
        assert!(root.join("theme.toml").is_file());
        assert!(root.join("templates/base.html").is_file());
        assert!(root.join("static/style.css").is_file());
    }

    #[test]
    fn errors_on_unknown_built_in_theme() {
        let error = materialize_builtin_theme("missing").expect_err("theme should fail");
        assert!(error.to_string().contains("unknown built-in theme"));
    }
}
