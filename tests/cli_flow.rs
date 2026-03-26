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

fn copy_dir_recursive_filtered(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("destination dir should be created");

    for entry in fs::read_dir(source).expect("source dir should be readable") {
        let entry = entry.expect("directory entry should be readable");
        let path = entry.path();
        let file_name = entry.file_name();
        let target = destination.join(&file_name);

        if path.is_dir() {
            if file_name == "dist" || file_name == ".git" {
                continue;
            }
            copy_dir_recursive_filtered(&path, &target);
        } else {
            fs::copy(&path, &target).expect("file should be copied");
        }
    }
}

#[test]
fn new_and_build_generate_expected_output() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let new_output = run_cli(root, &["new", "my-site"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-site");
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
    assert!(project.join("dist/robots.txt").is_file());
    assert!(project.join("dist/404.html").is_file());
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

    let robots_txt =
        fs::read_to_string(project.join("dist/robots.txt")).expect("robots.txt should be readable");
    assert!(robots_txt.contains("User-agent: *"));
    assert!(robots_txt.contains("Allow: /"));
    assert!(robots_txt.contains("Sitemap: https://example.com/sitemap.xml"));

    let not_found_html =
        fs::read_to_string(project.join("dist/404.html")).expect("404 html should be readable");
    assert!(not_found_html.contains("Page not found"));
    assert!(not_found_html.contains("Return home"));

    let base_template = fs::read_to_string(project.join("themes/default/templates/base.html"))
        .expect("base template should be readable");
    assert!(base_template.contains("{% include \"partials/head_assets.html\" %}"));

    let page_template = fs::read_to_string(project.join("themes/default/templates/page.html"))
        .expect("page template should be readable");
    assert!(page_template.contains("{% import \"macros/layout.html\" as layout %}"));
    assert!(page_template.contains("layout::page_shell(content_html=content_html)"));
}

#[test]
fn check_succeeds_for_new_scaffold() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let new_output = run_cli(root, &["new", "my-site"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-site");
    let check_output = run_cli(&project, &["check"]);
    assert!(
        check_output.status.success(),
        "check failed: {}",
        String::from_utf8_lossy(&check_output.stderr)
    );

    let stdout = String::from_utf8_lossy(&check_output.stdout);
    assert!(stdout.contains("Validated rendered routes:"));
    assert!(stdout.contains("Validated asset paths:"));
    assert!(stdout.contains("Check completed: project inputs are valid."));
    assert!(
        !project.join("dist").exists(),
        "check should not write dist output"
    );
}

#[test]
fn check_fails_for_missing_configured_favicon() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let new_output = run_cli(root, &["new", "my-site"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-site");
    fs::write(
        project.join("config.toml"),
        "title = \"My Site\"\nbase_url = \"https://example.com\"\ntheme = \"default\"\npalette = \"default\"\ndescription = \"My Rustipo site\"\n\n[site]\nfavicon = \"/missing.svg\"\n",
    )
    .expect("config should be updated");

    let output = run_cli(&project, &["check"]);
    assert!(!output.status.success(), "check should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("configured favicon file not found"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn check_fails_for_broken_internal_deep_link() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let new_output = run_cli(root, &["new", "my-site"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-site");
    fs::write(
        project.join("content/index.md"),
        "# Home\n\n[Missing section](/about/#missing-heading)\n",
    )
    .expect("index should be updated");
    fs::write(
        project.join("content/about.md"),
        "# About\n\n## Real heading\n",
    )
    .expect("about should be updated");

    let output = run_cli(&project, &["check"]);
    assert!(!output.status.success(), "check should fail");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid deep link"),
        "unexpected stderr: {stderr}"
    );
    assert!(
        stderr.contains("missing-heading"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn build_excludes_future_dated_content_from_production_output() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let new_output = run_cli(root, &["new", "my-site"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-site");
    fs::write(
        project.join("content/planned.md"),
        "---\ndate: 2099-01-01\n---\n\n# Planned",
    )
    .expect("planned page should be written");

    let output = run_cli(&project, &["build"]);
    assert!(
        output.status.success(),
        "build failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        !project.join("dist/planned/index.html").exists(),
        "future-dated content should be excluded from production build output"
    );
}

#[test]
fn new_scaffold_includes_builtin_palettes() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let new_output = run_cli(root, &["new", "my-site"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-site");
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

    let new_output = run_cli(root, &["new", "my-site"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-site");
    fs::write(
        project.join("config.toml"),
        "title = \"My Site\"\nbase_url = \"https://example.com\"\ntheme = \"default\"\npalette = \"catppuccin-mocha\"\ndescription = \"My Rustipo site\"\n",
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

    let new_output = run_cli(root, &["new", "my-site"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-site");
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

    let new_output = run_cli(root, &["new", "my-site"]);
    assert!(
        new_output.status.success(),
        "new failed: {}",
        String::from_utf8_lossy(&new_output.stderr)
    );

    let project = root.join("my-site");
    fs::create_dir_all(project.join("static/fonts")).expect("font dir should be created");
    fs::write(project.join("static/fonts/inter.woff2"), "font-bytes")
        .expect("font should be written");

    fs::write(
        project.join("config.toml"),
        r#"title = "My Site"
base_url = "https://example.com"
theme = "default"
palette = "default"
description = "My Rustipo site"

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
    assert!(
        stdout.contains("journal -> Journal (0.1.0)"),
        "unexpected stdout: {stdout}"
    );
    assert!(
        stdout.contains("atlas -> Atlas (0.1.0)"),
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
    assert!(content.contains("cargo install rustipo --locked"));
    assert!(content.contains("run: rustipo build"));
    assert!(!content.contains("cargo run -- build"));
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

#[test]
fn deploy_cloudflare_pages_generates_workflow_file() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let output = run_cli(root, &["deploy", "cloudflare-pages"]);
    assert!(
        output.status.success(),
        "deploy helper failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let workflow = root.join(".github/workflows/deploy-cloudflare-pages.yml");
    assert!(workflow.is_file());
    let content = fs::read_to_string(workflow).expect("workflow should be readable");
    assert!(content.contains("name: Deploy Cloudflare Pages"));
    assert!(content.contains("cloudflare/wrangler-action@v3"));
    assert!(content.contains("command: pages deploy dist"));
    assert!(content.contains("CLOUDFLARE_API_TOKEN"));
    assert!(content.contains("CLOUDFLARE_ACCOUNT_ID"));
    assert!(content.contains("CLOUDFLARE_PAGES_PROJECT"));
    assert!(content.contains("cargo install rustipo --locked"));
    assert!(content.contains("run: rustipo build"));
}

#[test]
fn deploy_cloudflare_pages_refuses_overwrite_without_force() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    fs::create_dir_all(root.join(".github/workflows")).expect("workflow dir should be created");
    fs::write(
        root.join(".github/workflows/deploy-cloudflare-pages.yml"),
        "name: existing",
    )
    .expect("existing workflow should be written");

    let output = run_cli(root, &["deploy", "cloudflare-pages"]);
    assert!(
        !output.status.success(),
        "deploy helper should fail without --force"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("workflow already exists"));

    let force_output = run_cli(root, &["deploy", "cloudflare-pages", "--force"]);
    assert!(
        force_output.status.success(),
        "deploy helper should overwrite with --force: {}",
        String::from_utf8_lossy(&force_output.stderr)
    );
}

#[test]
fn deploy_netlify_generates_workflow_file() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    let output = run_cli(root, &["deploy", "netlify"]);
    assert!(
        output.status.success(),
        "deploy helper failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let workflow = root.join(".github/workflows/deploy-netlify.yml");
    assert!(workflow.is_file());
    let content = fs::read_to_string(workflow).expect("workflow should be readable");
    assert!(content.contains("name: Deploy Netlify"));
    assert!(content.contains("actions/setup-node@v6"));
    assert!(content.contains("node-version: 22"));
    assert!(content.contains("npm install -g netlify-cli"));
    assert!(content.contains("netlify deploy --dir=dist --prod"));
    assert!(content.contains("NETLIFY_AUTH_TOKEN"));
    assert!(content.contains("NETLIFY_SITE_ID"));
    assert!(content.contains("cargo install rustipo --locked"));
    assert!(content.contains("run: rustipo build"));
}

#[test]
fn deploy_netlify_refuses_overwrite_without_force() {
    let dir = tempdir().expect("tempdir should be created");
    let root = dir.path();

    fs::create_dir_all(root.join(".github/workflows")).expect("workflow dir should be created");
    fs::write(
        root.join(".github/workflows/deploy-netlify.yml"),
        "name: existing",
    )
    .expect("existing workflow should be written");

    let output = run_cli(root, &["deploy", "netlify"]);
    assert!(
        !output.status.success(),
        "deploy helper should fail without --force"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("workflow already exists"));

    let force_output = run_cli(root, &["deploy", "netlify", "--force"]);
    assert!(
        force_output.status.success(),
        "deploy helper should overwrite with --force: {}",
        String::from_utf8_lossy(&force_output.stderr)
    );
}

#[test]
fn bundled_examples_build_successfully() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let examples_root = repo_root.join("examples");
    let temp = tempdir().expect("tempdir should be created");

    for example in ["basic-portfolio", "journal", "knowledge-base"] {
        let source = examples_root.join(example);
        let project = temp.path().join(example);
        copy_dir_recursive_filtered(&source, &project);

        let output = run_cli(&project, &["build"]);
        assert!(
            output.status.success(),
            "example build failed for {example}: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        assert!(
            project.join("dist/index.html").is_file(),
            "index output missing for {example}"
        );
        assert!(
            project.join("dist/style.css").is_file(),
            "style output missing for {example}"
        );
        assert!(
            project.join("dist/palette.css").is_file(),
            "palette output missing for {example}"
        );
        assert!(
            project.join("dist/sitemap.xml").is_file(),
            "sitemap output missing for {example}"
        );
        assert!(
            project.join("dist/robots.txt").is_file(),
            "robots output missing for {example}"
        );
        assert!(
            project.join("dist/404.html").is_file(),
            "404 output missing for {example}"
        );
    }
}

#[test]
fn docs_site_builds_successfully() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = repo_root.join("site");
    let temp = tempdir().expect("tempdir should be created");
    let project = temp.path().join("site");
    copy_dir_recursive_filtered(&source, &project);

    let output = run_cli(&project, &["build"]);
    assert!(
        output.status.success(),
        "docs site build failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(project.join("dist/index.html").is_file());
    assert!(project.join("dist/guides/index.html").is_file());
    assert!(
        project
            .join("dist/guides/getting-started/index.html")
            .is_file()
    );
    assert!(project.join("dist/reference/cli/index.html").is_file());
    assert!(project.join("dist/examples/index.html").is_file());
    assert!(project.join("dist/roadmap/index.html").is_file());
    assert!(project.join("dist/style.css").is_file());
    assert!(project.join("dist/palette.css").is_file());
    assert!(project.join("dist/search-index.json").is_file());
    assert!(project.join("dist/sitemap.xml").is_file());
    assert!(project.join("dist/robots.txt").is_file());
    assert!(project.join("dist/404.html").is_file());

    let index_html =
        fs::read_to_string(project.join("dist/index.html")).expect("index html should exist");
    assert!(index_html.contains("Rustipo Docs"));
    assert!(index_html.contains("Build the docs site"));

    let cli_html = fs::read_to_string(project.join("dist/reference/cli/index.html"))
        .expect("cli page should exist");
    assert!(cli_html.contains("rustipo check"));
    assert!(cli_html.contains("Theme And Palette Commands"));
}
