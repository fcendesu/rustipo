use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::theme::models::{Theme, ThemeMetadata, ThemeSummary};

const REQUIRED_TEMPLATES: &[&str] = &[
    "base.html",
    "page.html",
    "post.html",
    "project.html",
    "section.html",
    "index.html",
];

pub fn list_installed_themes(project_root: impl AsRef<Path>) -> Result<Vec<ThemeSummary>> {
    let themes_dir = project_root.as_ref().join("themes");
    if !themes_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut themes = Vec::new();
    for entry in fs::read_dir(&themes_dir)
        .with_context(|| format!("failed to read themes directory: {}", themes_dir.display()))?
    {
        let entry = entry.with_context(|| {
            format!(
                "failed to read themes directory entry: {}",
                themes_dir.display()
            )
        })?;

        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let metadata_path = path.join("theme.toml");
        if !metadata_path.is_file() {
            continue;
        }

        let metadata_raw = fs::read_to_string(&metadata_path).with_context(|| {
            format!("failed to read theme metadata: {}", metadata_path.display())
        })?;
        let metadata = toml::from_str::<ThemeMetadata>(&metadata_raw).with_context(|| {
            format!(
                "failed to parse theme metadata: {}",
                metadata_path.display()
            )
        })?;

        let directory_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .context("invalid theme directory name")?
            .to_string();

        themes.push(ThemeSummary {
            directory_name,
            metadata,
        });
    }

    themes.sort_by(|a, b| a.metadata.name.cmp(&b.metadata.name));
    Ok(themes)
}

pub fn load_active_theme(project_root: impl AsRef<Path>, theme_name: &str) -> Result<Theme> {
    let project_root = project_root.as_ref();
    let theme_root = project_root.join("themes").join(theme_name);

    if !theme_root.exists() {
        bail!("active theme not found: {}", theme_root.display());
    }

    let templates_dir = theme_root.join("templates");
    let static_dir = theme_root.join("static");
    let metadata_path = theme_root.join("theme.toml");

    if !templates_dir.is_dir() {
        bail!(
            "theme templates directory is missing: {}",
            templates_dir.display()
        );
    }
    if !static_dir.is_dir() {
        bail!(
            "theme static directory is missing: {}",
            static_dir.display()
        );
    }

    for template in REQUIRED_TEMPLATES {
        let path = templates_dir.join(template);
        if !path.is_file() {
            bail!("required theme template missing: {}", path.display());
        }
    }

    let metadata_raw = fs::read_to_string(&metadata_path)
        .with_context(|| format!("failed to read theme metadata: {}", metadata_path.display()))?;
    let metadata = toml::from_str::<ThemeMetadata>(&metadata_raw).with_context(|| {
        format!(
            "failed to parse theme metadata: {}",
            metadata_path.display()
        )
    })?;

    Ok(Theme {
        root_dir: theme_root,
        templates_dir,
        static_dir,
        metadata,
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use super::{list_installed_themes, load_active_theme};

    #[test]
    fn loads_theme_with_required_files() {
        let dir = tempdir().expect("tempdir should be created");
        let project_root = dir.path();

        let theme_root = project_root.join("themes/default");
        fs::create_dir_all(theme_root.join("templates")).expect("templates dir should be created");
        fs::create_dir_all(theme_root.join("static")).expect("static dir should be created");

        for template in [
            "base.html",
            "page.html",
            "post.html",
            "project.html",
            "section.html",
            "index.html",
        ] {
            fs::write(
                theme_root.join("templates").join(template),
                "{{ content_html }}",
            )
            .expect("template should be written");
        }

        fs::write(
            theme_root.join("theme.toml"),
            "name = \"default\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Default\"\n",
        )
        .expect("theme metadata should be written");

        let theme = load_active_theme(project_root, "default").expect("theme should load");
        assert_eq!(theme.metadata.name, "default");
        assert_eq!(theme.metadata.version, "0.1.0");
        assert!(
            theme
                .templates_dir
                .ends_with(Path::new("themes/default/templates"))
        );
    }

    #[test]
    fn errors_when_required_template_is_missing() {
        let dir = tempdir().expect("tempdir should be created");
        let project_root = dir.path();

        let theme_root = project_root.join("themes/default");
        fs::create_dir_all(theme_root.join("templates")).expect("templates dir should be created");
        fs::create_dir_all(theme_root.join("static")).expect("static dir should be created");

        fs::write(
            theme_root.join("theme.toml"),
            "name = \"default\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Default\"\n",
        )
        .expect("theme metadata should be written");

        let error = load_active_theme(project_root, "default")
            .expect_err("missing required template should fail");
        assert!(
            error
                .to_string()
                .contains("required theme template missing"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn lists_installed_themes_from_theme_toml() {
        let dir = tempdir().expect("tempdir should be created");
        let project_root = dir.path();

        let default_theme = project_root.join("themes/default");
        fs::create_dir_all(&default_theme).expect("default theme should be created");
        fs::write(
            default_theme.join("theme.toml"),
            "name = \"default\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Default theme\"\n",
        )
        .expect("default theme metadata should be written");

        let dark_theme = project_root.join("themes/dark");
        fs::create_dir_all(&dark_theme).expect("dark theme should be created");
        fs::write(
            dark_theme.join("theme.toml"),
            "name = \"dark\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Dark theme\"\n",
        )
        .expect("dark theme metadata should be written");

        let themes = list_installed_themes(project_root).expect("theme listing should succeed");
        assert_eq!(themes.len(), 2);
        assert_eq!(themes[0].metadata.name, "dark");
        assert_eq!(themes[1].metadata.name, "default");
    }
}
