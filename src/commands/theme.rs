use anyhow::Result;

pub fn list() -> Result<()> {
    let themes = crate::theme::loader::list_installed_themes(".")?;

    if themes.is_empty() {
        println!("No themes found.");
        return Ok(());
    }

    for theme in themes {
        println!(
            "{} ({}) - {} [{}]",
            theme.metadata.name,
            theme.metadata.version,
            theme.metadata.description,
            theme.directory_name
        );
    }

    Ok(())
}

pub fn install(source: &str, name_override: Option<&str>) -> Result<()> {
    let directory_name = crate::theme::installer::install_theme(".", source, name_override)?;
    println!("Installed theme: {directory_name}");
    Ok(())
}
