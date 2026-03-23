pub mod editor;
pub mod fonts;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};

pub use fonts::{ResolvedFontFace, SiteFontOptions, TypographyOptions};

#[derive(Debug, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    pub base_url: String,
    pub theme: String,
    pub palette: Option<String>,
    pub menus: Option<BTreeMap<String, Vec<MenuEntryConfig>>>,
    pub description: String,
    // Reserved for template contexts and future metadata outputs.
    #[allow(dead_code)]
    pub author: Option<AuthorConfig>,
    pub site: Option<SiteOptions>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct MenuEntryConfig {
    pub title: String,
    pub route: String,
}

#[derive(Debug, Deserialize)]
// Author keys are accepted in config now even though rendering does not consume them yet.
#[allow(dead_code)]
pub struct AuthorConfig {
    pub name: Option<String>,
    pub email: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SiteOptions {
    pub posts_per_page: Option<usize>,
    pub favicon: Option<String>,
    pub layout: Option<LayoutOptions>,
    pub typography: Option<TypographyOptions>,
}

#[derive(Debug, Deserialize)]
pub struct LayoutOptions {
    pub content_width: Option<String>,
    pub top_gap: Option<String>,
    pub vertical_align: Option<String>,
}

impl SiteConfig {
    pub fn public_url_path(&self, path: &str) -> String {
        crate::url::public_url_path(&self.base_url, path)
    }

    pub fn posts_per_page(&self) -> usize {
        self.site
            .as_ref()
            .and_then(|s| s.posts_per_page)
            .filter(|v| *v > 0)
            .unwrap_or(10)
    }

    pub fn resolve_favicon_links(&self, project_root: impl AsRef<Path>) -> Result<FaviconLinks> {
        let project_root = project_root.as_ref();
        let static_dir = project_root.join("static");

        let mut links = FaviconLinks {
            icon_href: Some(self.public_url_path("/favicon.ico")),
            ico_href: Some(self.public_url_path("/favicon.ico")),
            svg_href: None,
            apple_touch_icon_href: None,
        };

        if static_dir.join("favicon.svg").is_file() {
            links.svg_href = Some(self.public_url_path("/favicon.svg"));
        }
        if static_dir.join("apple-touch-icon.png").is_file() {
            links.apple_touch_icon_href = Some(self.public_url_path("/apple-touch-icon.png"));
        }

        if let Some(configured) = self
            .site
            .as_ref()
            .and_then(|site| site.favicon.as_deref())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            let href = normalize_favicon_href(configured);
            let expected = static_dir.join(href.trim_start_matches('/'));
            if !expected.is_file() {
                bail!(
                    "configured favicon file not found: '{}' (expected at '{}')",
                    href,
                    expected.display()
                );
            }

            links.icon_href = Some(self.public_url_path(&href));
            if href.ends_with(".svg") {
                links.svg_href = Some(self.public_url_path(&href));
            } else if href.ends_with(".ico") {
                links.ico_href = Some(self.public_url_path(&href));
            } else if href.ends_with("apple-touch-icon.png") {
                links.apple_touch_icon_href = Some(self.public_url_path(&href));
            }
        }

        Ok(links)
    }

    pub fn selected_palette(&self) -> &str {
        self.palette
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("default")
    }

    pub fn resolve_fonts(
        &self,
        project_root: impl AsRef<Path>,
        theme_static_dirs: &[PathBuf],
    ) -> Result<(SiteFontOptions, Vec<ResolvedFontFace>)> {
        fonts::resolve_font_options(
            self.site.as_ref().and_then(|site| site.typography.as_ref()),
            project_root.as_ref(),
            theme_static_dirs,
        )
    }

    pub fn style_options(&self) -> SiteStyleOptions {
        let layout = self.site.as_ref().and_then(|site| site.layout.as_ref());
        let typography = self.site.as_ref().and_then(|site| site.typography.as_ref());
        let site_fonts = fonts::site_font_options(typography);

        SiteStyleOptions {
            content_width: css_value_or_default(
                layout.and_then(|l| l.content_width.as_deref()),
                "90%",
            ),
            top_gap: css_value_or_default(layout.and_then(|l| l.top_gap.as_deref()), "2rem"),
            vertical_align: vertical_align_or_default(
                layout.and_then(|l| l.vertical_align.as_deref()),
                "center",
            ),
            line_height: css_value_or_default(
                typography.and_then(|t| t.line_height.as_deref()),
                "1.5",
            ),
            body_font: site_fonts.body_font,
            heading_font: site_fonts.heading_font,
            mono_font: site_fonts.mono_font,
        }
    }

    pub fn has_custom_css(&self, project_root: impl AsRef<Path>) -> bool {
        let _ = self;
        project_root.as_ref().join("static/custom.css").is_file()
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FaviconLinks {
    pub icon_href: Option<String>,
    pub svg_href: Option<String>,
    pub ico_href: Option<String>,
    pub apple_touch_icon_href: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SiteStyleOptions {
    pub content_width: String,
    pub top_gap: String,
    pub vertical_align: String,
    pub line_height: String,
    pub body_font: String,
    pub heading_font: String,
    pub mono_font: String,
}

fn normalize_favicon_href(value: &str) -> String {
    if value.starts_with('/') {
        value.to_string()
    } else {
        format!("/{value}")
    }
}

fn css_value_or_default(value: Option<&str>, default: &str) -> String {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(default)
        .to_string()
}

fn vertical_align_or_default(value: Option<&str>, default: &str) -> String {
    let normalized = value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_ascii_lowercase());

    match normalized.as_deref() {
        Some("center") => "center".to_string(),
        Some("start") => "start".to_string(),
        _ => default.to_string(),
    }
}

pub fn load(path: impl AsRef<Path>) -> Result<SiteConfig> {
    let path = path.as_ref();

    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;

    let config = toml::from_str::<SiteConfig>(&raw)
        .with_context(|| format!("failed to parse config file: {}", path.display()))?;
    validate_menus(&config).map_err(|error| {
        anyhow!(
            "invalid menu configuration in config file: {}: {error}",
            path.display()
        )
    })?;

    Ok(config)
}

fn validate_menus(config: &SiteConfig) -> Result<()> {
    let Some(menus) = &config.menus else {
        return Ok(());
    };

    for (name, entries) in menus {
        if name.trim().is_empty() {
            bail!("menu name must not be empty");
        }
        if entries.is_empty() {
            bail!("menu '{name}' must contain at least one entry");
        }

        for (index, entry) in entries.iter().enumerate() {
            let item_number = index + 1;
            if entry.title.trim().is_empty() {
                bail!("menu '{name}' item {item_number} title must not be empty");
            }
            if entry.route.trim().is_empty() {
                bail!("menu '{name}' item {item_number} route must not be empty");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{LayoutOptions, SiteConfig, SiteOptions, TypographyOptions, load};

    fn base_config() -> SiteConfig {
        SiteConfig {
            title: "Rustipo".to_string(),
            base_url: "https://example.com".to_string(),
            theme: "default".to_string(),
            palette: None,
            menus: None,
            description: "Site".to_string(),
            author: None,
            site: None,
        }
    }

    #[test]
    fn resolves_default_favicon_links() {
        let dir = tempdir().expect("tempdir should be created");
        fs::create_dir_all(dir.path().join("static")).expect("static dir should be created");
        fs::write(dir.path().join("static/favicon.svg"), "<svg/>")
            .expect("svg favicon should be written");

        let config = base_config();
        let links = config
            .resolve_favicon_links(dir.path())
            .expect("favicon links should resolve");

        assert_eq!(links.ico_href.as_deref(), Some("/favicon.ico"));
        assert_eq!(links.icon_href.as_deref(), Some("/favicon.ico"));
        assert_eq!(links.svg_href.as_deref(), Some("/favicon.svg"));
    }

    #[test]
    fn fails_when_configured_favicon_is_missing() {
        let dir = tempdir().expect("tempdir should be created");
        fs::create_dir_all(dir.path().join("static")).expect("static dir should be created");

        let mut config = base_config();
        config.site = Some(SiteOptions {
            posts_per_page: None,
            favicon: Some("/favicon.ico".to_string()),
            layout: None,
            typography: None,
        });

        let error = config
            .resolve_favicon_links(dir.path())
            .expect_err("missing favicon should fail");
        assert!(
            error
                .to_string()
                .contains("configured favicon file not found")
        );
    }

    #[test]
    fn prefixes_public_paths_when_base_url_has_subpath() {
        let mut config = base_config();
        config.base_url = "https://example.com/docs/".to_string();

        assert_eq!(config.public_url_path("/guides/"), "/docs/guides/");
        assert_eq!(config.public_url_path("style.css"), "/docs/style.css");
    }

    #[test]
    fn provides_default_style_options() {
        let config = base_config();
        let style = config.style_options();

        assert_eq!(style.content_width, "90%");
        assert_eq!(style.top_gap, "2rem");
        assert_eq!(style.vertical_align, "center");
        assert_eq!(style.line_height, "1.5");
        assert!(
            style.body_font.contains("ui-sans-serif"),
            "unexpected body font: {}",
            style.body_font
        );
        assert_eq!(style.heading_font, style.body_font);
        assert!(style.mono_font.contains("ui-monospace"));
    }

    #[test]
    fn uses_style_options_from_config_when_present() {
        let mut config = base_config();
        config.site = Some(SiteOptions {
            posts_per_page: None,
            favicon: None,
            layout: Some(LayoutOptions {
                content_width: Some("98%".to_string()),
                top_gap: Some("3rem".to_string()),
                vertical_align: Some("start".to_string()),
            }),
            typography: Some(TypographyOptions {
                line_height: Some("1.7".to_string()),
                body_font: Some("\"Inter\", sans-serif".to_string()),
                heading_font: Some("\"Fraunces\", serif".to_string()),
                mono_font: Some("\"JetBrains Mono\", monospace".to_string()),
                font_faces: Vec::new(),
            }),
        });

        let style = config.style_options();
        assert_eq!(style.content_width, "98%");
        assert_eq!(style.top_gap, "3rem");
        assert_eq!(style.vertical_align, "start");
        assert_eq!(style.line_height, "1.7");
        assert_eq!(style.body_font, "\"Inter\", sans-serif");
        assert_eq!(style.heading_font, "\"Fraunces\", serif");
        assert_eq!(style.mono_font, "\"JetBrains Mono\", monospace");
    }

    #[test]
    fn falls_back_to_default_palette_when_missing() {
        let config = base_config();
        assert_eq!(config.selected_palette(), "default");
    }

    #[test]
    fn detects_custom_css_file() {
        let dir = tempdir().expect("tempdir should be created");
        fs::create_dir_all(dir.path().join("static")).expect("static dir should be created");

        let config = base_config();
        assert!(!config.has_custom_css(dir.path()));

        fs::write(dir.path().join("static/custom.css"), "body{}")
            .expect("custom css should be written");
        assert!(config.has_custom_css(dir.path()));
    }

    #[test]
    fn loads_named_menus_from_config() {
        let dir = tempdir().expect("tempdir should be created");
        let config_path = dir.path().join("config.toml");
        fs::write(
            &config_path,
            r#"
title = "Rustipo"
base_url = "https://example.com"
theme = "default"
description = "Example"

[menus]
main = [
  { title = "Home", route = "/" },
  { title = "Blog", route = "/blog/" },
]
"#,
        )
        .expect("config should be written");

        let config = load(&config_path).expect("config should load");
        let main = config
            .menus
            .as_ref()
            .and_then(|menus| menus.get("main"))
            .expect("main menu should be present");

        assert_eq!(main.len(), 2);
        assert_eq!(main[0].title, "Home");
        assert_eq!(main[1].route, "/blog/");
    }

    #[test]
    fn rejects_menu_entries_with_blank_title() {
        let dir = tempdir().expect("tempdir should be created");
        let config_path = dir.path().join("config.toml");
        fs::write(
            &config_path,
            r#"
title = "Rustipo"
base_url = "https://example.com"
theme = "default"
description = "Example"

[menus]
main = [
  { title = "   ", route = "/" },
]
"#,
        )
        .expect("config should be written");

        let error = load(&config_path).expect_err("blank menu title should fail");
        assert!(
            error
                .to_string()
                .contains("menu 'main' item 1 title must not be empty"),
            "unexpected error: {error}"
        );
    }
}
