---
title: Template Context
summary: Stable Tera values, page state, site state, and helper functions available to Rustipo themes.
order: 4
---

# Template Context

Rustipo renders Markdown first, then passes page and site data into Tera templates.

This page is the focused public reference for the stable template surface theme authors can rely on while building layouts, partials, navigation, and metadata output.

## What This Page Covers

Use this page when you need to know:

- which values are available in normal page templates
- where structured frontmatter data appears in templates
- which navigation and page-state helpers Rustipo exposes
- which Rustipo-specific Tera functions and filters are available

For the broader theme filesystem contract, continue to [Themes and palettes](/reference/themes-and-palettes/).

## Core Page Values

These values are the most common starting point in page-oriented templates such as `index.html`, `page.html`, `post.html`, and `project.html`.

- `content_html`: rendered Markdown body
- `frontmatter`: parsed frontmatter object
- `page_extra`: structured page data from frontmatter `extra`
- `page_title`: page title
- `page_summary`: page summary when present
- `page_description`: convenience metadata value derived from `page_summary` or `site_description`
- `page_date`: page date for dated content
- `page_tags`: compatibility tag list for tag-oriented themes
- `page_taxonomies`: current taxonomy map for generic taxonomy-aware themes

Minimal example:

```html
<article>
  <h1>{{ page_title }}</h1>
  {{ content_html | safe }}
</article>
```

## `page_extra` And Structured Frontmatter

`page_extra` is Rustipo's stable convenience value for nested frontmatter data from `extra`.

Example frontmatter:

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

Example template usage:

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

Rustipo also keeps the raw value available at `frontmatter.extra`, but `page_extra` is the cleaner theme-facing entry point.

## Page State And Navigation Values

Rustipo provides page-state values that help themes choose layout behavior and render navigation.

Common values include:

- `page_kind`: one of `index`, `page`, `post`, `project`, `section`
- `current_section`: current top-level site area such as `home`, `pages`, `blog`, `projects`, `archive`, or `tags`
- `site_nav`: ordered primary navigation items with `title`, `route`, and `active`
- `site_menus`: named configured menus from `config.toml`
- `breadcrumbs`: route-derived breadcrumb items for the current page or section
- `page_toc`: nested heading list for the current page
- `page_has_math`: whether the current page rendered math content
- `site_taxonomies`: available built-in taxonomies with route metadata
- `previous_post` and `next_post`: adjacent post navigation when relevant

Example breadcrumb rendering:

```html
{% if breadcrumbs | length > 1 %}
<nav aria-label="Breadcrumbs">
  <ol>
    {% for item in breadcrumbs %}
      <li>
        {% if item.linkable and not item.active %}
          <a href="{{ item.route }}">{{ item.title }}</a>
        {% else %}
          <span>{{ item.title }}</span>
        {% endif %}
      </li>
    {% endfor %}
  </ol>
</nav>
{% endif %}
```

Example TOC rendering:

```html
{% if page_toc | length > 0 %}
  <aside>
    <h2>On this page</h2>
    <ul>
      {% for item in page_toc %}
        <li><a href="{{ item.href }}">{{ item.title }}</a></li>
      {% endfor %}
    </ul>
  </aside>
{% endif %}
```

## Pagination Values

Section-style templates can also receive pagination state when Rustipo renders paginated listings.

Useful values are:

- `current_page`
- `total_pages`
- `prev_url`
- `next_url`

Example:

```html
{% if total_pages > 1 %}
<nav aria-label="Pagination">
  {% if prev_url %}
    <a href="{{ prev_url }}">Previous</a>
  {% endif %}

  <span>Page {{ current_page }} of {{ total_pages }}</span>

  {% if next_url %}
    <a href="{{ next_url }}">Next</a>
  {% endif %}
</nav>
{% endif %}
```

## Site-Level Values

Rustipo also exposes site-wide values that are useful in base templates and shared partials.

Common values include:

- `site_title`
- `site_description`
- `site_asset_version`
- `site_has_custom_css`
- `site_font_faces_css`
- `site_analytics_head_html`
- `site_style.content_width`
- `site_style.top_gap`
- `site_style.vertical_align`
- `site_style.line_height`
- `site_style.body_font`
- `site_style.heading_font`
- `site_style.mono_font`

Typical base-template usage:

```html
<title>{{ page_title }} | {{ site_title }}</title>
{% if page_description %}
<meta name="description" content="{{ page_description }}" />
{% endif %}
<link rel="stylesheet" href="{{ asset_url(path='style.css') }}?v={{ site_asset_version }}" />
{% if site_analytics_head_html %}
{{ site_analytics_head_html | safe }}
{% endif %}
```

## Rustipo-Specific Helpers

Rustipo registers a small set of helpers on top of normal Tera functionality.

### Filters

- `slugify`
- `format_date(format="...")`

Example:

```html
{{ page_date | format_date(format="%B %d, %Y") }}
{{ "Site Gen" | slugify }}
```

### Functions

- `abs_url(path="...")`
- `asset_url(path="...")`
- `tag_url(name="...")`
- `taxonomy_url(taxonomy="...", term="...")`
- `resize_image(path="...", ...)`

Examples:

```html
<a href="{{ abs_url(path='/resume/') }}">Resume</a>
<img src="{{ asset_url(path='img/avatar.png') }}" alt="Avatar" />
<a href="{{ taxonomy_url(taxonomy='tags', term='Site Gen') }}">Site Gen</a>
```

`resize_image(...)` generates processed derivatives during rendering and returns:

- `url`
- `static_path`
- `width`
- `height`
- `orig_width`
- `orig_height`

Example:

```html
{% set cover = resize_image(path="/images/cover.png", width=640, height=360, op="fit", format="png") %}
<img src="{{ cover.url }}" width="{{ cover.width }}" height="{{ cover.height }}" alt="Cover" />
```

## What Themes Should Usually Rely On

When possible, themes should prefer the stable convenience values over lower-level config access.

Good examples:

- `page_description` instead of manually rebuilding metadata fallback logic
- `page_extra` instead of reaching deep into raw frontmatter for structured page data
- `site_analytics_head_html` instead of hardcoding analytics provider behavior in templates
- `site_nav` and `site_menus` instead of rebuilding navigation from routes manually

## Good Companion Pages

- [Theme authoring](/guides/theme-authoring/)
- [Themes and palettes](/reference/themes-and-palettes/)
- [Content model](/reference/content-model/)
- [Images](/reference/images/)
- [Taxonomies](/reference/taxonomies/)
