pub const PAGE_TEMPLATE_NAMES: &[&str] = &["index.html", "page.html", "post.html", "project.html"];

pub const INDEX_CONTENT: &str = r#"---
title: Home
---

# Welcome to Rustipo

Start with a homepage, notes, posts, or project pages.
"#;

pub const ABOUT_CONTENT: &str = r#"---
title: About
---

# About

Use this page to introduce your site, writing, or work.
"#;

pub const RESUME_CONTENT: &str = r#"---
title: Resume
---

# Resume

Use this page for a resume, links, notes, or anything else.
"#;

pub const CONFIG_TOML: &str = r#"title = "My Site"
base_url = "https://example.com"
theme = "default"
palette = "default"
description = "My Rustipo site"

[site]
favicon = "/favicon.svg"

# Basic design controls (applied by theme CSS variables).
# You can tune layout/typography here without editing CSS files.
# Built-in palettes:
# - dracula
# - default
# - catppuccin-frappe
# - catppuccin-latte
# - catppuccin-macchiato
# - catppuccin-mocha
# - gruvbox-dark
# - tokyonight-storm
# - tokyonight-moon
[site.layout]
content_width = "98%"
top_gap = "2rem"
vertical_align = "center"

[site.typography]
line_height = "1.5"
# body_font = "\"Inter\", sans-serif"
# heading_font = "\"Fraunces\", serif"
# mono_font = "\"JetBrains Mono\", monospace"
#
# [[site.typography.font_faces]]
# family = "Inter"
# source = "/fonts/inter.woff2"
# weight = "400"
# style = "normal"
"#;

pub const FAVICON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect width="64" height="64" rx="12" fill="#111827"/>
  <text x="50%" y="54%" text-anchor="middle" font-size="30" font-family="Arial, sans-serif" fill="#ffffff">R</text>
</svg>
"##;

pub const BASE_TEMPLATE: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{ page_title }}</title>
    {% include "partials/head_assets.html" %}
  </head>
  <body>
    {% block body %}{% endblock body %}
  </body>
</html>
"#;

pub const HEAD_ASSETS_PARTIAL: &str = r#"{% if site_favicon_svg %}<link rel="icon" href="{{ site_favicon_svg }}" type="image/svg+xml" />{% endif %}
{% if site_favicon_ico %}<link rel="icon" href="{{ site_favicon_ico }}" sizes="any" />{% endif %}
{% if site_apple_touch_icon %}<link rel="apple-touch-icon" href="{{ site_apple_touch_icon }}" />{% endif %}
{% if site_favicon and not site_favicon_svg and not site_favicon_ico %}<link rel="icon" href="{{ site_favicon }}" />{% endif %}
<style>
  :root {
    --rustipo-content-width: {{ site_style.content_width | default(value="98%") }};
    --rustipo-top-gap: {{ site_style.top_gap | default(value="2rem") }};
    --rustipo-vertical-align: {{ site_style.vertical_align | default(value="center") }};
    --rustipo-line-height: {{ site_style.line_height | default(value="1.5") }};
    --rustipo-font-body: {{ site_style.body_font }};
    --rustipo-font-heading: {{ site_style.heading_font }};
    --rustipo-font-mono: {{ site_style.mono_font }};
  }
</style>
{% if site_font_faces_css %}
<style>
  {{ site_font_faces_css | safe }}
</style>
{% endif %}
<link rel="stylesheet" href="{{ asset_url(path='style.css') }}" />
<link rel="stylesheet" href="{{ asset_url(path='palette.css') }}" />
{% if site_has_custom_css %}
<link rel="stylesheet" href="{{ asset_url(path='custom.css') }}" />
{% endif %}
"#;

pub const LAYOUT_MACROS: &str = r#"{% macro page_shell(content_html) %}
<main>
  {{ content_html | safe }}
</main>
{% endmacro page_shell %}

{% macro section_list(title, items) %}
<main>
  <h1>{{ title }}</h1>
  <ul>
    {% for item in items %}
    <li><a href="{{ item.route }}">{{ item.title }}</a></li>
    {% endfor %}
  </ul>
</main>
{% endmacro section_list %}
"#;

pub const CONTENT_TEMPLATE: &str = r#"{% extends "base.html" %}
{% import "macros/layout.html" as layout %}
{% block body %}
{{ layout::page_shell(content_html=content_html) }}
{% endblock body %}
"#;

pub const SECTION_TEMPLATE: &str = r#"{% extends "base.html" %}
{% import "macros/layout.html" as layout %}
{% block body %}
{{ layout::section_list(title=section_title, items=items) }}
{% endblock body %}
"#;

pub const DEFAULT_THEME_TOML: &str = r#"id = "default"
name = "default"
version = "0.1.0"
author = "Rustipo"
description = "Default Rustipo theme"
"#;

pub const THEME_STYLE_CSS: &str = r#"body {
  font-family: var(--rustipo-font-body, sans-serif);
  margin: 0;
  min-height: 100vh;
  padding: var(--rustipo-top-gap) 0 2rem;
  font-size: 17px;
  line-height: var(--rustipo-line-height);
  display: grid;
  place-items: var(--rustipo-vertical-align) center;
  background: var(--rustipo-base, var(--rustipo-bg));
  color: var(--rustipo-text);
}

main {
  width: fit-content;
  max-width: var(--rustipo-content-width);
  margin: 0 auto;
  padding-inline: 1rem;
  box-sizing: border-box;
}

main > * {
  max-width: 68ch;
}

main > :first-child {
  margin-top: 0;
}

main > :last-child {
  margin-bottom: 0;
}

main h1,
main h2,
main h3,
main h4,
main h5,
main h6,
main p,
main li {
  overflow-wrap: anywhere;
  word-break: break-word;
}

main h1,
main h2,
main h3,
main h4,
main h5,
main h6 {
  font-family: var(--rustipo-font-heading, var(--rustipo-font-body, sans-serif));
  font-weight: 700;
  letter-spacing: -0.02em;
  text-wrap: balance;
}

main h1 {
  margin: 0 0 1.15rem;
  font-size: clamp(2.4rem, 5vw, 3.25rem);
  line-height: 1.05;
  font-weight: 800;
}

main h2 {
  margin: 2.4rem 0 0.95rem;
  font-size: clamp(1.85rem, 3.5vw, 2.35rem);
  line-height: 1.1;
  font-weight: 775;
  color: var(--rustipo-subtext-1, var(--rustipo-text));
}

main h3 {
  margin: 1.85rem 0 0.75rem;
  font-size: clamp(1.45rem, 2.2vw, 1.7rem);
  line-height: 1.18;
  color: var(--rustipo-subtext-1, var(--rustipo-text));
}

main h4 {
  margin: 1.55rem 0 0.65rem;
  font-size: 1.2rem;
  line-height: 1.25;
}

main h5 {
  margin: 1.35rem 0 0.55rem;
  font-size: 1.05rem;
  line-height: 1.3;
}

main h6 {
  margin: 1.2rem 0 0.5rem;
  font-size: 0.95rem;
  line-height: 1.35;
  color: var(--rustipo-subtext-0, var(--rustipo-text));
}

main p {
  margin: 0 0 1.15rem;
  text-wrap: pretty;
}

main ul,
main ol {
  margin: 0 0 1.15rem 1.35rem;
  padding: 0;
}

main li + li {
  margin-top: 0.45rem;
}

main blockquote {
  margin: 1.5rem 0;
  padding: 0.5rem 1rem;
  border-left: 4px solid var(--rustipo-accent, var(--rustipo-blockquote-border));
  background: var(--rustipo-surface-0, var(--rustipo-surface-muted));
  color: var(--rustipo-subtext-1, var(--rustipo-text));
}

main blockquote[class^="markdown-alert-"] {
  padding: 0.9rem 1rem 0.85rem;
  border-left-width: 5px;
  border-radius: 0 12px 12px 0;
}

main blockquote[class^="markdown-alert-"]::before {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.82rem;
  font-weight: 800;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

main blockquote.markdown-alert-note {
  border-left-color: var(--rustipo-accent, var(--rustipo-blockquote-border));
}

main blockquote.markdown-alert-note::before {
  content: "Note";
  color: var(--rustipo-accent, var(--rustipo-link));
}

main blockquote.markdown-alert-tip {
  border-left-color: var(--rustipo-success, var(--rustipo-accent, var(--rustipo-blockquote-border)));
}

main blockquote.markdown-alert-tip::before {
  content: "Tip";
  color: var(--rustipo-success, var(--rustipo-accent, var(--rustipo-link)));
}

main blockquote.markdown-alert-important {
  border-left-color: var(--rustipo-accent-strong, var(--rustipo-accent, var(--rustipo-blockquote-border)));
}

main blockquote.markdown-alert-important::before {
  content: "Important";
  color: var(--rustipo-accent-strong, var(--rustipo-accent, var(--rustipo-link)));
}

main blockquote.markdown-alert-warning {
  border-left-color: var(--rustipo-warning, var(--rustipo-accent, var(--rustipo-blockquote-border)));
}

main blockquote.markdown-alert-warning::before {
  content: "Warning";
  color: var(--rustipo-warning, var(--rustipo-accent, var(--rustipo-link)));
}

main blockquote.markdown-alert-caution {
  border-left-color: var(--rustipo-danger, var(--rustipo-accent, var(--rustipo-blockquote-border)));
}

main blockquote.markdown-alert-caution::before {
  content: "Caution";
  color: var(--rustipo-danger, var(--rustipo-accent, var(--rustipo-link)));
}

main img {
  max-width: 100%;
  height: auto;
}

main figure.markdown-image {
  width: 100%;
  margin: 1.6rem auto;
}

main > figure.markdown-image {
  max-width: min(100%, 68ch);
}

main figure.markdown-image.markdown-image-wide {
  max-width: min(100%, 82ch);
}

main figure.markdown-image.markdown-image-full {
  max-width: 100%;
}

main figure.markdown-image.markdown-image-left {
  margin-inline: 0 auto;
}

main figure.markdown-image.markdown-image-center {
  margin-inline: auto;
}

main figure.markdown-image.markdown-image-right {
  margin-inline: auto 0;
}

main .markdown-image-img {
  display: block;
  width: 100%;
  border-radius: 14px;
  border: 1px solid var(--rustipo-surface-1, var(--rustipo-border));
  background: var(--rustipo-surface-0, var(--rustipo-surface-muted));
}

main .markdown-image-caption {
  margin-top: 0.7rem;
  font-size: 0.94rem;
  line-height: 1.45;
  color: var(--rustipo-subtext-0, var(--rustipo-text));
  text-align: center;
}

main hr {
  border: 0;
  border-top: 1px solid var(--rustipo-surface-1, var(--rustipo-border));
  margin: 2.25rem 0;
}

main a {
  color: var(--rustipo-accent, var(--rustipo-link));
  text-decoration: underline;
  text-underline-offset: 2px;
}

main a:hover {
  color: var(--rustipo-accent-strong, var(--rustipo-link-hover));
}

main strong {
  color: var(--rustipo-accent-strong, var(--rustipo-text));
}

main :not(pre) > code {
  font-family: var(--rustipo-font-mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
    "Liberation Mono", "Courier New", monospace);
  font-size: 0.92em;
  padding: 0.16em 0.38em;
  border-radius: 6px;
  background: var(--rustipo-surface-0, var(--rustipo-code-bg));
  color: var(--rustipo-code-text);
}

main pre {
  font-family: var(--rustipo-font-mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
    "Liberation Mono", "Courier New", monospace);
  margin: 1.4rem 0;
  padding: 1rem 1.05rem;
  border: 1px solid var(--rustipo-surface-1, var(--rustipo-border));
  border-radius: 10px;
  background: var(--rustipo-mantle, var(--rustipo-surface-muted));
  overflow-x: auto;
  font-size: 0.94rem;
}

main pre code {
  padding: 0;
  border-radius: 0;
  background: transparent;
  color: inherit;
}

main table {
  display: block;
  width: 100%;
  overflow-x: auto;
  border-collapse: collapse;
  margin: 1.35rem 0;
}

main th,
main td {
  border: 1px solid var(--rustipo-surface-1, var(--rustipo-border));
  padding: 0.6rem 0.8rem;
  text-align: left;
  white-space: nowrap;
}

main th {
  background: var(--rustipo-surface-0, var(--rustipo-table-header-bg));
}
"#;
