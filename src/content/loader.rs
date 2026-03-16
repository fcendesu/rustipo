use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use walkdir::WalkDir;

pub fn discover_markdown_files(content_dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let content_dir = content_dir.as_ref();
    if !content_dir.exists() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    for entry in WalkDir::new(content_dir) {
        let entry = entry.with_context(|| {
            format!(
                "failed while walking content directory: {}",
                content_dir.display()
            )
        })?;

        if !entry.file_type().is_file() {
            continue;
        }

        if entry.path().extension().and_then(|ext| ext.to_str()) == Some("md") {
            files.push(entry.into_path());
        }
    }

    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::discover_markdown_files;

    #[test]
    fn discovers_markdown_files_recursively() {
        let dir = tempdir().expect("tempdir should be created");
        let root = dir.path();

        fs::create_dir_all(root.join("content/blog")).expect("blog dir should be created");
        fs::create_dir_all(root.join("content/projects"))
            .expect("projects dir should be created");

        fs::write(root.join("content/index.md"), "# Home").expect("index should be written");
        fs::write(root.join("content/blog/post.md"), "# Post").expect("post should be written");
        fs::write(root.join("content/projects/app.md"), "# App").expect("app should be written");
        fs::write(root.join("content/readme.txt"), "nope").expect("txt should be written");

        let files = discover_markdown_files(root.join("content")).expect("discovery should pass");

        assert_eq!(files.len(), 3);
        assert!(files.iter().any(|p| p.ends_with("index.md")));
        assert!(files.iter().any(|p| p.ends_with("post.md")));
        assert!(files.iter().any(|p| p.ends_with("app.md")));
    }
}
