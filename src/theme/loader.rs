use std::collections::{HashMap, HashSet};
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
    let mut themes = discover_theme_summaries(project_root)?;
    themes.sort_by(|a, b| {
        a.theme_id
            .cmp(&b.theme_id)
            .then_with(|| a.metadata.name.cmp(&b.metadata.name))
    });
    Ok(themes)
}

pub fn load_active_theme(project_root: impl AsRef<Path>, theme_name: &str) -> Result<Theme> {
    let project_root = project_root.as_ref();
    let discovered = discover_theme_summaries(project_root)?;
    let theme_index = build_theme_index(&discovered)?;
    let mut layers = Vec::new();
    let mut visited = HashSet::new();
    let mut current = resolve_theme_directory_name(theme_name, &theme_index)?.to_string();

    loop {
        if !visited.insert(current.clone()) {
            bail!("theme inheritance cycle detected at: {current}");
        }

        let layer = load_theme_layer(project_root, &current)?;
        let next = layer
            .metadata
            .extends
            .as_deref()
            .map(|parent| resolve_theme_directory_name(parent, &theme_index))
            .transpose()?
            .map(str::to_string);
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

fn discover_theme_summaries(project_root: impl AsRef<Path>) -> Result<Vec<ThemeSummary>> {
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
        let theme_id = resolved_theme_id(&metadata, &directory_name);
        validate_theme_id(&theme_id, &metadata_path)?;

        themes.push(ThemeSummary {
            theme_id,
            directory_name,
            metadata,
        });
    }

    ensure_unique_theme_ids(&themes)?;
    Ok(themes)
}

fn resolved_theme_id(metadata: &ThemeMetadata, directory_name: &str) -> String {
    metadata
        .id
        .clone()
        .unwrap_or_else(|| directory_name.to_string())
}

fn validate_theme_id(theme_id: &str, metadata_path: &Path) -> Result<()> {
    if theme_id.is_empty() {
        bail!("theme id cannot be empty: {}", metadata_path.display());
    }
    if !theme_id
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
    {
        bail!(
            "theme id must use lowercase kebab-case ASCII: {}",
            metadata_path.display()
        );
    }
    if theme_id.starts_with('-') || theme_id.ends_with('-') || theme_id.contains("--") {
        bail!(
            "theme id must use lowercase kebab-case ASCII: {}",
            metadata_path.display()
        );
    }

    Ok(())
}

fn ensure_unique_theme_ids(themes: &[ThemeSummary]) -> Result<()> {
    let mut seen = HashMap::new();
    for theme in themes {
        if let Some(existing_dir) =
            seen.insert(theme.theme_id.clone(), theme.directory_name.clone())
        {
            bail!(
                "duplicate theme id '{}' found in '{}' and '{}'",
                theme.theme_id,
                existing_dir,
                theme.directory_name
            );
        }
    }
    Ok(())
}

fn build_theme_index(themes: &[ThemeSummary]) -> Result<HashMap<&str, &str>> {
    let mut index = HashMap::new();
    for theme in themes {
        index.insert(theme.directory_name.as_str(), theme.directory_name.as_str());
        if let Some(existing) = index.insert(theme.theme_id.as_str(), theme.directory_name.as_str())
            && existing != theme.directory_name
        {
            bail!(
                "theme reference '{}' is ambiguous between '{}' and '{}'",
                theme.theme_id,
                existing,
                theme.directory_name
            );
        }
    }
    Ok(index)
}

fn resolve_theme_directory_name<'a>(
    theme_reference: &str,
    index: &'a HashMap<&'a str, &'a str>,
) -> Result<&'a str> {
    index
        .get(theme_reference)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("theme not found by id or directory: {theme_reference}"))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use super::{REQUIRED_TEMPLATES, list_installed_themes, load_active_theme};

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
            "id = \"tokyonight-storm\"\nname = \"child\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Child\"\nextends = \"base\"\n",
        )
        .expect("child metadata should be written");

        let theme = load_active_theme(project_root, "tokyonight-storm").expect("theme should load");
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
            "id = \"catppuccin-mocha\"\nname = \"Catppuccin Mocha\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Catppuccin variant\"\n",
        )
        .expect("default theme metadata should be written");

        let dark_theme = project_root.join("themes/dark");
        fs::create_dir_all(&dark_theme).expect("dark theme should be created");
        fs::write(
            dark_theme.join("theme.toml"),
            "id = \"tokyonight-storm\"\nname = \"Tokyo Night Storm\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Tokyo Night variant\"\n",
        )
        .expect("dark theme metadata should be written");

        let themes = list_installed_themes(project_root).expect("theme listing should succeed");
        assert_eq!(themes.len(), 2);
        assert_eq!(themes[0].theme_id, "catppuccin-mocha");
        assert_eq!(themes[0].metadata.name, "Catppuccin Mocha");
        assert_eq!(themes[1].theme_id, "tokyonight-storm");
        assert_eq!(themes[1].metadata.name, "Tokyo Night Storm");
    }

    #[test]
    fn falls_back_to_directory_name_when_theme_id_is_missing() {
        let dir = tempdir().expect("tempdir should be created");
        let project_root = dir.path();

        let theme_root = project_root.join("themes/default");
        fs::create_dir_all(theme_root.join("templates")).expect("templates should be created");
        fs::create_dir_all(theme_root.join("static")).expect("static should be created");
        for template in REQUIRED_TEMPLATES {
            fs::write(
                theme_root.join("templates").join(template),
                "{{ content_html }}",
            )
            .expect("template should be written");
        }
        fs::write(
            theme_root.join("theme.toml"),
            "name = \"Default\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Default\"\n",
        )
        .expect("metadata should be written");

        let theme = load_active_theme(project_root, "default").expect("theme should load");
        assert_eq!(theme.metadata.name, "Default");
    }

    #[test]
    fn errors_on_duplicate_theme_ids() {
        let dir = tempdir().expect("tempdir should be created");
        let project_root = dir.path();

        for directory in ["one", "two"] {
            let theme_root = project_root.join("themes").join(directory);
            fs::create_dir_all(&theme_root).expect("theme dir should be created");
            fs::write(
                theme_root.join("theme.toml"),
                "id = \"catppuccin-mocha\"\nname = \"Theme\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Theme\"\n",
            )
            .expect("metadata should be written");
        }

        let error = list_installed_themes(project_root).expect_err("duplicate id should fail");
        assert!(error.to_string().contains("duplicate theme id"));
    }

    #[test]
    fn errors_on_invalid_theme_id() {
        let dir = tempdir().expect("tempdir should be created");
        let project_root = dir.path();

        let theme_root = project_root.join("themes/default");
        fs::create_dir_all(&theme_root).expect("theme dir should be created");
        fs::write(
            theme_root.join("theme.toml"),
            "id = \"Tokyo Night\"\nname = \"Theme\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Theme\"\n",
        )
        .expect("metadata should be written");

        let error = list_installed_themes(project_root).expect_err("invalid id should fail");
        assert!(
            error
                .to_string()
                .contains("theme id must use lowercase kebab-case ASCII")
        );
    }
}
