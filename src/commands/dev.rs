use anyhow::Result;

pub fn run(host: &str, port: u16) -> Result<()> {
    crate::commands::serve::run(host, port, true)
}
