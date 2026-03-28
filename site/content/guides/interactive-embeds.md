---
title: Interactive embeds
summary: Reuse iframe and demo shortcodes in Markdown without dropping down to custom HTML for every page.
order: 5
---

# Interactive embeds

Rustipo keeps interactive content Markdown-first with reusable shortcodes.

## Supported embed paths

- `iframe` for hosted demos, sandboxes, and external widgets
- `demo` for local interactive mounts that need their own script or stylesheet assets

## `iframe` shortcode

Use `iframe` when the interactive content already lives elsewhere:

```md
{{< iframe src="https://example.com/demo" title="Hosted demo" height="420" >}}
```

Rustipo renders a predictable wrapper and iframe block without requiring raw HTML in the page body.

## `demo` shortcode

Use `demo` when the interactive block lives in your own site assets:

```md
{{< demo id="counter-demo" script="/demos/counter-demo.js" style="/demos/counter-demo.css" title="Counter demo" >}}
```

Rustipo renders a stable mount point and injects the referenced assets once for the page.

## Live example

The block below uses the same mount contract and assets that the `demo` shortcode is designed to declare:

<link rel="stylesheet" href="/demos/counter-demo.css">
<div class="rustipo-shortcode rustipo-demo" data-rustipo-demo="counter-demo"></div>
<script type="module" src="/demos/counter-demo.js"></script>

## Authoring notes

- use root-relative asset paths for local demos
- keep reusable demo scripts in `static/` so they are copied into `dist/`
- use one shortcode per repeated embed pattern instead of custom HTML fragments on each page
