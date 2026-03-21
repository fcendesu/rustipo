# Rustipo MVP Checklist

This checklist is derived from the acceptance criteria in `rustipo_prd.md`.

## Acceptance criteria

- [x] `rustipo new my-site` creates a usable starter site project
- [x] `rustipo build` converts Markdown content into a static site in `dist/`
- [x] Generated site includes homepage, about, resume, blog, and projects pages
- [x] Blog and project entries render with frontmatter metadata
- [x] Selected theme is loaded and applied during rendering
- [x] Theme assets and user static assets are copied into output
- [x] `rustipo serve` serves the built site locally
- [x] Invalid config/theme/content produces readable errors
- [x] Repository is publishable as an open-source project

## MVP status

- [x] MVP acceptance criteria complete.
- [x] Core authoring model established: Markdown content plus reusable Tera theme templates.

## Post-MVP follow-up (tracked separately)

- [x] Default markdown prose style pack (typography scale, spacing rhythm, code/link/table/blockquote/list/hr styles)
- [x] Palette system split from layout themes
- [x] Custom font support for theme/layout typography
- [x] Default typography refinement after font support

## Foundation tasks

- [x] Add minimal `README.md`
- [x] Add `CONTRIBUTING.md`
- [x] Add `LICENSE.md`
- [x] Add `CODE_OF_CONDUCT.md`
