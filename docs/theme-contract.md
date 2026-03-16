# Theme Contract (MVP)

Themes are filesystem-based and selected via site config.

## Required files

Each theme must include:

- `templates/base.html`
- `templates/page.html`
- `templates/post.html`
- `templates/project.html`
- `templates/section.html`
- `templates/index.html`
- `theme.toml`

## `theme.toml` fields

Minimum metadata fields:

- `name`
- `version`
- `author`
- `description`

## Rendering responsibilities

Theme defines presentation:

- HTML templates
- Theme static assets
- Theme metadata

Generator responsibilities:

- Load active theme
- Validate required files
- Render content through templates
- Copy theme assets to output
