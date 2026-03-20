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
