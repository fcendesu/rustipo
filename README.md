# Rustipo

Rustipo is an open-source, themeable static site generator written in Rust for portfolio websites.

Rustipo is Markdown-first for content authoring and uses Tera templates for reusable layout.

## Status

MVP complete, active post-MVP development.

## CLI

- `rustipo new <site-name>`
- `rustipo build`
- `rustipo serve`
- `rustipo theme list`
- `rustipo theme install <source>`
- `rustipo deploy github-pages`

## Quick Start

```bash
cargo run -- new my-portfolio
cd my-portfolio
cargo run -- build
cargo run -- serve
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
```

## Current Features

- Markdown content pipeline with YAML frontmatter and draft filtering
- Tera-based theme templates for reusable page layouts
- Theme loading with inheritance support (`extends`) and contract validation
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

## Project Layout

```text
my-portfolio/
  content/
    index.md
    about.md
    resume.md
    blog/
    projects/
  static/
  themes/
  config.toml
```

## Authoring Model

- `content/` is where authors write Markdown content
- `themes/<theme>/templates/` defines reusable layout with Tera templates
- `themes/<theme>/static/` contains theme CSS and assets
- `dist/` is generated output only

## Example Project

- [basic-portfolio](./examples/basic-portfolio)

## Documentation

- [CLI reference](./docs/cli.md)
- [Project structure](./docs/project-structure.md)
- [Theme contract](./docs/theme-contract.md)
- [Content model](./docs/content-model.md)
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
