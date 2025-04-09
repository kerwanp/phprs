mod lsp;
mod parse;

use anyhow::Result;
use clap::{Parser, Subcommand, command};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Lsp(lsp::LspCommand),
    Parse(parse::ParseCommand),
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Lsp(cmd) => lsp::run(cmd).await,
        Command::Parse(cmd) => parse::run(cmd),
    }
}
