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
