# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.16.1](https://github.com/fcendesu/rustipo/compare/rustipo-v0.16.0...rustipo-v0.16.1) (2026-03-29)


### Bug Fixes

* **serve:** allow serving dist without config ([deae8c0](https://github.com/fcendesu/rustipo/commit/deae8c02748cb059987517b39b041354be8e0318))
* **serve:** allow serving dist without config ([f2e2446](https://github.com/fcendesu/rustipo/commit/f2e2446ddf0d41ec9619650ce38570347d862e56))

## [0.16.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.15.0...rustipo-v0.16.0) (2026-03-28)


### Documentation

* add a theme authoring guide for Rustipo theme builders
* add a template context reference page for Tera values and helpers
* add a deployment guide covering GitHub Pages, Cloudflare Pages, and Netlify
* add a publishing outputs reference page for generated `dist/` artifacts

## [0.15.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.14.0...rustipo-v0.15.0) (2026-03-28)


### Features

* **atlas:** redesign and refine the docs-site landing and navigation ([62d7f1d](https://github.com/fcendesu/rustipo/commit/62d7f1d618ffeb45a995676b5c3ca1d7dd0ce40e), [a043829](https://github.com/fcendesu/rustipo/commit/a04382929c5bba5b5c10f5bf0b28324dfd0ecde1), [fcbc862](https://github.com/fcendesu/rustipo/commit/fcbc8626a2eb04bcc2b2feb1bdee19a24514c88a))
* **config:** generalize analytics configuration ([f770bcc](https://github.com/fcendesu/rustipo/commit/f770bcca7e09d3399d9769c0de43e4b4966d7207))
* **content:** add extensible frontmatter data ([d865020](https://github.com/fcendesu/rustipo/commit/d865020d7dca57942ee264e1f2c3aaeb04cbf5bf))
* **styles:** add optional scss support ([6cca999](https://github.com/fcendesu/rustipo/commit/6cca999df048989a4b1c73b4da3c0fd7ee404c60), [a81db76](https://github.com/fcendesu/rustipo/commit/a81db7649208d134d3798f618cf7943ca3344086))


### Bug Fixes

* **dev:** serve projects from configured base path ([39591c8](https://github.com/fcendesu/rustipo/commit/39591c80aa86b91e4c173f2853cef7668bbea952))
* **render:** satisfy clippy question-mark lint ([5c91b84](https://github.com/fcendesu/rustipo/commit/5c91b84e171cf2ea36dc56992f0f7dcf1d222b41))

## [0.14.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.13.0...rustipo-v0.14.0) (2026-03-27)


### Features

* **config:** add optional analytics integration ([7b8fc1e](https://github.com/fcendesu/rustipo/commit/7b8fc1e0fcbb3623203d5246837acbfcfa5314b9))
* **palette:** formalize canonical token contract ([7ff02ca](https://github.com/fcendesu/rustipo/commit/7ff02ca8cd58bb28cf4e3900fd7214645fe5a43f))
* **render:** formalize taxonomy context ([00db5d2](https://github.com/fcendesu/rustipo/commit/00db5d2e440e6ce1eebff7eff85d0005a7905100))

## [0.13.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.12.0...rustipo-v0.13.0) (2026-03-27)


### Features

* **content:** add interactive embed shortcodes ([065c6bf](https://github.com/fcendesu/rustipo/commit/065c6bfc32ec3f17fc77cb49543ecd3a4920bd3a))
* **images:** add built-in resize helper ([a830e41](https://github.com/fcendesu/rustipo/commit/a830e41d7440403cef24786ab6b4c8631990e663))

## [0.12.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.11.0...rustipo-v0.12.0) (2026-03-26)


### Features

* **deploy:** add cloudflare pages workflow helper ([09cd257](https://github.com/fcendesu/rustipo/commit/09cd257293f1d6dd77e52a93b04cfbb9f3876362))
* **deploy:** add cloudflare pages workflow helper ([02e2eb2](https://github.com/fcendesu/rustipo/commit/02e2eb2d193c996ab35b056841974ad3d4ad67c0))
* **deploy:** add netlify workflow helper ([d1e90c7](https://github.com/fcendesu/rustipo/commit/d1e90c7064808e94b0bc04bc2573601e66331f30))
* **deploy:** add netlify workflow helper ([e3ce858](https://github.com/fcendesu/rustipo/commit/e3ce8584642a1d7cc6b08cac99d7556f45be6f96))
* **docs:** publish docs site with github pages ([26cf0d7](https://github.com/fcendesu/rustipo/commit/26cf0d7345e5b2e8d4f187ef9ec95319fada3e4a))
* **docs:** publish docs site with github pages ([83c1e69](https://github.com/fcendesu/rustipo/commit/83c1e694f746614714d4fc1200e869f3c4466a7b))
* **theme:** add atlas drawer nav and back-to-top ([5c9b5ee](https://github.com/fcendesu/rustipo/commit/5c9b5eefcd1cbd1182bda3759ca04324d80d5fb7))
* **theme:** add atlas drawer nav and back-to-top ([00eee98](https://github.com/fcendesu/rustipo/commit/00eee982ec96889e8beb9aa15cad52965d6d2793))
* **theme:** add default meta description support ([d52c22b](https://github.com/fcendesu/rustipo/commit/d52c22bca1650f27c5c81d753040432763096644))
* **theme:** add default meta description support ([cdb689d](https://github.com/fcendesu/rustipo/commit/cdb689d7e8a4ec881ff7b53a18c7808435b96ce3))


### Bug Fixes

* **ci:** use supported intel macos runner ([52b4c04](https://github.com/fcendesu/rustipo/commit/52b4c0464bb6c46365f2b51b495591aff7419302))
* **ci:** use supported intel macos runner ([5a418a7](https://github.com/fcendesu/rustipo/commit/5a418a708f4258469cb09044246e2c17b0b99acd))
* **render:** honor base_url subpaths in public urls ([df6ae03](https://github.com/fcendesu/rustipo/commit/df6ae03e49fc83f09b1b5cc892136a10520ae59c))
* **render:** honor base_url subpaths in public urls ([fe37b21](https://github.com/fcendesu/rustipo/commit/fe37b21035262f1bd5981be972221fc65daa4307))
* **theme:** make atlas sidebar toggleable and bust asset cache ([a67f28a](https://github.com/fcendesu/rustipo/commit/a67f28a2d22b6b0f0293fcc7a462c01fbc9c6dc8))
* **theme:** make atlas sidebar toggleable and bust asset cache ([56ca273](https://github.com/fcendesu/rustipo/commit/56ca273246e124b221bd9ce63ec9bbb68fb56c09))
* **theme:** remove atlas breadcrumb header ([da3080d](https://github.com/fcendesu/rustipo/commit/da3080dfad61c3f3a8b88abbb9f17e2239a2c115))
* **theme:** remove atlas breadcrumb header ([7c04077](https://github.com/fcendesu/rustipo/commit/7c040774ce23cb1b1173db6395209c10f6d625bc))
* **theme:** restore atlas static sidebar ([b039add](https://github.com/fcendesu/rustipo/commit/b039addd590f0381ffcbf34a4516c2e3a2ba1b29))
* **theme:** restore atlas static sidebar ([368aabf](https://github.com/fcendesu/rustipo/commit/368aabf4dc51a0d073ba6a82dc97f676d4656ccc))

## [0.11.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.10.0...rustipo-v0.11.0) (2026-03-22)


### Features

* **distribution:** add Homebrew distribution support ([3b825d9](https://github.com/fcendesu/rustipo/commit/3b825d9ae66c09368aee90f47520f8e4eb0005d4), [4f9fa0c](https://github.com/fcendesu/rustipo/commit/4f9fa0c7ef5d16d36e94cafac480956cd0f8f153))
* **site:** add in-repo Rustipo docs site ([566f4a0](https://github.com/fcendesu/rustipo/commit/566f4a0b664413f1abdce14c302edbebb5233386), [3e553f1](https://github.com/fcendesu/rustipo/commit/3e553f128e20c42efd68558fd4454935d9ef6b75))


### Bug Fixes

* **distribution:** satisfy homebrew formula audit ([0907cbc](https://github.com/fcendesu/rustipo/commit/0907cbc02c524d2dfdf3b5734a958dec46d7f69a))

## [0.10.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.9.0...rustipo-v0.10.0) (2026-03-22)


### Features

* **examples:** add flagship journal and knowledge-base sites ([376016a](https://github.com/fcendesu/rustipo/commit/376016af77ae9e71ef401897fbaf283de9c11852))
* **theme:** add built-in atlas and journal themes ([5a12f7e](https://github.com/fcendesu/rustipo/commit/5a12f7e0166ce262db023f7ee2255443e1178f87))


### Bug Fixes

* **ci:** sync github release notes body ([7892315](https://github.com/fcendesu/rustipo/commit/78923157044a7598f0fea97d7a9ad91e973f4ed2))

## [0.9.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.8.0...rustipo-v0.9.0) (2026-03-21)


### Features

* **config:** add configurable site menus ([43f88a7](https://github.com/fcendesu/rustipo/commit/43f88a7ba514bbc11927c87cb8d0e1f8a8c71293))
* **content:** add draft and scheduled publishing support ([94418a6](https://github.com/fcendesu/rustipo/commit/94418a673df7eec9c55b53f402754103c166f8b7))
* **output:** generate built-in not-found page ([89b1141](https://github.com/fcendesu/rustipo/commit/89b1141a0787e7c7d54fd42c25022ce748d77c86))
* **output:** generate default robots.txt ([02e3da6](https://github.com/fcendesu/rustipo/commit/02e3da6e891633d936013027cf05acc914728171))
* **theme:** add breadcrumb support ([6d57c15](https://github.com/fcendesu/rustipo/commit/6d57c15a8269b7857846c757db229be5cf1aabd8))

## [0.8.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.7.0...rustipo-v0.8.0) (2026-03-21)


### Features

* **content:** add markdown admonition support ([c75e83c](https://github.com/fcendesu/rustipo/commit/c75e83cb61abf0bf3b09dc66c828643ba572e301))
* **content:** add page table of contents support ([b2ff366](https://github.com/fcendesu/rustipo/commit/b2ff3661f23fe7877573582468c3901acb40b481), [c53c896](https://github.com/fcendesu/rustipo/commit/c53c8964c01576798f02306d8cf14385477997ec))
* **content:** add page-scoped markdown math support ([704fbcb](https://github.com/fcendesu/rustipo/commit/704fbcb7bdb2a679459d204e3f9b34251ab664ee), [d7726e8](https://github.com/fcendesu/rustipo/commit/d7726e882d9fb7ae087f9f0c860e6f14a3ca8f37))
* **content:** improve markdown image ergonomics ([54fcfc5](https://github.com/fcendesu/rustipo/commit/54fcfc5151d637c8249410b9ea3e0f80a8ecab95))
* **content:** validate internal links and deep links ([cbabbf1](https://github.com/fcendesu/rustipo/commit/cbabbf11c2203a6a9fb17eb41999fe17daac2e70))

## [0.7.0](https://github.com/fcendesu/rustipo/compare/rustipo-v0.6.0...rustipo-v0.7.0) (2026-03-21)


### Features

* **cli:** add rustipo check command ([5688b08](https://github.com/fcendesu/rustipo/commit/5688b08f81423e996428f57db35bf919d4ae1807))


### Bug Fixes

* **deploy:** generate valid github pages workflow ([620ed7b](https://github.com/fcendesu/rustipo/commit/620ed7b5e1ec7bf45b5fb1ef5514be8fa0d0202a))
* **crate:** tighten crates.io publication metadata ([58eb761](https://github.com/fcendesu/rustipo/commit/58eb7619301db766ae4778e0df26fc7dc440efe6))

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
