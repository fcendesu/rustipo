use anyhow::Result;

pub fn run() -> Result<()> {
    let addr = "127.0.0.1:3000";
    crate::server::serve_dist("dist", addr)
}
