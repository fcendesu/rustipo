---
title: Getting Started
summary: Install Rustipo, create a site, choose a palette, and preview locally.
order: 1
---

# Getting Started

This guide walks through the smallest useful Rustipo workflow: install the binary, create a site, choose a palette, and preview locally.

## Install Rustipo

```bash
cargo install rustipo
```

If you are working from the repository instead of the published crate, use `cargo run --` while developing Rustipo itself.

## Create A Site

```bash
rustipo new my-site
cd my-site
```

The scaffold gives you a homepage, a couple of standalone pages, `blog/`, `projects/`, a default local theme, and a starter config.

## Choose A Palette

```bash
rustipo palette list
rustipo palette use catppuccin-mocha
```

Palette selection is separate from layout. The current theme keeps its structure, while Rustipo regenerates the color tokens used by that theme.

## Preview And Validate

```bash
rustipo check
rustipo dev
```

- `rustipo check` validates config, content, routes, themes, palettes, and internal links without writing `dist/`
- `rustipo dev` builds, serves, watches, and live-reloads a local preview

## Build Static Output

```bash
rustipo build
```

That writes the generated site into `dist/`.

## Next Reads

- [Deploying Rustipo sites](/guides/deploying-rustipo-sites/)
- [CLI reference](/reference/cli/)
- [Content model](/reference/content-model/)
- [Themes and palettes](/reference/themes-and-palettes/)

> [!TIP]
> If you are trying to understand how a docs-style site fits together, jump to [Build the docs site](/guides/building-the-docs-site/).
