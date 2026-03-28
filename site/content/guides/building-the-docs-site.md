---
title: Building The Docs Site
summary: Understand the in-repo docs site project and how it is verified.
order: 6
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
- the docs site also publishes `https://fcendesu.github.io/rustipo/llms.txt` as a curated LLM-friendly entry point
- the workflow lives at `.github/workflows/docs-site.yml`

### Production-only analytics

The committed docs site config stays analytics-free. If maintainers want analytics on the published site, inject it at deploy time through the workflow instead of committing it into `site/config.toml`.

Set a repository variable named `DOCS_ANALYTICS_HEAD_HTML` to the full analytics snippet, for example:

```html
<script defer src="https://analytics.example.com/script.js" data-website-id="YOUR_UMAMI_SITE_ID"></script>
```

The docs-site workflow appends a temporary `[site.analytics]` block during the GitHub Pages build when that variable is present. This keeps forks and local builds from inheriting production analytics.

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

## Related Pages

- [Template-driven pages](/guides/template-driven-pages/)
- [CLI reference](/reference/cli/)
- [Examples](/examples/)
