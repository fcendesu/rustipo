mod cli;
mod commands;
mod config;
mod content;
mod output;
mod render;
mod server;
mod theme;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::New { site_name } => commands::new::run(&site_name),
        cli::Commands::Build => commands::build::run(),
        cli::Commands::Serve { host, port, watch } => commands::serve::run(&host, port, watch),
        cli::Commands::Theme { command } => match command {
            cli::ThemeCommands::List => commands::theme::list(),
            cli::ThemeCommands::Install { source, name } => {
                commands::theme::install(&source, name.as_deref())
            }
        },
    }
}
