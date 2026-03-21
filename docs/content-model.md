# Content Model (MVP)

Rustipo's content model is Markdown-first.
Content lives in `content/`, while layout is provided separately by theme templates.

## Source structure

```text
content/
  index.md
  about.md
  resume.md
  notes/
    index.md
    rust/
      tips.md
  blog/
    *.md
  projects/
    *.md
```

## Mental model

Rustipo keeps content and presentation separate:

- Markdown files in `content/` are the source of truth for page writing
- theme templates define reusable layout
- palettes define color systems
- static assets are copied into output

In practice:

- content authors mostly work in `content/` and `config.toml`
- theme authors work in `themes/<theme>/templates/` and `themes/<theme>/static/`

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
- nested custom pages outside `blog/` and `projects/` preserve directory structure
  - `content/notes/rust/tips.md` -> `/notes/rust/tips/`
  - `content/notes/index.md` -> `/notes/`
- blog/project items map to section routes (`/blog/<slug>/`, `/projects/<slug>/`)
- `slug` frontmatter overrides filename-derived slug
- nested directory indexes ignore `slug` for route derivation and keep directory-index routes
- slugs are normalized to lowercase kebab-case
- nested content under `blog/` and `projects/` remains invalid; those sections stay one-level only

## Draft behavior

Entries with `draft: true` are excluded from production build output.

## Code blocks

Fenced code blocks are syntax-highlighted in rendered HTML output.

Mermaid fences are also supported:

```md
```mermaid
graph TD
  A --> B
```
```

Rustipo renders Mermaid fences as diagram containers and injects the Mermaid runtime only on
pages that contain Mermaid blocks.

## Math

Rustipo also supports inline and block math in Markdown:

```md
Inline math: $a^2 + b^2 = c^2$

$$
\int_0^1 x^2 \, dx
$$
```

Rustipo parses math nodes during Markdown rendering and injects the KaTeX runtime only on pages
that contain math content.

## Admonitions

Rustipo supports GitHub-style alert blockquotes for callouts:

```md
> [!NOTE]
> Notes live here.

> [!TIP]
> Tips can include **Markdown** content.

> [!WARNING]
> Warnings stand out without raw HTML.
```

Supported alert types in v1:

- `NOTE`
- `TIP`
- `IMPORTANT`
- `WARNING`
- `CAUTION`

Supported alerts render as blockquotes with stable `markdown-alert-*` classes for theme styling.
Unsupported variants degrade to normal blockquotes.

## Shortcodes

Supported shortcodes:

- `{{< youtube id="VIDEO_ID" >}}`
- `{{< link href="URL" text="Label" >}}`

Notes:

- Shortcodes are rendered before Markdown conversion.
- Unknown or malformed shortcodes are left as plain text.
- Shortcodes inside fenced code blocks are not rendered.
