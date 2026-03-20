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
    /// Build, serve, and watch the site during development
    Dev {
        /// Host address to bind the local server to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Port to bind the local server to
        #[arg(long, default_value_t = 3000)]
        port: u16,
    },
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
    /// Update config.toml to use a palette
    Use {
        /// Selectable palette ID
        id: String,
    },
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

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{Cli, Commands, PaletteCommands};

    #[test]
    fn parses_dev_command_with_host_and_port() {
        let cli = Cli::parse_from(["rustipo", "dev", "--host", "0.0.0.0", "--port", "4000"]);

        match cli.command {
            Commands::Dev { host, port } => {
                assert_eq!(host, "0.0.0.0");
                assert_eq!(port, 4000);
            }
            other => panic!("expected dev command, got {other:?}"),
        }
    }

    #[test]
    fn parses_palette_use_command() {
        let cli = Cli::parse_from(["rustipo", "palette", "use", "catppuccin-mocha"]);

        match cli.command {
            Commands::Palette { command } => match command {
                PaletteCommands::Use { id } => assert_eq!(id, "catppuccin-mocha"),
                other => panic!("expected palette use command, got {other:?}"),
            },
            other => panic!("expected palette command, got {other:?}"),
        }
    }
}
