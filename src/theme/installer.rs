use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};

pub fn install_theme(
    project_root: impl AsRef<Path>,
    source: &str,
    name_override: Option<&str>,
) -> Result<String> {
    let project_root = project_root.as_ref();
    let source = ThemeSource::parse(source)?;

    let directory_name = match name_override {
        Some(value) => sanitize_directory_name(value)?,
        None => sanitize_directory_name(&source.default_directory_name)?,
    };

    let themes_dir = project_root.join("themes");
    fs::create_dir_all(&themes_dir).with_context(|| {
        format!(
            "failed to create themes directory: {}",
            themes_dir.display()
        )
    })?;

    let destination = themes_dir.join(&directory_name);
    if destination.exists() {
        bail!(
            "theme destination already exists: {} (use --name to install under a different directory)",
            destination.display()
        );
    }

    clone_repository(&source.clone_url, &destination)?;
    let git_dir = destination.join(".git");
    if git_dir.exists() {
        fs::remove_dir_all(&git_dir)
            .with_context(|| format!("failed to remove git metadata: {}", git_dir.display()))?;
    }

    if let Err(error) = crate::theme::loader::load_active_theme(project_root, &directory_name) {
        let _ = fs::remove_dir_all(&destination);
        return Err(error).context(format!(
            "installed theme is invalid: {}",
            destination.display()
        ));
    }

    Ok(directory_name)
}

fn clone_repository(clone_url: &str, destination: &Path) -> Result<()> {
    let status = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--filter=blob:none")
        .arg(clone_url)
        .arg(destination)
        .status()
        .context("failed to execute git; ensure git is installed and available in PATH")?;

    if !status.success() {
        bail!("git clone failed for source: {clone_url}");
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct ThemeSource {
    clone_url: String,
    default_directory_name: String,
}

impl ThemeSource {
    fn parse(input: &str) -> Result<Self> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            bail!("theme source cannot be empty");
        }

        if trimmed.contains("://") {
            return parse_url_source(trimmed);
        }

        let path_candidate = PathBuf::from(trimmed);
        if path_candidate.exists() {
            let directory = path_candidate
                .file_name()
                .and_then(|part| part.to_str())
                .map(str::to_string)
                .context("failed to infer directory name from local theme path")?;
            return Ok(ThemeSource {
                clone_url: path_candidate.to_string_lossy().to_string(),
                default_directory_name: directory,
            });
        }

        parse_github_shorthand(trimmed)
    }
}

fn parse_url_source(input: &str) -> Result<ThemeSource> {
    if !input.contains("github.com/") {
        bail!("theme URL must be a GitHub repository URL");
    }

    let without_query = input.split('?').next().unwrap_or(input);
    let without_fragment = without_query.split('#').next().unwrap_or(without_query);
    let normalized_url = without_fragment.trim_end_matches('/').to_string();

    let segments = normalized_url
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let repo_segment = segments
        .last()
        .copied()
        .context("failed to infer repository name from GitHub URL")?;
    let repo_name = repo_segment.trim_end_matches(".git").to_string();
    if repo_name.is_empty() {
        bail!("failed to infer repository name from GitHub URL");
    }

    Ok(ThemeSource {
        clone_url: if normalized_url.ends_with(".git") {
            normalized_url
        } else {
            format!("{normalized_url}.git")
        },
        default_directory_name: repo_name,
    })
}

fn parse_github_shorthand(input: &str) -> Result<ThemeSource> {
    let parts = input.split('/').collect::<Vec<_>>();
    if parts.len() != 2 || parts.iter().any(|part| part.is_empty()) {
        bail!("theme source must be 'owner/repo', a GitHub URL, or a local git path");
    }

    let repo = parts[1].trim_end_matches(".git");
    if repo.is_empty() {
        bail!("theme repository name cannot be empty");
    }

    Ok(ThemeSource {
        clone_url: format!("https://github.com/{}/{}.git", parts[0], repo),
        default_directory_name: repo.to_string(),
    })
}

fn sanitize_directory_name(input: &str) -> Result<String> {
    let trimmed = input.trim().trim_end_matches(".git");
    if trimmed.is_empty() {
        bail!("theme directory name cannot be empty");
    }

    let mut output = String::with_capacity(trimmed.len());
    let mut previous_dash = false;
    for ch in trimmed.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            output.push('-');
            previous_dash = true;
        }
    }

    let cleaned = output.trim_matches('-').to_string();
    if cleaned.is_empty() {
        bail!("theme directory name must contain at least one ASCII letter or digit");
    }

    Ok(cleaned)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{ThemeSource, sanitize_directory_name};

    #[test]
    fn parses_github_shorthand_source() {
        let parsed = ThemeSource::parse("fcendesu/rustipo-theme").expect("source should parse");
        assert_eq!(
            parsed.clone_url,
            "https://github.com/fcendesu/rustipo-theme.git"
        );
        assert_eq!(parsed.default_directory_name, "rustipo-theme");
    }

    #[test]
    fn parses_github_url_source() {
        let parsed = ThemeSource::parse("https://github.com/fcendesu/rustipo-theme")
            .expect("source should parse");
        assert_eq!(
            parsed.clone_url,
            "https://github.com/fcendesu/rustipo-theme.git"
        );
        assert_eq!(parsed.default_directory_name, "rustipo-theme");
    }

    #[test]
    fn parses_local_path_source() {
        let dir = tempdir().expect("tempdir should be created");
        let repo_dir = dir.path().join("local-theme");
        fs::create_dir_all(&repo_dir).expect("local repo should be created");

        let parsed =
            ThemeSource::parse(repo_dir.to_string_lossy().as_ref()).expect("source should parse");
        assert!(parsed.clone_url.contains("local-theme"));
        assert_eq!(parsed.default_directory_name, "local-theme");
    }

    #[test]
    fn sanitizes_directory_name_to_kebab_case() {
        let cleaned = sanitize_directory_name("Tokyo Night++").expect("name should sanitize");
        assert_eq!(cleaned, "tokyo-night");
    }
}
