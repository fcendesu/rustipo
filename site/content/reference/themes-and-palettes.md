---
title: Themes And Palettes
summary: Understand Rustipo's layout system, built-in themes, and generated palette tokens.
order: 3
---

# Themes And Palettes

Rustipo separates structure from color.

## Theme And Palette Selection

A site chooses one theme and one palette in `config.toml`.

```toml
theme = "atlas"
palette = "catppuccin-macchiato"
```

- themes define templates, layout, and static assets
- palettes define generated color tokens

## Built-In Themes

Rustipo currently ships these built-in themes:

| Theme | Best fit |
| --- | --- |
| `atlas` | docs, notes, nested-page sites |
| `journal` | blogs and long-form writing |

Local themes can still live under `themes/<name>/` when you want custom layouts or inheritance.

## Built-In Palettes

Rustipo currently ships these built-in palettes:

- `default`
- `dracula`
- `gruvbox-dark`
- `tokyonight-storm`
- `tokyonight-moon`
- `catppuccin-latte`
- `catppuccin-frappe`
- `catppuccin-macchiato`
- `catppuccin-mocha`

## Theme Listing

```bash
rustipo theme list
```

The output now distinguishes built-in themes from local project themes.

## Palette Listing

```bash
rustipo palette list
```

Palette CSS is generated into `dist/palette.css` during builds.

## Theme Contract

Themes consume stable semantic variables such as:

- `--rustipo-bg`
- `--rustipo-text`
- `--rustipo-link`
- `--rustipo-code-bg`

They can also use richer derived tokens such as:

- `--rustipo-base`
- `--rustipo-surface-0`
- `--rustipo-accent`
- `--rustipo-success`

## Image Processing Helper

Theme templates can also generate resized image derivatives during the build.

```html
{% set cover = resize_image(path="/images/cover.png", width=640, height=360, op="fit", format="png") %}
<img src="{{ cover.url }}" width="{{ cover.width }}" height="{{ cover.height }}" alt="Cover" />
```

The helper returns:

- `url`
- `static_path`
- `width`
- `height`
- `orig_width`
- `orig_height`

Rustipo writes those generated files into `dist/processed-images/`.

## Related Pages

- [CLI reference](/reference/cli/#theme-and-palette-commands)
- [Examples](/examples/)
