# Content Model (MVP)

Rustipo's content model is Markdown-first.
Content lives in `content/`, while layout is provided separately by theme templates.

## Source structure

```text
content/
  index.md
  about.md
  resume.md
  blog/
    *.md
  projects/
    *.md
```

## Frontmatter fields

Supported fields for MVP:

- `title`
- `date`
- `summary`
- `tags`
- `draft`
- `slug`
- `order`
- `links`

These frontmatter fields are exposed to page templates under `frontmatter` and page-level convenience keys (for example `page_date`, `page_summary`, `page_tags`).

## Content vs layout

Rustipo is designed so authors usually do not write page HTML.

- Markdown files provide the page content
- frontmatter provides page metadata
- theme templates provide reusable layout

For example:

- `content/about.md` provides the body content for the about page
- `templates/page.html` controls how standalone pages are laid out
- `content/blog/*.md` supplies blog post content
- `templates/post.html` controls the shared blog post layout

### Date format

- `date` must use strict `YYYY-MM-DD` format.
- Invalid dates fail frontmatter parsing with a readable error.

## URL rules

- `content/index.md` -> `/`
- standalone pages use pretty URLs (`/about/`, `/resume/`)
- blog/project items map to section routes (`/blog/<slug>/`, `/projects/<slug>/`)
- `slug` frontmatter overrides filename-derived slug
- slugs are normalized to lowercase kebab-case

Current limitation:

- generic nested custom page routes outside top-level pages plus `blog/` and `projects/` are not yet supported.

## Draft behavior

Entries with `draft: true` are excluded from production build output.

## Code blocks

Fenced code blocks are syntax-highlighted in rendered HTML output.

## Shortcodes

Supported shortcodes:

- `{{< youtube id="VIDEO_ID" >}}`
- `{{< link href="URL" text="Label" >}}`

Notes:

- Shortcodes are rendered before Markdown conversion.
- Unknown or malformed shortcodes are left as plain text.
- Shortcodes inside fenced code blocks are not rendered.
