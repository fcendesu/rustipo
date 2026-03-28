---
title: Taxonomies
summary: Understand Rustipo's current taxonomy model, generated routes, and taxonomy template values.
order: 5
---

# Taxonomies

Rustipo currently formalizes one built-in taxonomy:

- `tags`

That keeps the content model simple while still giving themes a stable taxonomy-oriented API.

## Current Generated Routes

Rustipo generates:

- `/tags/`
- `/tags/<tag>/`

Those pages are derived from blog post tags in the current model.

## Where Tags Come From

Tags are declared in frontmatter:

```yaml
---
title: Release Notes
tags:
  - Rust
  - Site Gen
---
```

Rustipo keeps the familiar page-level convenience key:

- `page_tags`

It also exposes the broader taxonomy contract so theme code does not need to stay tag-specific forever.

## Stable Template Values

Themes can rely on:

- `page_taxonomies`
- `site_taxonomies`
- `taxonomy_name`
- `taxonomy_title`
- `taxonomy_terms`
- `taxonomy_term`
- `taxonomy_items`

For the current built-in taxonomy, this means themes can read:

- `page_taxonomies.tags`

while older simple themes can still use:

- `page_tags`

## URL Helper

Templates can generate taxonomy routes with:

```html
<a href="{{ taxonomy_url(taxonomy="tags", term="Site Gen") }}">Site Gen</a>
```

Current output:

```text
/tags/site-gen/
```

## Why Rustipo Uses This Shape

Rustipo deliberately does not ship a large multi-taxonomy model yet.

The current contract is designed to:

- keep the content model understandable
- support stable theme code
- make future taxonomy expansion possible without redesigning templates again

## When To Use `page_tags` vs `page_taxonomies`

Use `page_tags` when:

- you are building a simple tag-only theme
- you want the smallest possible template code

Use `page_taxonomies` when:

- you want theme code that follows the broader taxonomy contract
- you want your templates to stay future-friendly

## Site-Level Taxonomy Context

`site_taxonomies` exposes the taxonomies Rustipo knows about, including route and display metadata.

That makes it possible to render site-wide taxonomy navigation without hardcoding `/tags/` in every template.

## Related Reference

- [Content model](/reference/content-model/)
- [Themes and palettes](/reference/themes-and-palettes/)
- [CLI reference](/reference/cli/)
