# Rustipo

Rustipo is an open-source, themeable static site generator written in Rust for blogs, notes, docs, personal sites, and other Markdown-first websites.

Rustipo is Markdown-first for content authoring and uses Tera templates for reusable layout.
Rustipo separates layout from color selection: `theme` controls structure, while `palette`
controls the generated color tokens.

## Status

MVP complete, active post-MVP development.

## Why Rustipo

Rustipo aims for a simpler shape than a full frontend framework:

- Markdown-first authoring instead of component-heavy content pipelines
- Tera themes for reusable layout without a JavaScript runtime requirement
- palettes as a separate concern from layout, so visual identity can change without rewriting templates
- built-in publishing outputs such as RSS, sitemap, search index, `robots.txt`, and `404.html`
- a small CLI that covers the normal authoring loop from `new` to `build` and deployment helpers

## Best For

Rustipo is a strong fit when you want to build:

- blogs and writing-focused sites
- notes or knowledge-base sites
- project or personal websites
- documentation sites with a static publishing flow

If you want a Markdown-first static generator with a smaller mental model than a full app
framework, Rustipo is meant to feel approachable.

## Quick Links

- [Published docs site](https://fcendesu.github.io/rustipo/)
- [Contributing guide](./CONTRIBUTING.md)
- [Roadmap](./docs/roadmap.md)
- [Example sites](#example-sites)
- [GitHub Releases](https://github.com/fcendesu/rustipo/releases)

## CLI Surface

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
- `rustipo deploy cloudflare-pages`
- `rustipo deploy netlify`

## Installation And Quick Start

Install Rustipo from crates.io:

```bash
cargo install rustipo
```

Install a prebuilt binary from the GitHub releases page:

- Linux x86_64: `x86_64-unknown-linux-gnu`
- macOS Intel: `x86_64-apple-darwin`
- macOS Apple Silicon: `aarch64-apple-darwin`
- Windows x86_64: `x86_64-pc-windows-msvc`

1. Download the archive for your platform from [GitHub Releases](https://github.com/fcendesu/rustipo/releases).
2. Extract it.
3. Move `rustipo` (or `rustipo.exe`) somewhere on your `PATH`.

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

## Core Strengths

- Markdown content with YAML frontmatter and draft filtering
- Tera-based theme templates with inheritance support (`extends`)
- theme installation from GitHub shorthand, URLs, or local git repositories
- palette-driven styling with built-in and local palettes
- docs-oriented authoring features like:
  - table of contents
  - math rendering
  - admonitions
  - image captions and layout helpers
  - internal link and deep-link validation
- built-in publishing outputs:
  - `dist/rss.xml`
  - `dist/sitemap.xml`
  - `dist/search-index.json`
  - `dist/robots.txt`
  - `dist/404.html`
- deployment workflow generation for:
  - GitHub Pages
  - Cloudflare Pages
  - Netlify

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

## Built-In Themes And Palettes

Built-in themes:

- `atlas`
- `journal`

Built-in palettes:

- `dracula`
- `default`
- `catppuccin-frappe`
- `catppuccin-latte`
- `catppuccin-macchiato`
- `catppuccin-mocha`
- `gruvbox-dark`
- `tokyonight-storm`
- `tokyonight-moon`

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
- [docs site project](./site)

## Documentation

- [published docs site](https://fcendesu.github.io/rustipo/)
- [docs site source](./site)
- [Contributing guide](./CONTRIBUTING.md)
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

Rustipo is an open-source project, and contributions are welcome across code, docs, examples, and
bug reports.

Start here:

- [Contributing guide](./CONTRIBUTING.md)
- [Roadmap](./docs/roadmap.md)
- [published docs site](https://fcendesu.github.io/rustipo/)

Quick contributor loop:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test -q
```

If your change affects the docs site:

```bash
cd site
cargo run --quiet -- build
```

## License

MIT license ([LICENSE.md](./LICENSE.md)).

## Credits

Rustipo was inspired by my friend's project, [Nerdfolio](https://github.com/atasoya/nerdfolio), created by [@atasoya](https://github.com/atasoya).
