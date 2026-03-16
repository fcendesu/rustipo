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

Status: command skeleton exists; full build pipeline is not implemented yet.

## `rustipo serve`

Serves built static output locally.

Status: command skeleton exists; local server is not implemented yet.

## `rustipo theme list`

Lists available themes.

Status: command skeleton exists; listing logic is not implemented yet.
