mod scaffold;

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

pub fn run(site_name: &str) -> Result<()> {
    if site_name.trim().is_empty() {
        bail!("site name cannot be empty");
    }

    let root = Path::new(site_name);
    if root.exists() {
        bail!("target directory already exists: {}", root.display());
    }

    create_dir(root)?;
    create_dir(&root.join("content"))?;
    create_dir(&root.join("content/blog"))?;
    create_dir(&root.join("content/projects"))?;
    create_dir(&root.join("static"))?;
    create_dir(&root.join("themes/default/templates/partials"))?;
    create_dir(&root.join("themes/default/templates/macros"))?;
    create_dir(&root.join("themes/default/static"))?;

    write_file(&root.join("content/index.md"), scaffold::INDEX_CONTENT)?;
    write_file(&root.join("content/about.md"), scaffold::ABOUT_CONTENT)?;
    write_file(&root.join("content/resume.md"), scaffold::RESUME_CONTENT)?;
    write_file(&root.join("config.toml"), scaffold::CONFIG_TOML)?;
    write_file(&root.join("static/favicon.svg"), scaffold::FAVICON_SVG)?;
    write_file(
        &root.join("themes/default/theme.toml"),
        scaffold::DEFAULT_THEME_TOML,
    )?;
    write_file(
        &root.join("themes/default/templates/base.html"),
        scaffold::BASE_TEMPLATE,
    )?;
    write_file(
        &root.join("themes/default/templates/partials/head_assets.html"),
        scaffold::HEAD_ASSETS_PARTIAL,
    )?;
    write_file(
        &root.join("themes/default/templates/macros/layout.html"),
        scaffold::LAYOUT_MACROS,
    )?;
    for template_name in scaffold::PAGE_TEMPLATE_NAMES {
        write_file(
            &root.join("themes/default/templates").join(template_name),
            scaffold::CONTENT_TEMPLATE,
        )?;
    }
    write_file(
        &root.join("themes/default/templates/section.html"),
        scaffold::SECTION_TEMPLATE,
    )?;
    write_file(
        &root.join("themes/default/static/style.css"),
        scaffold::THEME_STYLE_CSS,
    )?;

    println!("Created new Rustipo site: {}", root.display());
    Ok(())
}

fn create_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path)
        .with_context(|| format!("failed to create directory: {}", path.display()))
}

fn write_file(path: &Path, contents: &str) -> Result<()> {
    fs::write(path, contents).with_context(|| format!("failed to write file: {}", path.display()))
}
