# CLI Reference

Rustipo provides the following commands.

## `rustipo new <site-name>`

Creates a new starter portfolio project directory.

Example:

```bash
rustipo new my-portfolio
```

Current behavior:

- Creates starter `content/`, `static/`, and `themes/default/` structure
- Writes starter `config.toml` and Markdown pages
- Writes starter `static/favicon.svg` and configures favicon in `config.toml`
- Writes required default theme templates and starter CSS
- Fails if target directory already exists

## `rustipo build`

Builds site content into static output (`dist/`).

Current behavior:

- Loads and validates `config.toml`
- Loads active theme from `themes/<theme>/`
- Resolves theme inheritance chain when `extends` is used in `theme.toml`
- Validates required templates across the resolved theme chain
- Discovers Markdown files from `content/`
- Parses frontmatter, validates date format (`YYYY-MM-DD`), and excludes drafts
- Converts Markdown to HTML
- Renders supported shortcodes in Markdown content
- Applies syntax highlighting to fenced code blocks
- Renders pages through theme templates
- Resolves favicon links for template context (`site_favicon*` variables)
- Fails with a readable error when configured favicon path is missing in `static/`
- Writes rendered pages to `dist/` using pretty URL output paths
- Fails with readable error on duplicate rendered output route collisions
- Copies theme and user static assets into `dist/`
- Applies child-over-parent precedence when inherited themes provide the same template/asset path
- Fails on static asset path collisions
- Generates section index pages for `/blog/` and `/projects/`
- Generates tag index pages at `/tags/<tag>/` from blog post tags
- Generates RSS feed at `dist/rss.xml` from dated blog posts
- Generates sitemap at `dist/sitemap.xml` from rendered site routes
- Generates search index at `dist/search-index.json` from site content

## `rustipo serve`

Serves built static output locally.

Example:

```bash
rustipo serve --host 0.0.0.0 --port 4000
```

```bash
rustipo serve --watch
```

Current behavior:

- Serves files from `dist/`
- Default address: `127.0.0.1:3000`
- Supports custom host/port via `--host` and `--port`
- Supports watch mode with `--watch` (rebuilds on file changes)
- In watch mode, injects live-reload script into HTML responses and auto-refreshes browser after successful rebuild
- In watch mode, skips rebuild when file content hash is unchanged (reduces no-op save rebuild noise)
- Prints local URL on startup
- Returns readable error if `dist/` does not exist

## `rustipo theme list`

Lists available themes.

Current behavior:

- Reads installed themes from `themes/*/theme.toml`
- Prints theme name, version, description, and directory name

## `rustipo theme install <source>`

Installs a theme into `themes/`.

Examples:

```bash
rustipo theme install fcendesu/rustipo-theme
```

```bash
rustipo theme install https://github.com/fcendesu/rustipo-theme
```

```bash
rustipo theme install fcendesu/rustipo-theme --name cyberpunk
```

Current behavior:

- Accepts GitHub shorthand (`owner/repo`) or GitHub URL
- Also accepts local git repository path (useful for local development/testing)
- Clones repository into `themes/<name>/` (or inferred repo name)
- Removes cloned `.git` metadata from installed theme directory
- Validates required theme contract after install
- Fails with readable errors on clone/validation conflicts

## `rustipo deploy github-pages`

Generates a GitHub Actions workflow for deploying `dist/` to GitHub Pages.

Example:

```bash
rustipo deploy github-pages
```

```bash
rustipo deploy github-pages --force
```

Current behavior:

- Writes `.github/workflows/deploy-pages.yml`
- Workflow runs `cargo run -- build` and deploys `dist/` using Pages actions
- Refuses to overwrite existing workflow unless `--force` is provided
