use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

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

    write_file(
        &root.join("content/index.md"),
        r#"---
title: Home
---

# Welcome to Rustipo

This is your portfolio homepage.
"#,
    )?;
    write_file(
        &root.join("content/about.md"),
        r#"---
title: About
---

# About

Write about yourself here.
"#,
    )?;
    write_file(
        &root.join("content/resume.md"),
        r#"---
title: Resume
---

# Resume

Add your experience and skills here.
"#,
    )?;
    write_file(
        &root.join("themes/default/theme.toml"),
        r#"name = "default"
version = "0.1.0"
author = "Rustipo"
description = "Default Rustipo theme"
"#,
    )?;
    write_file(
        &root.join("config.toml"),
        r#"title = "My Portfolio"
base_url = "https://example.com"
theme = "default"
description = "My personal portfolio site"

[site]
favicon = "/favicon.svg"

# Basic design controls (applied by theme CSS variables).
# You can tune layout/typography here without editing CSS files.
[site.layout]
content_width = "98%"
top_gap = "2rem"
vertical_align = "center"

[site.typography]
line_height = "1.5"
"#,
    )?;
    write_file(
        &root.join("static/favicon.svg"),
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect width="64" height="64" rx="12" fill="#111827"/>
  <text x="50%" y="54%" text-anchor="middle" font-size="30" font-family="Arial, sans-serif" fill="#ffffff">R</text>
</svg>
"##,
    )?;
    write_file(
        &root.join("themes/default/templates/base.html"),
        r#"<!doctype html>
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
      }
    </style>
    <link rel="stylesheet" href="/style.css" />
    {% if site_has_custom_css %}
    <link rel="stylesheet" href="/custom.css" />
    {% endif %}
  </head>
  <body>
    {% block body %}{% endblock body %}
  </body>
</html>
"#,
    )?;
    write_file(
        &root.join("themes/default/templates/index.html"),
        r#"{% extends "base.html" %}
{% block body %}
<main>
  {{ content_html | safe }}
</main>
{% endblock body %}
"#,
    )?;
    write_file(
        &root.join("themes/default/templates/page.html"),
        r#"{% extends "base.html" %}
{% block body %}
<main>
  {{ content_html | safe }}
</main>
{% endblock body %}
"#,
    )?;
    write_file(
        &root.join("themes/default/templates/post.html"),
        r#"{% extends "base.html" %}
{% block body %}
<main>
  {{ content_html | safe }}
</main>
{% endblock body %}
"#,
    )?;
    write_file(
        &root.join("themes/default/templates/project.html"),
        r#"{% extends "base.html" %}
{% block body %}
<main>
  {{ content_html | safe }}
</main>
{% endblock body %}
"#,
    )?;
    write_file(
        &root.join("themes/default/templates/section.html"),
        r#"{% extends "base.html" %}
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
"#,
    )?;
    write_file(
        &root.join("themes/default/static/style.css"),
        r#"body {
  font-family: sans-serif;
  margin: 0;
  min-height: 100vh;
  padding: var(--rustipo-top-gap) 0 2rem;
  line-height: var(--rustipo-line-height);
  display: grid;
  place-items: var(--rustipo-vertical-align) center;
}

main {
  width: fit-content;
  max-width: var(--rustipo-content-width);
  margin: 0 auto;
  padding-inline: 1rem;
  box-sizing: border-box;
}

h1,
h2,
h3,
p,
li {
  overflow-wrap: anywhere;
  word-break: break-word;
}
"#,
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
