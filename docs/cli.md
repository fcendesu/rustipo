# CLI Reference

Rustipo provides the following commands.

## `rustipo new <site-name>`

Creates a new starter portfolio project directory.

Example:

```bash
rustipo new my-portfolio
```

Current behavior:

- Creates starter `content/`, `static/`, and `themes/default/` structure
- Writes starter `config.toml` and Markdown pages
- Writes required default theme templates and starter CSS
- Fails if target directory already exists

## `rustipo build`

Builds site content into static output (`dist/`).

Current behavior:

- Loads and validates `config.toml`
- Loads active theme from `themes/<theme>/`
- Validates required theme templates and `theme.toml`
- Discovers Markdown files from `content/`
- Parses frontmatter and excludes drafts
- Converts Markdown to HTML
- Renders pages through theme templates
- Writes rendered pages to `dist/` using pretty URL output paths
- Copies theme and user static assets into `dist/`
- Fails on static asset path collisions
- Generates section index pages for `/blog/` and `/projects/`

## `rustipo serve`

Serves built static output locally.

Status: command skeleton exists; local server is not implemented yet.

## `rustipo theme list`

Lists available themes.

Status: command skeleton exists; listing logic is not implemented yet.
