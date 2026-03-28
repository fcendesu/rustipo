---
title: Images
summary: How Rustipo handles Markdown images, figure rendering, and built-in image processing.
order: 4
---

# Images

Rustipo supports two image layers:

- normal Markdown images for everyday authoring
- `resize_image(...)` for template-driven image derivatives

Use Markdown images when the page author owns the content. Use `resize_image(...)` when the theme or page template needs a specific output size.

## Markdown Images

A normal Markdown image:

```md
![Cover image](/images/cover.png "A short caption")
```

Rustipo turns standalone Markdown images into figure-style output with better prose spacing.

Current behavior includes:

- responsive figure rendering for standalone images
- caption extraction from the Markdown image title
- heading-safe image output inside normal page content

## Size And Alignment Directives

Rustipo supports simple directives in image titles for layout control.

Supported directives include:

- `wide`
- `full`
- `left`
- `center`
- `right`

Use these when you want author-controlled layout inside Markdown without dropping raw HTML into the page.

## When To Stay In Markdown

Markdown images are the right tool when:

- the image belongs to one content page
- the natural source file should be used as-is
- a caption matters to the reader
- the page author should control placement

## `resize_image(...)`

Templates can generate processed image derivatives during build time.

```html
{% set cover = resize_image(path="/images/cover.png", width=640, height=360, op="fit", format="png") %}
<img src="{{ cover.url }}" width="{{ cover.width }}" height="{{ cover.height }}" alt="Cover" />
```

Returned fields:

- `url`
- `static_path`
- `width`
- `height`
- `orig_width`
- `orig_height`

## Output Location

Generated derivatives are written into:

- `dist/processed-images/`

Rustipo reserves that output path for generated assets, so user content should not try to write files there directly.

## Source Lookup Rules

`resize_image(...)` looks for source files across:

- the project root
- `static/`
- `content/`
- `public/`
- inherited theme `static/` directories

That makes it useful both for site-specific images and theme-owned assets.

## Supported Operations

Current operations are:

- `fit_width`
- `fit_height`
- `fit`
- `fill`

`fit` preserves aspect ratio and avoids upscaling when the source image is already smaller than the requested box.

## Supported Formats

Current formats are:

- `auto`
- `jpg`
- `png`
- `webp`

`quality` currently affects JPEG output. Lossless outputs ignore it.

## Good Fit For Theme Authors

Use `resize_image(...)` when a template needs:

- predictable card thumbnails
- consistent hero image dimensions
- responsive image derivatives from one source asset
- output dimensions for width and height attributes

## Good Fit For Content Authors

Stay with Markdown images when you want:

- simple prose-first authoring
- captions in the content flow
- direct control from the page body
- no dependency on theme logic

## Related Reference

- [Content model](/reference/content-model/)
- [Themes and palettes](/reference/themes-and-palettes/)
- [Template-driven pages](/guides/template-driven-pages/)
