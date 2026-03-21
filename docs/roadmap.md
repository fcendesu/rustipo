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

For historical post-MVP batch planning, see:

- [v0.4.0 checklist](./v0.4.0-checklist.md)
- [MVP checklist](./mvp-checklist.md) (includes current post-MVP follow-up note)

## Upcoming milestones

- `v0.7.0`: adoption and reliability
  - crates.io publishing
  - installation and quickstart polish
  - `rustipo check`
  - fix generated GitHub Pages workflow for normal Rustipo sites
- `v0.8.0`: authoring
  - table of contents
  - math rendering
  - admonitions/callouts
  - improved image ergonomics
  - improved internal link and deep-link ergonomics
- `v0.9.0`: publishing and site structure
  - drafts and scheduled publishing
  - menus
  - breadcrumbs
  - pagination
  - `robots.txt`
  - built-in 404 page
- `v0.10.0`: discovery
  - flagship example sites
  - more built-in layout themes
- `v0.11.0`: distribution
  - prebuilt binaries
  - Homebrew distribution
  - docs site built with Rustipo

`#55` should lightly influence `v0.7.0` docs: installation and quickstart wording should already describe Rustipo as a broader Markdown-first static site generator, with portfolio sites treated as one supported use case rather than the product's whole identity.

Small publishing completeness items such as `robots.txt` and a built-in 404 page should stay in `v0.9.0` for now. They are valuable, but they are less urgent than the adoption and authoring work in `v0.7.0` and `v0.8.0`.

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
- support reusable embeds or shortcodes for interactive content
- consider optional analytics integration for common hosted and self-hosted setups
- formalize taxonomy and palette token contracts as the ecosystem grows
- consider multilingual support only after the single-language publishing model is more mature
- evaluate optional SCSS support for theme authoring while keeping plain CSS as the default
