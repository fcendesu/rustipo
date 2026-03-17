# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `rustipo serve` now supports custom bind options with `--host` and `--port`.
- `rustipo theme list` now lists installed themes from `themes/*/theme.toml`.
- `rustipo serve --watch` now watches `content/`, `themes/`, `static/`, and `config.toml` and triggers rebuilds on change.
- Fenced Markdown code blocks are now rendered with syntax highlighting.
- `rustipo build` now generates RSS feed output at `dist/rss.xml` from dated blog posts.
- `rustipo build` now generates sitemap output at `dist/sitemap.xml` from rendered routes.

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
