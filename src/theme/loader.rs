use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::theme::models::{Theme, ThemeMetadata};

const REQUIRED_TEMPLATES: &[&str] = &[
    "base.html",
    "page.html",
    "post.html",
    "project.html",
    "section.html",
    "index.html",
];

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

    use super::load_active_theme;

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
}
