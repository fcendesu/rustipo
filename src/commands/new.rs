use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

const PAGE_TEMPLATE_NAMES: &[&str] = &["index.html", "page.html", "post.html", "project.html"];

const INDEX_CONTENT: &str = r#"---
title: Home
---

# Welcome to Rustipo

This is your portfolio homepage.
"#;

const ABOUT_CONTENT: &str = r#"---
title: About
---

# About

Write about yourself here.
"#;

const RESUME_CONTENT: &str = r#"---
title: Resume
---

# Resume

Add your experience and skills here.
"#;

const CONFIG_TOML: &str = r#"title = "My Portfolio"
base_url = "https://example.com"
theme = "default"
palette = "default"
description = "My personal portfolio site"

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

const FAVICON_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect width="64" height="64" rx="12" fill="#111827"/>
  <text x="50%" y="54%" text-anchor="middle" font-size="30" font-family="Arial, sans-serif" fill="#ffffff">R</text>
</svg>
"##;

const BASE_TEMPLATE: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{ page_title }}</title>
    {% if site_favicon_svg %}<link rel="icon" href="{{ site_favicon_svg }}" type="image/svg+xml" />{% endif %}
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
    <link rel="stylesheet" href="/style.css" />
    <link rel="stylesheet" href="/palette.css" />
    {% if site_has_custom_css %}
    <link rel="stylesheet" href="/custom.css" />
    {% endif %}
  </head>
  <body>
    {% block body %}{% endblock body %}
  </body>
</html>
"#;

const CONTENT_TEMPLATE: &str = r#"{% extends "base.html" %}
{% block body %}
<main>
  {{ content_html | safe }}
</main>
{% endblock body %}
"#;

const SECTION_TEMPLATE: &str = r#"{% extends "base.html" %}
{% block body %}
<main>
  <h1>{{ section_title }}</h1>
  <ul>
    {% for item in items %}
    <li><a href="{{ item.route }}">{{ item.title }}</a></li>
    {% endfor %}
  </ul>
</main>
{% endblock body %}
"#;

const DEFAULT_THEME_TOML: &str = r#"id = "default"
name = "default"
version = "0.1.0"
author = "Rustipo"
description = "Default Rustipo theme"
"#;

const THEME_STYLE_CSS: &str = r#"body {
  font-family: var(--rustipo-font-body, sans-serif);
  margin: 0;
  min-height: 100vh;
  padding: var(--rustipo-top-gap) 0 2rem;
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
}

main h1 {
  margin: 0 0 1rem;
  line-height: 1.2;
}

main h2 {
  margin: 1.8rem 0 0.85rem;
  line-height: 1.25;
  color: var(--rustipo-subtext-1, var(--rustipo-text));
}

main h3 {
  margin: 1.45rem 0 0.7rem;
  line-height: 1.3;
  color: var(--rustipo-subtext-1, var(--rustipo-text));
}

main p {
  margin: 0 0 1rem;
}

main ul,
main ol {
  margin: 0 0 1rem 1.35rem;
  padding: 0;
}

main li + li {
  margin-top: 0.35rem;
}

main blockquote {
  margin: 1.2rem 0;
  padding: 0.2rem 1rem;
  border-left: 4px solid var(--rustipo-accent, var(--rustipo-blockquote-border));
  background: var(--rustipo-surface-0, var(--rustipo-surface-muted));
}

main hr {
  border: 0;
  border-top: 1px solid var(--rustipo-surface-1, var(--rustipo-border));
  margin: 1.8rem 0;
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
  font-size: 0.95em;
  padding: 0.14em 0.32em;
  border-radius: 6px;
  background: var(--rustipo-surface-0, var(--rustipo-code-bg));
  color: var(--rustipo-code-text);
}

main pre {
  font-family: var(--rustipo-font-mono, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
    "Liberation Mono", "Courier New", monospace);
  margin: 1.1rem 0;
  padding: 0.95rem;
  border: 1px solid var(--rustipo-surface-1, var(--rustipo-border));
  border-radius: 10px;
  background: var(--rustipo-mantle, var(--rustipo-surface-muted));
  overflow-x: auto;
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
  margin: 1rem 0;
}

main th,
main td {
  border: 1px solid var(--rustipo-surface-1, var(--rustipo-border));
  padding: 0.5rem 0.7rem;
  text-align: left;
  white-space: nowrap;
}

main th {
  background: var(--rustipo-surface-0, var(--rustipo-table-header-bg));
}
"#;

pub fn run(site_name: &str) -> Result<()> {
    if site_name.trim().is_empty() {
        bail!("site name cannot be empty");
    }

    let root = Path::new(site_name);
    if root.exists() {
        bail!("target directory already exists: {}", root.display());
    }

    create_dir(root)?;
    create_dir(&root.join("content"))?;
    create_dir(&root.join("content/blog"))?;
    create_dir(&root.join("content/projects"))?;
    create_dir(&root.join("static"))?;
    create_dir(&root.join("themes/default/templates"))?;
    create_dir(&root.join("themes/default/static"))?;

    write_file(&root.join("content/index.md"), INDEX_CONTENT)?;
    write_file(&root.join("content/about.md"), ABOUT_CONTENT)?;
    write_file(&root.join("content/resume.md"), RESUME_CONTENT)?;
    write_file(&root.join("config.toml"), CONFIG_TOML)?;
    write_file(&root.join("static/favicon.svg"), FAVICON_SVG)?;
    write_file(&root.join("themes/default/theme.toml"), DEFAULT_THEME_TOML)?;
    write_file(
        &root.join("themes/default/templates/base.html"),
        BASE_TEMPLATE,
    )?;
    for template_name in PAGE_TEMPLATE_NAMES {
        write_file(
            &root.join("themes/default/templates").join(template_name),
            CONTENT_TEMPLATE,
        )?;
    }
    write_file(
        &root.join("themes/default/templates/section.html"),
        SECTION_TEMPLATE,
    )?;
    write_file(
        &root.join("themes/default/static/style.css"),
        THEME_STYLE_CSS,
    )?;

    println!("Created new Rustipo site: {}", root.display());
    Ok(())
}

fn create_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path)
        .with_context(|| format!("failed to create directory: {}", path.display()))
}

fn write_file(path: &Path, contents: &str) -> Result<()> {
    fs::write(path, contents).with_context(|| format!("failed to write file: {}", path.display()))
}
