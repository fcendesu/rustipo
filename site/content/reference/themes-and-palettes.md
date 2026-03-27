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

Theme authors can keep using plain `style.css`, or opt into SCSS by adding
`themes/<name>/static/style.scss`. Rustipo compiles that into the same `dist/style.css` output
path, so templates do not need a different asset reference.

Site-level overrides can follow the same pattern with `static/custom.scss`, which compiles into
`dist/custom.css`.

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

Themes consume two palette layers:

1. a stable semantic baseline that every palette must satisfy
2. a canonical richer layer that Rustipo always emits for stronger themes

### Stable semantic baseline

Themes can always rely on variables such as:

- `--rustipo-bg`
- `--rustipo-text`
- `--rustipo-link`
- `--rustipo-code-bg`

### Canonical richer layer

Themes can also use richer canonical tokens such as:

- `--rustipo-base`
- `--rustipo-surface-0`
- `--rustipo-accent`
- `--rustipo-success`

Rustipo derives those from palette-specific extra tokens when available, and otherwise falls back to the stable semantic fields.

### Raw family-specific extras

Palette files may also define extra tokens such as:

- `base`
- `surface0`
- `blue`
- `mauve`
- `green`

Rustipo exposes those as raw CSS variables like `--rustipo-token-blue`, but themes should treat those as optional family-specific extras rather than the core contract.

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
- [Theme contract](/reference/themes-and-palettes/)
