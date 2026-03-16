use std::path::Path;

use anyhow::{Context, Result, bail};
use axum::Router;
use tower_http::services::ServeDir;

pub async fn serve_dist(dist_dir: impl AsRef<Path>, addr: &str) -> Result<()> {
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

    let app = Router::new().fallback_service(ServeDir::new(dist_dir));

    println!("Serving dist/ at http://{addr}");
    axum::serve(listener, app)
        .await
        .context("local server failed unexpectedly")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::serve_dist;

    #[tokio::test]
    async fn fails_if_dist_directory_is_missing() {
        let dir = tempdir().expect("tempdir should be created");
        let dist = dir.path().join("missing-dist");

        let error = serve_dist(&dist, "127.0.0.1:0")
            .await
            .expect_err("missing dist dir should fail");

        assert!(
            error
                .to_string()
                .contains("build output directory not found"),
            "unexpected error: {error}"
        );
    }
}
