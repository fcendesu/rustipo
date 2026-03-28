---
title: Analytics
summary: Understand Rustipo's generic analytics hook and the configuration shapes that feed it.
order: 6
---

# Analytics

Rustipo keeps analytics opt-in and generic.

The stable part of the contract is the template hook Rustipo exposes to themes when analytics is configured.

## The Stable Hook

Rustipo exposes analytics markup to templates through:

- `site_analytics_head_html`

Themes can include it in the shared document head:

```html
{% if site_analytics_head_html %}
{{ site_analytics_head_html | safe }}
{% endif %}
```

If no analytics integration is configured:

- `site_analytics_head_html` stays empty
- no analytics snippet is rendered

Built-in themes already include this hook for you.

## Why Rustipo Uses A Hook

Rustipo does not want every theme to hardcode provider-specific analytics logic.

This hook-based model keeps analytics:

- opt-in
- easy to remove
- consistent across built-in themes
- open to different snippet sources

## Script-Based Analytics

If your analytics provider is loaded with a shared script tag, configure:

Add this to `config.toml`:

```toml
[site.analytics]
domain = "docs.example.com"
script_src = "https://stats.example.com/js/script.js"
```

`domain` is optional. When present, Rustipo emits it as `data-domain` on the generated script tag.

## Inline Analytics HTML

If you want full control over the rendered snippet, configure:

```toml
[site.analytics]
head_html = '<script defer src="https://stats.example.com/js/script.js"></script>'
```

## When Site Authors Need To Do Nothing

If you use a built-in theme and configure analytics in either supported shape, you do not need custom template work.

## When Theme Authors Need It

Theme authors should rely on `site_analytics_head_html` rather than reaching into analytics config directly.

That keeps theme code stable if Rustipo changes how analytics is configured later.

## Privacy And Scope

Rustipo does not force analytics on any site. Analytics support is entirely configuration-driven.

## Related Reference

- [CLI reference](/reference/cli/)
- [Themes and palettes](/reference/themes-and-palettes/)
