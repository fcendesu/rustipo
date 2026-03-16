use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let addr = "127.0.0.1:3000";
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to initialize async runtime for local server")?;

    runtime.block_on(crate::server::serve_dist("dist", addr))
}
