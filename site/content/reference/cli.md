---
title: CLI Reference
summary: Core commands for creating, checking, building, previewing, and deploying Rustipo sites.
order: 1
---

# CLI Reference

Rustipo currently centers on a small CLI surface that covers the full authoring loop.

## Core Commands

### `rustipo new <site-name>`

Creates a starter project with content, a local default theme, and starter configuration.

### `rustipo check`

Validates config, content, palettes, themes, routes, asset paths, and internal links without writing `dist/`.

### `rustipo dev`

Builds, serves, watches, and live-reloads the site during development.

### `rustipo build`

Writes generated output into `dist/`.

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
