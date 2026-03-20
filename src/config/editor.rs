use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use toml_edit::{DocumentMut, value};

pub fn set_top_level_string(path: impl AsRef<Path>, key: &str, new_value: &str) -> Result<()> {
    let path = path.as_ref();
    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    let mut document = raw.parse::<DocumentMut>().with_context(|| {
        format!(
            "failed to parse config file for editing: {}",
            path.display()
        )
    })?;

    document[key] = value(new_value);

    fs::write(path, document.to_string())
        .with_context(|| format!("failed to write config file: {}", path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::set_top_level_string;

    #[test]
    fn updates_existing_top_level_key() {
        let dir = tempdir().expect("tempdir should be created");
        let path = dir.path().join("config.toml");
        fs::write(
            &path,
            "title = \"Rustipo\"\npalette = \"default\"\n[site.layout]\ncontent_width = \"98%\"\n",
        )
        .expect("config should be written");

        set_top_level_string(&path, "palette", "catppuccin-mocha")
            .expect("palette should be updated");

        let updated = fs::read_to_string(&path).expect("config should be readable");
        assert!(updated.contains("palette = \"catppuccin-mocha\""));
        assert!(updated.contains("[site.layout]"));
    }

    #[test]
    fn inserts_missing_top_level_key() {
        let dir = tempdir().expect("tempdir should be created");
        let path = dir.path().join("config.toml");
        fs::write(&path, "title = \"Rustipo\"\n").expect("config should be written");

        set_top_level_string(&path, "palette", "default").expect("palette should be inserted");

        let updated = fs::read_to_string(&path).expect("config should be readable");
        assert!(updated.contains("palette = \"default\""));
    }
}
