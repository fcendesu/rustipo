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

See [theme-contract.md](./theme-contract.md) for the broader theme contract.

## Rustipo-specific Tera helpers

Rustipo currently registers:

- `slugify` filter
- `abs_url(path=\"...\")` function

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

## Recommendation

Keep templates focused on layout and repeated structure.
Keep page-specific writing in Markdown whenever possible.
