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
        /// Watch files and rebuild on changes
        #[arg(long, default_value_t = false)]
        watch: bool,
    },
    /// Theme-related commands
    Theme {
        #[command(subcommand)]
        command: ThemeCommands,
    },
    /// Palette-related commands
    Palette {
        #[command(subcommand)]
        command: PaletteCommands,
    },
    /// Deployment helper commands
    Deploy {
        #[command(subcommand)]
        command: DeployCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum ThemeCommands {
    /// List available themes
    List,
    /// Install a theme from GitHub (owner/repo or URL) or a local git path
    Install {
        /// GitHub source (owner/repo or URL) or local git repository path
        source: String,
        /// Override install directory name under themes/
        #[arg(long)]
        name: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum PaletteCommands {
    /// List available palettes
    List,
}

#[derive(Debug, Subcommand)]
pub enum DeployCommands {
    /// Generate GitHub Pages deployment workflow
    GithubPages {
        /// Overwrite existing workflow file if present
        #[arg(long, default_value_t = false)]
        force: bool,
    },
}
