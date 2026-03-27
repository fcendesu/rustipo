mod processor;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use walkdir::WalkDir;

pub use processor::{ImageProcessor, OutputFormat, ResizeOperation, ResizeRequest};

pub const PROCESSED_IMAGES_DIR: &str = "processed-images";

pub fn ensure_output_path_available(
    user_static_dir: impl AsRef<Path>,
    theme_static_dirs: &[PathBuf],
) -> Result<()> {
    let user_dir = user_static_dir.as_ref().join(PROCESSED_IMAGES_DIR);
    if user_dir.exists() {
        anyhow::bail!(
            "generated processed image output would collide with user asset path: {}",
            user_dir.display()
        );
    }

    for theme_static_dir in theme_static_dirs {
        let theme_dir = theme_static_dir.join(PROCESSED_IMAGES_DIR);
        if theme_dir.exists() {
            anyhow::bail!(
                "generated processed image output would collide with theme asset path: {}",
                theme_dir.display()
            );
        }
    }

    Ok(())
}

pub fn copy_generated_outputs(
    generated_root: impl AsRef<Path>,
    dist_dir: impl AsRef<Path>,
) -> Result<usize> {
    let generated_root = generated_root.as_ref();
    let dist_dir = dist_dir.as_ref();
    let source_dir = generated_root.join(PROCESSED_IMAGES_DIR);
    if !source_dir.exists() {
        return Ok(0);
    }

    let mut copied = 0;
    for entry in WalkDir::new(&source_dir) {
        let entry = entry.with_context(|| {
            format!(
                "failed to walk generated processed image directory: {}",
                source_dir.display()
            )
        })?;
        if !entry.file_type().is_file() {
            continue;
        }

        let relative = entry.path().strip_prefix(generated_root).with_context(|| {
            format!(
                "failed to compute generated processed image relative path: {}",
                entry.path().display()
            )
        })?;
        let target = dist_dir.join(relative);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create processed image output directory: {}",
                    parent.display()
                )
            })?;
        }

        std::fs::copy(entry.path(), &target).with_context(|| {
            format!(
                "failed to copy processed image from '{}' to '{}'",
                entry.path().display(),
                target.display()
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

    use super::{copy_generated_outputs, ensure_output_path_available};

    #[test]
    fn fails_when_user_static_uses_processed_images_dir() {
        let dir = tempdir().expect("tempdir should be created");
        let static_dir = dir.path().join("static/processed-images");
        fs::create_dir_all(&static_dir).expect("static dir should be created");

        let error =
            ensure_output_path_available(dir.path().join("static"), &[]).expect_err("should fail");
        assert!(
            error
                .to_string()
                .contains("generated processed image output would collide")
        );
    }

    #[test]
    fn fails_when_theme_static_uses_processed_images_dir() {
        let dir = tempdir().expect("tempdir should be created");
        let theme_static = dir.path().join("themes/default/static/processed-images");
        fs::create_dir_all(&theme_static).expect("theme dir should be created");

        let error = ensure_output_path_available(
            dir.path().join("static"),
            &[dir.path().join("themes/default/static")],
        )
        .expect_err("should fail");
        assert!(
            error
                .to_string()
                .contains("generated processed image output would collide")
        );
    }

    #[test]
    fn copies_generated_processed_images_into_dist() {
        let dir = tempdir().expect("tempdir should be created");
        let generated_root = dir.path().join("generated");
        let dist_dir = dir.path().join("dist");
        fs::create_dir_all(generated_root.join("processed-images"))
            .expect("generated output dir should be created");
        fs::write(
            generated_root.join("processed-images/thumb.png"),
            "processed-image",
        )
        .expect("generated image should be written");

        let copied =
            copy_generated_outputs(&generated_root, &dist_dir).expect("generated outputs copy");

        assert_eq!(copied, 1);
        assert!(dist_dir.join("processed-images/thumb.png").is_file());
    }
}
