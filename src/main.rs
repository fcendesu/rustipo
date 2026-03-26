mod cli;
mod commands;
mod config;
mod content;
mod output;
mod palette;
mod render;
mod server;
mod theme;
mod url;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::New { site_name } => commands::new::run(&site_name),
        cli::Commands::Check => commands::check::run(),
        cli::Commands::Build => commands::build::run(),
        cli::Commands::Dev { host, port } => commands::dev::run(&host, port),
        cli::Commands::Serve { host, port, watch } => commands::serve::run(&host, port, watch),
        cli::Commands::Theme { command } => match command {
            cli::ThemeCommands::List => commands::theme::list(),
            cli::ThemeCommands::Install { source, name } => {
                commands::theme::install(&source, name.as_deref())
            }
        },
        cli::Commands::Palette { command } => match command {
            cli::PaletteCommands::List => commands::palette::list(),
            cli::PaletteCommands::Use { id } => commands::palette::use_palette(&id),
        },
        cli::Commands::Deploy { command } => match command {
            cli::DeployCommands::GithubPages { force } => commands::deploy::github_pages(force),
            cli::DeployCommands::CloudflarePages { force } => {
                commands::deploy::cloudflare_pages(force)
            }
        },
    }
}
