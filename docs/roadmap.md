# Roadmap

MVP is complete.

Core product direction:

- authors write content in Markdown
- themes provide reusable Tera templates for layout and repeated structure
- palettes provide color tokens independently from layout themes

Current shipped post-MVP capabilities:

- `rustipo dev` for one-command local development
- palette selection and built-in presets
- custom font configuration and local `@font-face` support
- Mermaid fenced-code rendering
- nested custom pages outside special sections
- richer Tera helpers/context for theme authors
- refined default typography scale and prose rhythm
- completed `v0.7.0` adoption and reliability release
- completed `v0.8.0` core authoring release
- completed `v0.9.0` publishing and site structure release
- completed `v0.10.0` product identity and examples release
- completed `v0.11.0` distribution and docs release
- completed `v0.12.0` docs polish and deployment release
- flagship example sites for blog and docs-style use cases
- built-in `atlas` and `journal` layout themes
- generated publishing/search artifacts during build:
  - `dist/rss.xml`
  - `dist/sitemap.xml`
  - `dist/search-index.json`
  - `dist/robots.txt`
  - `dist/404.html`

For historical post-MVP batch planning, see:

- [v0.4.0 checklist](./v0.4.0-checklist.md)
- [MVP checklist](./mvp-checklist.md) (includes current post-MVP follow-up note)

## Recent releases

- `v0.7.0`: adoption and reliability
  - crates.io publishing
  - installation and quickstart polish
  - `rustipo check`
  - GitHub Pages workflow fix for normal Rustipo sites
- `v0.8.0`: authoring
  - table of contents
  - math rendering
  - admonitions/callouts
  - improved image ergonomics
  - improved internal link and deep-link ergonomics
- `v0.9.0`: publishing and site structure
  - draft and scheduled publishing
  - configurable menus
  - breadcrumbs
  - documented built-in pagination
  - `robots.txt`
  - built-in 404 page
- `v0.10.0`: product identity and examples
  - broader Markdown-first site-generator positioning
  - flagship `journal` and `knowledge-base` examples
  - built-in `atlas` and `journal` layout themes
- `v0.11.0`: distribution and docs
  - docs site built with Rustipo and published with GitHub Pages
  - prebuilt binaries
- `v0.12.0`: docs polish and deployment
  - docs-site visual and subpath fixes
  - built-in default meta descriptions
  - Cloudflare Pages deployment helper
  - Netlify deployment helper

## Completed upcoming batch work

- `v0.14.0`: rich content and media
  - reusable embeds or shortcodes for interactive content
  - built-in image processing and resize helpers

## Upcoming milestones

- `v0.15.0`: ecosystem consistency and reusable site conventions
  - formalize and expand taxonomy support
  - formalize the canonical palette token contract
  - optional analytics integration

Tracked maintenance follow-up:

- audit GitHub Actions workflows for Node 24 compatibility
  - kept separate because the remaining warning is blocked on upstream action updates

## Milestone 1: Foundation

- [x] CLI skeleton (`new`, `build`, `serve`, `theme list`)
- [x] `rustipo new` starter scaffold
- [x] config loader

## Milestone 2: Content pipeline

- [x] discover Markdown files
- [x] parse frontmatter
- [x] convert Markdown to HTML
- [x] create internal page models

## Milestone 3: Theme system

- [x] load active theme
- [x] validate theme structure
- [x] render pages with templates

## Milestone 4: Output generation

- [x] write pretty URLs to `dist/`
- [x] copy static and theme assets
- [x] generate blog/projects section indexes

## Milestone 5: Local preview

- [x] implement `rustipo serve`

## Milestone 6: OSS polish

- [x] README
- [x] CONTRIBUTING
- [x] LICENSE
- [x] CODE_OF_CONDUCT
- [x] example project

## Future improvements

- expand Tera theme ergonomics as real theme-author needs emerge:
  - more shared context fields
  - more helper functions/filters
  - stronger macro/partial conventions and examples
- continue growing the layout theme ecosystem now that palette/font foundations are in place
- consider optional analytics integration for common hosted and self-hosted setups
- formalize taxonomy and palette token contracts as the ecosystem grows
- evaluate optional SCSS support for theme authoring while keeping plain CSS as the default
- consider multilingual support only after the single-language publishing model is more mature
