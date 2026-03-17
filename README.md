# Rustipo

Rustipo is an open-source, themeable static site generator written in Rust for portfolio websites.

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

## Current Features

- Markdown content pipeline with YAML frontmatter and draft filtering
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
