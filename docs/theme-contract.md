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

- `id` (explicit selectable theme ID, recommended for variants)
- `extends` (parent theme ID or directory name)

Example:

```toml
id = "cyberpunk-neon"
name = "cyberpunk"
version = "0.1.0"
author = "Rustipo"
description = "Cyberpunk variant"
extends = "default"
```

Theme IDs should use lowercase kebab-case. Variant themes should prefer `family-variant`.

When `id` is omitted, Rustipo falls back to the theme directory name for selection and listing.
`config.toml` `theme = "..."` can reference either the explicit theme ID or the directory name,
but explicit IDs are the recommended public interface.

Color presets such as `catppuccin-mocha` and `tokyonight-storm` belong in the palette system, not
in theme IDs. Themes define structure; palettes define color tokens.

Generated `palette.css` always includes the stable semantic variables used by the default theme:

- `--rustipo-bg`
- `--rustipo-text`
- `--rustipo-surface-muted`
- `--rustipo-border`
- `--rustipo-blockquote-border`
- `--rustipo-link`
- `--rustipo-link-hover`
- `--rustipo-code-bg`
- `--rustipo-code-text`
- `--rustipo-table-header-bg`

Palettes can also expose additional token variables. Rustipo writes those as
`--rustipo-token-<name>`, which lets richer themes use full palette vocabularies such as the
official Catppuccin flavor tokens.

Rustipo also derives a small richer theme contract from those tokens so themes can style more
expressively without depending on palette-family-specific names:

- `--rustipo-base`
- `--rustipo-mantle`
- `--rustipo-crust`
- `--rustipo-surface-0`
- `--rustipo-surface-1`
- `--rustipo-surface-2`
- `--rustipo-overlay-0`
- `--rustipo-overlay-1`
- `--rustipo-overlay-2`
- `--rustipo-subtext-0`
- `--rustipo-subtext-1`
- `--rustipo-accent`
- `--rustipo-accent-strong`
- `--rustipo-success`
- `--rustipo-warning`
- `--rustipo-danger`

Theme authors should prefer these richer variables with fallbacks to the stable semantic ones.

Example:

```css
.card {
  background: var(--rustipo-surface-0, var(--rustipo-surface-muted));
  border: 1px solid var(--rustipo-surface-1, var(--rustipo-border));
}

.button-primary {
  background: var(--rustipo-accent, var(--rustipo-link));
  color: var(--rustipo-base, var(--rustipo-bg));
}
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
  - `page_has_math`
  - `page_toc`
  - `previous_post`
  - `next_post`
- favicon helpers: `site_favicon`, `site_favicon_svg`, `site_favicon_ico`, `site_apple_touch_icon`
- style helpers from config:
  - `site_style.content_width`
  - `site_style.top_gap`
  - `site_style.vertical_align`
  - `site_style.line_height`
  - `site_style.body_font`
  - `site_style.heading_font`
  - `site_style.mono_font`
- `site_has_custom_css` (boolean, true when `static/custom.css` exists)
- `site_font_faces_css` (optional rendered `@font-face` rules for configured local fonts)

Rustipo also registers small Tera helpers for theme authors:

- `slugify` filter
- `format_date(format="...")` filter
- `abs_url(path="...")` function
- `asset_url(path="...")` function
- `tag_url(name="...")` function

### Markdown alert blockquotes

Rustipo renders supported GitHub-style alert blockquotes as normal blockquotes with stable
classes:

- `markdown-alert-note`
- `markdown-alert-tip`
- `markdown-alert-important`
- `markdown-alert-warning`
- `markdown-alert-caution`

This keeps admonitions theme-styleable without requiring raw HTML in content.

### Standalone Markdown images

Rustipo renders standalone Markdown images as figures with stable classes:

- `markdown-image`
- `markdown-image-wide`
- `markdown-image-full`
- `markdown-image-left`
- `markdown-image-center`
- `markdown-image-right`
- `markdown-image-img`
- `markdown-image-caption`

This keeps image captions, sizing, and alignment theme-styleable without requiring raw HTML in
content.

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

`page_toc` is an ordered, nested list of heading items for the current page.
Each item includes:

- `title`
- `id`
- `level`
- `children`

### Theme authoring conventions

Theme authors are encouraged to keep reusable Tera pieces under:

- `templates/partials/`
- `templates/macros/`

For a broader author guide, see [theme-tera.md](./theme-tera.md).
