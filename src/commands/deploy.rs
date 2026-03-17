use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

const GITHUB_PAGES_WORKFLOW: &str = r#"name: Deploy GitHub Pages

on:
  push:
    branches: [master]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: pages
  cancel-in-progress: true

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build Site
        run: cargo run -- build

      - name: Configure Pages
        uses: actions/configure-pages@v5

      - name: Upload Artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: dist

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    needs: build
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
"#;

pub fn github_pages(force: bool) -> Result<()> {
    let workflow_path = Path::new(".github/workflows/deploy-pages.yml");
    if workflow_path.exists() && !force {
        bail!(
            "workflow already exists: {} (use --force to overwrite)",
            workflow_path.display()
        );
    }

    let parent = workflow_path
        .parent()
        .context("failed to resolve workflow parent directory")?;
    fs::create_dir_all(parent)
        .with_context(|| format!("failed to create workflow directory: {}", parent.display()))?;
    fs::write(workflow_path, GITHUB_PAGES_WORKFLOW)
        .with_context(|| format!("failed to write workflow file: {}", workflow_path.display()))?;

    println!("Created GitHub Pages workflow: {}", workflow_path.display());
    println!("Next: push to master, then enable Pages in repository settings.");
    Ok(())
}
