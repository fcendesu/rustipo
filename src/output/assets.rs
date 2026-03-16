use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use walkdir::WalkDir;

pub fn copy_assets_with_collision_check(
    user_static_dir: impl AsRef<Path>,
    theme_static_dir: impl AsRef<Path>,
    dist_dir: impl AsRef<Path>,
) -> Result<usize> {
    let user_static_dir = user_static_dir.as_ref();
    let theme_static_dir = theme_static_dir.as_ref();
    let dist_dir = dist_dir.as_ref();

    let user_files = collect_relative_files(user_static_dir)?;
    let theme_files = collect_relative_files(theme_static_dir)?;

    for rel in user_files.intersection(&theme_files) {
        bail!("asset path collision detected: {}", rel.display());
    }

    let mut copied = 0;
    copied += copy_files(theme_static_dir, dist_dir, &theme_files)?;
    copied += copy_files(user_static_dir, dist_dir, &user_files)?;

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
        files.insert(rel);
    }

    Ok(files)
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

        let copied = copy_assets_with_collision_check(&user_static, &theme_static, &dist)
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

        let error = copy_assets_with_collision_check(&user_static, &theme_static, &dist)
            .expect_err("collision should fail");

        assert!(error.to_string().contains("asset path collision detected"));
    }
}
