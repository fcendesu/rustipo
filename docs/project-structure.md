# Project Structure

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

## Directory purpose

- `content/`: Markdown source content for pages, blog posts, and projects
  - generic custom pages can now be nested (for example `content/notes/rust/tips.md`)
  - nested `index.md` works as a directory index (`content/notes/index.md` -> `/notes/`)
  - `blog/` and `projects/` stay special one-level sections
- `static/`: user-provided static assets copied to output
- `static/custom.css` (optional): loaded after theme CSS when present for user overrides
- `themes/`: theme files (templates + theme assets + metadata)
- `palettes/` (optional): local palette overrides and custom color schemes (`*.toml`)
- built-in palettes are embedded in Rustipo and selectable without copying files into the project:
  - `default`
  - `catppuccin-frappe`
  - `catppuccin-latte`
  - `catppuccin-macchiato`
  - `catppuccin-mocha`
  - `tokyonight-storm`
  - `tokyonight-moon`
- `config.toml`: site-level configuration
- `config.toml` can define style knobs under `site.layout` and `site.typography` (for example `content_width`, `top_gap`, `vertical_align`, `line_height`)
- `dist/`: generated static output (created by build step)
