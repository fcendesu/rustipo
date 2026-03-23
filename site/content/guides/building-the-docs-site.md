---
title: Building The Docs Site
summary: Understand the in-repo docs site project and how it is verified.
order: 2
---

# Building The Docs Site

Rustipo's own documentation site lives in the repository under `site/`. It is a normal Rustipo project with its own `config.toml`, `content/`, and `static/` directories.

## Why Keep It In This Repository

Keeping the docs site here gives Rustipo a strong dogfooding loop:

- docs changes and product changes can evolve together
- CI can build the docs site as part of repository verification
- the site can demonstrate built-in themes, palettes, breadcrumbs, and page TOCs on real content

## Docs Site Structure

```text
site/
  config.toml
  content/
    guides/
    reference/
    examples/
    index.md
    roadmap.md
  static/
    favicon.svg
```

This docs site intentionally uses a built-in theme instead of a local theme copy:

```toml
theme = "atlas"
palette = "catppuccin-macchiato"
```

## Verifying The Site

From the repository root:

```bash
cd site
../target/debug/rustipo build
```

In CI, Rustipo also copies `site/` to a temporary directory and runs a full build as an end-to-end check.

## Publishing

The docs site is published from this repository with GitHub Pages.

- pushes to `master` rebuild and deploy the site automatically
- the published URL is `https://fcendesu.github.io/rustipo/`
- the workflow lives at `.github/workflows/docs-site.yml`

## What This Site Should Showcase

### Content model

The docs site should exercise nested pages and section indexes so breadcrumbs and table-of-contents behavior stay real.

### Themes and palettes

It should use a built-in layout theme plus a built-in palette, then link to the broader [themes and palettes reference](/reference/themes-and-palettes/).

### Docs structure

The top-level navigation should make the product easy to approach in this order:

1. getting started
2. reference material
3. examples
4. roadmap

## Related Pages

- [CLI reference](/reference/cli/)
- [Examples](/examples/)
- [Roadmap](/roadmap/)
