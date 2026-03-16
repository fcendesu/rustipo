# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.1.0...rustipo-v0.2.0) (2026-03-16)


### Features

* **cli:** add command handler module skeleton ([d6f3820](https://github.com/fcendesu/rustipo/commit/d6f3820d5c72affcd678585f0751afa72ce75561))
* **cli:** wire command dispatch in main ([e431377](https://github.com/fcendesu/rustipo/commit/e43137739845d1760dbe50d31b4bdf05f1bb887b))
* **config:** add config loader and command validation ([a52afe7](https://github.com/fcendesu/rustipo/commit/a52afe7606d21b5b8c0cb5ec8890c5601a5261f9))
* **content:** add markdown renderer with pulldown-cmark ([3ed74ff](https://github.com/fcendesu/rustipo/commit/3ed74ffdb7b1ede094bcb983a381f9acbfe9ced1))
* **content:** build page models from markdown content ([1a1dfcb](https://github.com/fcendesu/rustipo/commit/1a1dfcba38d5a1fd4e0a0af573fd8d962134595e))
* **content:** discover markdown files from content directory ([e77b531](https://github.com/fcendesu/rustipo/commit/e77b5310d60660f9902dd5663b876053d6c723d0))
* **content:** parse YAML frontmatter from markdown ([d00de17](https://github.com/fcendesu/rustipo/commit/d00de175a7f850b5300e8f5a2b5f0fcd0a93b877))
* **new:** scaffold starter site structure ([ac99064](https://github.com/fcendesu/rustipo/commit/ac990643d2d4de1dfd34bdf577cd36311e4b98e5))
* **output:** write dist files and copy static assets ([600e33a](https://github.com/fcendesu/rustipo/commit/600e33a6211792399a850cc2ad24ffaa46e36621))
* **render:** generate section index pages ([675be46](https://github.com/fcendesu/rustipo/commit/675be4625548134ea40d7e3cff752348610df91e))
* **render:** render pages with tera templates ([319c745](https://github.com/fcendesu/rustipo/commit/319c7459a3e2f4a127fd9d64a1c93e7256d20b58))
* **serve:** add local dist preview server ([e4871a0](https://github.com/fcendesu/rustipo/commit/e4871a0e0106ef81b7b75e671d41f01667d281c0))
* **theme:** implement theme list command ([3ddd531](https://github.com/fcendesu/rustipo/commit/3ddd5317d05d1d52a5a8ac7195e944e70e4ef9fb))
* **theme:** load and validate active theme ([f37c97f](https://github.com/fcendesu/rustipo/commit/f37c97f0c46e3f451f2872b220cfa999f819657b))


### Bug Fixes

* **new:** scaffold required default theme templates ([1749eaa](https://github.com/fcendesu/rustipo/commit/1749eaa49991d556bbda8581385abddd23d02c38))

## [Unreleased]

### Added

- Nothing yet.

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
