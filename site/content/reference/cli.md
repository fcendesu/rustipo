---
title: CLI Reference
summary: Core commands for creating, checking, building, previewing, and deploying Rustipo sites.
order: 1
---

# CLI Reference

Rustipo currently centers on a small CLI surface that covers the full authoring loop.

## Config-Driven Extras

### Analytics

Rustipo supports opt-in Plausible analytics from `config.toml`:

```toml
[site.analytics.plausible]
domain = "docs.example.com"
# Optional for self-hosted Plausible:
# script_src = "https://stats.example.com/js/script.js"
```

Built-in themes render that snippet automatically through their shared head partials.

## Core Commands

### `rustipo new <site-name>`

Creates a starter project with content, a local default theme, and starter configuration.

### `rustipo check`

Validates config, content, palettes, themes, routes, asset paths, and internal links without writing `dist/`.
It also validates optional `themes/<theme>/static/style.scss` and `static/custom.scss` inputs when
those files are present.

### `rustipo dev`

Builds, serves, watches, and live-reloads the site during development.

### `rustipo build`

Writes generated output into `dist/`.
When a theme provides `static/style.scss` or a site provides `static/custom.scss`, Rustipo compiles
them into `dist/style.css` and `dist/custom.css`.

### `rustipo serve`

Serves an existing `dist/` output directory. `--watch` adds rebuilds and live reload.

## Theme And Palette Commands

### `rustipo theme list`

Lists built-in and local themes.

### `rustipo theme install <source>`

Installs a theme from GitHub shorthand, a GitHub URL, or a local git repository.

### `rustipo palette list`

Lists built-in palettes and local palette files.

### `rustipo palette use <id>`

Updates `config.toml` to use the selected palette.

For the theme and palette model behind those commands, continue to [Themes and palettes](/reference/themes-and-palettes/#theme-and-palette-selection).

## Deploy Command

### `rustipo deploy github-pages`

Generates a GitHub Pages workflow that installs the published `rustipo` binary and runs `rustipo build`.

### `rustipo deploy cloudflare-pages`

Generates a Cloudflare Pages workflow that builds `dist/` and deploys it with Wrangler.

The generated workflow expects:

- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`
- `CLOUDFLARE_PAGES_PROJECT`

If you prefer Cloudflare Pages Git integration, use:

- build command: `cargo install rustipo --locked && rustipo build`
- build output directory: `dist`

### `rustipo deploy netlify`

Generates a Netlify deployment workflow that installs Rustipo, builds `dist/`, and deploys it with Netlify CLI.

The generated workflow expects:

- `NETLIFY_AUTH_TOKEN`
- `NETLIFY_SITE_ID`

## Recommended Local Workflow

```bash
rustipo check
rustipo dev
```

Then, before publishing:

```bash
rustipo build
```

## Related Reference

- [Content model](/reference/content-model/)
- [Themes and palettes](/reference/themes-and-palettes/)
