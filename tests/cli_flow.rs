use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn run_cli(cwd: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rustipo"))
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("rustipo command should run")
}

fn run_git(cwd: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("git command should run");
    assert!(
        output.status.success(),
        "git command failed: {}\n{}",
        String::from_utf8_lossy(&output.stderr),
        args.join(" ")
    );
}

#[test]
fn new_and_build_generate_expected_output() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let new_output = run_cli(root, &["new", "my-portfolio"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-portfolio");
    let build_output = run_cli(&project, &["build"]);
    assert!(
        build_output.status.success(),
        "build failed: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    assert!(project.join("dist/index.html").is_file());
    assert!(project.join("dist/about/index.html").is_file());
    assert!(project.join("dist/resume/index.html").is_file());
    assert!(project.join("dist/blog/index.html").is_file());
    assert!(project.join("dist/projects/index.html").is_file());
    assert!(project.join("dist/style.css").is_file());
    assert!(project.join("dist/palette.css").is_file());
    assert!(project.join("dist/favicon.svg").is_file());
    assert!(project.join("dist/search-index.json").is_file());
    assert!(
        project
            .join("themes/default/templates/partials/head_assets.html")
            .is_file()
    );
    assert!(
        project
            .join("themes/default/templates/macros/layout.html")
            .is_file()
    );

    let index_html =
        fs::read_to_string(project.join("dist/index.html")).expect("index html should be readable");
    assert!(index_html.contains("rel=\"icon\""));
    assert!(index_html.contains("favicon.svg"));

    let style_css =
        fs::read_to_string(project.join("dist/style.css")).expect("style css should be readable");
    assert!(style_css.contains("main blockquote"));
    assert!(style_css.contains("main pre"));
    assert!(style_css.contains("main table"));
    assert!(style_css.contains("main h4"));
    assert!(style_css.contains("font-size: clamp(2.4rem, 5vw, 3.25rem);"));
    assert!(style_css.contains("max-width: 68ch;"));

    let base_template = fs::read_to_string(project.join("themes/default/templates/base.html"))
        .expect("base template should be readable");
    assert!(base_template.contains("{% include \"partials/head_assets.html\" %}"));

    let page_template = fs::read_to_string(project.join("themes/default/templates/page.html"))
        .expect("page template should be readable");
    assert!(page_template.contains("{% import \"macros/layout.html\" as layout %}"));
    assert!(page_template.contains("layout::page_shell(content_html=content_html)"));
}

#[test]
fn new_scaffold_includes_builtin_palettes() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let new_output = run_cli(root, &["new", "my-portfolio"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-portfolio");
    let list_output = run_cli(&project, &["palette", "list"]);
    assert!(
        list_output.status.success(),
        "palette list failed: {}",
        String::from_utf8_lossy(&list_output.stderr)
    );

    let stdout = String::from_utf8_lossy(&list_output.stdout);
    assert!(stdout.contains("dracula -> Dracula"));
    assert!(stdout.contains("default -> Default"));
    assert!(stdout.contains("catppuccin-frappe -> Catppuccin Frappe"));
    assert!(stdout.contains("catppuccin-latte -> Catppuccin Latte"));
    assert!(stdout.contains("catppuccin-macchiato -> Catppuccin Macchiato"));
    assert!(stdout.contains("catppuccin-mocha -> Catppuccin Mocha"));
    assert!(stdout.contains("gruvbox-dark -> Gruvbox Dark"));
    assert!(stdout.contains("tokyonight-storm -> Tokyo Night Storm"));
    assert!(stdout.contains("tokyonight-moon -> Tokyo Night Moon"));
}

#[test]
fn build_supports_builtin_palette_variants() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let new_output = run_cli(root, &["new", "my-portfolio"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-portfolio");
    fs::write(
        project.join("config.toml"),
        "title = \"My Portfolio\"\nbase_url = \"https://example.com\"\ntheme = \"default\"\npalette = \"catppuccin-mocha\"\ndescription = \"My personal portfolio site\"\n",
    )
    .expect("config should be updated");

    let build_output = run_cli(&project, &["build"]);
    assert!(
        build_output.status.success(),
        "build failed: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    let palette_css = fs::read_to_string(project.join("dist/palette.css"))
        .expect("palette css should be readable");
    assert!(palette_css.contains("--rustipo-bg: #1e1e2e;"));
    assert!(palette_css.contains("--rustipo-link: #89b4fa;"));
    assert!(palette_css.contains("--rustipo-token-rosewater: #f5e0dc;"));
    assert!(palette_css.contains("--rustipo-token-surface0: #313244;"));
    assert!(palette_css.contains("--rustipo-accent: #89b4fa;"));
    assert!(palette_css.contains("--rustipo-surface-0: #313244;"));
}

#[test]
fn palette_use_updates_config_toml() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let new_output = run_cli(root, &["new", "my-portfolio"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-portfolio");
    let output = run_cli(&project, &["palette", "use", "catppuccin-macchiato"]);
    assert!(
        output.status.success(),
        "palette use failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let config = fs::read_to_string(project.join("config.toml")).expect("config should exist");
    assert!(config.contains("palette = \"catppuccin-macchiato\""));

    let build_output = run_cli(&project, &["build"]);
    assert!(
        build_output.status.success(),
        "build failed: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    let palette_css = fs::read_to_string(project.join("dist/palette.css"))
        .expect("palette css should be readable");
    assert!(palette_css.contains("--rustipo-bg: #24273a;"));
    assert!(palette_css.contains("--rustipo-token-lavender: #b7bdf8;"));
}

#[test]
fn build_supports_custom_font_config_and_assets() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let new_output = run_cli(root, &["new", "my-portfolio"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-portfolio");
    fs::create_dir_all(project.join("static/fonts")).expect("font dir should be created");
    fs::write(project.join("static/fonts/inter.woff2"), "font-bytes")
        .expect("font should be written");

    fs::write(
        project.join("config.toml"),
        r#"title = "My Portfolio"
base_url = "https://example.com"
theme = "default"
palette = "default"
description = "My personal portfolio site"

[site]
favicon = "/favicon.svg"

[site.layout]
content_width = "98%"
top_gap = "2rem"
vertical_align = "center"

[site.typography]
line_height = "1.5"
body_font = "\"Inter\", sans-serif"
heading_font = "\"Fraunces\", serif"
mono_font = "\"JetBrains Mono\", monospace"

[[site.typography.font_faces]]
family = "Inter"
source = "/fonts/inter.woff2"
weight = "400"
style = "normal"
"#,
    )
    .expect("config should be updated");

    let build_output = run_cli(&project, &["build"]);
    assert!(
        build_output.status.success(),
        "build failed: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    assert!(project.join("dist/fonts/inter.woff2").is_file());

    let index_html =
        fs::read_to_string(project.join("dist/index.html")).expect("index html should be readable");
    assert!(index_html.contains("@font-face"));
    assert!(index_html.contains("font-family: \"Inter\";"));
    assert!(index_html.contains("--rustipo-font-body: &quot;Inter&quot;, sans-serif;"));
    assert!(index_html.contains("--rustipo-font-heading: &quot;Fraunces&quot;, serif;"));
    assert!(index_html.contains("--rustipo-font-mono: &quot;JetBrains Mono&quot;, monospace;"));
}

#[test]
fn serve_fails_when_dist_is_missing() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let output = run_cli(root, &["serve"]);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("build output directory not found"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn theme_list_reads_local_theme_metadata() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let theme_dir = root.join("themes/default");
    fs::create_dir_all(&theme_dir).expect("theme dir should be created");
    fs::write(
        theme_dir.join("theme.toml"),
        "name = \"default\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Default theme\"\n",
    )
    .expect("theme metadata should be written");

    let output = run_cli(root, &["theme", "list"]);
    assert!(
        output.status.success(),
        "theme list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("default (0.1.0)"),
        "unexpected stdout: {stdout}"
    );
}

#[test]
fn theme_install_clones_from_local_git_repo() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let remote_repo = root.join("remote-theme");
    fs::create_dir_all(remote_repo.join("templates")).expect("templates should be created");
    fs::create_dir_all(remote_repo.join("static")).expect("static should be created");
    fs::write(remote_repo.join("static/style.css"), "body {}").expect("static asset should exist");
    fs::write(
        remote_repo.join("theme.toml"),
        "name = \"test-theme\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Test theme\"\n",
    )
    .expect("theme metadata should be written");

    for template in [
        "base.html",
        "page.html",
        "post.html",
        "project.html",
        "section.html",
        "index.html",
    ] {
        fs::write(
            remote_repo.join("templates").join(template),
            "{{ content_html }}",
        )
        .expect("template should be written");
    }

    run_git(root, &["init", "--initial-branch=main", "remote-theme"]);
    run_git(&remote_repo, &["config", "user.email", "test@example.com"]);
    run_git(&remote_repo, &["config", "user.name", "Test User"]);
    run_git(&remote_repo, &["add", "."]);
    run_git(&remote_repo, &["commit", "-m", "init theme"]);

    let output = run_cli(
        root,
        &[
            "theme",
            "install",
            remote_repo.to_string_lossy().as_ref(),
            "--name",
            "installed-theme",
        ],
    );
    assert!(
        output.status.success(),
        "theme install failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(root.join("themes/installed-theme/theme.toml").is_file());
    assert!(
        root.join("themes/installed-theme/templates/section.html")
            .is_file()
    );
    assert!(!root.join("themes/installed-theme/.git").exists());
}

#[test]
fn build_supports_inherited_theme_overrides() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    fs::create_dir_all(root.join("content")).expect("content dir should be created");
    fs::create_dir_all(root.join("static")).expect("static dir should be created");
    fs::write(root.join("content/index.md"), "# Home").expect("index should be written");
    fs::write(
        root.join("config.toml"),
        "title = \"Rustipo\"\nbase_url = \"https://example.com\"\ntheme = \"child\"\ndescription = \"Test\"\n",
    )
    .expect("config should be written");

    let base = root.join("themes/base");
    fs::create_dir_all(base.join("templates")).expect("base templates should be created");
    fs::create_dir_all(base.join("static")).expect("base static should be created");
    fs::write(
        base.join("theme.toml"),
        "name = \"base\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Base\"\n",
    )
    .expect("base metadata should be written");
    for template in [
        "base.html",
        "page.html",
        "post.html",
        "project.html",
        "section.html",
        "index.html",
    ] {
        fs::write(
            base.join("templates").join(template),
            "{% extends \"base.html\" %}{% block body %}<main>base</main>{{ content_html | safe }}{% endblock body %}",
        )
        .expect("base template should be written");
    }
    fs::write(
        base.join("templates/base.html"),
        "{% block body %}{% endblock body %}",
    )
    .expect("base layout should be written");
    fs::write(base.join("static/style.css"), "base-style").expect("base style should be written");

    let child = root.join("themes/child");
    fs::create_dir_all(child.join("templates")).expect("child templates should be created");
    fs::create_dir_all(child.join("static")).expect("child static should be created");
    fs::write(child.join("theme.toml"), "name = \"child\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Child\"\nextends = \"base\"\n").expect("child metadata should be written");
    fs::write(
        child.join("templates/index.html"),
        "{% extends \"base.html\" %}{% block body %}<main>child-index</main>{{ content_html | safe }}{% endblock body %}",
    )
    .expect("child index template should be written");
    fs::write(child.join("static/style.css"), "child-style")
        .expect("child style should be written");

    let output = run_cli(root, &["build"]);
    assert!(
        output.status.success(),
        "build failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let index_html =
        fs::read_to_string(root.join("dist/index.html")).expect("index output should exist");
    assert!(index_html.contains("child-index"));

    let style = fs::read_to_string(root.join("dist/style.css")).expect("style should exist");
    assert_eq!(style, "child-style");
}

#[test]
fn build_fails_when_configured_favicon_is_missing() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    fs::create_dir_all(root.join("content")).expect("content dir should be created");
    fs::create_dir_all(root.join("themes/default/templates"))
        .expect("theme templates should be created");
    fs::create_dir_all(root.join("themes/default/static")).expect("theme static should be created");
    fs::create_dir_all(root.join("static")).expect("static dir should be created");

    fs::write(root.join("content/index.md"), "# Home").expect("index should be written");
    fs::write(
        root.join("config.toml"),
        "title = \"Rustipo\"\nbase_url = \"https://example.com\"\ntheme = \"default\"\ndescription = \"Test\"\n\n[site]\nfavicon = \"/favicon.ico\"\n",
    )
    .expect("config should be written");
    fs::write(
        root.join("themes/default/theme.toml"),
        "name = \"default\"\nversion = \"0.1.0\"\nauthor = \"Rustipo\"\ndescription = \"Default\"\n",
    )
    .expect("theme metadata should be written");
    for template in [
        "base.html",
        "page.html",
        "post.html",
        "project.html",
        "section.html",
        "index.html",
    ] {
        fs::write(
            root.join("themes/default/templates").join(template),
            "{% extends \"base.html\" %}{% block body %}{{ content_html | safe }}{% endblock body %}",
        )
        .expect("template should be written");
    }
    fs::write(
        root.join("themes/default/templates/base.html"),
        "{% block body %}{% endblock body %}",
    )
    .expect("base template should be written");

    let output = run_cli(root, &["build"]);
    assert!(!output.status.success(), "build should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("configured favicon file not found"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn deploy_github_pages_generates_workflow_file() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let output = run_cli(root, &["deploy", "github-pages"]);
    assert!(
        output.status.success(),
        "deploy helper failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let workflow = root.join(".github/workflows/deploy-pages.yml");
    assert!(workflow.is_file());
    let content = fs::read_to_string(workflow).expect("workflow should be readable");
    assert!(content.contains("name: Deploy GitHub Pages"));
    assert!(content.contains("actions/deploy-pages@v4"));
}

#[test]
fn deploy_github_pages_refuses_overwrite_without_force() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    fs::create_dir_all(root.join(".github/workflows")).expect("workflow dir should be created");
    fs::write(
        root.join(".github/workflows/deploy-pages.yml"),
        "name: existing",
    )
    .expect("existing workflow should be written");

    let output = run_cli(root, &["deploy", "github-pages"]);
    assert!(
        !output.status.success(),
        "deploy helper should fail without --force"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("workflow already exists"));

    let force_output = run_cli(root, &["deploy", "github-pages", "--force"]);
    assert!(
        force_output.status.success(),
        "deploy helper should overwrite with --force: {}",
        String::from_utf8_lossy(&force_output.stderr)
    );
}
