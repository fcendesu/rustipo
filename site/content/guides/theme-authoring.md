---
title: Theme Authoring
summary: Build or customize a Rustipo theme by splitting content, Tera layout, palettes, static assets, and optional SCSS cleanly.
order: 3
---

# Theme Authoring

Rustipo themes are where site structure becomes reusable.

Site authors primarily write Markdown under `content/`. Theme authors decide how that content is framed, navigated, and styled through Tera templates, palette tokens, and static assets.

This guide is for building or customizing a theme, not for writing one special landing page. If you only need a designed homepage or one-off page layout, start with [Template-driven pages](/guides/template-driven-pages/).

## The Split To Keep

Use this model when you design a Rustipo theme:

- Markdown owns the page content
- frontmatter owns metadata and page-specific structured data
- Tera owns layout and repeated markup
- palettes own color tokens
- CSS and optional JS own presentation and interaction

A theme should make many Markdown pages feel coherent. It should not force authors to write layout HTML inside content files.

## A Typical Theme Layout

A local theme lives under `themes/<name>/`.

A healthy layout looks like this:

```text
my-site/
  themes/
    my-theme/
      theme.toml
      templates/
        base.html
        index.html
        page.html
        post.html
        project.html
        section.html
        partials/
      static/
        style.css
```

Themes can also inherit from another theme. In that case, Rustipo resolves templates and static assets across the inheritance chain and lets child files override parent files by relative path.

## Required Theme Files

A standalone theme should provide:

- `theme.toml`
- `templates/base.html`
- `templates/index.html`
- `templates/page.html`
- `templates/post.html`
- `templates/project.html`
- `templates/section.html`

Optional file:

- `templates/404.html`

Inherited themes can rely on required templates from a parent theme, so child themes do not need to duplicate every file.

## `theme.toml`

Every theme needs metadata in `theme.toml`.

```toml
name = "my theme"
version = "0.1.0"
author = "You"
description = "A Rustipo theme"
```

Useful optional fields:

- `id` for an explicit public theme ID
- `extends` for theme inheritance

Example:

```toml
id = "my-theme-clean"
name = "my theme"
version = "0.1.0"
author = "You"
description = "A clean docs-oriented theme"
extends = "atlas"
```

## What Each Template Usually Owns

A common Rustipo theme shape is:

- `base.html`: outer shell, head, global navigation, footer, shared assets
- `index.html`: homepage layout
- `page.html`: generic standalone pages
- `post.html`: blog post layout
- `project.html`: project detail layout
- `section.html`: listing pages such as blog and projects
- `404.html`: not-found page override when you want a custom one

This keeps one layout reusable across many Markdown files.

## Minimal Theme Example

### `templates/base.html`

```html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{{ page_title }} | {{ site_title }}</title>
  {% if page_description %}
  <meta name="description" content="{{ page_description }}" />
  {% endif %}
  <link rel="stylesheet" href="{{ asset_url(path='style.css') }}?v={{ site_asset_version }}" />
  <link rel="stylesheet" href="{{ asset_url(path='palette.css') }}?v={{ site_asset_version }}" />
  {% if site_has_custom_css %}
  <link rel="stylesheet" href="{{ asset_url(path='custom.css') }}?v={{ site_asset_version }}" />
  {% endif %}
  {% if site_analytics_head_html %}
  {{ site_analytics_head_html | safe }}
  {% endif %}
</head>
<body>
  {% block body %}{% endblock body %}
</body>
</html>
```

### `templates/page.html`

```html
{% extends "base.html" %}

{% block body %}
<main class="page-shell">
  <article class="page-body">
    <h1>{{ page_title }}</h1>
    {{ content_html | safe }}
  </article>
</main>
{% endblock body %}
```

That is enough to show the core Rustipo flow:

- Markdown becomes `content_html`
- frontmatter becomes page metadata
- the theme stays responsible for layout and assets

## Where `page_extra` Fits

Themes often need more than `title`, `summary`, and the page body.

Use frontmatter `extra` when a page needs structured values such as:

- hero copy
- card lists
- action buttons
- page-specific layout blocks

Rustipo exposes those values to themes as `page_extra`.

Example page frontmatter:

```yaml
---
title: Docs Home
extra:
  hero:
    heading: Build a site with a point of view.
  actions:
    - label: Documentation
      href: /guides/getting-started/
---
```

Example template code:

```html
{% if page_extra.hero %}
  <h1>{{ page_extra.hero.heading }}</h1>
{% endif %}

{% if page_extra.actions %}
  <nav>
    {% for action in page_extra.actions %}
      <a href="{{ action.href }}">{{ action.label }}</a>
    {% endfor %}
  </nav>
{% endif %}
```

If the data is unique to one page, it belongs in frontmatter or Markdown. If the markup is structural and reusable, it belongs in the theme template.

## Static Assets And SCSS

Themes can ship normal static files under `themes/<name>/static/`.

Typical examples:

- `style.css`
- images
- fonts
- JavaScript

Rustipo also supports optional SCSS for themes.

If a theme provides:

- `themes/<name>/static/style.scss`

Rustipo compiles it into:

- `dist/style.css`

That means templates can keep referencing the same final asset path whether the theme uses CSS or SCSS.

Site authors can layer on top of the theme with:

- `static/custom.css`
- `static/custom.scss`

Those compile or copy into `dist/custom.css`.

## Palettes And Theme Responsibility

Rustipo separates structure from color on purpose.

- themes define layout, assets, and presentation rules
- palettes define color tokens

In practice, theme CSS should rely on Rustipo's generated variables instead of hardcoding one palette family.

Good theme CSS usually starts from stable tokens such as:

- `--rustipo-bg`
- `--rustipo-text`
- `--rustipo-link`
- `--rustipo-code-bg`

And for more expressive themes, the canonical richer layer:

- `--rustipo-surface-0`
- `--rustipo-surface-1`
- `--rustipo-accent`
- `--rustipo-success`
- `--rustipo-warning`
- `--rustipo-danger`

That keeps the theme reusable across different palettes.

## Navigation, Page State, And Common Helpers

Rustipo gives themes more than rendered page HTML.

Useful context values include:

- `page_kind`
- `current_section`
- `site_nav`
- `site_menus`
- `breadcrumbs`
- `page_toc`
- `page_has_math`
- `page_description`
- `page_taxonomies`
- `site_taxonomies`
- `site_asset_version`
- `site_analytics_head_html`

Useful Rustipo-specific Tera helpers include:

- `asset_url(path="...")`
- `abs_url(path="...")`
- `taxonomy_url(taxonomy="...", term="...")`
- `tag_url(name="...")`
- `resize_image(path="...", ...)`

You do not need to memorize the full surface while reading this guide. The important part is understanding that themes can stay declarative and still access navigation, metadata, analytics, and generated image derivatives.

## A Good Theme Author Workflow

1. Start with a theme shape and decide whether it is standalone or inherited.
2. Create `theme.toml` and the core templates.
3. Render a simple page through `base.html` and `page.html` first.
4. Add homepage or section-specific layout next.
5. Style with palette-aware CSS variables rather than palette-specific hardcoding.
6. Reach for `page_extra` when one page needs structured layout data.
7. Add optional SCSS only when it genuinely makes theme maintenance easier.

## Common Mistakes

The most common theme-authoring mistakes are:

- putting layout HTML into Markdown instead of templates
- hardcoding palette-family color names instead of Rustipo tokens
- duplicating content strings in Tera that should live with the page
- building one-off templates instead of reusable layout pieces
- treating `style.scss` as a different output asset instead of a source for `dist/style.css`

## Good Companion Pages

- [Template-driven pages](/guides/template-driven-pages/)
- [Themes and palettes](/reference/themes-and-palettes/)
- [Template context](/reference/template-context/)
- [Content model](/reference/content-model/)
- [CLI reference](/reference/cli/)
