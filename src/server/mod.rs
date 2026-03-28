use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::{Context, Result, bail};
use axum::Json;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use axum::http::{HeaderValue, Response, StatusCode};
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Serialize;
use tower_http::services::ServeDir;

const MAX_HTML_BYTES: usize = 10 * 1024 * 1024;
const LIVE_RELOAD_SNIPPET: &str = r#"<script>
(() => {
  const endpoint = "/__rustipo_reload";
  let current = null;
  async function tick() {
    try {
      const response = await fetch(endpoint, { cache: "no-store" });
      if (!response.ok) return;
      const payload = await response.json();
      if (current === null) {
        current = payload.version;
        return;
      }
      if (payload.version !== current) {
        window.location.reload();
      }
    } catch (_) {}
  }
  setInterval(tick, 1000);
  tick();
})();
</script>"#;

pub async fn serve_dist(
    dist_dir: impl AsRef<Path>,
    addr: &str,
    live_reload_version: Option<Arc<AtomicU64>>,
    base_path: &str,
) -> Result<()> {
    let dist_dir = dist_dir.as_ref();
    if !dist_dir.is_dir() {
        bail!(
            "build output directory not found: {} (run `rustipo build` first)",
            dist_dir.display()
        );
    }

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind local server at {addr}"))?;

    let app = build_app(dist_dir, live_reload_version, base_path);
    let public_url = if base_path == "/" {
        format!("http://{addr}")
    } else {
        format!("http://{addr}{}/", base_path.trim_end_matches('/'))
    };
    let root_note = if base_path == "/" {
        String::new()
    } else {
        format!(" (root also available at http://{addr}/)")
    };
    println!("Serving dist/ at {public_url}{root_note}");
    axum::serve(listener, app)
        .await
        .context("local server failed unexpectedly")?;

    Ok(())
}

fn build_app(
    dist_dir: &Path,
    live_reload_version: Option<Arc<AtomicU64>>,
    base_path: &str,
) -> Router {
    let base_path = normalize_base_path(base_path);
    let mut app = Router::new().fallback_service(ServeDir::new(dist_dir));
    if base_path != "/" {
        let redirect_target = format!("{}/", base_path.trim_end_matches('/'));
        let redirect_target_for_root = redirect_target.clone();
        app = app
            .route(
                "/",
                get(move || {
                    let target = redirect_target_for_root.clone();
                    async move { axum::response::Redirect::temporary(&target) }
                }),
            )
            .nest_service(&base_path, ServeDir::new(dist_dir));
    }

    if let Some(version) = live_reload_version {
        let version_for_handler = Arc::clone(&version);
        app = app
            .route(
                "/__rustipo_reload",
                get(move || {
                    let version = Arc::clone(&version_for_handler);
                    async move {
                        Json(LiveReloadPayload {
                            version: version.load(Ordering::SeqCst),
                        })
                    }
                }),
            )
            .layer(middleware::map_response(inject_live_reload_script));
    }

    app
}

#[derive(Serialize)]
struct LiveReloadPayload {
    version: u64,
}

async fn inject_live_reload_script(mut response: Response<Body>) -> Response<Body> {
    if !is_html_response(&response) {
        return response;
    }

    let (parts, body) = response.into_parts();
    let Ok(bytes) = to_bytes(body, MAX_HTML_BYTES).await else {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(
                "Failed to read HTML response for live reload injection",
            ))
            .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response());
    };

    let Ok(mut html) = String::from_utf8(bytes.to_vec()) else {
        return Response::from_parts(parts, Body::from(bytes));
    };

    if html.contains("/__rustipo_reload") {
        return Response::from_parts(parts, Body::from(html));
    }

    if let Some(pos) = html.rfind("</body>") {
        html.insert_str(pos, LIVE_RELOAD_SNIPPET);
    } else {
        html.push_str(LIVE_RELOAD_SNIPPET);
    }

    let len = html.len();
    response = Response::from_parts(parts, Body::from(html));
    if let Ok(value) = HeaderValue::from_str(&len.to_string()) {
        let _ = response.headers_mut().insert(CONTENT_LENGTH, value);
    }
    response
}

fn is_html_response(response: &Response<Body>) -> bool {
    response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.starts_with("text/html"))
}

fn normalize_base_path(base_path: &str) -> String {
    let trimmed = base_path.trim();
    if trimmed.is_empty() || trimmed == "/" {
        return "/".to_string();
    }

    let without_slashes = trimmed.trim_matches('/');
    format!("/{}", without_slashes)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::body::to_bytes;
    use axum::http::Request;
    use axum::http::Response;
    use axum::http::StatusCode;
    use axum::http::Uri;
    use axum::http::header::CONTENT_TYPE;
    use tempfile::tempdir;
    use tower::util::ServiceExt;

    use super::{build_app, inject_live_reload_script, normalize_base_path, serve_dist};

    #[tokio::test]
    async fn fails_if_dist_directory_is_missing() {
        let dir = tempdir().expect("tempdir should be created");
        let dist = dir.path().join("missing-dist");

        let error = serve_dist(&dist, "127.0.0.1:0", None, "/")
            .await
            .expect_err("missing dist dir should fail");

        assert!(
            error
                .to_string()
                .contains("build output directory not found"),
            "unexpected error: {error}"
        );
    }

    #[tokio::test]
    async fn fails_if_dist_directory_is_missing_at_subpath() {
        let dir = tempdir().expect("tempdir should be created");
        let dist = dir.path().join("missing-dist");

        let error = serve_dist(&dist, "127.0.0.1:0", None, "/docs")
            .await
            .expect_err("missing dist dir should fail");

        assert!(
            error
                .to_string()
                .contains("build output directory not found"),
            "unexpected error: {error}"
        );
    }

    #[tokio::test]
    async fn injects_reload_snippet_into_html_response() {
        let response = Response::builder()
            .header(CONTENT_TYPE, "text/html; charset=utf-8")
            .body(Body::from("<html><body><h1>Hello</h1></body></html>"))
            .expect("response should build");

        let response = inject_live_reload_script(response).await;
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body should read");
        let html = String::from_utf8(body.to_vec()).expect("body should be utf8");

        assert!(html.contains("/__rustipo_reload"));
        assert!(html.contains("<h1>Hello</h1>"));
    }

    #[tokio::test]
    async fn does_not_inject_into_non_html_response() {
        let response = Response::builder()
            .header(CONTENT_TYPE, "application/javascript")
            .body(Body::from("console.log('hi');"))
            .expect("response should build");

        let response = inject_live_reload_script(response).await;
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body should read");
        let text = String::from_utf8(body.to_vec()).expect("body should be utf8");

        assert_eq!(text, "console.log('hi');");
    }

    #[tokio::test]
    async fn serves_dist_files_from_configured_base_path() {
        let dir = tempdir().expect("tempdir should be created");
        let dist = dir.path().join("dist");
        std::fs::create_dir_all(&dist).expect("dist should exist");
        std::fs::write(dist.join("style.css"), "body{color:red;}").expect("style should exist");

        let app = build_app(&dist, None, "/rustipo");
        let response = app
            .oneshot(
                Request::builder()
                    .uri(Uri::from_static("/rustipo/style.css"))
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body should read");
        assert_eq!(&body[..], b"body{color:red;}");
    }

    #[tokio::test]
    async fn redirects_root_to_configured_base_path() {
        let dir = tempdir().expect("tempdir should be created");
        let dist = dir.path().join("dist");
        std::fs::create_dir_all(&dist).expect("dist should exist");

        let app = build_app(&dist, None, "/rustipo");
        let response = app
            .oneshot(
                Request::builder()
                    .uri(Uri::from_static("/"))
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
        assert_eq!(
            response
                .headers()
                .get(axum::http::header::LOCATION)
                .and_then(|value| value.to_str().ok()),
            Some("/rustipo/")
        );
    }

    #[test]
    fn normalizes_base_path_input() {
        assert_eq!(normalize_base_path("/"), "/");
        assert_eq!(normalize_base_path(""), "/");
        assert_eq!(normalize_base_path("/rustipo/"), "/rustipo");
        assert_eq!(normalize_base_path("rustipo"), "/rustipo");
    }
}
