# Rustipo

Rustipo is an open-source, themeable static site generator written in Rust for blogs, notes, docs, personal sites, and other Markdown-first websites.

Rustipo is Markdown-first for content authoring and uses Tera templates for reusable layout.
Rustipo separates layout from color selection: `theme` controls structure, while `palette`
controls the generated color tokens.

## Status

MVP complete, active post-MVP development.

## CLI

- `rustipo new <site-name>`
- `rustipo check`
- `rustipo dev`
- `rustipo build`
- `rustipo serve`
- `rustipo theme list`
- `rustipo palette list`
- `rustipo palette use <id>`
- `rustipo theme install <source>`
- `rustipo deploy github-pages`

## Installation And Quick Start

Install Rustipo:

```bash
cargo install rustipo
```

Create a site:

```bash
rustipo new my-site
cd my-site
```

Choose a palette:

```bash
rustipo palette list
rustipo palette use catppuccin-mocha
```

Preview locally:

```bash
rustipo dev
```

Build static output:

```bash
rustipo build
```

Theme discovery:

```bash
rustipo theme list
rustipo theme install owner/repo
```

Current built-in themes:

- `atlas`
- `journal`

## Local Development From The Repository

If you are working on Rustipo itself instead of the published crate:

```bash
cargo run -- new my-site
cd my-site
../target/debug/rustipo dev
```

## Layout Without CSS Editing

Use `config.toml` for basic style controls:

```toml
[site.layout]
content_width = "98%"
top_gap = "2rem"
vertical_align = "center" # or "start"

[site.typography]
line_height = "1.5"
body_font = "\"Inter\", sans-serif"
heading_font = "\"Fraunces\", serif"
mono_font = "\"JetBrains Mono\", monospace"

[[site.typography.font_faces]]
family = "Inter"
source = "/fonts/inter.woff2"
weight = "400"
style = "normal"
```

## Current Features

- Markdown content pipeline with YAML frontmatter and draft filtering
- Tera-based theme templates for reusable page layouts
- Theme loading with inheritance support (`extends`) and contract validation
- Explicit theme IDs for clearer selection and variant naming (`family-variant`)
- Rich palette token aliases for expressive theme styling with semantic fallbacks
- Config-driven custom font families and local `@font-face` injection
- Refined default typography scale and prose rhythm for the starter theme
- Built-in palettes:
  - `dracula`
  - `default`
  - `catppuccin-frappe`
  - `catppuccin-latte`
  - `catppuccin-macchiato`
  - `catppuccin-mocha`
  - `gruvbox-dark`
  - `tokyonight-storm`
  - `tokyonight-moon`
- Pretty URL output in `dist/`
- Section/tag/archive generation:
  - `/blog/`, `/projects/`
  - `/tags/<tag>/`
  - `/blog/archive/`
- Output artifacts:
  - `dist/rss.xml`
  - `dist/sitemap.xml`
  - `dist/search-index.json`
- Local preview server with watch mode and live reload (`rustipo serve --watch`)
- Theme installation from GitHub shorthand/URL or local git repo
- GitHub Pages deploy workflow scaffolding
- Config-driven layout knobs (`content_width`, `top_gap`, `vertical_align`, `line_height`)
- Optional `static/custom.css` override loaded after theme CSS

## Anatomy Of A Rustipo Site

Rustipo projects are organized around a simple model:

- Markdown = content
- themes = layout
- palettes = colors
- static = assets
- dist = generated output

Typical project layout:

```text
my-site/
  config.toml
  content/
    index.md
    about.md
    resume.md
    blog/
    projects/
  palettes/
    dracula.toml
  static/
    fonts/
    img/
    js/
    favicon.svg
    custom.css
  themes/
    default/
      theme.toml
      templates/
      static/
  dist/
```

What each part is for:

- `config.toml` is the main site configuration file
  - site title, description, theme, palette, layout, and typography live here
- `content/` is where you write Markdown content
  - each Markdown file becomes a page
  - `content/index.md` is the homepage
  - `content/blog/` is for blog posts
  - `content/projects/` is for project pages
  - nested custom pages are supported outside `blog/` and `projects/`
- `themes/` contains reusable layout themes
  - `templates/` holds Tera HTML templates
  - `static/` holds theme CSS and theme assets
  - `theme.toml` describes the theme
- `palettes/` is for optional local color palettes
  - built-in palettes are embedded in Rustipo
  - local palettes let you add your own color systems with `*.toml`
- `static/` is for user assets copied into the final site
  - images, fonts, JavaScript, favicons, and optional `custom.css` belong here
- `dist/` is generated output
  - Rustipo recreates it when you build

## Authoring Model

- `content/` is where authors write Markdown content
- `themes/<theme>/templates/` defines reusable layout with Tera templates
- `themes/<theme>/static/` contains theme CSS and assets
- `palette = "..."` selects a built-in or local color palette
- optional local palettes live under `palettes/<palette>.toml`
- `dist/` is generated output only

## Example Sites

- [starter example site](./examples/basic-portfolio)
- [journal example](./examples/journal)
- [knowledge base example](./examples/knowledge-base)

## Documentation

- [CLI reference](./docs/cli.md)
- [Project structure](./docs/project-structure.md)
- [Theme contract](./docs/theme-contract.md)
- [Theme Tera guide](./docs/theme-tera.md)
- [Content model](./docs/content-model.md)
- [Release and publish workflow](./docs/release.md)
- [Roadmap](./docs/roadmap.md)
- [MVP checklist](./docs/mvp-checklist.md)
- [CI](./docs/ci.md)
- [Technical debt](./docs/tech-debt.md)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

MIT license ([LICENSE.md](./LICENSE.md)).

## Credits

Rustipo was inspired by my friend's project, [Nerdfolio](https://github.com/atasoya/nerdfolio), created by [@atasoya](https://github.com/atasoya).
