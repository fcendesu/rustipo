use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::palette::models::Palette;

pub fn write_palette_css(dist_dir: impl AsRef<Path>, palette: &Palette) -> Result<()> {
    let dist_dir = dist_dir.as_ref();
    fs::create_dir_all(dist_dir)
        .with_context(|| format!("failed to create output directory: {}", dist_dir.display()))?;

    let css = crate::palette::loader::render_palette_css(palette);
    fs::write(dist_dir.join("palette.css"), css).with_context(|| {
        format!(
            "failed to write palette css: {}",
            dist_dir.join("palette.css").display()
        )
    })?;

    Ok(())
}

pub fn ensure_palette_output_path_available(
    user_static_dir: impl AsRef<Path>,
    theme_static_dirs: &[std::path::PathBuf],
) -> Result<()> {
    let user_palette_css = user_static_dir.as_ref().join("palette.css");
    if user_palette_css.is_file() {
        anyhow::bail!(
            "generated palette output would collide with user asset: {}",
            user_palette_css.display()
        );
    }

    for theme_static_dir in theme_static_dirs {
        let theme_palette_css = theme_static_dir.join("palette.css");
        if theme_palette_css.is_file() {
            anyhow::bail!(
                "generated palette output would collide with theme asset: {}",
                theme_palette_css.display()
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::palette::loader::load_palette;

    use super::{ensure_palette_output_path_available, write_palette_css};

    #[test]
    fn writes_generated_palette_css() {
        let dir = tempdir().expect("tempdir should be created");
        let palette = load_palette(dir.path(), "default").expect("palette should load");

        write_palette_css(dir.path().join("dist"), &palette).expect("palette css should write");

        let css = fs::read_to_string(dir.path().join("dist/palette.css"))
            .expect("palette css should exist");
        assert!(css.contains("--rustipo-bg: #ffffff;"));
    }

    #[test]
    fn fails_when_user_static_palette_css_exists() {
        let dir = tempdir().expect("tempdir should be created");
        let static_dir = dir.path().join("static");
        fs::create_dir_all(&static_dir).expect("static dir should be created");
        fs::write(static_dir.join("palette.css"), "custom").expect("palette css should exist");

        let error = ensure_palette_output_path_available(&static_dir, &[])
            .expect_err("collision should fail");
        assert!(
            error
                .to_string()
                .contains("generated palette output would collide")
        );
    }
}
