---
title: Publishing Outputs
summary: Understand the generated artifacts Rustipo writes during build, when they appear, and what they are for.
order: 9
---

# Publishing Outputs

When you run `rustipo build`, Rustipo does more than render HTML pages into `dist/`.

It also generates a small set of publishing artifacts that help the site behave like a real published product rather than a folder of HTML files.

This page explains the built-in generated outputs Rustipo writes today, when they appear, and where theme authors can influence the result.

## What Rustipo Always Writes

Every normal Rustipo build writes rendered pages and copied assets into `dist/`.

In addition, Rustipo can generate:

- `dist/rss.xml`
- `dist/sitemap.xml`
- `dist/search-index.json`
- `dist/robots.txt`
- `dist/404.html`

Some outputs always appear. Others depend on site content such as dated blog posts.

## `dist/rss.xml`

Rustipo generates `dist/rss.xml` from dated blog posts.

This is the feed output for the built-in blog publishing model.

### When it appears

- when the site has dated blog posts that belong in the feed

### What it is for

- feed readers
- subscribers who want post updates
- integrations that consume a standard RSS feed

### What theme authors should know

RSS is a publishing artifact, not a theme template surface. Themes do not render it directly.

## `dist/sitemap.xml`

Rustipo generates a sitemap from the rendered site routes.

### When it appears

- during normal builds with a valid `base_url`

### What it is for

- search engine discovery
- crawler-friendly route listing
- giving other systems a canonical list of published URLs

### What theme authors should know

The sitemap comes from the final rendered route set, so it reflects the pages Rustipo actually publishes rather than the theme structure alone.

## `dist/search-index.json`

Rustipo generates a JSON search index from site content.

### When it appears

- during normal builds

### What it is for

- client-side or custom search implementations
- docs and notes sites that want lightweight local search data

### What theme authors should know

Rustipo writes the index file, but themes decide whether to consume it in the frontend. A theme can ship search UI and read `search-index.json` without reimplementing index generation.

## `dist/robots.txt`

Rustipo generates a default `robots.txt` file.

### When it appears

- during normal builds

### What it is for

- crawler guidance
- pointing crawlers at the generated sitemap

### What theme authors should know

This is generated outside the theme layer. It is part of the publishing pipeline rather than the page template contract.

## `dist/404.html`

Rustipo generates a not-found page.

### When it appears

- during normal builds

### What it is for

- host-level fallback when a route is missing
- giving static hosts a real not-found document instead of a generic server response

### Theme override behavior

This output is the publishing artifact most directly influenced by themes.

Rustipo uses:

- `templates/404.html` when the theme provides it
- otherwise `templates/page.html` as the fallback layout

That means theme authors can provide a custom branded not-found experience without changing the rest of the publishing pipeline.

## What These Outputs Depend On

These outputs come from different parts of the product:

- content model and routes influence `sitemap.xml`, `search-index.json`, and `rss.xml`
- theme templates influence `404.html`
- build-time publishing logic generates `robots.txt` and the other derived files
- `base_url` matters for absolute URL-aware publishing artifacts such as the sitemap and crawler hints

## What To Expect From `rustipo build`

A healthy build usually means:

- rendered HTML pages in `dist/`
- copied static assets
- generated `palette.css`
- generated publishing artifacts where applicable

So `rustipo build` is not only “HTML output.” It is the full static publishing step.

## What Themes Should Usually Rely On

Themes should not try to reimplement these publishing artifacts in template code.

Instead:

- let Rustipo generate feed, sitemap, search, and crawler files
- provide `templates/404.html` only when you want a custom not-found page
- build frontend behavior on top of generated files such as `search-index.json` when needed

## Good Companion Pages

- [CLI reference](/reference/cli/)
- [Content model](/reference/content-model/)
- [Deployment](/reference/deployment/)
- [Template context](/reference/template-context/)
- [Theme authoring](/guides/theme-authoring/)
