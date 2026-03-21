# Project Structure

## Anatomy Of A Rustipo Site

Rustipo projects are organized around a simple model:

- Markdown = content
- themes = layout
- palettes = colors
- static = assets
- dist = generated output

## Generated site scaffold

`rustipo new <site-name>` generates:

```text
<site-name>/
  content/
    index.md
    about.md
    resume.md
    blog/
    projects/
  static/
    favicon.svg
    custom.css (optional)
  themes/
    default/
      templates/
      static/
      theme.toml
  palettes/ (optional)
    dracula.toml
  config.toml
```

## What each part is for

- `config.toml`: the main site configuration file
  - site title, description, selected theme, selected palette, layout settings, and typography settings live here
- `content/`: Markdown source content for pages, blog posts, and projects
  - `index.md` is the homepage
  - standalone pages such as `about.md` and `resume.md` become normal site pages
  - generic custom pages can be nested (for example `content/notes/rust/tips.md`)
  - nested `index.md` works as a directory index (`content/notes/index.md` -> `/notes/`)
  - `blog/` and `projects/` stay special one-level sections
- `themes/`: layout themes
  - `templates/` contains reusable Tera templates
  - `static/` contains theme CSS and theme assets
  - `theme.toml` contains theme metadata
- `palettes/` (optional): local palette overrides and custom color schemes (`*.toml`)
- built-in palettes are embedded in Rustipo and selectable without copying files into the project:
  - `dracula`
  - `default`
  - `catppuccin-frappe`
  - `catppuccin-latte`
  - `catppuccin-macchiato`
  - `catppuccin-mocha`
  - `gruvbox-dark`
  - `tokyonight-storm`
  - `tokyonight-moon`
- `static/`: user-provided assets copied into the output
  - images, fonts, JavaScript files, favicons, and optional `custom.css` belong here
  - `static/custom.css` (optional) is loaded after theme CSS when present for user overrides
- `dist/`: generated static output (created by build step)

## Layout and typography configuration

- `config.toml` can define style knobs under `site.layout` and `site.typography`
- common settings include:
  - `content_width`
  - `top_gap`
  - `vertical_align`
  - `line_height`
  - `body_font`
  - `heading_font`
  - `mono_font`
- local font files can live under `static/fonts/` and be referenced from `[[site.typography.font_faces]]`
