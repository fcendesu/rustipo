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
    favicon.svg
    custom.css (optional)
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
- `static/custom.css` (optional): loaded after theme CSS when present for user overrides
- `themes/`: theme files (templates + theme assets + metadata)
- `config.toml`: site-level configuration
- `config.toml` can define style knobs under `site.layout` and `site.typography` (for example `content_width`, `top_gap`, `vertical_align`, `line_height`)
- `dist/`: generated static output (created by build step)
