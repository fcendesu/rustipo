# Implemented Features

## CLI

- `rustipo new <site-name>`
- `rustipo check`
- `rustipo build`
- `rustipo dev`
  - `--host`
  - `--port`
- `rustipo serve`
  - `--host`
  - `--port`
  - `--watch`
- `rustipo theme list`
- `rustipo theme install <source>`
  - GitHub shorthand source
  - GitHub URL source
  - local git repository path source
  - `--name`
- `rustipo palette list`
- `rustipo palette use <id>`
- `rustipo deploy github-pages`
  - `--force`
- `rustipo deploy cloudflare-pages`
  - `--force`
- `rustipo deploy netlify`
  - `--force`

## Distribution And Release

- crates.io publication
- GitHub release generation
- GitHub release notes synchronization
- published docs site
- prebuilt release archives
  - `x86_64-unknown-linux-gnu`
  - `x86_64-apple-darwin`
  - `aarch64-apple-darwin`
  - `x86_64-pc-windows-msvc`
- release checksum file generation

## Project Structure

- `config.toml`
- `content/`
- `themes/`
- `palettes/`
- `static/`
- `dist/`
- `examples/basic-portfolio/`
- `examples/journal/`
- `examples/knowledge-base/`
- `site/`

## Configuration

- `title`
- `base_url`
- `theme`
- `palette`
- `description`
- `author`
  - `name`
  - `email`
  - `github`
  - `linkedin`
- `site`
  - `posts_per_page`
  - `favicon`
  - `layout`
    - `content_width`
    - `top_gap`
    - `vertical_align`
  - `typography`
    - `line_height`
    - `body_font`
    - `heading_font`
    - `mono_font`
    - `font_faces`
      - `family`
      - `source`
      - `weight`
      - `style`
      - `display`

## Content Sources

- homepage
- top-level standalone pages
- blog posts
- project pages
- nested custom pages outside `blog/` and `projects/`
- nested `index.md` pages outside `blog/` and `projects/`

## Frontmatter

- `title`
- `date`
- `summary`
- `tags`
- `draft`
- `slug`
- `order`
- `links`

## Content Pipeline

- YAML frontmatter parsing
- Markdown to HTML
- strict `YYYY-MM-DD` date parsing
- draft exclusion
- future-dated content exclusion in production output
- preview-mode inclusion for drafts and future-dated content
- slug normalization
- homepage route generation
- standalone page route generation
- blog post route generation
- project route generation
- nested custom page route generation
- nested directory-index route generation
- internal Markdown link validation
- deep-link fragment validation for content pages when possible
- syntax highlighting for fenced code blocks
- heading anchor ID generation
- page table of contents extraction
- inline math parsing
- block math parsing
- page-scoped KaTeX runtime injection
- GitHub-style alert blockquote parsing
- supported admonition classes
  - `markdown-alert-note`
  - `markdown-alert-tip`
  - `markdown-alert-important`
  - `markdown-alert-warning`
  - `markdown-alert-caution`
- standalone Markdown image figure rendering
- standalone image caption extraction from Markdown image titles
- standalone image size and alignment directives
  - `wide`
  - `full`
  - `left`
  - `center`
  - `right`
- Mermaid fenced code blocks
- page-scoped Mermaid runtime injection
- shortcodes
  - `youtube`
  - `link`
  - `iframe`
  - `demo`
- page-scoped shortcode stylesheet injection
- page-scoped shortcode script injection

## Routing and Generated Page Types

- `/`
- `/<page>/`
- `/blog/<post>/`
- `/projects/<project>/`
- `/<nested>/<page>/`
- `/<nested>/`
- `/blog/`
- `/projects/`
- `/blog/archive/`
- `/tags/<tag>/`

## Theme System

- filesystem themes
- `theme.toml`
- theme metadata
  - `id`
  - `name`
  - `version`
  - `author`
  - `description`
  - `extends`
- explicit theme IDs
- theme selection by explicit ID
- theme selection by directory name
- theme inheritance
- inherited template resolution
- inherited static asset resolution
- child template override precedence
- child static asset override precedence
- built-in theme registry
  - `atlas`
  - `journal`
- required templates
  - `base.html`
  - `index.html`
  - `page.html`
  - `post.html`
  - `project.html`
  - `section.html`
- theme listing
- theme installation
  - GitHub shorthand clone
  - GitHub URL clone
  - local git clone
  - install directory override
  - installed theme validation
  - `.git` removal after install

## Tera Templates

- `base.html`
- `index.html`
- `page.html`
- `post.html`
- `project.html`
- `section.html`
- partials
- macros
- template inheritance
- template includes
- content page rendering
- section rendering
- archive rendering
- tag rendering

## Template Context

- `site_title`
- `site_description`
- `site_favicon`
- `site_favicon_svg`
- `site_favicon_ico`
- `site_apple_touch_icon`
- `site_asset_version`
- `site_style`
  - `content_width`
  - `top_gap`
  - `vertical_align`
  - `line_height`
  - `body_font`
  - `heading_font`
  - `mono_font`
- `site_palette`
- `site_has_custom_css`
- `site_font_faces_css`
- `content_html`
- `frontmatter`
- `page_title`
- `page_date`
- `page_summary`
- `page_description`
- `page_tags`
- `page_has_math`
- `page_toc`
- `page_kind`
- `current_section`
- `site_nav`
- `site_menus`
- `breadcrumbs`
- `previous_post`
- `next_post`
- `page_has_mermaid`

## Built-in Metadata Output

- default `<meta name="description">` support in built-in themes
- resolution order:
  - `page_summary`
  - `site_description`
- `route`
- `section_name`
- `section_title`
- `items`
- `archive_groups`
- `current_page`
- `total_pages`
- `prev_url`
- `next_url`

## Tera Helpers

- `slugify`
- `format_date`
- `abs_url`
- `asset_url`
- `tag_url`

## Navigation and Page State

- automatic `site_nav`
  - `Home`
  - standalone pages
  - `Blog`
  - `Projects`
- configured named menus from `config.toml`
  - `site_menus`
  - `menus.main` override for `site_nav`
- active navigation state
- standalone page ordering by frontmatter `order`
- route-derived breadcrumbs
  - exact-page titles when available
  - fallback segment titles
  - `linkable` flag for non-page intermediate segments
- `previous_post`
- `next_post`
- built-in blog listing pagination
  - `/blog/`
  - `/blog/page/<n>/`
  - `current_page`
  - `total_pages`
  - `prev_url`
  - `next_url`

## Palette System

- built-in palettes
  - `default`
  - `dracula`
  - `catppuccin-frappe`
  - `catppuccin-latte`
  - `catppuccin-macchiato`
  - `catppuccin-mocha`
  - `gruvbox-dark`
  - `tokyonight-storm`
  - `tokyonight-moon`
- local palettes from `palettes/*.toml`
- palette metadata
  - `id`
  - `name`
  - `description`
  - `color_scheme`
- semantic palette fields
  - `bg`
  - `text`
  - `surface_muted`
  - `border`
  - `blockquote_border`
  - `link`
  - `link_hover`
  - `code_bg`
  - `code_text`
  - `table_header_bg`
- extra palette tokens
- generated `dist/palette.css`
- stable semantic CSS variables
  - `--rustipo-bg`
  - `--rustipo-text`
  - `--rustipo-surface-muted`
  - `--rustipo-border`
  - `--rustipo-blockquote-border`
  - `--rustipo-link`
  - `--rustipo-link-hover`
  - `--rustipo-code-bg`
  - `--rustipo-code-text`
  - `--rustipo-table-header-bg`
- raw palette token CSS variables
  - `--rustipo-token-*`
- derived palette token aliases
  - `--rustipo-base`
  - `--rustipo-mantle`
  - `--rustipo-crust`
  - `--rustipo-surface-0`
  - `--rustipo-surface-1`
  - `--rustipo-surface-2`
  - `--rustipo-overlay-0`
  - `--rustipo-overlay-1`
  - `--rustipo-overlay-2`
  - `--rustipo-subtext-0`
  - `--rustipo-subtext-1`
  - `--rustipo-accent`
  - `--rustipo-accent-strong`
  - `--rustipo-success`
  - `--rustipo-warning`
  - `--rustipo-danger`

## Typography and Fonts

- `body_font`
- `heading_font`
- `mono_font`
- local font faces from `static/`
- local font faces from inherited theme `static/`
- remote font-face sources
- data URL font-face sources
- font format inference
  - `woff2`
  - `woff`
  - `ttf`
  - `otf`
- generated `@font-face` CSS
- default typography scale
- default prose rhythm

## Favicon and Style Features

- default favicon links
- configured favicon path
- `favicon.ico`
- `favicon.svg`
- `apple-touch-icon.png`
- optional `static/custom.css`

## Output Generation

- pretty URL HTML output
- generated `dist/palette.css`
- generated `dist/rss.xml`
- generated `dist/sitemap.xml`
- generated `dist/search-index.json`
- generated `dist/robots.txt`
- generated `dist/404.html`
- `/blog/` section index
- `/projects/` section index
- `/blog/archive/` page
- `/tags/<tag>/` pages

## RSS

- dated blog post feed items
- feed item title
- feed item link
- feed item description
- feed item publication date

## Sitemap

- XML sitemap generation
- absolute URL generation from `base_url`
- duplicate route deduplication

## Robots

- default `robots.txt` generation
- `Allow: /`
- sitemap location hint

## Not Found

- built-in `404.html` generation
- optional `templates/404.html` override
- fallback to `templates/page.html`

## Search

- JSON search index generation
- search document fields
  - `route`
  - `title`
  - `summary`
  - `tags`
  - `section`
  - `content`

## Static Assets

- theme static asset copy
- user static asset copy
- inherited theme static asset merge
- asset version fingerprint for theme and site asset URLs

## Local Development and Preview

- local file server
- custom host binding
- custom port binding
- watch mode rebuilds
- live reload endpoint
- live reload HTML injection
- no-op rebuild skipping on unchanged file content
- `rustipo dev` build + serve + watch flow

## Deployment

- GitHub Pages workflow generation
- workflow overwrite protection
- docs site GitHub Pages publish workflow

## Validation and Failure Handling

- invalid YAML frontmatter errors
- invalid date errors
- empty slug errors
- unsupported nested blog content errors
- unsupported nested project content errors
- missing favicon errors
- missing font asset errors
- theme inheritance cycle detection
- missing required template errors
- duplicate theme ID detection
- ambiguous theme reference detection
- duplicate rendered route detection
- static asset collision detection
- generated `palette.css` collision detection
- invalid palette ID detection
- invalid palette token name detection
- missing `dist/` error on serve
