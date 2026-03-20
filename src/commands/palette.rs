use anyhow::Result;

pub fn list() -> Result<()> {
    let palettes = crate::palette::loader::list_available_palettes(".")?;

    if palettes.is_empty() {
        println!("No palettes found.");
        return Ok(());
    }

    for palette in palettes {
        println!(
            "{} -> {} - {} [{}]",
            palette.id,
            palette.name,
            palette.description,
            palette.source.as_label()
        );
    }

    Ok(())
}

pub fn use_palette(id: &str) -> Result<()> {
    let palette = crate::palette::loader::load_palette(".", id)?;
    crate::config::editor::set_top_level_string("config.toml", "palette", &palette.id)?;
    println!("Updated palette in config.toml: {}", palette.id);
    Ok(())
}
