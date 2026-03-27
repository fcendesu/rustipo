---
title: Content Model
summary: How Rustipo turns Markdown files into pages, sections, blog posts, projects, and nested docs routes.
order: 2
---

# Content Model

Rustipo keeps the content model intentionally simple: routes come from Markdown file paths.

## Core Mapping

- `content/index.md` -> `/`
- `content/about.md` -> `/about/`
- `content/guides/getting-started.md` -> `/guides/getting-started/`
- `content/guides/index.md` -> `/guides/`

## Special Sections

Rustipo still reserves two named content groups:

- `content/blog/*.md`
- `content/projects/*.md`

Those power blog/project listing behavior and archive-related output.

## Nested Docs Pages

For docs and notes sites, nested pages are often the most important part of the model.

### Directory index pages

A nested `index.md` acts as the route entry for that directory.

### Nested content pages

Any other Markdown file becomes a leaf route below that directory.

This docs site uses both patterns under `/guides/`, `/reference/`, and `/examples/` so breadcrumbs and route generation stay exercised in practice.

## Frontmatter

Common frontmatter fields include:

- `title`
- `summary`
- `date`
- `tags`
- `draft`
- `slug`
- `order`
- `links`

## Taxonomy Contract

Rustipo currently formalizes one built-in taxonomy:

- `tags`

That means Rustipo now has both:

- raw `page_tags` for compatibility
- generic `page_taxonomies.tags` entries for theme code

Generated taxonomy routes are:

- `/tags/`
- `/tags/<tag>/`

In the current model, taxonomy listing pages are derived from blog post tags.

## Shortcodes And Interactive Embeds

Rustipo supports reusable shortcodes inside Markdown content.

Current built-ins include:

- `youtube`
- `link`
- `iframe`
- `demo`

`demo` shortcodes can declare page-scoped script and stylesheet assets, which Rustipo injects once for the rendered page.

For a real example, see [Interactive embeds](/guides/interactive-embeds/).

## Drafts And Scheduled Pages

Production builds exclude:

- `draft: true`
- future-dated pages

Preview commands include them, which makes `rustipo dev` and `rustipo serve --watch` safer for editorial work.

## Internal Links And Deep Links

Rustipo validates internal Markdown links during `check` and `build`. It also validates heading fragments on content pages when possible.

For example:

- [CLI reference](/reference/cli/)
- [Theme and palette selection](/reference/themes-and-palettes/#theme-and-palette-selection)

## Related Reference

- [CLI reference](/reference/cli/)
- [Themes and palettes](/reference/themes-and-palettes/)
