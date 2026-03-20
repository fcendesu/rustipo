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
- Starter CSS includes default markdown prose styling (headings, spacing, code, links, tables, blockquotes)
- Fails if target directory already exists

## `rustipo dev`

Builds once, starts the local server, and watches for changes.

Example:

```bash
rustipo dev
```

```bash
rustipo dev --host 0.0.0.0 --port 4000
```

Current behavior:

- Runs the same local workflow as `rustipo serve --watch`
- Performs an initial build before serving
- Rebuilds on file changes and triggers live reload after successful rebuilds
- Uses the same default address as `serve`: `127.0.0.1:3000`

## `rustipo build`

Builds site content into static output (`dist/`).

Current behavior:

- Loads and validates `config.toml`
- Loads active theme from `themes/<theme>/`
- Loads selected palette from built-in palettes or local `palettes/<palette>.toml`
- Resolves theme inheritance chain when `extends` is used in `theme.toml`
- Validates required templates across the resolved theme chain
- Discovers Markdown files from `content/`
- Parses frontmatter, validates date format (`YYYY-MM-DD`), and excludes drafts
- Converts Markdown to HTML
- Renders supported shortcodes in Markdown content
- Applies syntax highlighting to fenced code blocks
- Renders `mermaid` fenced blocks as diagrams
- Injects Mermaid runtime only on pages that contain Mermaid blocks
- Renders pages through theme templates
- Resolves favicon links for template context (`site_favicon*` variables)
- Fails with a readable error when configured favicon path is missing in `static/`
- Resolves custom font families from `site.typography`
- Validates configured local font-face assets from `static/` or inherited theme `static/` directories
- Injects `@font-face` CSS only when configured font faces are present
- Exposes style context from `config.toml` to templates (`site_style.*`):
  - `site.layout.content_width`
  - `site.layout.top_gap`
  - `site.layout.vertical_align` (`center` or `start`, default: `center`)
  - `site.typography.line_height`
  - `site.typography.body_font`
  - `site.typography.heading_font`
  - `site.typography.mono_font`
- Exposes `site_font_faces_css` to templates for optional font-face injection
- Auto-includes `static/custom.css` in template context when present (`site_has_custom_css`)
- Writes rendered pages to `dist/` using pretty URL output paths
- Writes generated palette variables to `dist/palette.css`
- Fails with a readable error if generated `palette.css` would collide with a user/theme asset
- Fails with readable error on duplicate rendered output route collisions
- Copies theme and user static assets into `dist/`
- Applies child-over-parent precedence when inherited themes provide the same template/asset path
- Fails on static asset path collisions
- Generates section index pages for `/blog/` and `/projects/`
- Generates tag index pages at `/tags/<tag>/` from blog post tags
- Generates RSS feed at `dist/rss.xml` from dated blog posts
- Generates sitemap at `dist/sitemap.xml` from rendered site routes
- Generates search index at `dist/search-index.json` from site content
- Uses a CDN-backed Mermaid ESM runtime in v1

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
- Prints selectable theme ID, name, version, description, and directory name
- Theme IDs use lowercase kebab-case; variant themes should use `family-variant`

Config example:

```toml
theme = "default"
```

## `rustipo palette list`

Lists available palettes.

Current behavior:

- Lists built-in palettes shipped with Rustipo
- Lists local palettes from `palettes/*.toml` when present
- Prints palette ID, name, description, and source (`built-in` or `local`)

Config example:

```toml
palette = "catppuccin-mocha"
```

Current built-in Catppuccin flavors:

- `catppuccin-latte`
- `catppuccin-frappe`
- `catppuccin-macchiato`
- `catppuccin-mocha`

Additional built-in palettes:

- `default`
- `dracula`
- `gruvbox-dark`
- `tokyonight-storm`
- `tokyonight-moon`

## `rustipo palette use <id>`

Updates `config.toml` to use the selected palette.

Example:

```bash
rustipo palette use catppuccin-mocha
```

Current behavior:

- Validates the palette exists before updating config
- Writes or updates the top-level `palette = "..."` key in `config.toml`
- Prints the selected palette ID after updating

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

## Style Options (`config.toml`)

You can control default layout behavior without editing theme CSS:

```toml
[site.layout]
content_width = "98%"
top_gap = "2rem"
vertical_align = "center" # "center" (default) or "start"

[site.typography]
line_height = "1.5"
body_font = "\"Inter\", sans-serif"
heading_font = "\"Fraunces\", serif"
mono_font = "\"JetBrains Mono\", monospace"

[[site.typography.font_faces]]
family = "Inter"
source = "/fonts/inter.woff2"
weight = "400"
style = "normal"
```

- `vertical_align = "center"` keeps the classic vertically centered intro layout.
- `vertical_align = "start"` aligns content to the top while keeping horizontal centering.
- `body_font`, `heading_font`, and `mono_font` let you swap font stacks without editing theme CSS.
- `font_faces` lets you ship local fonts from `static/` or inherited theme assets.
- The default theme now ships a fuller editor-like heading scale (`h1`-`h6`) and tighter prose spacing.

Theme and palette example:

```toml
theme = "default"
palette = "tokyonight-storm"
```
