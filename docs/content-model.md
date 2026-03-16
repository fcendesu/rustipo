# Content Model (MVP)

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

## URL rules

- `content/index.md` -> `/`
- standalone pages use pretty URLs (`/about/`, `/resume/`)
- blog/project items map to section routes (`/blog/<slug>/`, `/projects/<slug>/`)
- `slug` frontmatter overrides filename-derived slug
- slugs are normalized to lowercase kebab-case

## Draft behavior

Entries with `draft: true` are excluded from production build output.
