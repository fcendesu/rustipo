---
title: Color Spaces
summary: A compact note about preserving intent across generated outputs.
---

# Color Spaces

## Signals and channels

Color data is only useful when the pipeline agrees on what the channels mean.

![Routing overview | wide]("/img/routing-overview.svg")

## Working assumption

In many web publishing workflows, the practical target is a web-safe space with predictable browser behavior.

> [!WARNING]
> If you discard or misread color information, images can still render, but they may no longer match the original intent.

## Cross-links

This note is linked from [Publishing checklist](/guides/publishing-checklist/#after-deploy) to exercise deep-link validation across nested pages.
