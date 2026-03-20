use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TypographyOptions {
    pub line_height: Option<String>,
    pub body_font: Option<String>,
    pub heading_font: Option<String>,
    pub mono_font: Option<String>,
    #[serde(default)]
    pub font_faces: Vec<FontFaceConfig>,
}

#[derive(Debug, Deserialize)]
pub struct FontFaceConfig {
    pub family: String,
    pub source: String,
    pub weight: Option<String>,
    pub style: Option<String>,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SiteFontOptions {
    pub body_font: String,
    pub heading_font: String,
    pub mono_font: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedFontFace {
    pub family: String,
    pub source: String,
    pub weight: String,
    pub style: String,
    pub display: String,
    pub format: Option<String>,
}

pub fn resolve_font_options(
    typography: Option<&TypographyOptions>,
    project_root: &Path,
    theme_static_dirs: &[PathBuf],
) -> Result<(SiteFontOptions, Vec<ResolvedFontFace>)> {
    let site_fonts = site_font_options(typography);

    let font_faces = typography
        .map(|t| {
            t.font_faces
                .iter()
                .map(|face| resolve_font_face(face, project_root, theme_static_dirs))
                .collect::<Result<Vec<_>>>()
        })
        .transpose()?
        .unwrap_or_default();

    Ok((site_fonts, font_faces))
}

pub fn site_font_options(typography: Option<&TypographyOptions>) -> SiteFontOptions {
    let body_font = css_font_value_or_default(
        typography.and_then(|t| t.body_font.as_deref()),
        "ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, \"Segoe UI\", sans-serif",
    );
    let heading_font = css_font_value_or_default(
        typography.and_then(|t| t.heading_font.as_deref()),
        &body_font,
    );
    let mono_font = css_font_value_or_default(
        typography.and_then(|t| t.mono_font.as_deref()),
        "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace",
    );

    SiteFontOptions {
        body_font,
        heading_font,
        mono_font,
    }
}

pub fn render_font_faces_css(font_faces: &[ResolvedFontFace]) -> String {
    let mut css = String::new();

    for face in font_faces {
        css.push_str("@font-face {\n");
        css.push_str(&format!(
            "  font-family: {};\n",
            quote_css_string(&face.family)
        ));
        css.push_str(&format!("  src: url({})", quote_css_string(&face.source)));
        if let Some(format) = &face.format {
            css.push_str(&format!(" format({})", quote_css_string(format)));
        }
        css.push_str(";\n");
        css.push_str(&format!("  font-weight: {};\n", face.weight));
        css.push_str(&format!("  font-style: {};\n", face.style));
        css.push_str(&format!("  font-display: {};\n", face.display));
        css.push_str("}\n");
    }

    css
}

fn resolve_font_face(
    face: &FontFaceConfig,
    project_root: &Path,
    theme_static_dirs: &[PathBuf],
) -> Result<ResolvedFontFace> {
    let family = face.family.trim();
    if family.is_empty() {
        bail!("font face family cannot be empty");
    }

    let source = face.source.trim();
    if source.is_empty() {
        bail!("font face source cannot be empty for family '{family}'");
    }

    let source = normalize_font_source(source);
    if is_local_asset_source(&source)
        && !font_asset_exists(project_root, theme_static_dirs, &source)
    {
        bail!("font asset not found for family '{}': '{}'", family, source);
    }

    Ok(ResolvedFontFace {
        family: family.to_string(),
        source: source.clone(),
        weight: css_value_or_default(face.weight.as_deref(), "400"),
        style: css_value_or_default(face.style.as_deref(), "normal"),
        display: css_value_or_default(face.display.as_deref(), "swap"),
        format: infer_font_format(&source),
    })
}

fn font_asset_exists(project_root: &Path, theme_static_dirs: &[PathBuf], source: &str) -> bool {
    let rel = source.trim_start_matches('/');
    let site_candidate = project_root.join("static").join(rel);
    if site_candidate.is_file() {
        return true;
    }

    theme_static_dirs.iter().any(|dir| dir.join(rel).is_file())
}

fn normalize_font_source(value: &str) -> String {
    if value.starts_with("http://") || value.starts_with("https://") || value.starts_with("data:") {
        return value.to_string();
    }

    if value.starts_with('/') {
        value.to_string()
    } else {
        format!("/{value}")
    }
}

fn is_local_asset_source(value: &str) -> bool {
    !(value.starts_with("http://") || value.starts_with("https://") || value.starts_with("data:"))
}

fn css_font_value_or_default(value: Option<&str>, default: &str) -> String {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(default)
        .to_string()
}

fn css_value_or_default(value: Option<&str>, default: &str) -> String {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(default)
        .to_string()
}

fn infer_font_format(source: &str) -> Option<String> {
    let ext = Path::new(source)
        .extension()
        .and_then(|ext| ext.to_str())?
        .to_ascii_lowercase();

    match ext.as_str() {
        "woff2" => Some("woff2".to_string()),
        "woff" => Some("woff".to_string()),
        "ttf" => Some("truetype".to_string()),
        "otf" => Some("opentype".to_string()),
        _ => None,
    }
}

fn quote_css_string(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{
        FontFaceConfig, TypographyOptions, render_font_faces_css, resolve_font_options,
        site_font_options,
    };

    #[test]
    fn builds_font_family_defaults_without_faces() {
        let typography = TypographyOptions {
            line_height: Some("1.6".to_string()),
            body_font: Some("\"Inter\", sans-serif".to_string()),
            heading_font: None,
            mono_font: Some("\"JetBrains Mono\", monospace".to_string()),
            font_faces: Vec::new(),
        };

        let fonts = site_font_options(Some(&typography));
        assert_eq!(fonts.body_font, "\"Inter\", sans-serif");
        assert_eq!(fonts.heading_font, "\"Inter\", sans-serif");
        assert_eq!(fonts.mono_font, "\"JetBrains Mono\", monospace");
    }

    #[test]
    fn resolves_default_font_options() {
        let dir = tempdir().expect("tempdir should be created");
        let (fonts, font_faces) =
            resolve_font_options(None, dir.path(), &[]).expect("default fonts should resolve");

        assert!(fonts.body_font.contains("ui-sans-serif"));
        assert_eq!(fonts.heading_font, fonts.body_font);
        assert!(fonts.mono_font.contains("ui-monospace"));
        assert!(font_faces.is_empty());
    }

    #[test]
    fn resolves_local_site_font_assets() {
        let dir = tempdir().expect("tempdir should be created");
        fs::create_dir_all(dir.path().join("static/fonts")).expect("font dir should exist");
        fs::write(dir.path().join("static/fonts/inter.woff2"), "font")
            .expect("font should be written");

        let typography = TypographyOptions {
            line_height: None,
            body_font: Some("\"Inter\", sans-serif".to_string()),
            heading_font: Some("\"Inter\", sans-serif".to_string()),
            mono_font: None,
            font_faces: vec![FontFaceConfig {
                family: "Inter".to_string(),
                source: "/fonts/inter.woff2".to_string(),
                weight: Some("400".to_string()),
                style: Some("normal".to_string()),
                display: None,
            }],
        };

        let (fonts, font_faces) =
            resolve_font_options(Some(&typography), dir.path(), &[]).expect("fonts should resolve");
        assert_eq!(fonts.body_font, "\"Inter\", sans-serif");
        assert_eq!(font_faces.len(), 1);
        assert_eq!(font_faces[0].source, "/fonts/inter.woff2");
        assert_eq!(font_faces[0].format.as_deref(), Some("woff2"));
    }

    #[test]
    fn resolves_theme_font_assets() {
        let dir = tempdir().expect("tempdir should be created");
        let theme_static = dir.path().join("themes/default/static");
        fs::create_dir_all(theme_static.join("fonts")).expect("theme font dir should exist");
        fs::write(theme_static.join("fonts/heading.woff"), "font").expect("font should be written");

        let typography = TypographyOptions {
            line_height: None,
            body_font: None,
            heading_font: Some("\"Heading\", serif".to_string()),
            mono_font: None,
            font_faces: vec![FontFaceConfig {
                family: "Heading".to_string(),
                source: "/fonts/heading.woff".to_string(),
                weight: Some("700".to_string()),
                style: Some("normal".to_string()),
                display: None,
            }],
        };

        let (_fonts, font_faces) = resolve_font_options(
            Some(&typography),
            dir.path(),
            std::slice::from_ref(&theme_static),
        )
        .expect("theme font should resolve");
        assert_eq!(font_faces[0].format.as_deref(), Some("woff"));
    }

    #[test]
    fn fails_when_local_font_asset_is_missing() {
        let dir = tempdir().expect("tempdir should be created");
        let typography = TypographyOptions {
            line_height: None,
            body_font: None,
            heading_font: None,
            mono_font: None,
            font_faces: vec![FontFaceConfig {
                family: "Missing".to_string(),
                source: "/fonts/missing.woff2".to_string(),
                weight: None,
                style: None,
                display: None,
            }],
        };

        let error = resolve_font_options(Some(&typography), dir.path(), &[])
            .expect_err("missing font should fail");
        assert!(error.to_string().contains("font asset not found"));
    }

    #[test]
    fn renders_font_face_css() {
        let css = render_font_faces_css(&[super::ResolvedFontFace {
            family: "Inter".to_string(),
            source: "/fonts/inter.woff2".to_string(),
            weight: "400".to_string(),
            style: "normal".to_string(),
            display: "swap".to_string(),
            format: Some("woff2".to_string()),
        }]);

        assert!(css.contains("@font-face"));
        assert!(css.contains("font-family: \"Inter\";"));
        assert!(css.contains("src: url(\"/fonts/inter.woff2\") format(\"woff2\");"));
    }
}
