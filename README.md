# Rustipo

Rustipo is an open-source, themeable static site generator written in Rust for portfolio websites.

## Status

MVP in progress.

## Planned CLI

- `rustipo new <site-name>`
- `rustipo build`
- `rustipo serve`
- `rustipo theme list`

## Quick Start (target workflow)

```bash
cargo run -- new my-portfolio
cd my-portfolio
cargo run -- build
cargo run -- serve
```

## Project Layout (target)

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

## Roadmap

See [MVP checklist](./docs/mvp-checklist.md).

## Documentation

- [CLI reference](./docs/cli.md)
- [Project structure](./docs/project-structure.md)
- [Theme contract](./docs/theme-contract.md)
- [Content model](./docs/content-model.md)
- [Roadmap](./docs/roadmap.md)
- [CI](./docs/ci.md)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

MIT license ([LICENSE.md](./LICENSE.md)).

## Credits

Rustipo was inspired by my friend's project, [Nerdfolio](https://github.com/atasoya/nerdfolio), created by [@atasoya](https://github.com/atasoya).
