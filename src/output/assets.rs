use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use walkdir::WalkDir;

pub fn copy_assets_with_collision_check(
    user_static_dir: impl AsRef<Path>,
    theme_static_dirs: &[PathBuf],
    dist_dir: impl AsRef<Path>,
) -> Result<usize> {
    let dist_dir = dist_dir.as_ref();
    let prepared = prepare_asset_maps(user_static_dir.as_ref(), theme_static_dirs)?;

    let mut copied = 0;
    copied += copy_files_from_map(dist_dir, &prepared.theme_files)?;
    copied += copy_files(prepared.user_static_dir, dist_dir, &prepared.user_files)?;

    Ok(copied)
}

pub fn validate_assets_with_collision_check(
    user_static_dir: impl AsRef<Path>,
    theme_static_dirs: &[PathBuf],
) -> Result<usize> {
    let prepared = prepare_asset_maps(user_static_dir.as_ref(), theme_static_dirs)?;
    Ok(prepared.user_files.len() + prepared.theme_files.len())
}

fn copy_files_from_map(dist_dir: &Path, files: &BTreeMap<PathBuf, PathBuf>) -> Result<usize> {
    let mut copied = 0;
    for (rel, src) in files {
        let dst = dist_dir.join(rel);
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create asset output path: {}", parent.display())
            })?;
        }

        fs::copy(src, &dst).with_context(|| {
            format!(
                "failed to copy asset from '{}' to '{}'",
                src.display(),
                dst.display()
            )
        })?;
        copied += 1;
    }

    Ok(copied)
}

fn collect_relative_files(root: &Path) -> Result<HashSet<PathBuf>> {
    if !root.exists() {
        return Ok(HashSet::new());
    }

    let mut files = HashSet::new();
    for entry in WalkDir::new(root) {
        let entry =
            entry.with_context(|| format!("failed walking assets dir: {}", root.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }

        let rel = entry
            .path()
            .strip_prefix(root)
            .with_context(|| {
                format!(
                    "failed making relative asset path: {}",
                    entry.path().display()
                )
            })?
            .to_path_buf();
        if crate::output::styles::should_skip_asset_copy(&rel) {
            continue;
        }
        files.insert(rel);
    }

    Ok(files)
}

struct PreparedAssetMaps<'a> {
    user_static_dir: &'a Path,
    user_files: HashSet<PathBuf>,
    theme_files: BTreeMap<PathBuf, PathBuf>,
}

fn prepare_asset_maps<'a>(
    user_static_dir: &'a Path,
    theme_static_dirs: &[PathBuf],
) -> Result<PreparedAssetMaps<'a>> {
    let user_files = collect_relative_files(user_static_dir)?;
    let theme_style_is_compiled = crate::output::styles::theme_style_uses_scss(theme_static_dirs)?;
    let user_custom_is_compiled = crate::output::styles::user_custom_uses_scss(user_static_dir)?;
    let mut theme_files = BTreeMap::new();
    for theme_dir in theme_static_dirs {
        let rel_files = collect_relative_files(theme_dir)?;
        for rel in rel_files {
            if theme_style_is_compiled && rel == Path::new("style.css") {
                continue;
            }
            theme_files.insert(rel.clone(), theme_dir.join(rel));
        }
    }

    if theme_style_is_compiled && user_files.contains(Path::new("style.css")) {
        bail!(
            "asset path collision detected: style.css is reserved for compiled theme SCSS output"
        );
    }

    if user_custom_is_compiled && theme_files.contains_key(Path::new("custom.css")) {
        bail!(
            "asset path collision detected: custom.css is reserved for compiled site SCSS output"
        );
    }

    if let Some(rel) = user_files.iter().find(|rel| theme_files.contains_key(*rel)) {
        bail!("asset path collision detected: {}", rel.display());
    }

    Ok(PreparedAssetMaps {
        user_static_dir,
        user_files,
        theme_files,
    })
}

fn copy_files(root: &Path, dist_dir: &Path, files: &HashSet<PathBuf>) -> Result<usize> {
    let mut copied = 0;
    for rel in files {
        let src = root.join(rel);
        let dst = dist_dir.join(rel);

        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create asset output path: {}", parent.display())
            })?;
        }

        fs::copy(&src, &dst).with_context(|| {
            format!(
                "failed to copy asset from '{}' to '{}'",
                src.display(),
                dst.display()
            )
        })?;
        copied += 1;
    }

    Ok(copied)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::copy_assets_with_collision_check;

    #[test]
    fn copies_theme_and_user_assets() {
        let dir = tempdir().expect("tempdir should be created");
        let root = dir.path();

        let user_static = root.join("static");
        let theme_static = root.join("themes/default/static");
        let dist = root.join("dist");

        fs::create_dir_all(&user_static).expect("user static should be created");
        fs::create_dir_all(theme_static.join("css")).expect("theme static should be created");

        fs::write(user_static.join("avatar.png"), "png").expect("user asset should be written");
        fs::write(theme_static.join("css/style.css"), "body{}")
            .expect("theme asset should be written");

        let copied = copy_assets_with_collision_check(
            &user_static,
            std::slice::from_ref(&theme_static),
            &dist,
        )
        .expect("asset copy should succeed");

        assert_eq!(copied, 2);
        assert!(dist.join("avatar.png").is_file());
        assert!(dist.join("css/style.css").is_file());
    }

    #[test]
    fn fails_on_asset_collision() {
        let dir = tempdir().expect("tempdir should be created");
        let root = dir.path();

        let user_static = root.join("static");
        let theme_static = root.join("themes/default/static");
        let dist = root.join("dist");

        fs::create_dir_all(&user_static).expect("user static should be created");
        fs::create_dir_all(&theme_static).expect("theme static should be created");

        fs::write(user_static.join("shared.css"), "user").expect("user asset should be written");
        fs::write(theme_static.join("shared.css"), "theme").expect("theme asset should be written");

        let error = copy_assets_with_collision_check(
            &user_static,
            std::slice::from_ref(&theme_static),
            &dist,
        )
        .expect_err("collision should fail");

        assert!(error.to_string().contains("asset path collision detected"));
    }

    #[test]
    fn child_theme_assets_override_parent_assets() {
        let dir = tempdir().expect("tempdir should be created");
        let root = dir.path();

        let user_static = root.join("static");
        let parent_static = root.join("themes/base/static");
        let child_static = root.join("themes/child/static");
        let dist = root.join("dist");

        fs::create_dir_all(&parent_static).expect("parent static should be created");
        fs::create_dir_all(&child_static).expect("child static should be created");

        fs::write(parent_static.join("style.css"), "base").expect("base asset should be written");
        fs::write(child_static.join("style.css"), "child").expect("child asset should be written");

        let copied = copy_assets_with_collision_check(
            &user_static,
            &[parent_static.clone(), child_static.clone()],
            &dist,
        )
        .expect("asset copy should succeed");

        assert_eq!(copied, 1);
        let written = fs::read_to_string(dist.join("style.css")).expect("style should exist");
        assert_eq!(written, "child");
    }

    #[test]
    fn skips_scss_sources_when_copying_assets() {
        let dir = tempdir().expect("tempdir should be created");
        let root = dir.path();

        let user_static = root.join("static");
        let theme_static = root.join("themes/default/static");
        let dist = root.join("dist");

        fs::create_dir_all(&user_static).expect("user static should be created");
        fs::create_dir_all(&theme_static).expect("theme static should be created");

        fs::write(user_static.join("custom.scss"), "body { color: red; }")
            .expect("scss should be written");
        fs::write(theme_static.join("style.scss"), "body { color: blue; }")
            .expect("scss should be written");
        fs::write(theme_static.join("logo.svg"), "<svg/>").expect("svg should be written");

        let copied = copy_assets_with_collision_check(
            &user_static,
            std::slice::from_ref(&theme_static),
            &dist,
        )
        .expect("asset copy should succeed");

        assert_eq!(copied, 1);
        assert!(dist.join("logo.svg").is_file());
        assert!(!dist.join("custom.scss").exists());
        assert!(!dist.join("style.scss").exists());
    }

    #[test]
    fn child_theme_scss_overrides_parent_style_css() {
        let dir = tempdir().expect("tempdir should be created");
        let root = dir.path();

        let user_static = root.join("static");
        let parent_static = root.join("themes/base/static");
        let child_static = root.join("themes/child/static");
        let dist = root.join("dist");

        fs::create_dir_all(&parent_static).expect("parent static should be created");
        fs::create_dir_all(&child_static).expect("child static should be created");

        fs::write(parent_static.join("style.css"), "base").expect("base asset should be written");
        fs::write(child_static.join("style.scss"), "body { color: blue; }")
            .expect("child scss should be written");

        let copied = copy_assets_with_collision_check(
            &user_static,
            &[parent_static.clone(), child_static.clone()],
            &dist,
        )
        .expect("asset copy should succeed");

        assert_eq!(copied, 0);
        assert!(!dist.join("style.css").exists());
    }

    #[test]
    fn fails_when_user_style_css_conflicts_with_compiled_theme_scss() {
        let dir = tempdir().expect("tempdir should be created");
        let root = dir.path();

        let user_static = root.join("static");
        let theme_static = root.join("themes/default/static");
        let dist = root.join("dist");

        fs::create_dir_all(&user_static).expect("user static should be created");
        fs::create_dir_all(&theme_static).expect("theme static should be created");

        fs::write(user_static.join("style.css"), "user").expect("user css should be written");
        fs::write(theme_static.join("style.scss"), "body { color: blue; }")
            .expect("theme scss should be written");

        let error = copy_assets_with_collision_check(
            &user_static,
            std::slice::from_ref(&theme_static),
            &dist,
        )
        .expect_err("collision should fail");

        assert!(
            error
                .to_string()
                .contains("style.css is reserved for compiled theme SCSS output")
        );
    }
}
