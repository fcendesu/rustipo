use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

const THEME_STYLE_CSS: &str = "style.css";
const THEME_STYLE_SCSS: &str = "style.scss";
const USER_CUSTOM_CSS: &str = "custom.css";
const USER_CUSTOM_SCSS: &str = "custom.scss";

pub fn compile_optional_scss(
    user_static_dir: impl AsRef<Path>,
    theme_static_dirs: &[PathBuf],
    dist_dir: impl AsRef<Path>,
) -> Result<usize> {
    let dist_dir = dist_dir.as_ref();
    let mut generated = 0;

    if let Some(source) = resolve_theme_style_source(theme_static_dirs)? {
        generated += compile_if_scss(&source, dist_dir.join(THEME_STYLE_CSS), "theme style")?;
    }

    if let Some(source) = resolve_user_custom_source(user_static_dir.as_ref())? {
        generated += compile_if_scss(&source, dist_dir.join(USER_CUSTOM_CSS), "custom style")?;
    }

    Ok(generated)
}

pub fn validate_optional_scss(
    user_static_dir: impl AsRef<Path>,
    theme_static_dirs: &[PathBuf],
) -> Result<usize> {
    let mut compiled = 0;

    if let Some(source) = resolve_theme_style_source(theme_static_dirs)? {
        compiled += validate_if_scss(&source, "theme style")?;
    }

    if let Some(source) = resolve_user_custom_source(user_static_dir.as_ref())? {
        compiled += validate_if_scss(&source, "custom style")?;
    }

    Ok(compiled)
}

pub fn should_skip_asset_copy(relative_path: &Path) -> bool {
    relative_path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("scss"))
}

pub fn has_custom_stylesheet(project_root: impl AsRef<Path>) -> bool {
    let static_dir = project_root.as_ref().join("static");
    static_dir.join(USER_CUSTOM_CSS).is_file() || static_dir.join(USER_CUSTOM_SCSS).is_file()
}

pub fn theme_style_uses_scss(theme_static_dirs: &[PathBuf]) -> Result<bool> {
    Ok(matches!(
        resolve_theme_style_source(theme_static_dirs)?,
        Some(StyleSource::Scss(_))
    ))
}

pub fn user_custom_uses_scss(user_static_dir: impl AsRef<Path>) -> Result<bool> {
    Ok(matches!(
        resolve_user_custom_source(user_static_dir.as_ref())?,
        Some(StyleSource::Scss(_))
    ))
}

#[derive(Clone)]
enum StyleSource {
    Css,
    Scss(PathBuf),
}

fn resolve_theme_style_source(theme_static_dirs: &[PathBuf]) -> Result<Option<StyleSource>> {
    let mut selected = None;

    for dir in theme_static_dirs {
        if let Some(source) =
            resolve_target_source(dir, THEME_STYLE_CSS, THEME_STYLE_SCSS, "theme style")?
        {
            selected = Some(source);
        }
    }

    Ok(selected)
}

fn resolve_user_custom_source(user_static_dir: &Path) -> Result<Option<StyleSource>> {
    resolve_target_source(
        user_static_dir,
        USER_CUSTOM_CSS,
        USER_CUSTOM_SCSS,
        "custom style",
    )
}

fn resolve_target_source(
    root: &Path,
    css_name: &str,
    scss_name: &str,
    label: &str,
) -> Result<Option<StyleSource>> {
    let css_path = root.join(css_name);
    let scss_path = root.join(scss_name);
    let has_css = css_path.is_file();
    let has_scss = scss_path.is_file();

    if has_css && has_scss {
        bail!(
            "{label} conflict: '{}' and '{}' both exist; keep plain CSS as the default path by using only one",
            css_path.display(),
            scss_path.display()
        );
    }

    if has_scss {
        return Ok(Some(StyleSource::Scss(scss_path)));
    }

    if has_css {
        let _ = css_path;
        return Ok(Some(StyleSource::Css));
    }

    Ok(None)
}

fn compile_if_scss(source: &StyleSource, dst: PathBuf, label: &str) -> Result<usize> {
    match source {
        StyleSource::Css => Ok(0),
        StyleSource::Scss(path) => {
            let css = compile_scss(path, label)?;
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!(
                        "failed to create compiled stylesheet output path: {}",
                        parent.display()
                    )
                })?;
            }
            fs::write(&dst, css).with_context(|| {
                format!(
                    "failed to write compiled stylesheet '{}' to '{}'",
                    path.display(),
                    dst.display()
                )
            })?;
            Ok(1)
        }
    }
}

fn validate_if_scss(source: &StyleSource, label: &str) -> Result<usize> {
    match source {
        StyleSource::Css => Ok(0),
        StyleSource::Scss(path) => {
            let _ = compile_scss(path, label)?;
            Ok(1)
        }
    }
}

fn compile_scss(path: &Path, label: &str) -> Result<String> {
    grass::from_path(path, &grass::Options::default())
        .with_context(|| format!("failed to compile {label} SCSS: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use super::{
        compile_optional_scss, has_custom_stylesheet, should_skip_asset_copy,
        theme_style_uses_scss, user_custom_uses_scss, validate_optional_scss,
    };

    #[test]
    fn compiles_theme_style_scss_to_style_css() {
        let dir = tempdir().expect("tempdir should be created");
        let theme_static = dir.path().join("themes/default/static");
        let dist = dir.path().join("dist");
        fs::create_dir_all(&theme_static).expect("theme static should be created");
        fs::write(theme_static.join("_tokens.scss"), "$bg: #123456;")
            .expect("scss partial should be written");
        fs::write(
            theme_static.join("style.scss"),
            "@use \"tokens\";\nbody { color: tokens.$bg; }",
        )
        .expect("scss file should be written");

        let generated = compile_optional_scss(dir.path().join("static"), &[theme_static], &dist)
            .expect("scss compilation should succeed");

        assert_eq!(generated, 1);
        let css = fs::read_to_string(dist.join("style.css")).expect("compiled css should exist");
        assert!(css.contains("color: #123456;"));
    }

    #[test]
    fn validates_scss_without_writing_output() {
        let dir = tempdir().expect("tempdir should be created");
        let theme_static = dir.path().join("themes/default/static");
        fs::create_dir_all(&theme_static).expect("theme static should be created");
        fs::write(theme_static.join("style.scss"), "body { color: red; }")
            .expect("scss file should be written");

        let validated = validate_optional_scss(dir.path().join("static"), &[theme_static])
            .expect("scss validation should succeed");

        assert_eq!(validated, 1);
        assert!(
            !dir.path().join("dist/style.css").exists(),
            "validation should not write compiled output"
        );
    }

    #[test]
    fn fails_when_css_and_scss_exist_for_same_target() {
        let dir = tempdir().expect("tempdir should be created");
        let static_dir = dir.path().join("static");
        fs::create_dir_all(&static_dir).expect("static dir should be created");
        fs::write(static_dir.join("custom.css"), "body{}").expect("css should be written");
        fs::write(static_dir.join("custom.scss"), "body { color: red; }")
            .expect("scss should be written");

        let error = validate_optional_scss(&static_dir, &[]).expect_err("conflict should fail");
        assert!(error.to_string().contains("custom style conflict"));
    }

    #[test]
    fn detects_custom_scss_as_custom_stylesheet() {
        let dir = tempdir().expect("tempdir should be created");
        let static_dir = dir.path().join("static");
        fs::create_dir_all(&static_dir).expect("static dir should be created");
        fs::write(static_dir.join("custom.scss"), "body { color: red; }")
            .expect("scss should be written");

        assert!(has_custom_stylesheet(dir.path()));
    }

    #[test]
    fn skips_scss_sources_when_copying_assets() {
        assert!(should_skip_asset_copy(Path::new("style.scss")));
        assert!(should_skip_asset_copy(Path::new("styles/_tokens.scss")));
        assert!(!should_skip_asset_copy(Path::new("style.css")));
    }

    #[test]
    fn detects_when_theme_style_output_is_reserved_for_scss() {
        let dir = tempdir().expect("tempdir should be created");
        let theme_static = dir.path().join("themes/default/static");
        fs::create_dir_all(&theme_static).expect("theme static should be created");
        fs::write(theme_static.join("style.scss"), "body { color: red; }")
            .expect("scss file should be written");

        assert!(theme_style_uses_scss(&[theme_static]).expect("detection should succeed"));
    }

    #[test]
    fn detects_when_user_custom_output_is_reserved_for_scss() {
        let dir = tempdir().expect("tempdir should be created");
        let static_dir = dir.path().join("static");
        fs::create_dir_all(&static_dir).expect("static dir should be created");
        fs::write(static_dir.join("custom.scss"), "body { color: red; }")
            .expect("scss file should be written");

        assert!(user_custom_uses_scss(&static_dir).expect("detection should succeed"));
    }
}
