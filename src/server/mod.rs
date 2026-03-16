use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use tiny_http::{Header, Response, Server, StatusCode};

pub fn serve_dist(dist_dir: impl AsRef<Path>, addr: &str) -> Result<()> {
    let dist_dir = dist_dir.as_ref();
    if !dist_dir.is_dir() {
        bail!(
            "build output directory not found: {} (run `rustipo build` first)",
            dist_dir.display()
        );
    }

    let server = Server::http(addr)
        .map_err(|e| anyhow::anyhow!("failed to bind local server at {addr}: {e}"))?;
    println!("Serving dist/ at http://{addr}");

    for request in server.incoming_requests() {
        let target = match resolve_request_path(dist_dir, request.url()) {
            Some(path) if path.is_file() => path,
            _ => {
                let _ = request.respond(
                    Response::from_string("Not Found")
                        .with_status_code(StatusCode(404))
                        .with_header(content_type_header("text/plain; charset=utf-8")),
                );
                continue;
            }
        };

        match fs::read(&target) {
            Ok(bytes) => {
                let content_type = content_type_for_path(&target);
                let _ = request.respond(
                    Response::from_data(bytes).with_header(content_type_header(content_type)),
                );
            }
            Err(_) => {
                let _ = request.respond(
                    Response::from_string("Internal Server Error")
                        .with_status_code(StatusCode(500))
                        .with_header(content_type_header("text/plain; charset=utf-8")),
                );
            }
        }
    }

    Ok(())
}

fn resolve_request_path(dist_dir: &Path, url: &str) -> Option<PathBuf> {
    let clean_url = url.split('?').next().unwrap_or(url);
    let clean_url = clean_url.trim_start_matches('/');

    let rel = if clean_url.is_empty() {
        PathBuf::from("index.html")
    } else {
        let path = PathBuf::from(clean_url);
        if path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            return None;
        }

        if path.extension().is_some() {
            path
        } else {
            path.join("index.html")
        }
    };

    Some(dist_dir.join(rel))
}

fn content_type_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    }
}

fn content_type_header(value: &str) -> Header {
    Header::from_bytes(b"Content-Type", value.as_bytes()).expect("valid content-type header")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::resolve_request_path;

    #[test]
    fn resolves_pretty_url_to_index_file() {
        let dist = Path::new("dist");
        assert_eq!(
            resolve_request_path(dist, "/about/").expect("about should resolve"),
            dist.join("about/index.html")
        );
        assert_eq!(
            resolve_request_path(dist, "/").expect("root should resolve"),
            dist.join("index.html")
        );
        assert_eq!(
            resolve_request_path(dist, "/style.css").expect("asset should resolve"),
            dist.join("style.css")
        );
    }

    #[test]
    fn blocks_parent_directory_traversal() {
        let dist = Path::new("dist");
        assert!(resolve_request_path(dist, "/../secret.txt").is_none());
    }
}
