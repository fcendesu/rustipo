# Project Structure

## Generated site scaffold

`rustipo new <site-name>` generates:

```text
<site-name>/
  content/
    index.md
    about.md
    resume.md
    blog/
    projects/
  static/
  themes/
    default/
      templates/
      static/
      theme.toml
  config.toml
```

## Directory purpose

- `content/`: Markdown source content for pages, blog posts, and projects
- `static/`: user-provided static assets copied to output
- `themes/`: theme files (templates + theme assets + metadata)
- `config.toml`: site-level configuration
- `dist/`: generated static output (created by build step)
