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

Current limitation:

- Rendered output is not written to `dist/` yet (Milestone 4)

## `rustipo serve`

Serves built static output locally.

Status: command skeleton exists; local server is not implemented yet.

## `rustipo theme list`

Lists available themes.

Status: command skeleton exists; listing logic is not implemented yet.
