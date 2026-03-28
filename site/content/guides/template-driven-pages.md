---
title: Template-Driven Pages
summary: Keep page content in Markdown while using Tera for custom landing pages, hero layouts, and structured page sections.
order: 2
---

# Template-Driven Pages

Rustipo is Markdown-first, but some pages want more than prose flowing through a generic layout.

Good examples:

- a custom homepage hero
- a product landing page
- a page with repeated cards or action blocks
- a docs entry page with structured sections

The goal is not to abandon Markdown. The goal is to keep content in Markdown and frontmatter while letting Tera handle layout.

## The Split To Aim For

Use this mental model:

- Markdown owns the message
- frontmatter owns page metadata and structured page data
- Tera owns layout and reusable markup
- CSS and JS own presentation and interaction

If a page starts drifting toward large HTML blocks inside `.md`, that is usually a sign the layout belongs in a template instead.

## What Should Stay In Markdown

Keep normal page content in the page body when the page is mostly prose:

- headings
- paragraphs
- lists
- code blocks
- images
- shortcodes

Use frontmatter for content that configures the template:

- `title`
- `summary`
- `tags`
- `order`
- `extra`

### Good Markdown candidates

Markdown is the right home when the content mostly reads like a document:

- a guide page with headings and prose
- a reference page with examples
- a changelog-style page
- a homepage section that is mostly editorial copy

### Bad Markdown candidates

Markdown becomes awkward when the page needs repeated visual structure:

- a hero with multiple action buttons
- a grid of feature or site-shape cards
- a landing page with install controls and animated UI pieces
- a page with nested blocks that want shared wrappers and classes

## When To Reach For `extra`

`extra` is Rustipo's structured frontmatter escape hatch for page-specific data.

Use it when a page needs nested data such as:

- hero copy
- action buttons
- install commands
- feature cards
- page-specific callout groups

Example:

```yaml
---
title: Rustipo Docs
summary: Publish with structure, not sprawl.
extra:
  eyebrow: Markdown-first publishing system
  hero:
    heading: Build a site with a point of view.
    lead: Rustipo is for docs, journals, notes, and personal sites that want structure without becoming framework sprawl.
  actions:
    - label: Documentation
      href: /guides/getting-started/
    - label: GitHub
      href: https://github.com/fcendesu/rustipo
      external: true
  install:
    command: cargo install rustipo
---
```

Rustipo exposes that data in templates as:

- `frontmatter.extra`
- `page_extra`

In most theme code, `page_extra` is the cleaner choice.

### Why `extra` matters

`extra` lets Rustipo stay small at the core.

Without it, every custom page shape would pressure Rustipo to add more built-in frontmatter fields. With it, page authors can carry structured page-specific data while theme authors keep the markup in Tera.

## What Should Stay In Tera

Keep layout logic in templates:

- hero wrappers
- card grids
- repeated call-to-action markup
- loops over structured page data
- fallback rendering when optional data is missing

Example:

```html
{% if page_extra.hero %}
  <section class="hero">
    <p class="eyebrow">{{ page_extra.eyebrow }}</p>
    <h1>{{ page_extra.hero.heading }}</h1>
    <p>{{ page_extra.hero.lead }}</p>
  </section>
{% endif %}

{% if page_extra.actions %}
  <div class="hero-actions">
    {% for action in page_extra.actions %}
      <a href="{{ action.href }}">{{ action.label }}</a>
    {% endfor %}
  </div>
{% endif %}
```

That keeps the structure reusable while letting page authors update the content from frontmatter.

### What Tera should own

Tera should be responsible for:

- loops over repeated content blocks
- conditional rendering for optional content
- structural wrappers and shared classes
- links between page data and visual layout
- fallback rendering when page data is incomplete

## A Good Rustipo Workflow

For a highly designed page:

1. Start with the normal Markdown page and decide what content is truly page-specific.
2. Move repeated or structured page data into `extra`.
3. Render that data from `index.html`, `page.html`, or another page template.
4. Keep the page body for normal prose sections if the page still needs them.
5. Let CSS and optional JS handle polish and interactions.

This is the pattern Rustipo's own docs landing page now uses.

## A Small Example

Here is a healthy split for a custom landing page.

### `content/index.md`

```yaml
---
title: Rustipo Docs
summary: Publish with structure, not sprawl.
extra:
  hero:
    heading: Build a site with a point of view.
    lead: Write in Markdown. Shape with Tera.
  actions:
    - label: Documentation
      href: /guides/getting-started/
    - label: GitHub
      href: https://github.com/fcendesu/rustipo
      external: true
---
```

### `templates/index.html`

```html
{% if page_extra.hero %}
  <section class="hero">
    <h1>{{ page_extra.hero.heading }}</h1>
    <p>{{ page_extra.hero.lead }}</p>
  </section>
{% endif %}

{% if page_extra.actions %}
  <nav class="hero-actions">
    {% for action in page_extra.actions %}
      <a href="{{ action.href }}">{{ action.label }}</a>
    {% endfor %}
  </nav>
{% endif %}
```

That gives you:

- content authors editing structured page content in Markdown frontmatter
- theme authors controlling layout in Tera
- no giant HTML block inside the Markdown file

## Rules Of Thumb

When you are unsure where something belongs, use these checks:

### Put it in Markdown when

- the content is mostly written prose
- the page should still make sense without custom layout
- the author should be able to edit it like a normal document

### Put it in `extra` when

- the data is page-specific
- the content is structured, nested, or repeated
- the page template needs predictable named values

### Put it in Tera when

- the markup is structural
- the pattern repeats across multiple values
- the page needs real layout composition instead of flowing prose

## Common Mistakes

The most common problems are:

- putting full layout HTML into Markdown
- adding too many one-off top-level frontmatter fields
- making Tera templates own content text that should live with the page
- mixing content wording, structural markup, and visual styling in the same file

## What To Avoid

Try not to:

- dump large raw HTML blocks into Markdown just to force a layout
- turn every custom page need into a new built-in frontmatter field
- mix content decisions and presentation decisions in the same place

If the content is unique to the page, put it in frontmatter or Markdown.
If the markup is structural or repeated, put it in the template.

## Where This Fits In The Product

This pattern is especially useful for:

- custom homepages
- docs landing pages
- example index pages
- product overview pages
- personal-site profile pages with structured highlights

It lets Rustipo stay honest about its model:

- Markdown for content
- Tera for shape
- palettes for tone

## Next Step

Once the content and layout split feels right, continue to [Building the docs site](/guides/building-the-docs-site/) to see how Rustipo's own docs project applies the same pattern in a real in-repo site.

## Related Pages

- [Getting started](/guides/getting-started/)
- [Theme authoring](/guides/theme-authoring/)
- [Building the docs site](/guides/building-the-docs-site/)
- [Content model](/reference/content-model/)
- [Themes and palettes](/reference/themes-and-palettes/)
