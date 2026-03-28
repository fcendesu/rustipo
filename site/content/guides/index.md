---
title: Guides
summary: Step-by-step workflows for starting a Rustipo site, authoring themes, deploying, shaping custom pages, and understanding the in-repo docs project.
---

Rustipo's guides are for workflow questions, not raw feature lookup. Start here when you want a practical path through the product and move to [Reference](/reference/) when you need exact behavior.

## Start Here

### [Getting started](/guides/getting-started/)

Create a site, preview it locally, and learn the normal authoring loop.

### [Template-driven pages](/guides/template-driven-pages/)

Learn the split between Markdown content, frontmatter `extra`, and Tera layout for custom homepages and other designed pages.

## Build With Rustipo

### [Theme authoring](/guides/theme-authoring/)

Build or customize a Rustipo theme by splitting Tera layout, palettes, assets, and optional SCSS cleanly.

### [Interactive embeds](/guides/interactive-embeds/)

Use reusable iframe and demo shortcodes without dropping large HTML blocks into content pages.

### [Deploying Rustipo sites](/guides/deploying-rustipo-sites/)

Choose between GitHub Pages, Cloudflare Pages, and Netlify, then generate the right workflow.

### [Building the docs site](/guides/building-the-docs-site/)

Understand how Rustipo's own docs project is structured, verified, and published.

## Suggested Paths

### First-time site author

1. Read [Getting started](/guides/getting-started/).
2. Move to [Template-driven pages](/guides/template-driven-pages/) if your homepage or landing page needs stronger structure.
3. Read [Deploying Rustipo sites](/guides/deploying-rustipo-sites/) when you are ready to publish.
4. Use [Reference](/reference/) when you need exact command or template details.

### Theme or landing-page author

1. Start with [Theme authoring](/guides/theme-authoring/).
2. Use [Template-driven pages](/guides/template-driven-pages/) when a single page needs structured layout data.
3. Jump to [Themes and palettes](/reference/themes-and-palettes/) for the theme contract and palette model.
4. Use [Images](/reference/images/) when you need processed image derivatives in templates.

### Docs-site contributor

1. Read [Building the docs site](/guides/building-the-docs-site/).
2. Use [Reference](/reference/) for the underlying product behavior.
3. Use [Contributing](https://github.com/fcendesu/rustipo/blob/master/CONTRIBUTING.md) for GitHub-side contribution expectations.

## Guides Focus On

- installation, first build, and deployment workflows
- theme authoring with Tera, palettes, assets, and optional SCSS
- the split between content, layout, and structured page data
- reusable embed authoring patterns
- how Rustipo's own docs project is organized and published

## Good Companion Sections

- [Reference](/reference/) for exact behavior and stable template values
- [Examples](/examples/) to compare finished site shapes
