mod cli;
mod commands;
mod config;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::New { site_name } => commands::new::run(&site_name),
        cli::Commands::Build => commands::build::run(),
        cli::Commands::Serve => commands::serve::run(),
        cli::Commands::Theme { command } => match command {
            cli::ThemeCommands::List => commands::theme::list(),
        },
    }
}
