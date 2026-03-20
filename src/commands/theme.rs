use anyhow::Result;

pub fn list() -> Result<()> {
    let themes = crate::theme::loader::list_installed_themes(".")?;

    if themes.is_empty() {
        println!("No themes found.");
        return Ok(());
    }

    for theme in themes {
        println!("{}", format_theme_summary(&theme));
    }

    Ok(())
}

pub fn install(source: &str, name_override: Option<&str>) -> Result<()> {
    let directory_name = crate::theme::installer::install_theme(".", source, name_override)?;
    println!("Installed theme: {directory_name}");
    Ok(())
}

fn format_theme_summary(theme: &crate::theme::models::ThemeSummary) -> String {
    format!(
        "{} -> {} ({}) - {} [{}]",
        theme.theme_id,
        theme.metadata.name,
        theme.metadata.version,
        theme.metadata.description,
        theme.directory_name
    )
}

#[cfg(test)]
mod tests {
    use crate::theme::models::{ThemeMetadata, ThemeSummary};

    use super::format_theme_summary;

    #[test]
    fn formats_theme_summary_with_selectable_id_first() {
        let theme = ThemeSummary {
            theme_id: "catppuccin-mocha".to_string(),
            directory_name: "catppuccin".to_string(),
            metadata: ThemeMetadata {
                id: Some("catppuccin-mocha".to_string()),
                name: "Catppuccin Mocha".to_string(),
                version: "0.1.0".to_string(),
                author: "Rustipo".to_string(),
                description: "Catppuccin variant".to_string(),
                extends: None,
            },
        };

        let line = format_theme_summary(&theme);
        assert_eq!(
            line,
            "catppuccin-mocha -> Catppuccin Mocha (0.1.0) - Catppuccin variant [catppuccin]"
        );
    }
}
