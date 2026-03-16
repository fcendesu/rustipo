use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let config = crate::config::load("config.toml")?;
    println!(
        "Loaded config: title='{}', theme='{}'",
        config.title, config.theme
    );
    let markdown_files = crate::content::loader::discover_markdown_files("content")?;
    println!("Discovered markdown files: {}", markdown_files.len());

    let index_markdown = std::fs::read_to_string("content/index.md")
        .context("failed to read content/index.md for build preview")?;
    let parsed = crate::content::frontmatter::parse(&index_markdown)?;
    if let Some(title) = parsed.frontmatter.title.as_deref() {
        println!("Parsed index frontmatter: title='{}'", title);
    }
    let index_html = crate::content::markdown::render_html(&parsed.content);
    println!("Rendered index markdown to HTML ({} bytes)", index_html.len());
    println!("`rustipo build` is not implemented yet");
    Ok(())
}
