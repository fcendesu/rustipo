use std::collections::HashSet;
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
    let mut layers = Vec::new();
    let mut visited = HashSet::new();
    let mut current = theme_name.to_string();

    loop {
        if !visited.insert(current.clone()) {
            bail!("theme inheritance cycle detected at: {current}");
        }

        let layer = load_theme_layer(project_root, &current)?;
        let next = layer.metadata.extends.clone();
        layers.push(layer);

        match next {
            Some(parent) => current = parent,
            None => break,
        }
    }

    layers.reverse();
    validate_required_templates(&layers)?;

    let template_dirs = layers
        .iter()
        .map(|layer| layer.templates_dir.clone())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    let static_dirs = layers
        .iter()
        .map(|layer| layer.static_dir.clone())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();

    let active_layer = layers
        .last()
        .context("theme inheritance resolution produced no active layer")?;

    Ok(Theme {
        root_dir: active_layer.root_dir.clone(),
        template_dirs,
        static_dirs,
        metadata: active_layer.metadata.clone(),
    })
}

fn validate_required_templates(layers: &[ThemeLayer]) -> Result<()> {
    for template in REQUIRED_TEMPLATES {
        let exists = layers
            .iter()
            .rev()
            .any(|layer| layer.templates_dir.join(template).is_file());
        if !exists {
            bail!("required theme template missing in inheritance chain: {template}");
        }
    }

    Ok(())
}

#[derive(Debug)]
struct ThemeLayer {
    root_dir: std::path::PathBuf,
    templates_dir: std::path::PathBuf,
    static_dir: std::path::PathBuf,
    metadata: ThemeMetadata,
}

fn load_theme_layer(project_root: &Path, theme_name: &str) -> Result<ThemeLayer> {
    let root_dir = project_root.join("themes").join(theme_name);
    if !root_dir.is_dir() {
        bail!("active theme not found: {}", root_dir.display());
    }

    let metadata_path = root_dir.join("theme.toml");
    let metadata_raw = fs::read_to_string(&metadata_path)
        .with_context(|| format!("failed to read theme metadata: {}", metadata_path.display()))?;
    let metadata = toml::from_str::<ThemeMetadata>(&metadata_raw).with_context(|| {
        format!(
            "failed to parse theme metadata: {}",
            metadata_path.display()
        )
    })?;

    Ok(ThemeLayer {
        templates_dir: root_dir.join("templates"),
        static_dir: root_dir.join("static"),
        root_dir,
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
                .template_dirs
                .last()
                .expect("theme should have templates")
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
                .contains("required theme template missing in inheritance chain"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn loads_inherited_theme_and_resolves_parent_templates() {
        let dir = tempdir().expect("tempdir should be created");
        let project_root = dir.path();

        let base_root = project_root.join("themes/base");
        fs::create_dir_all(base_root.join("templates")).expect("base templates should be created");
        fs::create_dir_all(base_root.join("static")).expect("base static should be created");
        for template in [
            "base.html",
            "page.html",
            "post.html",
            "project.html",
            "section.html",
            "index.html",
        ] {
            fs::write(
                base_root.join("templates").join(template),
                "{{ content_html }}",
            )
            .expect("template should be written");
        }
        fs::write(base_root.join("static/style.css"), "body {}").expect("asset should be written");
        fs::write(
            base_root.join("theme.toml"),
            "name = \"base\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Base\"\n",
        )
        .expect("base metadata should be written");

        let child_root = project_root.join("themes/child");
        fs::create_dir_all(child_root.join("templates"))
            .expect("child templates should be created");
        fs::write(
            child_root.join("templates/post.html"),
            "<article>{{ content_html }}</article>",
        )
        .expect("child template should be written");
        fs::write(
            child_root.join("theme.toml"),
            "name = \"child\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Child\"\nextends = \"base\"\n",
        )
        .expect("child metadata should be written");

        let theme = load_active_theme(project_root, "child").expect("theme should load");
        assert_eq!(theme.template_dirs.len(), 2);
        assert_eq!(theme.static_dirs.len(), 1);
        assert_eq!(theme.metadata.name, "child");
        assert!(theme.template_dirs[0].ends_with(Path::new("themes/base/templates")));
        assert!(theme.template_dirs[1].ends_with(Path::new("themes/child/templates")));
    }

    #[test]
    fn errors_on_inheritance_cycle() {
        let dir = tempdir().expect("tempdir should be created");
        let project_root = dir.path();

        for (name, parent) in [("a", "b"), ("b", "a")] {
            let root = project_root.join("themes").join(name);
            fs::create_dir_all(root.join("templates")).expect("templates should be created");
            fs::create_dir_all(root.join("static")).expect("static should be created");
            fs::write(
                root.join("theme.toml"),
                format!(
                    "name = \"{name}\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"{name}\"\nextends = \"{parent}\"\n"
                ),
            )
            .expect("metadata should be written");
        }

        let error = load_active_theme(project_root, "a").expect_err("cycle should fail");
        assert!(
            error
                .to_string()
                .contains("theme inheritance cycle detected")
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
