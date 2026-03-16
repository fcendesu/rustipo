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
"#,
    )?;
    write_file(
        &root.join("themes/default/templates/base.html"),
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{ page_title }}</title>
    <link rel="stylesheet" href="/style.css" />
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
  margin: 2rem auto;
  max-width: 720px;
  line-height: 1.5;
  padding: 0 1rem;
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
