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

- page-state helpers:
  - `page_kind`
  - `current_section`
  - `site_nav`
  - `previous_post`
  - `next_post`
- favicon helpers: `site_favicon`, `site_favicon_svg`, `site_favicon_ico`, `site_apple_touch_icon`
- style helpers from config:
  - `site_style.content_width`
  - `site_style.top_gap`
  - `site_style.vertical_align`
  - `site_style.line_height`
- `site_has_custom_css` (boolean, true when `static/custom.css` exists)

Rustipo also registers small Tera helpers for theme authors:

- `slugify` filter
- `format_date(format="...")` filter
- `abs_url(path="...")` function
- `asset_url(path="...")` function
- `tag_url(name="...")` function

### Navigation and page-state details

`site_nav` is an ordered list of objects with:

- `title`
- `route`
- `active`

Rustipo builds it from available content:

- `Home` when `content/index.md` exists
- standalone pages from `content/*.md`
- `Blog` when blog posts exist
- `Projects` when project pages exist

`previous_post` and `next_post` are only populated for blog post pages.
They include:

- `title`
- `route`
- `summary`
- `date`

### Theme authoring conventions

Theme authors are encouraged to keep reusable Tera pieces under:

- `templates/partials/`
- `templates/macros/`

For a broader author guide, see [theme-tera.md](./theme-tera.md).
