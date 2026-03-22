---
title: Rustipo Docs
summary: Start here for installation, content authoring, themes, palettes, and example sites.
---

# Rustipo Docs

Rustipo is a Markdown-first static site generator for blogs, notes, docs, and personal sites. This site is itself built with Rustipo and uses the built-in `atlas` theme.

> [!NOTE]
> The docs site lives in [`site/`](https://github.com/fcendesu/rustipo/tree/master/site) inside the Rustipo repository. It is meant to dogfood the product instead of describing it from the outside.

## Start Here

- [Get started](/guides/getting-started/)
- [Build the docs site](/guides/building-the-docs-site/)
- [CLI reference](/reference/cli/)
- [Content model](/reference/content-model/)
- [Themes and palettes](/reference/themes-and-palettes/)
- [Flagship examples](/examples/)

## What Rustipo Covers Well

- Markdown content with YAML frontmatter
- Tera-based themes with reusable templates
- separate theme and palette selection
- built-in outputs such as RSS, sitemap, search index, `robots.txt`, and `404.html`
- docs-friendly features like breadcrumbs, page TOCs, internal link validation, admonitions, math, and image captions

## Quick Install

```bash
cargo install rustipo
rustipo new my-site
cd my-site
rustipo dev
```

## Explore The Product

Rustipo is easiest to understand in layers:

1. content and routes: [Content model](/reference/content-model/)
2. commands and workflows: [CLI reference](/reference/cli/)
3. layout and color systems: [Themes and palettes](/reference/themes-and-palettes/)
4. finished site shapes: [Examples](/examples/)

## Why This Site Exists

The docs site is part of the product story for `v0.11.0`. It gives Rustipo a real docs-and-notes project inside the main repository, which helps validate the product outside the earlier portfolio-only framing.
