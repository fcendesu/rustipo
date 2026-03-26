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

      - name: Install Rustipo
        run: cargo install rustipo --locked

      - name: Build Site
        run: rustipo build

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

const CLOUDFLARE_PAGES_WORKFLOW: &str = r#"name: Deploy Cloudflare Pages

on:
  push:
    branches: [master]
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: cloudflare-pages
  cancel-in-progress: true

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v6

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install Rustipo
        run: cargo install rustipo --locked

      - name: Build Site
        run: rustipo build

      - name: Deploy to Cloudflare Pages
        uses: cloudflare/wrangler-action@v3
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          gitHubToken: ${{ secrets.GITHUB_TOKEN }}
          command: pages deploy dist --project-name=${{ vars.CLOUDFLARE_PAGES_PROJECT }}
"#;

pub fn github_pages(force: bool) -> Result<()> {
    let workflow_path = Path::new(".github/workflows/deploy-pages.yml");
    write_workflow_file(workflow_path, GITHUB_PAGES_WORKFLOW, force)?;

    println!("Created GitHub Pages workflow: {}", workflow_path.display());
    println!("Next: push to master, then enable Pages in repository settings.");
    Ok(())
}

pub fn cloudflare_pages(force: bool) -> Result<()> {
    let workflow_path = Path::new(".github/workflows/deploy-cloudflare-pages.yml");
    write_workflow_file(workflow_path, CLOUDFLARE_PAGES_WORKFLOW, force)?;

    println!(
        "Created Cloudflare Pages workflow: {}",
        workflow_path.display()
    );
    println!("Next: create a Cloudflare Pages project, then add these repository settings:");
    println!("- secret: CLOUDFLARE_API_TOKEN");
    println!("- secret: CLOUDFLARE_ACCOUNT_ID");
    println!("- variable: CLOUDFLARE_PAGES_PROJECT");
    Ok(())
}

fn write_workflow_file(workflow_path: &Path, content: &str, force: bool) -> Result<()> {
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
    fs::write(workflow_path, content)
        .with_context(|| format!("failed to write workflow file: {}", workflow_path.display()))?;

    Ok(())
}
