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
