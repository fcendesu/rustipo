use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::palette::models::{Palette, PaletteSource, PaletteSummary};

const BUILTIN_PALETTE_FILES: &[(&str, &str)] = &[
    (
        "default",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/palettes/default.toml"
        )),
    ),
    (
        "catppuccin-frappe",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/palettes/catppuccin-frappe.toml"
        )),
    ),
    (
        "catppuccin-latte",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/palettes/catppuccin-latte.toml"
        )),
    ),
    (
        "catppuccin-macchiato",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/palettes/catppuccin-macchiato.toml"
        )),
    ),
    (
        "catppuccin-mocha",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/palettes/catppuccin-mocha.toml"
        )),
    ),
    (
        "tokyonight-storm",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/palettes/tokyonight-storm.toml"
        )),
    ),
    (
        "tokyonight-moon",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/palettes/tokyonight-moon.toml"
        )),
    ),
];

pub fn list_available_palettes(project_root: impl AsRef<Path>) -> Result<Vec<PaletteSummary>> {
    let registry = build_palette_registry(project_root.as_ref())?;
    Ok(registry
        .values()
        .map(|entry| PaletteSummary {
            id: entry.palette.id.clone(),
            name: entry.palette.name.clone(),
            description: entry.palette.description.clone(),
            source: entry.source,
        })
        .collect())
}

pub fn load_palette(project_root: impl AsRef<Path>, palette_id: &str) -> Result<Palette> {
    let registry = build_palette_registry(project_root.as_ref())?;
    registry
        .get(palette_id)
        .map(|entry| entry.palette.clone())
        .ok_or_else(|| anyhow::anyhow!("palette not found: {palette_id}"))
}

fn build_palette_registry(project_root: &Path) -> Result<BTreeMap<String, PaletteEntry>> {
    let mut registry = BTreeMap::new();

    for (label, raw) in BUILTIN_PALETTE_FILES {
        let palette = parse_palette(raw, label, PaletteSource::BuiltIn)?;
        registry.insert(
            palette.id.clone(),
            PaletteEntry {
                palette,
                source: PaletteSource::BuiltIn,
            },
        );
    }

    for local in discover_local_palettes(project_root)? {
        registry.insert(local.palette.id.clone(), local);
    }

    Ok(registry)
}

fn discover_local_palettes(project_root: &Path) -> Result<Vec<PaletteEntry>> {
    let palettes_dir = project_root.join("palettes");
    if !palettes_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(&palettes_dir).with_context(|| {
        format!(
            "failed to read palettes directory: {}",
            palettes_dir.display()
        )
    })? {
        let entry = entry.with_context(|| {
            format!(
                "failed to read palettes directory entry: {}",
                palettes_dir.display()
            )
        })?;
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
            continue;
        }

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read palette file: {}", path.display()))?;
        let label = path
            .file_stem()
            .and_then(|name| name.to_str())
            .context("invalid palette file name")?;
        let palette = parse_palette(&raw, label, PaletteSource::Local)?;
        entries.push(PaletteEntry {
            palette,
            source: PaletteSource::Local,
        });
    }

    Ok(entries)
}

fn parse_palette(raw: &str, label: &str, source: PaletteSource) -> Result<Palette> {
    let palette = toml::from_str::<Palette>(raw)
        .with_context(|| format!("failed to parse {} palette '{}'", source.as_label(), label))?;
    validate_palette(&palette, label, source)?;
    Ok(palette)
}

fn validate_palette(palette: &Palette, label: &str, source: PaletteSource) -> Result<()> {
    if palette.id.trim().is_empty() {
        bail!("{} palette '{}' has empty id", source.as_label(), label);
    }
    if palette.id != label {
        bail!(
            "{} palette '{}' must match directory/asset name '{}'",
            source.as_label(),
            palette.id,
            label
        );
    }
    if !palette
        .id
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
        || palette.id.starts_with('-')
        || palette.id.ends_with('-')
        || palette.id.contains("--")
    {
        bail!(
            "{} palette '{}' must use lowercase kebab-case ASCII",
            source.as_label(),
            palette.id
        );
    }
    if !matches!(palette.color_scheme.as_str(), "light" | "dark") {
        bail!(
            "{} palette '{}' color_scheme must be 'light' or 'dark'",
            source.as_label(),
            palette.id
        );
    }

    for token_name in palette.extra_tokens.keys() {
        if token_name.trim().is_empty() {
            bail!(
                "{} palette '{}' contains an empty token name",
                source.as_label(),
                palette.id
            );
        }
        if !token_name
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
        {
            bail!(
                "{} palette '{}' token '{}' must use lowercase kebab-case ASCII",
                source.as_label(),
                palette.id,
                token_name
            );
        }
    }

    Ok(())
}

struct PaletteEntry {
    palette: Palette,
    source: PaletteSource,
}

pub fn render_palette_css(palette: &Palette) -> String {
    let mut css = format!(
        ":root {{\n  color-scheme: {};\n  --rustipo-bg: {};\n  --rustipo-text: {};\n  --rustipo-surface-muted: {};\n  --rustipo-border: {};\n  --rustipo-blockquote-border: {};\n  --rustipo-link: {};\n  --rustipo-link-hover: {};\n  --rustipo-code-bg: {};\n  --rustipo-code-text: {};\n  --rustipo-table-header-bg: {};\n}}\n",
        palette.color_scheme,
        palette.bg,
        palette.text,
        palette.surface_muted,
        palette.border,
        palette.blockquote_border,
        palette.link,
        palette.link_hover,
        palette.code_bg,
        palette.code_text,
        palette.table_header_bg
    );

    if !palette.extra_tokens.is_empty() {
        let insert_at = css
            .rfind("}\n")
            .expect("palette css root block should end with closing brace");
        let mut token_css = String::new();
        for (token_name, value) in &palette.extra_tokens {
            token_css.push_str(&format!("  --rustipo-token-{}: {};\n", token_name, value));
        }
        css.insert_str(insert_at, &token_css);
    }

    css
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{list_available_palettes, load_palette, render_palette_css};

    #[test]
    fn lists_builtin_palettes() {
        let dir = tempdir().expect("tempdir should be created");

        let palettes = list_available_palettes(dir.path()).expect("palette listing should succeed");
        let ids = palettes.into_iter().map(|item| item.id).collect::<Vec<_>>();

        assert!(ids.contains(&"default".to_string()));
        assert!(ids.contains(&"catppuccin-frappe".to_string()));
        assert!(ids.contains(&"catppuccin-macchiato".to_string()));
        assert!(ids.contains(&"catppuccin-mocha".to_string()));
        assert!(ids.contains(&"tokyonight-storm".to_string()));
    }

    #[test]
    fn local_palette_overrides_builtin_with_same_id() {
        let dir = tempdir().expect("tempdir should be created");
        let palette_dir = dir.path().join("palettes");
        fs::create_dir_all(&palette_dir).expect("palette dir should be created");
        fs::write(
            palette_dir.join("default.toml"),
            "id = \"default\"\nname = \"Custom Default\"\ndescription = \"Override\"\ncolor_scheme = \"dark\"\nbg = \"#000000\"\ntext = \"#ffffff\"\nsurface_muted = \"#111111\"\nborder = \"#222222\"\nblockquote_border = \"#333333\"\nlink = \"#444444\"\nlink_hover = \"#555555\"\ncode_bg = \"#666666\"\ncode_text = \"#777777\"\ntable_header_bg = \"#888888\"\n",
        )
        .expect("palette file should be written");

        let palette = load_palette(dir.path(), "default").expect("palette should load");
        assert_eq!(palette.name, "Custom Default");
        assert_eq!(palette.bg, "#000000");
    }

    #[test]
    fn renders_palette_css_variables() {
        let dir = tempdir().expect("tempdir should be created");
        let palette = load_palette(dir.path(), "catppuccin-mocha").expect("palette should load");

        let css = render_palette_css(&palette);
        assert!(css.contains("--rustipo-bg: #1e1e2e;"));
        assert!(css.contains("color-scheme: dark;"));
        assert!(css.contains("--rustipo-token-rosewater: #f5e0dc;"));
        assert!(css.contains("--rustipo-token-crust: #11111b;"));
    }
}
