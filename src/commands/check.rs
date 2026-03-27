use crate::content::pages::PublicationMode;
use anyhow::Result;

pub fn run() -> Result<()> {
    let prepared = crate::commands::site::prepare_site(true, PublicationMode::Production)?;
    let rendered_routes = crate::output::writer::validate_rendered_pages(&prepared.rendered_pages)?;
    crate::output::palette::ensure_palette_output_path_available(
        "static",
        &prepared.theme.static_dirs,
    )?;
    let validated_styles =
        crate::output::styles::validate_optional_scss("static", &prepared.theme.static_dirs)?;
    let asset_count = crate::output::assets::validate_assets_with_collision_check(
        "static",
        &prepared.theme.static_dirs,
    )?;

    println!("Validated rendered routes: {}", rendered_routes);
    println!("Validated optional SCSS inputs: {}", validated_styles);
    println!("Validated asset paths: {}", asset_count);
    println!("Check completed: project inputs are valid.");

    Ok(())
}
