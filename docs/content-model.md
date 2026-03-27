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

These frontmatter fields are exposed to page templates under `frontmatter` and page-level convenience keys (for example `page_date`, `page_summary`, `page_description`, `page_tags`, and `page_taxonomies`).

`summary` is also the first input Rustipo uses for built-in meta description rendering in themes. If `summary` is empty, Rustipo falls back to the site-level `description` from `config.toml`.

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
- In production output, future-dated content stays hidden until the current UTC date reaches the
  configured `date`.

## URL rules

- `content/index.md` -> `/`
- standalone pages use pretty URLs (`/about/`, `/resume/`)
- nested custom pages outside `blog/` and `projects/` preserve directory structure
  - `content/notes/rust/tips.md` -> `/notes/rust/tips/`
  - `content/notes/index.md` -> `/notes/`
- blog/project items map to section routes (`/blog/<slug>/`, `/projects/<slug>/`)
- when blog pagination is needed, listing routes become `/blog/` and `/blog/page/<n>/`
- `slug` frontmatter overrides filename-derived slug
- nested directory indexes ignore `slug` for route derivation and keep directory-index routes
- slugs are normalized to lowercase kebab-case
- nested content under `blog/` and `projects/` remains invalid; those sections stay one-level only

## Internal links and deep links

Rustipo supports normal Markdown links for internal navigation:

```md
[About](/about/)
[Guide](../guide/)
[Install section](#install)
[Guide install](/guide/#install)
```

Recommended authoring model in v1:

- use root-relative links like `/about/` when you want the clearest and most stable page links
- use normal relative links when linking within a nested section
- use `#heading-id` for same-page deep links
- use `/page/#heading-id` for cross-page deep links

Heading IDs come from heading text and follow the same slugification rules as `page_toc`.
Duplicate headings are de-duplicated with numeric suffixes such as `-2` and `-3`.

During `rustipo build` and `rustipo check`, Rustipo validates internal Markdown links against
generated routes and validates heading fragments for content pages when possible.
External links and asset-like paths are left alone.

## Taxonomy model

In `v0.15`, Rustipo formalizes `tags` as the only built-in taxonomy.

Current behavior:

- `tags` in frontmatter remain the authoring field
- Rustipo exposes raw `page_tags` for compatibility
- Rustipo also exposes generic `page_taxonomies.tags` entries with:
  - `name`
  - `slug`
  - `route`
- Rustipo generates:
  - `/tags/` taxonomy index page
  - `/tags/<tag>/` term pages
- taxonomy listing pages are currently derived from blog post tags

This keeps the current product simple while giving theme authors a stable template contract for future taxonomy expansion.

## Draft and scheduled publishing behavior

- Entries with `draft: true` are excluded from production build output.
- Entries with a future `date` are excluded from production build output until the current UTC
  date reaches that frontmatter value.
- `rustipo build` and `rustipo check` use production behavior.
- `rustipo dev` and `rustipo serve --watch` use preview behavior and include drafts and
  future-dated content locally.

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

## Images

Rustipo improves standalone Markdown images without requiring raw HTML.

Basic standalone images render as responsive figure wrappers:

```md
![A lighthouse](/img/lighthouse.jpg "Lighthouse at dusk")
```

Standalone image titles become captions.

Rustipo also supports an optional leading directive block in the image title for alignment and
size:

```md
![JPEG pipeline](/img/jpeg.png "{wide right} JPEG pipeline overview")
```

Supported image directives in v1:

- `left`
- `center`
- `right`
- `wide`
- `full`

Supported standalone images render with stable `markdown-image*` classes for theme styling.

For generated thumbnails, cover derivatives, or resized screenshots in templates, Rustipo also
provides a built-in `resize_image(...)` Tera helper that writes processed derivatives into
`dist/processed-images/`.
Unknown directives degrade safely by leaving the full title text as the caption.

## Shortcodes

Supported shortcodes:

- `{{< youtube id="VIDEO_ID" >}}`
- `{{< link href="URL" text="Label" >}}`
- `{{< iframe src="URL" title="Label" height="420" >}}`
- `{{< demo id="NAME" script="/demo.js" style="/demo.css" title="Label" >}}`

Notes:

- Shortcodes are rendered before Markdown conversion.
- `demo` shortcodes can declare page-scoped script and stylesheet assets, which Rustipo injects
  once for the rendered page.
- Unknown or malformed shortcodes are left as plain text.
- Shortcodes inside fenced code blocks are not rendered.
