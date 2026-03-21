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
- `page_has_math`
- `page_toc`
- `site_title`
- `site_description`
- `site_style.*`

Rustipo also injects stable navigation and page-state values:

- `page_kind`
- `current_section`
- `site_nav`
- `site_menus`
- `breadcrumbs`
- `page_has_math`
- `page_toc`
- `previous_post`
- `next_post`

See [theme-contract.md](./theme-contract.md) for the broader theme contract.

## Rustipo-specific Tera helpers

Rustipo currently registers:

- `slugify` filter
- `format_date(format="...")` filter
- `abs_url(path="...")` function
- `asset_url(path="...")` function
- `tag_url(name="...")` function

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
- `site_menus`: named menus from `config.toml`, exposed as `{ menu_name -> [items...] }`
- `breadcrumbs`: ordered breadcrumb items with `title`, `route`, `active`, `linkable`
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

`breadcrumbs` exposes route-derived breadcrumb items for the current page or section. Themes can
use `linkable` to avoid rendering dead links for intermediate route segments that do not have a
real page.

Example:

```html
{% if breadcrumbs | length > 1 %}
<nav aria-label="Breadcrumb">
  <ol>
    {% for item in breadcrumbs %}
    <li>
      {% if item.linkable and not item.active %}
      <a href="{{ item.route }}">{{ item.title }}</a>
      {% else %}
      <span {% if item.active %}aria-current="page"{% endif %}>{{ item.title }}</span>
      {% endif %}
    </li>
    {% endfor %}
  </ol>
</nav>
{% endif %}
```

`page_toc` is a nested list of heading items for the current page. Each item includes:

- `title`
- `id`
- `level`
- `children`

Example:

```html
{% if page_toc | length > 0 %}
<aside>
  <h2>Table of contents</h2>
  <ul>
    {% for item in page_toc %}
    <li>
      <a href="#{{ item.id }}">{{ item.title }}</a>
      {% if item.children | length > 0 %}
      <ul>
        {% for child in item.children %}
        <li><a href="#{{ child.id }}">{{ child.title }}</a></li>
        {% endfor %}
      </ul>
      {% endif %}
    </li>
    {% endfor %}
  </ul>
</aside>
{% endif %}
```

## Configured menus

Rustipo supports named menus in `config.toml`:

```toml
[menus]
main = [
  { title = "Home", route = "/" },
  { title = "Blog", route = "/blog/" },
  { title = "About", route = "/about/" },
]

footer = [
  { title = "GitHub", route = "https://github.com/fcendesu" },
]
```

Each menu item exposes:

- `title`
- `route`
- `active`

Templates can access them through `site_menus`:

```html
<nav>
  {% for item in site_menus.main %}
  <a href="{{ item.route }}" {% if item.active %}aria-current="page"{% endif %}>
    {{ item.title }}
  </a>
  {% endfor %}
</nav>

<footer>
  {% for item in site_menus.footer %}
  <a href="{{ item.route }}">{{ item.title }}</a>
  {% endfor %}
</footer>
```

When `menus.main` is configured, Rustipo uses it for `site_nav` as well. Without
`menus.main`, `site_nav` keeps the default generated navigation from available content.

Themes can use the same `page_toc` data for inline TOCs, sticky sidebar TOCs, or mobile
collapsible outlines. Rustipo only provides the heading tree and stable anchor ids; theme-side
layout and scroll behavior stay fully customizable.

## Starter theme structure

Rustipo does not require a special Tera directory layout beyond `templates/`, but this structure keeps themes easier to maintain:

```text
themes/<theme>/
  theme.toml
  templates/
    base.html
    index.html
    page.html
    post.html
    project.html
    section.html
    partials/
      head_assets.html
    macros/
      layout.html
  static/
    style.css
```

Recommended conventions:

- put reusable head, nav, footer, and metadata fragments in `templates/partials/`
- put reusable layout wrappers and helper rendering logic in `templates/macros/`
- keep page templates small and focused on composition
- keep page-specific writing in Markdown whenever possible

## Starter pattern example

A simple starter theme can use one shared include plus one macro import.

`base.html`:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{ page_title }}</title>
    {% include "partials/head_assets.html" %}
  </head>
  <body>
    {% block body %}{% endblock body %}
  </body>
</html>
```

`partials/head_assets.html`:

```html
<link rel="stylesheet" href="/style.css" />
<link rel="stylesheet" href="/palette.css" />
```

`macros/layout.html`:

```html
{% macro page_shell(content_html) %}
<main>
  {{ content_html | safe }}
</main>
{% endmacro page_shell %}
```

`page.html`:

```html
{% extends "base.html" %}
{% import "macros/layout.html" as layout %}
{% block body %}
{{ layout::page_shell(content_html=content_html) }}
{% endblock body %}
```

This pattern keeps the reusable shell in Tera while leaving the actual page writing in Markdown.

## Recommendation

Keep templates focused on layout and repeated structure.
Keep page-specific writing in Markdown whenever possible.

The stable template API should grow additively over time. As Rustipo gains more theme use
cases, new context values and helper functions may be added, but existing documented keys
should remain the baseline contract for theme authors.
