---
title: Publishing Checklist
summary: A release-oriented checklist for shipping a static site without surprises.
---

# Publishing Checklist

> [!IMPORTANT]
> Run `rustipo check` before you publish so broken internal links or missing configured assets are caught early.

## Before the build

- verify `base_url`
- confirm the selected theme and palette
- review draft content state

## Before deploy

- run `rustipo build`
- inspect `dist/404.html`
- inspect `dist/rss.xml`, `dist/sitemap.xml`, and `dist/robots.txt`

## After deploy

- open a nested page such as [Image pipeline](/notes/image-pipeline/)
- validate a deep link like [Color spaces > Signals and channels](/notes/color-spaces/#signals-and-channels)
