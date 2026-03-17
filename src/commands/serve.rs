use anyhow::{Context, Result};

pub fn run(host: &str, port: u16) -> Result<()> {
    let addr = format_addr(host, port);
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("failed to initialize async runtime for local server")?;

    runtime.block_on(crate::server::serve_dist("dist", &addr))
}

fn format_addr(host: &str, port: u16) -> String {
    format!("{host}:{port}")
}

#[cfg(test)]
mod tests {
    use super::format_addr;

    #[test]
    fn formats_host_and_port() {
        assert_eq!(format_addr("127.0.0.1", 3000), "127.0.0.1:3000");
        assert_eq!(format_addr("0.0.0.0", 8080), "0.0.0.0:8080");
    }
}
