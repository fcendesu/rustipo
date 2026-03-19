# Theme Contract

Themes are filesystem-based and selected via site config.
Rustipo uses Tera as its template engine for theme rendering.

In practice, this means:

- site authors primarily write content in Markdown under `content/`
- theme authors define reusable page structure in `templates/*.html`
- Rustipo renders Markdown into HTML first, then injects that result into Tera templates

This separation is intentional. Markdown handles content authoring, while Tera templates handle layout and repeated page structure.

## Required files

For a standalone theme (no parent), include:

- `templates/base.html`
- `templates/page.html`
- `templates/post.html`
- `templates/project.html`
- `templates/section.html`
- `templates/index.html`
- `theme.toml`

For an inherited theme, templates can be provided by parent themes in the inheritance chain.

## `theme.toml` fields

Minimum metadata fields:

- `name`
- `version`
- `author`
- `description`

Optional field:

- `extends` (parent theme directory name)

Example:

```toml
name = "cyberpunk"
version = "0.1.0"
author = "Rustipo"
description = "Cyberpunk variant"
extends = "default"
```

## Rendering responsibilities

Theme defines presentation:

- HTML templates
- Theme static assets
- Theme metadata

Generator responsibilities:

- Load active theme
- Resolve inheritance chain (`parent -> child`) and detect cycles
- Validate required templates across the full inheritance chain
- Render content through merged templates (child overrides parent files by relative path)
- Copy merged theme static assets to output (child overrides parent files by relative path)

## How Tera fits the workflow

A common pattern is:

- `base.html` defines the outer shell
- `index.html` defines homepage layout
- `page.html` defines generic standalone pages
- `post.html` defines blog post layout
- `section.html` defines list/index pages such as blog and projects

Rustipo then reuses those templates across all Markdown content.

For example, every file under `content/blog/*.md` is rendered through the same `post.html` template. The Markdown content changes per file, but the layout stays consistent.

## Template context notes

Rustipo injects common site variables into template contexts, including:

- favicon helpers: `site_favicon`, `site_favicon_svg`, `site_favicon_ico`, `site_apple_touch_icon`
- style helpers from config:
  - `site_style.content_width`
  - `site_style.top_gap`
  - `site_style.vertical_align`
  - `site_style.line_height`
- `site_has_custom_css` (boolean, true when `static/custom.css` exists)
