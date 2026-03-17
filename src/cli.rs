use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "rustipo",
    version,
    about = "Portfolio-first static site generator"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new site scaffold
    New { site_name: String },
    /// Build the site into dist/
    Build,
    /// Serve the built site locally
    Serve {
        /// Host address to bind the local server to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Port to bind the local server to
        #[arg(long, default_value_t = 3000)]
        port: u16,
    },
    /// Theme-related commands
    Theme {
        #[command(subcommand)]
        command: ThemeCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum ThemeCommands {
    /// List available themes
    List,
}
