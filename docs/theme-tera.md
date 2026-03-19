# Theme Tera Guide

Rustipo uses Tera for theme templating.

The intended authoring split is:

- site authors write content in Markdown under `content/`
- theme authors define reusable layout in `themes/<theme>/templates/`

Rustipo converts Markdown to HTML first, then injects that HTML and related metadata into Tera templates.

## Common template roles

- `base.html`: outer page shell
- `index.html`: homepage layout
- `page.html`: standalone pages such as about/resume
- `post.html`: blog posts
- `project.html`: project detail pages
- `section.html`: listing pages such as blog/projects

## Minimal example

```html
{% extends "base.html" %}
{% block body %}
<main>
  <h1>{{ page_title }}</h1>
  {{ content_html | safe }}
</main>
{% endblock body %}
```

In that example:

- `page_title` comes from page/frontmatter context
- `content_html` is the rendered Markdown body
- `safe` is required so the rendered Markdown HTML is not escaped

## Reuse across Markdown content

One template is reused across many Markdown files.

For example:

- every file in `content/blog/*.md` is rendered with `post.html`
- every standalone page such as `content/about.md` is rendered with `page.html`

That keeps content independent from presentation.

## Includes and inheritance

Tera supports template inheritance and includes, which Rustipo themes can use normally.

Example:

```html
{% include "partials/header.html" %}
{% block body %}{% endblock body %}
```

This is useful for shared headers, footers, nav blocks, and metadata fragments.

## Common context values

Rustipo injects common values such as:

- `content_html`
- `frontmatter`
- `page_title`
- `page_date`
- `page_summary`
- `page_tags`
- `site_title`
- `site_description`
- `site_style.*`

Rustipo also injects stable navigation and page-state values:

- `page_kind`
- `current_section`
- `site_nav`
- `previous_post`
- `next_post`

See [theme-contract.md](./theme-contract.md) for the broader theme contract.

## Rustipo-specific Tera helpers

Rustipo currently registers:

- `slugify` filter
- `format_date(format="...")` filter
- `abs_url(path=\"...\")` function
- `asset_url(path=\"...\")` function
- `tag_url(name=\"...\")` function

### `slugify`

```html
{{ "My Custom Tag" | slugify }}
```

Output:

```text
my-custom-tag
```

### `abs_url`

```html
{{ abs_url(path="/resume/") }}
```

If `base_url = "https://example.com"`, output becomes:

```text
https://example.com/resume/
```

### `format_date`

```html
{{ page_date | format_date(format="%B %d, %Y") }}
```

If `page_date = "2026-03-19"`, output becomes:

```text
March 19, 2026
```

### `asset_url`

```html
<img src="{{ asset_url(path="img/avatar.png") }}" alt="Avatar" />
```

Output:

```text
/img/avatar.png
```

### `tag_url`

```html
<a href="{{ tag_url(name="Site Gen") }}">Site Gen</a>
```

Output:

```text
/tags/site-gen/
```

## Stable template API

Theme authors can rely on these context keys being present in normal page templates:

- `page_kind`: one of `index`, `page`, `post`, `project`, `section`
- `current_section`: one of `home`, `pages`, `blog`, `projects`, `archive`, `tags`
- `site_nav`: ordered navigation items with `title`, `route`, `active`
- `previous_post` / `next_post`: adjacent blog post metadata when rendering a blog post

`previous_post` and `next_post` expose:

- `title`
- `route`
- `summary`
- `date`

Example:

```html
<nav>
  {% for item in site_nav %}
  <a href="{{ item.route }}" {% if item.active %}aria-current="page"{% endif %}>
    {{ item.title }}
  </a>
  {% endfor %}
</nav>

{% if previous_post %}
<a href="{{ previous_post.route }}">Previous: {{ previous_post.title }}</a>
{% endif %}
```

## Recommended theme structure

Rustipo does not require a special Tera directory layout beyond `templates/`, but these
conventions keep themes easier to maintain:

- `templates/base.html` for the outer shell
- `templates/partials/` for shared includes like nav, footer, metadata
- `templates/macros/` for reusable Tera macros

Example macro import:

```html
{% import "macros/meta.html" as meta %}
{{ meta::page_header(title=page_title, summary=page_summary) }}
```

## Recommendation

Keep templates focused on layout and repeated structure.
Keep page-specific writing in Markdown whenever possible.
