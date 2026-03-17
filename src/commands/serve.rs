use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

pub fn run(host: &str, port: u16, watch: bool) -> Result<()> {
    let addr = format_addr(host, port);
    let live_reload_version = if watch {
        Some(Arc::new(AtomicU64::new(0)))
    } else {
        None
    };

    if watch {
        println!("Watch mode enabled");
        crate::commands::build::build_site()?;
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
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let _ = tx.send(res);
        },
        Config::default(),
    )
    .context("failed to initialize file watcher")?;

    for path in watch_paths() {
        if path.exists() {
            watcher
                .watch(&path, RecursiveMode::Recursive)
                .with_context(|| format!("failed to watch path: {}", path.display()))?;
        }
    }

    thread::spawn(move || {
        let _watcher = watcher;
        let debounce = Duration::from_millis(300);

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

                    println!("Change detected. Rebuilding...");
                    match crate::commands::build::build_site() {
                        Ok(_) => {
                            live_reload_version.fetch_add(1, Ordering::SeqCst);
                            println!("Rebuild completed");
                        }
                        Err(err) => eprintln!("Rebuild failed: {err}"),
                    }
                }
                Ok(Err(err)) => eprintln!("Watch error: {err}"),
                Err(_) => return,
            }
        }
    });

    Ok(())
}

fn watch_paths() -> Vec<std::path::PathBuf> {
    vec![
        Path::new("content").to_path_buf(),
        Path::new("themes").to_path_buf(),
        Path::new("static").to_path_buf(),
        Path::new("config.toml").to_path_buf(),
    ]
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{format_addr, watch_paths};

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
}
