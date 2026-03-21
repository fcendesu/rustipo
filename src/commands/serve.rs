use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use walkdir::WalkDir;

pub fn run(host: &str, port: u16, watch: bool) -> Result<()> {
    let addr = format_addr(host, port);
    let live_reload_version = if watch {
        Some(Arc::new(AtomicU64::new(0)))
    } else {
        None
    };

    if watch {
        println!("Watch mode enabled");
        let start = Instant::now();
        crate::commands::build::build_site_for_preview()?;
        println!(
            "Initial build completed in {}",
            format_duration(start.elapsed())
        );
        if let Some(version) = &live_reload_version {
            spawn_watch_thread(Arc::clone(version))?;
        }
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to initialize async runtime for local server")?;

    runtime.block_on(crate::server::serve_dist(
        "dist",
        &addr,
        live_reload_version,
    ))
}

fn format_addr(host: &str, port: u16) -> String {
    format!("{host}:{port}")
}

fn spawn_watch_thread(live_reload_version: Arc<AtomicU64>) -> Result<()> {
    let watch_paths = watch_paths();
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let _ = tx.send(res);
        },
        Config::default(),
    )
    .context("failed to initialize file watcher")?;

    for path in &watch_paths {
        if path.exists() {
            watcher
                .watch(path, RecursiveMode::Recursive)
                .with_context(|| format!("failed to watch path: {}", path.display()))?;
        }
    }

    thread::spawn(move || {
        let _watcher = watcher;
        let debounce = Duration::from_millis(300);
        let mut last_fingerprint = compute_watch_fingerprint(&watch_paths).ok();

        loop {
            match rx.recv() {
                Ok(Ok(_event)) => {
                    loop {
                        match rx.recv_timeout(debounce) {
                            Ok(Ok(_)) => continue,
                            Ok(Err(err)) => {
                                eprintln!("Watch error: {err}");
                                continue;
                            }
                            Err(RecvTimeoutError::Timeout) => break,
                            Err(RecvTimeoutError::Disconnected) => return,
                        }
                    }

                    let current_fingerprint = match compute_watch_fingerprint(&watch_paths) {
                        Ok(value) => value,
                        Err(err) => {
                            eprintln!("Watch fingerprint error: {err}");
                            continue;
                        }
                    };
                    if last_fingerprint == Some(current_fingerprint) {
                        continue;
                    }

                    println!("Change detected. Rebuilding...");
                    let start = Instant::now();
                    match crate::commands::build::build_site_for_preview_quiet() {
                        Ok(_) => {
                            last_fingerprint = Some(current_fingerprint);
                            live_reload_version.fetch_add(1, Ordering::SeqCst);
                            println!("Rebuild completed in {}", format_duration(start.elapsed()));
                        }
                        Err(err) => eprintln!(
                            "Rebuild failed in {}: {err}",
                            format_duration(start.elapsed())
                        ),
                    }
                }
                Ok(Err(err)) => eprintln!("Watch error: {err}"),
                Err(_) => return,
            }
        }
    });

    Ok(())
}

fn compute_watch_fingerprint(paths: &[std::path::PathBuf]) -> Result<u64> {
    let mut file_paths = Vec::new();

    for path in paths {
        if path.is_file() {
            file_paths.push(path.clone());
            continue;
        }
        if !path.is_dir() {
            continue;
        }

        for entry in WalkDir::new(path) {
            let entry = entry.with_context(|| format!("failed to walk: {}", path.display()))?;
            if entry.file_type().is_file() {
                file_paths.push(entry.path().to_path_buf());
            }
        }
    }

    file_paths.sort();

    let mut hasher = DefaultHasher::new();
    for path in &file_paths {
        path.hash(&mut hasher);
        let bytes = fs::read(path)
            .with_context(|| format!("failed to read watched file: {}", path.display()))?;
        bytes.hash(&mut hasher);
    }

    Ok(hasher.finish())
}

fn watch_paths() -> Vec<std::path::PathBuf> {
    vec![
        Path::new("content").to_path_buf(),
        Path::new("themes").to_path_buf(),
        Path::new("static").to_path_buf(),
        Path::new("config.toml").to_path_buf(),
    ]
}

fn format_duration(duration: Duration) -> String {
    let millis = duration.as_millis();
    if millis < 1_000 {
        return format!("{millis}ms");
    }

    let seconds = duration.as_secs_f64();
    format!("{seconds:.2}s")
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::time::Duration;

    use std::fs;

    use tempfile::tempdir;

    use super::{compute_watch_fingerprint, format_addr, format_duration, watch_paths};

    #[test]
    fn formats_host_and_port() {
        assert_eq!(format_addr("127.0.0.1", 3000), "127.0.0.1:3000");
        assert_eq!(format_addr("0.0.0.0", 8080), "0.0.0.0:8080");
    }

    #[test]
    fn includes_expected_watch_paths() {
        let paths = watch_paths();
        assert!(paths.contains(&Path::new("content").to_path_buf()));
        assert!(paths.contains(&Path::new("themes").to_path_buf()));
        assert!(paths.contains(&Path::new("static").to_path_buf()));
        assert!(paths.contains(&Path::new("config.toml").to_path_buf()));
    }

    #[test]
    fn fingerprint_changes_only_when_file_content_changes() {
        let dir = tempdir().expect("tempdir should be created");
        let content_dir = dir.path().join("content");
        fs::create_dir_all(&content_dir).expect("content dir should be created");
        let markdown = content_dir.join("index.md");
        fs::write(&markdown, "# Home").expect("markdown should be written");

        let paths = vec![content_dir];
        let first = compute_watch_fingerprint(&paths).expect("fingerprint should be computed");

        // Rewrite same content: fingerprint should stay stable.
        fs::write(&markdown, "# Home").expect("markdown should be rewritten");
        let second = compute_watch_fingerprint(&paths).expect("fingerprint should be computed");
        assert_eq!(first, second);

        // Change content: fingerprint should change.
        fs::write(&markdown, "# Home Updated").expect("markdown should be updated");
        let third = compute_watch_fingerprint(&paths).expect("fingerprint should be computed");
        assert_ne!(second, third);
    }

    #[test]
    fn formats_duration_for_millis_and_seconds() {
        assert_eq!(format_duration(Duration::from_millis(140)), "140ms");
        assert_eq!(format_duration(Duration::from_millis(1450)), "1.45s");
    }
}
