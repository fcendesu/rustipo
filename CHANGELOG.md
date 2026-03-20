# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.5.0...rustipo-v0.6.0) (2026-03-20)


### Features

* **content:** support generic nested custom pages for arbitrary directories ([76a8ddf](https://github.com/fcendesu/rustipo/commit/76a8ddfde5db1d962a9b407f9ad81fc1946fc32c))
* **content:** add Mermaid fenced code support with page-scoped runtime injection ([efab30f](https://github.com/fcendesu/rustipo/commit/efab30ff558bf7b8620ddb5326944ad0ebf53e77))
* **palette:** add palette selection, built-in Catppuccin/Tokyo Night/Gruvbox/Dracula palettes, and richer palette tokens ([285fbe0](https://github.com/fcendesu/rustipo/commit/285fbe00913296394999f2c066d055d053aae19a))
* **cli:** add `rustipo dev` and `rustipo palette use` workflows ([d99f7a6](https://github.com/fcendesu/rustipo/commit/d99f7a621747abb4e9ec2bde6040353f7a47cc97))
* **font:** add config-driven custom font support for themes and site config ([8622df0](https://github.com/fcendesu/rustipo/commit/8622df0e1d67184d79da8da189854e29160ea5c6))
* **theme:** add markdown prose defaults, explicit theme IDs, and refined typography defaults ([0f89e75](https://github.com/fcendesu/rustipo/commit/0f89e75837366fc84a1e55319e1a1808b449e2cf))
* **tera:** add helper functions, shared page context, and starter theme authoring patterns ([298415b](https://github.com/fcendesu/rustipo/commit/298415bbea5ec9265ad2e85fd83ee4c1ef5a7242))


### Bug Fixes

* **tera:** simplify shared render context ([45cc8e2](https://github.com/fcendesu/rustipo/commit/45cc8e20c590a08169a45df2aa96733b7d4bde47))

## [0.5.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.4.0...rustipo-v0.5.0) (2026-03-18)


### Features

* **serve:** print watch rebuild durations ([19366b1](https://github.com/fcendesu/rustipo/commit/19366b17ced8b6070c9bbb345d7be02fb1c1f8f1))


### Bug Fixes

* **serve:** skip no-op rebuilds in watch mode ([b2b2c72](https://github.com/fcendesu/rustipo/commit/b2b2c72d72ffd64d873a5cce3434900ffdfc4a9d))

## [0.4.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.3.1...rustipo-v0.4.0) (2026-03-17)


### Features

* **core:** add favicon support to scaffold and build ([0767723](https://github.com/fcendesu/rustipo/commit/07677235999a9443fedd0b562dc09cc0495f02b0))

## [0.3.1](https://github.com/fcendesu/rustipo/compare/rustipo-v0.3.0...rustipo-v0.3.1) (2026-03-17)


### Bug Fixes

* **content:** avoid panic when highlight theme is missing ([26b74ab](https://github.com/fcendesu/rustipo/commit/26b74ab75e59ea058330944fae20a31bcfabe6ea))

## [0.3.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.2.0...rustipo-v0.3.0) (2026-03-17)


### Features

* **render:** expose frontmatter in page templates ([57353e3](https://github.com/fcendesu/rustipo/commit/57353e3496a4fef3b6f2852239ed5202eb7223f3))
* **serve:** add live reload in watch mode ([b071ee4](https://github.com/fcendesu/rustipo/commit/b071ee47886dbf81d39db76605b1d484c580d6ff))


### Bug Fixes

* **output:** detect duplicate output route collisions ([09bda30](https://github.com/fcendesu/rustipo/commit/09bda300a58cbb2a75c7bde2efcfc17fec81f285))

## [Unreleased]

### Added

- `rustipo serve --watch` now injects live-reload script and auto-refreshes browser after successful rebuilds.
- Content page templates now receive full `frontmatter` metadata in render context.

### Fixed

- `rustipo build` now fails with a readable error when multiple pages map to the same output path.

## [0.2.0] - 2026-03-17

### Added

- `rustipo serve` now supports custom bind options with `--host` and `--port`.
- `rustipo theme list` now lists installed themes from `themes/*/theme.toml`.
- `rustipo serve --watch` now watches `content/`, `themes/`, `static/`, and `config.toml` and triggers rebuilds on change.
- Fenced Markdown code blocks are now rendered with syntax highlighting.
- `rustipo build` now generates RSS feed output at `dist/rss.xml` from dated blog posts.
- `rustipo build` now generates sitemap output at `dist/sitemap.xml` from rendered routes.
- Tag index pages are now generated at `/tags/<tag>/` from blog post frontmatter tags.
- Blog archive page is now generated at `/blog/archive/` with month-based groups for dated posts.
- `rustipo theme install <source>` now installs themes from GitHub shorthand/URL (and local git paths for development).
- `rustipo build` now generates search index output at `dist/search-index.json` for client-side search.
- Theme inheritance is now supported via `extends` in `theme.toml`, with parent-child template and static asset overrides.
- `rustipo deploy github-pages` now generates a GitHub Pages deployment workflow under `.github/workflows/deploy-pages.yml`.
- Markdown content now supports basic shortcodes: `youtube` and `link`.

## [0.1.0] - 2026-03-17

### Added

- CLI command structure for:
  - `rustipo new <site-name>`
  - `rustipo build`
  - `rustipo serve`
  - `rustipo theme list`
- Project scaffold generation in `rustipo new` with starter content, config, default theme templates, and starter CSS.
- Config loader for `config.toml` with typed models and readable error context.
- Content pipeline:
  - Markdown discovery from `content/`
  - YAML frontmatter parsing
  - Markdown to HTML conversion
  - Internal page modeling with route and slug rules
  - Draft exclusion (`draft: true`)
- Theme system:
  - Active theme loading from `themes/<theme>/`
  - Required template and metadata validation
  - Template rendering with `tera`
- Output generation:
  - Pretty URL file writing to `dist/`
  - Section index page rendering (`/blog/`, `/projects/`)
  - Static asset copy for theme and user assets
  - Collision detection for asset path conflicts
- Local preview server (`rustipo serve`) using `axum`, serving `dist/` at `127.0.0.1:3000`.
- CI workflow (`.github/workflows/ci.yml`) for format, clippy, and tests.
- Release automation with Release Please:
  - `.github/workflows/release-please.yml`
  - `release-please-config.json`
  - `.release-please-manifest.json`
- Integration test suite in `tests/cli_flow.rs` for:
  - `new -> build` output flow
  - `serve` missing-`dist` failure
  - `theme list` output behavior
- Core project docs and guides:
  - README, CONTRIBUTING, CODE_OF_CONDUCT, LICENSE
  - CLI/content/theme/roadmap/CI/tech-debt docs

### Changed

- Migrated local preview server implementation from `tiny_http` to `axum`.
- Updated roadmap and CLI docs to reflect milestone progress and implemented behavior.
- Example project is now source-only (example `dist/` output is no longer committed).

### Fixed

- `rustipo new` now scaffolds required default theme templates so a fresh project can run `rustipo build` successfully.
- Implemented `rustipo theme list` to read installed themes from `themes/*/theme.toml`.
